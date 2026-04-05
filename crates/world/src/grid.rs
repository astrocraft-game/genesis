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

    /// (width, height) in tiles — convenient for passing to image crates.
    #[inline]
    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    // -----------------------------------------------------------------------
    // Texture export helpers
    //
    // Produce raw byte buffers suitable for feeding directly to game engine
    // textures or to the `image` crate. All RGB outputs are packed
    // row-major, top-to-bottom (row 0 = north pole).
    // -----------------------------------------------------------------------

    /// Pack the biome layer as W×H×3 RGB bytes using a fixed palette.
    pub fn export_biome_rgb(&self) -> Vec<u8> {
        use crate::types::BiomeType;
        let n = self.tile_count();
        let mut buf = Vec::with_capacity(n * 3);
        for &biome in &self.layers.biome {
            let (r, g, b) = match biome {
                BiomeType::Tundra => (180, 190, 200),
                BiomeType::Taiga => (40, 90, 60),
                BiomeType::TemperateForest => (60, 120, 60),
                BiomeType::TropicalForest => (20, 100, 40),
                BiomeType::Grassland => (150, 200, 100),
                BiomeType::Desert => (220, 200, 130),
                BiomeType::Savanna => (200, 180, 100),
                BiomeType::Wetland => (100, 130, 80),
                BiomeType::Alpine => (200, 200, 220),
                BiomeType::Volcanic => (80, 40, 40),
                BiomeType::IceCap => (240, 250, 255),
                BiomeType::Ocean => (20, 80, 180),
                BiomeType::Barren => (140, 130, 120),
                _ => (255, 0, 255), // magenta for unknown future variants
            };
            buf.push(r);
            buf.push(g);
            buf.push(b);
        }
        buf
    }

    /// Pack elevation as W×H grayscale bytes. Sea level maps to 128;
    /// deepest ocean → 0, highest peak → 255.
    pub fn export_elevation_grayscale(&self) -> Vec<u8> {
        let (min_e, max_e) = self
            .layers
            .elevation_m
            .iter()
            .copied()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), e| {
                (lo.min(e), hi.max(e))
            });
        let sl = self.sea_level_m;
        // Split scaling: 0..128 covers min_e..sea_level, 128..255 covers sea_level..max_e.
        let below_span = (sl - min_e).max(1.0);
        let above_span = (max_e - sl).max(1.0);
        self.layers
            .elevation_m
            .iter()
            .map(|&e| {
                if e < sl {
                    let t = ((e - min_e) / below_span).clamp(0.0, 1.0);
                    (t * 128.0) as u8
                } else {
                    let t = ((e - sl) / above_span).clamp(0.0, 1.0);
                    (128.0 + t * 127.0) as u8
                }
            })
            .collect()
    }

    /// Pack temperature as W×H×3 RGB bytes using a blue-white-red colormap.
    /// Range: −40 °C → deep blue, 0 °C → white, +40 °C → red.
    pub fn export_temperature_rgb(&self) -> Vec<u8> {
        let n = self.tile_count();
        let mut buf = Vec::with_capacity(n * 3);
        for &t in &self.layers.temperature_c {
            let (r, g, b) = temperature_color(t);
            buf.push(r);
            buf.push(g);
            buf.push(b);
        }
        buf
    }

    /// Pack precipitation as W×H×3 RGB bytes using a tan-to-blue colormap.
    /// Range: 0 mm → tan, 3000 mm → deep blue.
    pub fn export_precipitation_rgb(&self) -> Vec<u8> {
        let n = self.tile_count();
        let mut buf = Vec::with_capacity(n * 3);
        for &p in &self.layers.precipitation_mm {
            let (r, g, b) = precipitation_color(p);
            buf.push(r);
            buf.push(g);
            buf.push(b);
        }
        buf
    }

    /// Pack the is_ocean mask as W×H grayscale bytes (0 = land, 255 = ocean).
    pub fn export_ocean_mask(&self) -> Vec<u8> {
        self.layers
            .is_ocean
            .iter()
            .map(|&o| if o { 255 } else { 0 })
            .collect()
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t.clamp(0.0, 1.0)) as u8
}

fn temperature_color(t_c: f32) -> (u8, u8, u8) {
    // Cold (-40°C) = deep blue, cool (0°C) = white, hot (40°C) = red.
    if t_c < 0.0 {
        let t = ((t_c + 40.0) / 40.0).clamp(0.0, 1.0);
        (lerp_u8(0, 240, t), lerp_u8(0, 240, t), lerp_u8(180, 240, t))
    } else {
        let t = (t_c / 40.0).clamp(0.0, 1.0);
        (
            lerp_u8(240, 200, t),
            lerp_u8(240, 30, t),
            lerp_u8(240, 30, t),
        )
    }
}

