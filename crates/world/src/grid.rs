//! Tile-level surface grids for terrestrial worlds.
//!
//! Produces physically-motivated 2D layers — elevation, temperature,
//! wind, precipitation, biome — at caller-chosen resolution using an
//! equirectangular projection. Phase 1 of the roadmap establishes the
//! geology layer (plates + elevation); subsequent phases add climate,
//! hydrology, and biome classification.

/// Resolution presets for `SurfaceGrid` generation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GridResolution {
    /// 72×36 tiles (5° cells, ~2.6k tiles). Fast interactive generation.
    Fast,
    /// 144×72 tiles (2.5° cells, ~10k tiles). Standard quality.
    Standard,
    /// 360×180 tiles (1° cells, ~65k tiles). Detailed; slow.
    Detailed,
    /// Arbitrary resolution (width_longitude_cells, height_latitude_cells).
    /// Height should be ~half of width to keep square-ish tiles on the
    /// equirectangular projection.
    Custom(u16, u16),
}

impl GridResolution {
    pub fn dimensions(self) -> (u16, u16) {
        match self {
            GridResolution::Fast => (72, 36),
            GridResolution::Standard => (144, 72),
            GridResolution::Detailed => (360, 180),
            GridResolution::Custom(w, h) => (w, h),
        }
    }

    pub fn tile_count(self) -> usize {
        let (w, h) = self.dimensions();
        w as usize * h as usize
    }
}

/// Classification of a tectonic plate boundary between two adjacent plates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BoundaryKind {
    /// Not on a boundary (interior tile).
    #[default]
    None,
    /// Plates diverging — rift valleys, mid-ocean ridges.
    Divergent,
    /// Plates converging — mountain ranges, subduction trenches.
    Convergent,
    /// Plates sliding past each other — fault scarps.
    Transform,
}

/// Classification of a tectonic plate itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PlateKind {
    #[default]
    Continental,
    Oceanic,
}

/// Metadata for a single tectonic plate.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Plate {
    pub id: u8,
    pub kind: PlateKind,
    /// Plate velocity vector in grid units (longitude, latitude).
    /// Magnitude roughly in cells per 10 million years.
    pub velocity: (f32, f32),
    /// Age in millions of years (older plates are cooler, denser, lower).
    pub age_myr: f32,
    /// Seed cell (lon_col, lat_row) where this plate was nucleated.
    pub seed_cell: (u16, u16),
}

/// Physical layers of a surface grid. Row-major order with
/// `idx = lat_row * width + lon_col`, row 0 at the north pole.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SurfaceLayers {
    // Geology (Phase 1)
    pub elevation_m: Vec<f32>,
    pub plate_id: Vec<u8>,
    pub is_ocean: Vec<bool>,
    pub tectonic_boundary: Vec<BoundaryKind>,
    // Climate scalars (Phases 2, 4)
    pub temperature_c: Vec<f32>,
    pub temperature_summer_c: Vec<f32>,
    pub temperature_winter_c: Vec<f32>,
    pub precipitation_mm: Vec<f32>,
    pub humidity_relative: Vec<f32>,
    pub pet_ratio: Vec<f32>,
    // Climate vectors (Phases 3, 5)
    pub wind_direction_deg: Vec<f32>,
    pub wind_speed_ms: Vec<f32>,
    pub ocean_current_direction_deg: Vec<f32>,
    pub ocean_current_speed_ms: Vec<f32>,
    pub sea_surface_temp_c: Vec<f32>,
    // Hydrology (Phase 6)
    pub flow_accumulation: Vec<u32>,
    pub river_discharge_m3s: Vec<f32>,
    pub drainage_basin_id: Vec<u16>,
    // Classification (Phase 7)
    pub biome: Vec<crate::types::BiomeType>,
    pub koppen_class: Vec<crate::types::KoppenClass>,
}

/// A 2D surface grid in equirectangular projection.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SurfaceGrid {
    pub width: u16,
    pub height: u16,
    pub sea_level_m: f32,
    pub plates: Vec<Plate>,
    pub layers: SurfaceLayers,
}

impl SurfaceGrid {
    /// Create an empty grid pre-sized for the given resolution. All layer
    /// vectors are allocated but zero-initialised.
    pub fn empty(resolution: GridResolution) -> Self {
        let (width, height) = resolution.dimensions();
        let n = width as usize * height as usize;
        Self {
            width,
            height,
            sea_level_m: 0.0,
            plates: Vec::new(),
            layers: SurfaceLayers {
                elevation_m: vec![0.0; n],
                plate_id: vec![0; n],
                is_ocean: vec![false; n],
                tectonic_boundary: vec![BoundaryKind::None; n],
                temperature_c: vec![0.0; n],
                temperature_summer_c: vec![0.0; n],
                temperature_winter_c: vec![0.0; n],
                precipitation_mm: vec![0.0; n],
                humidity_relative: vec![0.0; n],
                pet_ratio: vec![0.0; n],
                wind_direction_deg: vec![0.0; n],
                wind_speed_ms: vec![0.0; n],
                ocean_current_direction_deg: vec![0.0; n],
                ocean_current_speed_ms: vec![0.0; n],
                sea_surface_temp_c: vec![0.0; n],
                flow_accumulation: vec![0; n],
                river_discharge_m3s: vec![0.0; n],
                drainage_basin_id: vec![0; n],
                biome: vec![crate::types::BiomeType::default(); n],
                koppen_class: vec![crate::types::KoppenClass::default(); n],
            },
        }
    }

