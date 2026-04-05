//! Geology: re-exports for summary mineral/volcanic types, and tile-level
//! plate tectonics / elevation grid generation.

use crate::grid::{BoundaryKind, GridResolution, Plate, PlateKind, SurfaceGrid};
use crate::types::PlanetSimulationInput;
use seeded_dice_roller::SeededDiceRoller;

pub use crate::types::{
    Mineral, MineralDeposit, MineralDiversity, MineralEvolutionStage, ResourceAbundance,
    SeismicProfile, SeismicitySource, VolcanicProfile, VolcanoType,
};

/// Continental-plate baseline elevation, metres.
const CONTINENTAL_BASE_M: f32 = 400.0;
/// Oceanic-plate baseline elevation, metres.
const OCEANIC_BASE_M: f32 = -3000.0;

/// Generate the geology layers (plates + elevation + sea level) into an
/// empty grid. Consumes the target hydrosphere fraction from `context`.
pub fn generate_geology(
    context: &PlanetSimulationInput,
    hydrosphere_pct: f32,
    resolution: GridResolution,
    seed: &str,
) -> SurfaceGrid {
    let mut grid = SurfaceGrid::empty(resolution);
    let scope = format!("body{}_geology", context.body_id);
    let mut rng = SeededDiceRoller::new(seed, &scope);

    let plate_count = pick_plate_count(context, &mut rng);
    grid.plates = seed_plates(plate_count, &grid, &mut rng);
    assign_plate_ids(&mut grid);
    classify_boundaries(&mut grid);
    apply_base_elevation(&mut grid);
    apply_boundary_modifiers(&mut grid, &mut rng);
    apply_fractal_noise(&mut grid, &mut rng);
    grid.sea_level_m = find_sea_level(&mut grid, hydrosphere_pct);

    grid
}

/// Pick a plausible plate count scaled by body radius.
fn pick_plate_count(context: &PlanetSimulationInput, rng: &mut SeededDiceRoller) -> u8 {
    // Earth has ~15 major and minor plates. Larger bodies support more.
    let radius = context.body_radius_earth.max(0.3) as f32;
    let base = (radius.powf(1.5) * 12.0) as i32;
    let jitter = rng.roll(1, 9, -4) as i32;
    (base + jitter).clamp(8, 40) as u8
}

/// Seed N plates by area-weighted random selection on the sphere.
fn seed_plates(count: u8, grid: &SurfaceGrid, rng: &mut SeededDiceRoller) -> Vec<Plate> {
    let mut plates = Vec::with_capacity(count as usize);
    let mut used: std::collections::HashSet<(u16, u16)> = std::collections::HashSet::new();

    for id in 0..count {
        let (col, row) = loop {
            // Area-weighted latitude: sample sin-latitude uniformly so that
            // plates distribute by actual surface area.
            let u: f64 = rng.gen_f64().clamp(0.0, 0.99999);
            let lat_deg = ((1.0f64 - 2.0 * u).asin() * 180.0 / std::f64::consts::PI) as f32;
            let lon_deg = (rng.gen_f64() * 360.0 - 180.0) as f32;
            let lat_norm: f32 = ((90.0 - lat_deg) / 180.0).clamp(0.0, 0.9999);
            let lon_norm: f32 = ((lon_deg + 180.0) / 360.0).rem_euclid(1.0);
            let row = (lat_norm * grid.height as f32) as u16;
            let col = (lon_norm * grid.width as f32) as u16;
            if !used.contains(&(col, row)) {
                used.insert((col, row));
                break (col, row);
            }
        };

        let kind = if rng.gen_f64() < 0.4 {
            PlateKind::Continental
        } else {
            PlateKind::Oceanic
        };
        // Velocity vector in grid cells per 10 Myr, magnitude 0.5–2.5.
        let speed = 0.5 + rng.gen_f64() as f32 * 2.0;
        let angle = rng.gen_f64() as f32 * std::f32::consts::TAU;
        let velocity = (speed * angle.cos(), speed * angle.sin());
        let age_myr = 10.0 + rng.gen_f64() as f32 * 190.0;

        plates.push(Plate {
            id,
            kind,
            velocity,
            age_myr,
            seed_cell: (col, row),
        });
    }
    plates
}

