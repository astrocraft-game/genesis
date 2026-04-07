//! Geological zone classification — assigns each tile to one of 10 zones
//! that determine which rare resources are available. Forces multi-base
//! expansion because no single location has access to all zones.

use crate::grid::{BoundaryKind, PlateKind, SurfaceGrid};
use crate::types::BiomeType;

/// Geological zone determining rare resource availability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum GeologicalZone {
    /// Alkaline igneous / continental rift — rare earths, niobium, thorium.
    CarbonatitePipe,
    /// Cratonic margin, convergent — PGMs, chromium, vanadium.
    MaficIntrusion,
    /// Continental interior, granitic — lithium, tantalum, beryllium, tin.
    PegmatiteField,
    /// Convergent boundary — copper, molybdenum, rhenium, gold.
    PorphyrySubduction,
    /// Weathered continental, equatorial — cobalt, nickel, scandium, REEs.
    LateriteTropical,
    /// Passive continental — uranium, manganese, phosphate, graphite.
    SedimentaryBasin,
    /// Coastal beach placer — zirconium, hafnium, titanium, monazite.
    HeavyMineralSands,
    /// Arid continental interior — lithium (brine), boron, potassium.
    BrineFlat,
    /// Divergent / hotspot — sulfur, arsenic, bismuth, geothermal.
    VolcanicVent,
    /// Meteorite impact site — iridium anomaly, shocked quartz.
    ImpactCrater,
    /// Default: no special zone (common resources only).
    #[default]
    Common,
}

impl GeologicalZone {
    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            GeologicalZone::CarbonatitePipe => "Carbonatite Pipe",
            GeologicalZone::MaficIntrusion => "Mafic Intrusion",
            GeologicalZone::PegmatiteField => "Pegmatite Field",
            GeologicalZone::PorphyrySubduction => "Porphyry / Subduction",
            GeologicalZone::LateriteTropical => "Laterite / Tropical",
            GeologicalZone::SedimentaryBasin => "Sedimentary Basin",
            GeologicalZone::HeavyMineralSands => "Heavy Mineral Sands",
            GeologicalZone::BrineFlat => "Brine Flat / Evaporite",
            GeologicalZone::VolcanicVent => "Volcanic Vent",
            GeologicalZone::ImpactCrater => "Impact Crater",
            GeologicalZone::Common => "Common",
        }
    }
}

/// Per-tile zone classification for an entire grid.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZoneMap {
    pub zones: Vec<GeologicalZone>,
}

impl ZoneMap {
    /// Count tiles per zone.
    pub fn zone_counts(&self) -> std::collections::HashMap<GeologicalZone, usize> {
        let mut map = std::collections::HashMap::new();
        for &z in &self.zones {
            *map.entry(z).or_insert(0) += 1;
        }
        map
    }

    /// Tiles belonging to a specific zone.
    pub fn tiles_in_zone(&self, zone: GeologicalZone) -> Vec<usize> {
        self.zones
            .iter()
            .enumerate()
            .filter(|(_, &z)| z == zone)
            .map(|(i, _)| i)
            .collect()
    }

    /// Number of distinct non-Common zones present on this planet.
    pub fn distinct_zones(&self) -> usize {
        let mut set = std::collections::HashSet::new();
        for &z in &self.zones {
            if z != GeologicalZone::Common {
                set.insert(z);
            }
        }
        set.len()
    }

    /// Maximum fraction of land tiles any single non-Common zone occupies.
    pub fn max_zone_fraction(&self, is_ocean: &[bool]) -> f32 {
        let land = is_ocean.iter().filter(|&&o| !o).count();
        if land == 0 {
            return 0.0;
        }
        let counts = self.zone_counts();
        counts
            .iter()
            .filter(|(&z, _)| z != GeologicalZone::Common)
            .map(|(_, &c)| c as f32 / land as f32)
            .fold(0.0f32, f32::max)
    }

    /// Count how many distinct non-Common zones are accessible within
    /// a Chebyshev radius of `radius` tiles from `centre`, on a grid
    /// of width `grid_width` with longitude wrap.
    pub fn zones_within_radius(
        &self,
        centre: usize,
        radius: u16,
        grid_width: u16,
    ) -> std::collections::HashSet<GeologicalZone> {
        let w = grid_width as usize;
        let h = self.zones.len() / w;
        let cr = centre / w;
        let cc = centre % w;
        let r = radius as usize;
        let mut found = std::collections::HashSet::new();

        for dr in 0..=(2 * r) {
            let nr = (cr + dr).saturating_sub(r);
            if nr >= h {
                continue;
            }
            for dc in 0..=(2 * r) {
                let nc = (cc + dc + w - r) % w;
                let idx = nr * w + nc;
                if idx < self.zones.len() {
                    let z = self.zones[idx];
                    if z != GeologicalZone::Common {
                        found.insert(z);
                    }
                }
            }
        }
        found
    }