    /// Total number of tiles.
    #[inline]
    pub fn tile_count(&self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Convert `(lon_col, lat_row)` to a flat index, wrapping longitude.
    #[inline]
    pub fn idx(&self, lon_col: u16, lat_row: u16) -> usize {
        let lon = lon_col % self.width;
        let lat = lat_row.min(self.height - 1);
        lat as usize * self.width as usize + lon as usize
    }

    /// Convert `(lat_deg, lon_deg)` (WGS-style) to a flat index.
    /// Latitude 90° (north pole) → row 0; latitude −90° (south pole) → last row.
    pub fn idx_latlon(&self, lat_deg: f32, lon_deg: f32) -> usize {
        let lat_norm = ((90.0 - lat_deg) / 180.0).clamp(0.0, 0.9999);
        let lon_norm = ((lon_deg + 180.0) / 360.0).rem_euclid(1.0);
        let row = (lat_norm * self.height as f32) as u16;
        let col = (lon_norm * self.width as f32) as u16;
        self.idx(col, row)
    }

    /// Latitude (degrees) at the centre of row `r`.
    #[inline]
    pub fn row_latitude(&self, r: u16) -> f32 {
        let frac = (r as f32 + 0.5) / self.height as f32;
        90.0 - frac * 180.0
    }

    /// Longitude (degrees) at the centre of column `c`.
    #[inline]
    pub fn col_longitude(&self, c: u16) -> f32 {
        let frac = (c as f32 + 0.5) / self.width as f32;
        -180.0 + frac * 360.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_dimensions_match_expected() {
        assert_eq!(GridResolution::Fast.dimensions(), (72, 36));
        assert_eq!(GridResolution::Standard.dimensions(), (144, 72));
        assert_eq!(GridResolution::Detailed.dimensions(), (360, 180));
        assert_eq!(GridResolution::Custom(100, 50).dimensions(), (100, 50));
    }

    #[test]
    fn empty_grid_is_zeroed_and_sized() {
        let g = SurfaceGrid::empty(GridResolution::Fast);
        assert_eq!(g.width, 72);
        assert_eq!(g.height, 36);
        assert_eq!(g.tile_count(), 72 * 36);
        assert_eq!(g.layers.elevation_m.len(), 72 * 36);
        assert!(g.layers.elevation_m.iter().all(|&e| e == 0.0));
        assert!(g.layers.is_ocean.iter().all(|&o| !o));
    }

    #[test]
    fn idx_wraps_longitude() {
        let g = SurfaceGrid::empty(GridResolution::Custom(10, 5));
        assert_eq!(g.idx(0, 0), 0);
        assert_eq!(g.idx(10, 0), 0); // wraps
        assert_eq!(g.idx(11, 0), 1); // wraps
        assert_eq!(g.idx(9, 4), 49);
    }

    #[test]
    fn row_latitude_runs_pole_to_pole() {
        let g = SurfaceGrid::empty(GridResolution::Custom(72, 36));
        // Row 0 is near the north pole, last row near south.
        assert!(g.row_latitude(0) > 80.0);
        assert!(g.row_latitude(35) < -80.0);
        // Monotonic decreasing.
        for r in 0..35u16 {
            assert!(g.row_latitude(r) > g.row_latitude(r + 1));
        }
    }

    #[test]
    fn col_longitude_runs_west_to_east() {
        let g = SurfaceGrid::empty(GridResolution::Custom(72, 36));
        assert!(g.col_longitude(0) < -170.0);
        assert!(g.col_longitude(71) > 170.0);
    }

    #[test]
    fn idx_latlon_round_trip() {
        let g = SurfaceGrid::empty(GridResolution::Custom(72, 36));
        // Equator at longitude 0 should hit the middle.
        let eq_idx = g.idx_latlon(0.0, 0.0);
        let lat = g.row_latitude((eq_idx / 72) as u16);
        let lon = g.col_longitude((eq_idx % 72) as u16);
        assert!(lat.abs() < 5.0, "equator row gave lat {}", lat);
        assert!(lon.abs() < 5.0, "0° lon col gave lon {}", lon);
    }
}

// ---------------------------------------------------------------------------
// Full surface-grid pipeline
//
// Runs all seven physical layers (geology → temperature → wind →
// precipitation → ocean dynamics → hydrology → biome) in order, producing
// a fully-populated SurfaceGrid ready for life distribution.
// ---------------------------------------------------------------------------

/// Generate a complete surface grid for a terrestrial body.
///
/// Calls every physics stage in dependency order. Inputs:
/// - `context`: the body's astronomical facts (used for tilt, temperature,
///   radius).
/// - `greenhouse_delta_k`: atmospheric warming above the blackbody
///   equilibrium (pass 0.0 for airless bodies).
/// - `atmospheric_pressure`: surface pressure in Earth atm.
/// - `hydrosphere_pct`: target ocean coverage as percentage (0–100).
/// - `resolution`: grid dimensions.
/// - `seed`: deterministic seed string.
///
/// Returns a fully-populated `SurfaceGrid`.
pub fn generate_surface_grid(
    context: &crate::types::PlanetSimulationInput,
    greenhouse_delta_k: f32,
    atmospheric_pressure: f32,
    hydrosphere_pct: f32,
    resolution: GridResolution,
    seed: &str,
) -> SurfaceGrid {
    let mut grid = crate::geology::generate_geology(context, hydrosphere_pct, resolution, seed);
    crate::climate::generate_temperature(context, greenhouse_delta_k, &mut grid);
    crate::climate::generate_wind(context, atmospheric_pressure, &mut grid);
    crate::hydrology::generate_precipitation(
        context,
        atmospheric_pressure,
        hydrosphere_pct,
        &mut grid,
    );
    crate::ocean::generate_ocean_dynamics(&mut grid);
    crate::hydrology::generate_hydrology(context.body_radius_earth as f32, &mut grid);
    crate::climate::generate_biomes(&mut grid);
    grid
}

#[cfg(test)]
mod pipeline_tests {
    use super::*;
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};

    fn earth_input() -> PlanetSimulationInput {
        PlanetSimulationInput {
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
        }
    }

    #[test]
    fn pipeline_produces_populated_grid() {
        let g = generate_surface_grid(
            &earth_input(),
            33.0,
            1.0,
            71.0,
            GridResolution::Fast,
            "pipe",
        );
        assert_eq!(g.tile_count(), 72 * 36);
        // Every layer should have been written.
        assert!(
            g.layers.temperature_c.iter().any(|&t| t != 0.0),
            "temperature not populated"
        );
        assert!(
            g.layers.precipitation_mm.iter().any(|&p| p > 0.0),
            "precipitation not populated"
        );
        assert!(
            g.layers.wind_speed_ms.iter().any(|&w| w > 0.0),
            "wind not populated"
        );
        assert!(
            g.layers.river_discharge_m3s.iter().any(|&d| d > 0.0),
            "discharge not populated"
        );
        assert!(g.plates.len() >= 8, "plates not generated");
    }

    #[test]
    fn pipeline_is_deterministic() {
        let a = generate_surface_grid(&earth_input(), 33.0, 1.0, 71.0, GridResolution::Fast, "det");
        let b = generate_surface_grid(&earth_input(), 33.0, 1.0, 71.0, GridResolution::Fast, "det");
        assert_eq!(a.layers.elevation_m, b.layers.elevation_m);
        assert_eq!(a.layers.temperature_c, b.layers.temperature_c);
        assert_eq!(a.layers.biome, b.layers.biome);
        assert_eq!(a.layers.river_discharge_m3s, b.layers.river_discharge_m3s);
    }

    #[test]
    fn different_seeds_produce_different_worlds() {
        let a = generate_surface_grid(
            &earth_input(),
            33.0,
            1.0,
            71.0,
            GridResolution::Fast,
            "seed_a",
        );
        let b = generate_surface_grid(
            &earth_input(),
            33.0,
            1.0,
            71.0,
            GridResolution::Fast,
            "seed_b",
        );
        assert_ne!(a.layers.elevation_m, b.layers.elevation_m);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};

    fn earth_input() -> PlanetSimulationInput {
        PlanetSimulationInput {
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
        }
    }

    #[test]
    fn grid_round_trips_through_json() {
        let g = generate_surface_grid(
            &earth_input(),
            33.0,
            1.0,
            71.0,
            GridResolution::Fast,
            "serde_test",
        );
        let json = serde_json::to_string(&g).expect("serialize");
        let parsed: SurfaceGrid = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed.width, g.width);
        assert_eq!(parsed.height, g.height);
        assert_eq!(parsed.sea_level_m, g.sea_level_m);
        assert_eq!(parsed.plates.len(), g.plates.len());
        assert_eq!(parsed.layers.elevation_m, g.layers.elevation_m);
        assert_eq!(parsed.layers.biome, g.layers.biome);
        assert_eq!(parsed.layers.koppen_class, g.layers.koppen_class);
    }

    #[test]
    fn resolution_round_trips() {
        for res in [
            GridResolution::Fast,
            GridResolution::Standard,
            GridResolution::Detailed,
            GridResolution::Custom(200, 100),
        ] {
            let json = serde_json::to_string(&res).unwrap();
            let parsed: GridResolution = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, res);
        }
    }
}