/// Assign each cell to its nearest plate by great-circle distance.
fn assign_plate_ids(grid: &mut SurfaceGrid) {
    let plates = grid.plates.clone();
    for r in 0..grid.height {
        let lat = grid.row_latitude(r);
        for c in 0..grid.width {
            let lon = grid.col_longitude(c);
            let mut best_id = 0u8;
            let mut best_d = f32::INFINITY;
            for p in &plates {
                let plat = grid.row_latitude(p.seed_cell.1);
                let plon = grid.col_longitude(p.seed_cell.0);
                let d = great_circle_distance(lat, lon, plat, plon);
                if d < best_d {
                    best_d = d;
                    best_id = p.id;
                }
            }
            let idx = grid.idx(c, r);
            grid.layers.plate_id[idx] = best_id;
        }
    }
}

/// Classify each tile's tectonic boundary based on its plate's motion
/// relative to its neighbours'.
fn classify_boundaries(grid: &mut SurfaceGrid) {
    let w = grid.width;
    let h = grid.height;
    let plates = grid.plates.clone();

    for r in 0..h {
        for c in 0..w {
            let idx = grid.idx(c, r);
            let my_plate_id = grid.layers.plate_id[idx];

            // Check 4-connected neighbours (with longitude wrap).
            let neighbours = [
                (c, r.saturating_sub(1)),
                (c, (r + 1).min(h - 1)),
                ((c + w - 1) % w, r),
                ((c + 1) % w, r),
            ];

            let mut boundary = BoundaryKind::None;
            for &(nc, nr) in &neighbours {
                let nidx = grid.idx(nc, nr);
                let their_plate_id = grid.layers.plate_id[nidx];
                if their_plate_id == my_plate_id {
                    continue;
                }
                let my_p = &plates[my_plate_id as usize];
                let their_p = &plates[their_plate_id as usize];

                // Relative velocity of my plate with respect to theirs.
                let rel = (
                    my_p.velocity.0 - their_p.velocity.0,
                    my_p.velocity.1 - their_p.velocity.1,
                );
                // Boundary normal pointing from me → neighbour, in (col, row).
                let dc = nc as f32 - c as f32;
                let dr = nr as f32 - r as f32;
                let normal_mag = (dc * dc + dr * dr).sqrt().max(1e-6);
                let normal = (dc / normal_mag, dr / normal_mag);

                // Project relative velocity onto normal. Positive = plates
                // moving apart (divergent), negative = closing (convergent).
                let proj = rel.0 * normal.0 + rel.1 * normal.1;
                let tangent = rel.0 * normal.1 - rel.1 * normal.0;

                let kind = if proj.abs() > tangent.abs() * 1.2 {
                    if proj > 0.1 {
                        BoundaryKind::Divergent
                    } else if proj < -0.1 {
                        BoundaryKind::Convergent
                    } else {
                        BoundaryKind::Transform
                    }
                } else {
                    BoundaryKind::Transform
                };
                // Priority: Convergent > Divergent > Transform > None.
                boundary = match (boundary, kind) {
                    (BoundaryKind::None, k) => k,
                    (
                        BoundaryKind::Transform,
                        BoundaryKind::Divergent | BoundaryKind::Convergent,
                    ) => kind,
                    (BoundaryKind::Divergent, BoundaryKind::Convergent) => kind,
                    _ => boundary,
                };
            }
            grid.layers.tectonic_boundary[idx] = boundary;
        }
    }
}

/// Set elevation to the plate's baseline everywhere.
fn apply_base_elevation(grid: &mut SurfaceGrid) {
    let plates = grid.plates.clone();
    for idx in 0..grid.tile_count() {
        let pid = grid.layers.plate_id[idx];
        grid.layers.elevation_m[idx] = match plates[pid as usize].kind {
            PlateKind::Continental => CONTINENTAL_BASE_M,
            PlateKind::Oceanic => OCEANIC_BASE_M,
        };
    }
}

/// Add mountain/trench/ridge modifiers based on boundary kind and plate types.
fn apply_boundary_modifiers(grid: &mut SurfaceGrid, rng: &mut SeededDiceRoller) {
    let plates = grid.plates.clone();
    for idx in 0..grid.tile_count() {
        let boundary = grid.layers.tectonic_boundary[idx];
        if boundary == BoundaryKind::None {
            continue;
        }
        let pid = grid.layers.plate_id[idx];
        let kind = plates[pid as usize].kind;
        let jitter = (rng.gen_f64() as f32 - 0.5) * 800.0;

        let delta = match (boundary, kind) {
            // Continental-continental convergence → mountain range.
            (BoundaryKind::Convergent, PlateKind::Continental) => 3500.0 + jitter,
            // Oceanic-continental / oceanic convergence → trench on ocean side.
            (BoundaryKind::Convergent, PlateKind::Oceanic) => -1500.0 + jitter * 0.5,
            // Divergent on land → rift valley (slightly lowered).
            (BoundaryKind::Divergent, PlateKind::Continental) => -400.0 + jitter * 0.3,
            // Divergent in ocean → mid-ocean ridge (raised).
            (BoundaryKind::Divergent, PlateKind::Oceanic) => 1500.0 + jitter * 0.3,
            // Transform → minor offsets.
            (BoundaryKind::Transform, _) => jitter * 0.2,
            _ => 0.0,
        };
        grid.layers.elevation_m[idx] += delta;
    }
}

