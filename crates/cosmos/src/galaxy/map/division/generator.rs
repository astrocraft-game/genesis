use crate::internal::*;
use crate::prelude::*;
use std::collections::HashMap;

impl GalacticMapDivision {
    pub fn generate(
        index: SpaceCoordinates,
        level: u8,
        parent_division_level: &GalacticMapDivisionLevel,
        galaxy: &Galaxy,
    ) -> Self {
        let mut division = Self {
            name: "Sector".into(),
            region: GalacticRegion::Multiple,
            level,
            x: (index.x % parent_division_level.x_subdivisions as i64) as u8,
            y: (index.y % parent_division_level.y_subdivisions as i64) as u8,
            z: (index.z % parent_division_level.z_subdivisions as i64) as u8,
            index,
            size: SpaceCoordinates::new(-1, -1, -1),
        };
        division.region = get_region(&mut division, galaxy);
        division.name = generate_division_name(index, level, &division.region);
        division
    }
}

fn generate_division_name(
    index: SpaceCoordinates,
    level: u8,
    region: &GalacticRegion,
) -> Rc<str> {
    let region_prefix = match region {
        GalacticRegion::Core => "Core",
        GalacticRegion::Nucleus => "Nucleus",
        GalacticRegion::Bulge => "Bulge",
        GalacticRegion::Bar => "Bar",
        GalacticRegion::Arm => "Arm",
        GalacticRegion::Disk => "Disk",
        GalacticRegion::Ellipse => "Sector",
        GalacticRegion::Halo => "Halo",
        GalacticRegion::Aura => "Aura",
        GalacticRegion::Void => "Void",
        GalacticRegion::GlobularCluster => "GC",
        GalacticRegion::OpenCluster => "OC",
        GalacticRegion::Association => "Assoc",
        GalacticRegion::Stream => "Stream",
        GalacticRegion::Exile => "Exile",
        GalacticRegion::Multiple => "Sector",
    };
    format!("{}-L{}-{}.{}.{}", region_prefix, level, index.x, index.y, index.z).into()
}

fn get_region(division: &mut GalacticMapDivision, galaxy: &Galaxy) -> GalacticRegion {
    let start = division.get_top_left_up(galaxy);
    let size = division.get_size(galaxy);
    let half_size = size / SpaceCoordinates::new(2, 2, 2);
    let mut region_count = HashMap::new();

    for xi in 0..3 {
        let x = if xi == 0 {
            start.x
        } else if xi == 1 {
            half_size.x
        } else {
            size.x
        };
        for yi in 0..3 {
            let y = if yi == 0 {
                start.y
            } else if yi == 1 {
                half_size.y
            } else {
                size.y
            };
            for zi in 0..3 {
                let z = if zi == 0 {
                    start.z
                } else if zi == 1 {
                    half_size.z
                } else {
                    size.z
                };

                // Finds which region this point belongs to and remembers it
                let point_region = generate_region(SpaceCoordinates::new(x, y, z), galaxy);
                *region_count.entry(point_region).or_insert(0) += 1;
            }
        }
    }

    // If there was only one region in the whole division, set that division's region to it, otherwise use Multiple.
    if region_count.len() == 1 {
        let (region, _) = region_count.into_iter().next().unwrap();
        region
    } else {
        GalacticRegion::Multiple
    }
}