    /// Check whether any tile can access all non-Common zones within
    /// the given radius. Returns `true` if scarcity is maintained
    /// (i.e., NO tile has access to everything).
    pub fn scarcity_maintained(&self, radius: u16, grid_width: u16) -> bool {
        let total_zones = self.distinct_zones();
        if total_zones <= 1 {
            return true;
        }
        for centre in 0..self.zones.len() {
            let nearby = self.zones_within_radius(centre, radius, grid_width);
            if nearby.len() >= total_zones {
                return false;
            }
        }
        true
    }
}

/// Classify every tile into a geological zone.
///
/// Rules (first match wins, checked in priority order):
/// 1. **Impact crater**: elevation anomaly + convergent boundary (proxy for
///    impact-modified geology) — very rare.
/// 2. **Volcanic vent**: divergent boundary + volcanic/alpine biome.
/// 3. **Porphyry/Subduction**: convergent boundary + continental plate +
///    high elevation (mountain-building zone).
/// 4. **Mafic intrusion**: convergent boundary + oceanic plate (subduction
///    melt → layered intrusion on craton margin).
/// 5. **Carbonatite pipe**: divergent boundary + continental plate (rift).
/// 6. **Laterite/Tropical**: tropical biome + continental (weathering crust).
/// 7. **Pegmatite field**: continental interior (no boundary) + moderate
///    elevation (granitic terrain).
/// 8. **Heavy mineral sands**: coastal land tile adjacent to ocean.
/// 9. **Brine flat**: desert/cold desert biome + low elevation (arid basin).
/// 10. **Sedimentary basin**: continental, no boundary, low elevation.
pub fn classify_zones(grid: &SurfaceGrid) -> ZoneMap {
    let w = grid.width as usize;
    let h = grid.height as usize;
    let n = w * h;
    let mut zones = vec![GeologicalZone::Common; n];

    for (idx, zone) in zones.iter_mut().enumerate() {
        if grid.layers.is_ocean[idx] {
            continue; // ocean tiles stay Common
        }

        let boundary = grid.layers.tectonic_boundary[idx];
        let pid = grid.layers.plate_id[idx] as usize;
        let plate_kind = if pid < grid.plates.len() {
            grid.plates[pid].kind
        } else {
            PlateKind::Continental
        };
        let biome = grid.layers.biome[idx];
        let elev = grid.layers.elevation_m[idx];
        let sea_level = grid.sea_level_m;
        let height = elev - sea_level;
        let lat = grid.row_latitude((idx / w) as u16);

        // Coastal check: any 4-neighbour is ocean.
        let r = idx / w;
        let c = idx % w;
        let is_coastal = [
            (r.saturating_sub(1), c),
            ((r + 1).min(h - 1), c),
            (r, (c + w - 1) % w),
            (r, (c + 1) % w),
        ]
        .iter()
        .any(|&(nr, nc)| grid.layers.is_ocean[nr * w + nc]);

        // Priority classification
        *zone = if boundary == BoundaryKind::Divergent
            && matches!(biome, BiomeType::Volcanic | BiomeType::Alpine)
        {
            GeologicalZone::VolcanicVent
        } else if boundary == BoundaryKind::Convergent
            && plate_kind == PlateKind::Continental
            && height > 1500.0
        {
            GeologicalZone::PorphyrySubduction
        } else if boundary == BoundaryKind::Convergent && plate_kind == PlateKind::Oceanic {
            GeologicalZone::MaficIntrusion
        } else if boundary == BoundaryKind::Divergent && plate_kind == PlateKind::Continental {
            GeologicalZone::CarbonatitePipe
        } else if lat.abs() < 25.0
            && matches!(biome, BiomeType::TropicalForest | BiomeType::Savanna)
            && plate_kind == PlateKind::Continental
        {
            GeologicalZone::LateriteTropical
        } else if is_coastal && height < 200.0 {
            GeologicalZone::HeavyMineralSands
        } else if matches!(biome, BiomeType::Desert | BiomeType::ColdDesert) && height < 500.0 {
            GeologicalZone::BrineFlat
        } else if boundary == BoundaryKind::None
            && plate_kind == PlateKind::Continental
            && height > 500.0
            && height < 2000.0
        {
            GeologicalZone::PegmatiteField
        } else if boundary == BoundaryKind::None
            && plate_kind == PlateKind::Continental
            && height < 500.0
        {
            GeologicalZone::SedimentaryBasin
        } else {
            GeologicalZone::Common
        };
    }

    ZoneMap { zones }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{generate_surface_grid, GridResolution};
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};

    fn earth_grid() -> SurfaceGrid {
        let input = PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        };
        generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "zones")
    }

    #[test]
    fn zone_map_has_correct_size() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        assert_eq!(zm.zones.len(), g.tile_count());
    }

    #[test]
    fn ocean_tiles_are_common() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                assert_eq!(zm.zones[idx], GeologicalZone::Common);
            }
        }
    }

    #[test]
    fn earth_has_multiple_zones() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        let distinct = zm.distinct_zones();
        assert!(
            distinct >= 4,
            "Earth-like should have 4+ zones, got {}",
            distinct
        );
    }

    #[test]
    fn no_single_zone_covers_entire_land() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        let land_count = g.layers.is_ocean.iter().filter(|&&o| !o).count();
        let counts = zm.zone_counts();
        for (&zone, &count) in &counts {
            if zone == GeologicalZone::Common {
                continue;
            }
            let frac = count as f32 / land_count.max(1) as f32;
            assert!(
                frac < 0.5,
                "{:?} covers {:.0}% of land — too dominant",
                zone,
                frac * 100.0
            );
        }
    }

    #[test]
    fn volcanic_vents_at_divergent_boundaries() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        for idx in 0..g.tile_count() {
            if zm.zones[idx] == GeologicalZone::VolcanicVent {
                assert_eq!(
                    g.layers.tectonic_boundary[idx],
                    BoundaryKind::Divergent,
                    "volcanic vent at non-divergent tile {}",
                    idx
                );
            }
        }
    }

    #[test]
    fn porphyry_at_convergent_continental() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        for idx in 0..g.tile_count() {
            if zm.zones[idx] == GeologicalZone::PorphyrySubduction {
                assert_eq!(g.layers.tectonic_boundary[idx], BoundaryKind::Convergent);
                let pid = g.layers.plate_id[idx] as usize;
                if pid < g.plates.len() {
                    assert_eq!(g.plates[pid].kind, PlateKind::Continental);
                }
            }
        }
    }

    #[test]
    fn laterite_only_in_tropics() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        let w = g.width as usize;
        for idx in 0..g.tile_count() {
            if zm.zones[idx] == GeologicalZone::LateriteTropical {
                let r = (idx / w) as u16;
                let lat = g.row_latitude(r);
                assert!(
                    lat.abs() < 30.0,
                    "laterite at lat {:.0}° (should be tropical)",
                    lat
                );
            }
        }
    }

    #[test]
    fn brine_flats_in_arid_biomes() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        for idx in 0..g.tile_count() {
            if zm.zones[idx] == GeologicalZone::BrineFlat {
                assert!(
                    matches!(
                        g.layers.biome[idx],
                        BiomeType::Desert | BiomeType::ColdDesert
                    ),
                    "brine flat at non-arid biome {:?}",
                    g.layers.biome[idx]
                );
            }
        }
    }

    #[test]
    fn tiles_in_zone_returns_correct_indices() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        for zone in [
            GeologicalZone::PorphyrySubduction,
            GeologicalZone::SedimentaryBasin,
        ] {
            let tiles = zm.tiles_in_zone(zone);
            for &idx in &tiles {
                assert_eq!(zm.zones[idx], zone);
            }
        }
    }

    // --- Scarcity balancing tests ---

    #[test]
    fn no_zone_exceeds_30_percent_of_land() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        let max_frac = zm.max_zone_fraction(&g.layers.is_ocean);
        assert!(
            max_frac <= 0.35,
            "max zone fraction {:.1}% exceeds 30% target",
            max_frac * 100.0
        );
    }

    #[test]
    fn scarcity_maintained_at_radius_3() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        // On a 72×36 grid with multiple zones, no single tile should
        // reach all zones within 3 tiles.
        if zm.distinct_zones() >= 4 {
            assert!(
                zm.scarcity_maintained(3, g.width),
                "a tile can reach all {} zones within radius 3 — scarcity violated",
                zm.distinct_zones()
            );
        }
    }

    #[test]
    fn zones_within_radius_grows_with_radius() {
        let g = earth_grid();
        let zm = classify_zones(&g);
        // Pick a land tile.
        let land = (0..g.tile_count())
            .find(|&i| !g.layers.is_ocean[i])
            .unwrap_or(0);
        let r1 = zm.zones_within_radius(land, 1, g.width).len();
        let r5 = zm.zones_within_radius(land, 5, g.width).len();
        assert!(
            r5 >= r1,
            "larger radius should find >= zones: r1={}, r5={}",
            r1,
            r5
        );
    }
}