/// Overlay 4-octave value noise for terrain detail.
fn apply_fractal_noise(grid: &mut SurfaceGrid, rng: &mut SeededDiceRoller) {
    let seed_offset = rng.gen_u32();
    let amplitude = 800.0;
    for r in 0..grid.height {
        for c in 0..grid.width {
            let idx = grid.idx(c, r);
            let nx = c as f32 / grid.width as f32 * std::f32::consts::TAU;
            let ny = (r as f32 / grid.height as f32) * std::f32::consts::PI;
            let mut sample = 0.0f32;
            let mut amp = 1.0f32;
            let mut freq = 4.0f32;
            let mut total_amp = 0.0f32;
            for _octave in 0..4 {
                sample += amp * hash_noise_2d(nx * freq, ny * freq, seed_offset);
                total_amp += amp;
                amp *= 0.5;
                freq *= 2.0;
            }
            sample /= total_amp;
            grid.layers.elevation_m[idx] += sample * amplitude;
        }
    }
}

/// Binary-search a sea level such that the ocean coverage matches the
/// target hydrosphere fraction (%). Updates `is_ocean`, returns the
/// chosen sea level in metres.
fn find_sea_level(grid: &mut SurfaceGrid, hydrosphere_pct: f32) -> f32 {
    let target = hydrosphere_pct.clamp(0.0, 100.0) / 100.0;
    let (mut lo, mut hi) = {
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;
        for &e in &grid.layers.elevation_m {
            min = min.min(e);
            max = max.max(e);
        }
        (min, max)
    };
    let n = grid.layers.elevation_m.len() as f32;
    let mut sea_level = 0.0;
    for _iter in 0..32 {
        sea_level = (lo + hi) * 0.5;
        let ocean = grid
            .layers
            .elevation_m
            .iter()
            .filter(|&&e| e < sea_level)
            .count() as f32
            / n;
        if (ocean - target).abs() < 0.005 {
            break;
        }
        if ocean < target {
            lo = sea_level;
        } else {
            hi = sea_level;
        }
    }
    for (i, &e) in grid.layers.elevation_m.iter().enumerate() {
        grid.layers.is_ocean[i] = e < sea_level;
    }
    sea_level
}

/// Great-circle angular distance in radians between two lat/lon points.
fn great_circle_distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let dlon = (lon2 - lon1).to_radians();
    (lat1.sin() * lat2.sin() + lat1.cos() * lat2.cos() * dlon.cos())
        .clamp(-1.0, 1.0)
        .acos()
}

/// Cheap deterministic 2D value noise using integer hashing + bilinear
/// interpolation. Produces values in [-1, 1].
fn hash_noise_2d(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor();
    let y0 = y.floor();
    let xf = x - x0;
    let yf = y - y0;
    let x0i = x0 as i32;
    let y0i = y0 as i32;
    let v00 = hash_to_float(x0i, y0i, seed);
    let v10 = hash_to_float(x0i + 1, y0i, seed);
    let v01 = hash_to_float(x0i, y0i + 1, seed);
    let v11 = hash_to_float(x0i + 1, y0i + 1, seed);
    // Smoothstep the interpolation fractions.
    let sx = xf * xf * (3.0 - 2.0 * xf);
    let sy = yf * yf * (3.0 - 2.0 * yf);
    let a = v00 + (v10 - v00) * sx;
    let b = v01 + (v11 - v01) * sx;
    a + (b - a) * sy
}