/// Returns the proper region for a given coordinate based on distance from center.
fn generate_region(coord: SpaceCoordinates, galaxy: &Galaxy) -> GalacticRegion {
    let (radius, height) = match galaxy.category {
        GalaxyCategory::Spiral(r, h) | GalaxyCategory::Lenticular(r, h) => (r as f64, h as f64),
        GalaxyCategory::Elliptical(r) | GalaxyCategory::DominantElliptical(r) => {
            (r as f64, r as f64)
        }
        GalaxyCategory::Irregular(x, y, z) => (x.max(y) as f64, z as f64),
        _ => return GalacticRegion::Void,
    };

    let center = galaxy.get_galactic_center();
    let dx = (coord.x - center.x) as f64;
    let dy = (coord.y - center.y) as f64;
    let dz = (coord.z - center.z) as f64;
    let planar_dist = (dx * dx + dy * dy).sqrt();
    let dist_ratio = planar_dist / radius;
    let z_ratio = dz.abs() / height.max(1.0);

    // Outside the galaxy
    if dist_ratio > 1.0 && z_ratio > 1.0 {
        return GalacticRegion::Void;
    }

    // For ellipticals, use simple distance-based regions
    if matches!(
        galaxy.category,
        GalaxyCategory::Elliptical(_) | GalaxyCategory::DominantElliptical(_)
    ) {
        if dist_ratio > 1.0 {
            // Check for tidal streams from interacting galaxies
            let has_tail = galaxy.special_traits.iter().any(|t| {
                matches!(t, GalaxySpecialTrait::Interacting | GalaxySpecialTrait::Tail)
            });
            return if has_tail && dist_ratio < 1.5 {
                let hash = ((coord.x.wrapping_mul(73) ^ coord.y.wrapping_mul(179)) % 100).unsigned_abs();
                if hash < 15 { GalacticRegion::Stream } else { GalacticRegion::Void }
            } else {
                GalacticRegion::Void
            };
        }
        let hash = ((coord.x.wrapping_mul(73) ^ coord.y.wrapping_mul(179) ^ coord.z.wrapping_mul(283)) % 1000).unsigned_abs();
        return if dist_ratio < 0.05 {
            GalacticRegion::Core
        } else if dist_ratio < 0.5 {
            if hash < 5 { GalacticRegion::GlobularCluster } else { GalacticRegion::Ellipse }
        } else {
            if hash < 3 { GalacticRegion::GlobularCluster } else { GalacticRegion::Halo }
        };
    }

    // For irregular galaxies
    if matches!(galaxy.category, GalaxyCategory::Irregular(_, _, _)) {
        if dist_ratio > 1.0 || z_ratio > 1.0 {
            return GalacticRegion::Void;
        }
        // Irregular galaxies can have open clusters near their core
        if dist_ratio < 0.3 {
            // Use coordinate hash for deterministic cluster placement
            let hash = ((coord.x.wrapping_mul(73) ^ coord.y.wrapping_mul(179) ^ coord.z.wrapping_mul(283)) % 100).unsigned_abs();
            if hash < 5 {
                return GalacticRegion::OpenCluster;
            }
        }
        return GalacticRegion::Aura;
    }

    // Spirals and lenticulars: layered regions
    if z_ratio > 2.0 {
        return GalacticRegion::Void;
    }
    if z_ratio > 1.0 {
        // Halo region can contain globular clusters
        let hash = ((coord.x.wrapping_mul(73) ^ coord.y.wrapping_mul(179) ^ coord.z.wrapping_mul(283)) % 1000).unsigned_abs();
        return if hash < 3 {
            GalacticRegion::GlobularCluster
        } else {
            GalacticRegion::Halo
        };
    }

    if dist_ratio < 0.03 {
        GalacticRegion::Nucleus
    } else if dist_ratio < 0.1 {
        GalacticRegion::Bulge
    } else if dist_ratio < 0.15
        && matches!(galaxy.sub_category, GalaxySubCategory::BarredSpiral)
    {
        GalacticRegion::Bar
    } else if dist_ratio <= 0.85 {
        // In a spiral, check if we're near an arm
        // Simplified: use angular position to determine arm vs disk
        let angle = dy.atan2(dx);
        let arm_count = match galaxy.sub_category {
            GalaxySubCategory::BarredSpiral | GalaxySubCategory::ClassicSpiral => 4,
            GalaxySubCategory::FlatSpiral => 2,
            _ => 3,
        };
        // Logarithmic spiral: arm_angle = b * ln(r/a), check proximity
        let arm_angle_spacing = std::f64::consts::TAU / arm_count as f64;
        let spiral_tightness = 0.3;
        let expected_angle = spiral_tightness * (dist_ratio * 10.0).ln();
        let angular_offset = ((angle - expected_angle) % arm_angle_spacing).abs();
        let arm_width = arm_angle_spacing * 0.3;

        if angular_offset < arm_width || angular_offset > (arm_angle_spacing - arm_width) {
            // Inside an arm - check for open clusters and associations
            let hash = ((coord.x.wrapping_mul(73) ^ coord.y.wrapping_mul(179) ^ coord.z.wrapping_mul(283)) % 100).unsigned_abs();
            if hash < 3 {
                GalacticRegion::OpenCluster
            } else if hash < 6 {
                GalacticRegion::Association
            } else {
                GalacticRegion::Arm
            }
        } else {
            GalacticRegion::Disk
        }
    } else if dist_ratio <= 1.0 {
        GalacticRegion::Halo
    } else {
        GalacticRegion::Void
    }
}

/// Returns true it the given point is within the area the given galaxy (that must be a spheroid).
fn is_within_sphere_in_non_equal_planes(
    coord: SpaceCoordinates,
    sizes: SpaceCoordinates,
    galaxy: &Galaxy,
) -> bool {
    let biggest_size = sizes.x.max(sizes.y).max(sizes.z);
    let scaled_point = SpaceCoordinates {
        x: coord.x * biggest_size / sizes.x,
        y: coord.y * biggest_size / sizes.y,
        z: coord.z * biggest_size / sizes.z,
    };
    let center = galaxy.get_galactic_center();

    is_within_sphere(scaled_point, center, biggest_size)
}

/// Returns true if the given point is within the area of the sphere whose center and radius are given in parameters.
fn is_within_sphere(point: SpaceCoordinates, center: SpaceCoordinates, radius: i64) -> bool {
    i64::pow(point.x - center.x, 2)
        + i64::pow(point.y - center.y, 2)
        + i64::pow(point.z - center.z, 2)
        <= i64::pow(radius, 2)
}