fn precipitation_color(mm: f32) -> (u8, u8, u8) {
    let t = (mm / 3000.0).clamp(0.0, 1.0);
    (
        lerp_u8(220, 40, t),
        lerp_u8(200, 80, t),
        lerp_u8(130, 200, t),
    )
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

#[cfg(test)]
mod texture_tests {
    use super::*;
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
        generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "tex")
    }

    #[test]
    fn biome_rgb_has_correct_size() {
        let g = earth_grid();
        let buf = g.export_biome_rgb();
        assert_eq!(buf.len(), g.tile_count() * 3);
    }

    #[test]
    fn ocean_tiles_colored_blue() {
        let g = earth_grid();
        let buf = g.export_biome_rgb();
        // Find first ocean tile and confirm its RGB matches the Ocean palette.
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                let r = buf[idx * 3];
                let g_ = buf[idx * 3 + 1];
                let b = buf[idx * 3 + 2];
                assert_eq!((r, g_, b), (20, 80, 180));
                return;
            }
        }
        panic!("no ocean tile found in test grid");
    }

    #[test]
    fn elevation_grayscale_has_correct_size() {
        let g = earth_grid();
        let buf = g.export_elevation_grayscale();
        assert_eq!(buf.len(), g.tile_count());
    }

    #[test]
    fn deep_ocean_is_black_peaks_are_white() {
        let g = earth_grid();
        let buf = g.export_elevation_grayscale();
        let min = *buf.iter().min().unwrap();
        let max = *buf.iter().max().unwrap();
        assert!(min < 30, "deepest pixel should be near 0, got {}", min);
        assert!(max > 200, "highest pixel should be near 255, got {}", max);
    }

    #[test]
    fn temperature_rgb_has_correct_size() {
        let g = earth_grid();
        let buf = g.export_temperature_rgb();
        assert_eq!(buf.len(), g.tile_count() * 3);
    }

    #[test]
    fn warm_tiles_redder_than_cold_tiles() {
        // Unit test of the color function.
        let (r_cold, _, _) = super::temperature_color(-30.0);
        let (r_warm, _, _) = super::temperature_color(30.0);
        assert!(r_warm > r_cold);
        let (_, _, b_cold) = super::temperature_color(-30.0);
        let (_, _, b_warm) = super::temperature_color(30.0);
        assert!(b_cold > b_warm);
    }

    #[test]
    fn precipitation_rgb_has_correct_size() {
        let g = earth_grid();
        let buf = g.export_precipitation_rgb();
        assert_eq!(buf.len(), g.tile_count() * 3);
    }

    #[test]
    fn dry_tiles_tan_wet_tiles_blue() {
        let (r_dry, _, b_dry) = super::precipitation_color(0.0);
        let (r_wet, _, b_wet) = super::precipitation_color(3000.0);
        assert!(r_dry > r_wet);
        assert!(b_wet > b_dry);
    }

    #[test]
    fn ocean_mask_matches_is_ocean() {
        let g = earth_grid();
        let mask = g.export_ocean_mask();
        assert_eq!(mask.len(), g.tile_count());
        for (idx, &byte) in mask.iter().enumerate() {
            let expected = if g.layers.is_ocean[idx] { 255 } else { 0 };
            assert_eq!(byte, expected);
        }
    }

    #[test]
    fn dimensions_match_grid() {
        let g = earth_grid();
        assert_eq!(g.dimensions(), (72, 36));
    }

    #[test]
    fn all_biome_variants_have_colors() {
        use crate::types::BiomeType;
        // Manually construct a grid with one of each biome, verify no
        // magenta fallback pixels (indicating an unmapped variant).
        let biomes = [
            BiomeType::Tundra,
            BiomeType::Taiga,
            BiomeType::TemperateForest,
            BiomeType::TropicalForest,
            BiomeType::Grassland,
            BiomeType::Desert,
            BiomeType::Savanna,
            BiomeType::Wetland,
            BiomeType::Alpine,
            BiomeType::Volcanic,
            BiomeType::IceCap,
            BiomeType::Ocean,
            BiomeType::Barren,
        ];
        let mut g = SurfaceGrid::empty(GridResolution::Custom(biomes.len() as u16, 1));
        for (i, b) in biomes.iter().enumerate() {
            g.layers.biome[i] = *b;
        }
        let buf = g.export_biome_rgb();
        // Magenta fallback is (255, 0, 255). None should hit it.
        for chunk in buf.chunks(3) {
            let (r, g_, b) = (chunk[0], chunk[1], chunk[2]);
            assert!(
                !(r == 255 && g_ == 0 && b == 255),
                "magenta fallback triggered"
            );
        }
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