fn hash_to_float(x: i32, y: i32, seed: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(374761393);
    h = h.wrapping_add((y as u32).wrapping_mul(668265263));
    h = h.wrapping_add(seed.wrapping_mul(1274126177));
    h ^= h >> 13;
    h = h.wrapping_mul(1274126177);
    h ^= h >> 16;
    (h as f32 / u32::MAX as f32) * 2.0 - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StarContext;

    fn earth_like_input() -> PlanetSimulationInput {
        PlanetSimulationInput {
            body_id: 7,
            body_radius_earth: 1.0,
            blackbody_temp_k: 288,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn produces_grid_at_requested_resolution() {
        let g = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "seed");
        assert_eq!(g.width, 72);
        assert_eq!(g.height, 36);
        assert_eq!(g.tile_count(), 72 * 36);
    }

    #[test]
    fn plate_count_in_valid_range() {
        let g = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "seed");
        assert!(g.plates.len() >= 8 && g.plates.len() <= 40);
    }

    #[test]
    fn every_tile_assigned_to_a_plate() {
        let g = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "seed");
        let max_id = g.plates.len() as u8;
        for &id in &g.layers.plate_id {
            assert!(id < max_id);
        }
    }

    #[test]
    fn hydrosphere_target_respected() {
        let g = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "seed");
        let ocean_count = g.layers.is_ocean.iter().filter(|&&b| b).count();
        let fraction = ocean_count as f32 / g.tile_count() as f32;
        // Allow ±10% tolerance from the 71% target.
        assert!(
            (fraction - 0.71).abs() < 0.10,
            "hydrosphere drift: target 71%, got {:.1}%",
            fraction * 100.0
        );
    }

    #[test]
    fn dry_world_has_no_ocean() {
        let g = generate_geology(&earth_like_input(), 0.0, GridResolution::Fast, "dry");
        let ocean_count = g.layers.is_ocean.iter().filter(|&&b| b).count();
        assert!(
            ocean_count < g.tile_count() / 20,
            "dry world has {} ocean tiles",
            ocean_count
        );
    }

    #[test]
    fn water_world_is_mostly_ocean() {
        let g = generate_geology(&earth_like_input(), 95.0, GridResolution::Fast, "wet");
        let ocean_count = g.layers.is_ocean.iter().filter(|&&b| b).count();
        let fraction = ocean_count as f32 / g.tile_count() as f32;
        assert!(fraction > 0.85);
    }

    #[test]
    fn geology_is_deterministic() {
        let a = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "det");
        let b = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "det");
        assert_eq!(a.plates.len(), b.plates.len());
        assert_eq!(a.layers.plate_id, b.layers.plate_id);
        assert_eq!(a.layers.elevation_m, b.layers.elevation_m);
        assert_eq!(a.sea_level_m, b.sea_level_m);
    }

    #[test]
    fn boundaries_cover_some_tiles() {
        let g = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "bnd");
        let boundary_count = g
            .layers
            .tectonic_boundary
            .iter()
            .filter(|&&b| b != BoundaryKind::None)
            .count();
        // Expect a reasonable fraction of cells to be on a boundary.
        let frac = boundary_count as f32 / g.tile_count() as f32;
        assert!(
            frac > 0.05 && frac < 0.80,
            "boundary fraction {:.2} out of expected range",
            frac
        );
    }

    #[test]
    fn elevation_distribution_has_mountains_and_trenches() {
        let g = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "elev");
        let max_e = g
            .layers
            .elevation_m
            .iter()
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);
        let min_e = g
            .layers
            .elevation_m
            .iter()
            .cloned()
            .fold(f32::INFINITY, f32::min);
        assert!(max_e > 1500.0, "max elevation {} too low", max_e);
        assert!(min_e < -1500.0, "min elevation {} too high", min_e);
    }

    #[test]
    fn larger_bodies_have_more_plates() {
        // Mars-size vs Jupiter-size body → different plate counts.
        let mut small = earth_like_input();
        small.body_radius_earth = 0.5;
        let g_small = generate_geology(&small, 50.0, GridResolution::Fast, "small");

        let mut big = earth_like_input();
        big.body_radius_earth = 2.0;
        let g_big = generate_geology(&big, 50.0, GridResolution::Fast, "big");

        assert!(
            g_big.plates.len() >= g_small.plates.len(),
            "big {} plates should be ≥ small {} plates",
            g_big.plates.len(),
            g_small.plates.len()
        );
    }

    #[test]
    fn continental_and_oceanic_plates_both_generated() {
        let g = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "mix");
        let continental = g
            .plates
            .iter()
            .filter(|p| p.kind == PlateKind::Continental)
            .count();
        let oceanic = g
            .plates
            .iter()
            .filter(|p| p.kind == PlateKind::Oceanic)
            .count();
        assert!(continental > 0);
        assert!(oceanic > 0);
    }
}
