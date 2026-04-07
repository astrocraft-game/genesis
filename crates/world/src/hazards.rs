//! Environmental hazard zones — per-tile flags derived from physics layers.
//!
//! Hazards affect species habitability and are computed once from the
//! surface grid at generation time.

use crate::grid::{BoundaryKind, SurfaceGrid};
use crate::types::BiomeType;

/// Individual hazard flags for a tile (bitfield-style).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HazardFlags {
    /// Toxic atmospheric gases (volcanic outgassing, high SO₂).
    pub toxic_atmosphere: bool,
    /// Elevated radiation (thin atmosphere + high UV, or volcanic).
    pub radiation: bool,
    /// Corrosive precipitation from volcanic activity.
    pub acid_rain: bool,
    /// Mean temperature below −30 °C.
    pub extreme_cold: bool,
    /// Mean temperature above 50 °C.
    pub extreme_heat: bool,
    /// Near active tectonic boundary — earthquake/eruption risk.
    pub seismic: bool,
    /// High elevation (>3500 m above sea level) — thin air, difficult access.
    pub high_altitude: bool,
}

impl HazardFlags {
    /// Number of active hazards on this tile.
    pub fn count(&self) -> u8 {
        self.toxic_atmosphere as u8
            + self.radiation as u8
            + self.acid_rain as u8
            + self.extreme_cold as u8
            + self.extreme_heat as u8
            + self.seismic as u8
            + self.high_altitude as u8
    }

    /// True if no hazards are present.
    pub fn is_safe(&self) -> bool {
        self.count() == 0
    }

    /// A 0.0–1.0 danger score (higher = more hazardous).
    pub fn danger_score(&self) -> f32 {
        self.count() as f32 / 7.0
    }
}

/// Per-tile hazard data for an entire grid.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HazardMap {
    pub flags: Vec<HazardFlags>,
}

impl HazardMap {
    /// Number of tiles with at least one hazard.
    pub fn hazardous_tile_count(&self) -> usize {
        self.flags.iter().filter(|f| !f.is_safe()).count()
    }

    /// Number of tiles that are completely safe.
    pub fn safe_tile_count(&self) -> usize {
        self.flags.iter().filter(|f| f.is_safe()).count()
    }

    /// Tiles with a specific hazard.
    pub fn tiles_with(&self, predicate: impl Fn(&HazardFlags) -> bool) -> Vec<usize> {
        self.flags
            .iter()
            .enumerate()
            .filter(|(_, f)| predicate(f))
            .map(|(i, _)| i)
            .collect()
    }
}

/// Derive hazard zones from a surface grid's physics layers.
/// Derive hazard zones from a surface grid's physics layers.
pub fn generate_hazards(grid: &SurfaceGrid) -> HazardMap {
    let n = grid.tile_count();
    let mut flags = Vec::with_capacity(n);

    for idx in 0..n {
        let boundary = grid.layers.tectonic_boundary[idx];
        let temp = grid.layers.temperature_c[idx];
        let elev = grid.layers.elevation_m[idx];
        let sea_level = grid.sea_level_m;
        let is_ocean = grid.layers.is_ocean[idx];
        let biome = grid.layers.biome[idx];

        let seismic = matches!(
            boundary,
            BoundaryKind::Convergent | BoundaryKind::Divergent | BoundaryKind::Transform
        );

        // Toxic atmosphere: volcanic biomes or convergent boundaries with
        // high elevation (volcanic outgassing).
        let toxic_atmosphere = biome == BiomeType::Volcanic
            || (boundary == BoundaryKind::Convergent && elev > sea_level + 2000.0);

        // Radiation: very high altitude (thin atmosphere) or volcanic
        // regions (radon emissions).
        let radiation = (!is_ocean && elev > sea_level + 4000.0) || biome == BiomeType::Volcanic;

        // Acid rain: near active volcanics (SO₂ outgassing).
        let acid_rain = boundary == BoundaryKind::Convergent && biome == BiomeType::Volcanic;

        let extreme_cold = !is_ocean && temp < -30.0;
        let extreme_heat = !is_ocean && temp > 50.0;
        let high_altitude = !is_ocean && elev > sea_level + 3500.0;

        flags.push(HazardFlags {
            toxic_atmosphere,
            radiation,
            acid_rain,
            extreme_cold,
            extreme_heat,
            seismic,
            high_altitude,
        });
    }

    HazardMap { flags }
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
        generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "hazards")
    }

    #[test]
    fn hazard_map_has_correct_size() {
        let g = earth_grid();
        let hm = generate_hazards(&g);
        assert_eq!(hm.flags.len(), g.tile_count());
    }

    #[test]
    fn seismic_only_at_boundaries() {
        let g = earth_grid();
        let hm = generate_hazards(&g);
        for (idx, flags) in hm.flags.iter().enumerate() {
            if flags.seismic {
                assert_ne!(
                    g.layers.tectonic_boundary[idx],
                    BoundaryKind::None,
                    "seismic flag at non-boundary tile {}",
                    idx
                );
            }
        }
    }

    #[test]
    fn extreme_cold_matches_temperature() {
        let g = earth_grid();
        let hm = generate_hazards(&g);
        for (idx, flags) in hm.flags.iter().enumerate() {
            if flags.extreme_cold {
                assert!(
                    g.layers.temperature_c[idx] < -30.0,
                    "extreme_cold at tile {} with temp {}",
                    idx,
                    g.layers.temperature_c[idx]
                );
            }
        }
    }

    #[test]
    fn high_altitude_matches_elevation() {
        let g = earth_grid();
        let hm = generate_hazards(&g);
        for (idx, flags) in hm.flags.iter().enumerate() {
            if flags.high_altitude {
                assert!(
                    g.layers.elevation_m[idx] > g.sea_level_m + 3500.0,
                    "high_altitude at tile {} with elev {}",
                    idx,
                    g.layers.elevation_m[idx]
                );
            }
        }
    }

    #[test]
    fn some_tiles_are_safe() {
        let g = earth_grid();
        let hm = generate_hazards(&g);
        assert!(
            hm.safe_tile_count() > 0,
            "Earth-like world should have some safe tiles"
        );
    }

    #[test]
    fn some_tiles_are_hazardous() {
        let g = earth_grid();
        let hm = generate_hazards(&g);
        assert!(
            hm.hazardous_tile_count() > 0,
            "Earth-like world should have some hazardous tiles"
        );
    }

    #[test]
    fn danger_score_bounded() {
        let g = earth_grid();
        let hm = generate_hazards(&g);
        for flags in &hm.flags {
            let score = flags.danger_score();
            assert!((0.0..=1.0).contains(&score), "score {} out of range", score);
        }
    }

    #[test]
    fn tiles_with_filter_works() {
        let g = earth_grid();
        let hm = generate_hazards(&g);
        let seismic = hm.tiles_with(|f| f.seismic);
        for &idx in &seismic {
            assert!(hm.flags[idx].seismic);
        }
    }
}
