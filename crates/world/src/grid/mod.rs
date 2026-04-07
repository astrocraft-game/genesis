//! Tile-level surface grids for terrestrial worlds.
//!
//! Produces physically-motivated 2D layers — elevation, temperature,
//! wind, precipitation, biome — at caller-chosen resolution using an
//! equirectangular projection. Phase 1 of the roadmap establishes the
//! geology layer (plates + elevation); subsequent phases add climate,
//! hydrology, and biome classification.

pub mod alt;
pub mod diff;

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

/// One of the six faces of a unit cube, used by engines that render
/// planets via cube-sphere projection.
///
/// The `(u, v)` range is `[-1, 1]` on each face. The mapping to 3D
/// matches the standard GPU cubemap convention: `PosX` is the right
/// face (`+x`), `PosY` is the top (`+y`, north cap), and so on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CubeFace {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl CubeFace {
    /// Project a cube-face `(u, v)` coordinate onto the 3D direction
    /// that inflates to a unit sphere.
    pub fn to_xyz(self, u: f32, v: f32) -> (f32, f32, f32) {
        match self {
            CubeFace::PosX => (1.0, v, u),
            CubeFace::NegX => (-1.0, v, -u),
            CubeFace::PosY => (u, 1.0, v),
            CubeFace::NegY => (u, -1.0, -v),
            CubeFace::PosZ => (-u, v, 1.0),
            CubeFace::NegZ => (u, v, -1.0),
        }
    }

    /// All six cube faces.
    pub const ALL: [CubeFace; 6] = [
        CubeFace::PosX,
        CubeFace::NegX,
        CubeFace::PosY,
        CubeFace::NegY,
        CubeFace::PosZ,
        CubeFace::NegZ,
    ];
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
    // Monthly climate (12-element arrays per tile)
    pub temperature_monthly_c: Vec<[f32; 12]>,
    pub precipitation_monthly_mm: Vec<[f32; 12]>,
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
                temperature_monthly_c: vec![[0.0; 12]; n],
                precipitation_monthly_mm: vec![[0.0; 12]; n],
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
    // Sphere sampling API
    //
    // Let engines look up tile data from 3D positions or cube-face (u, v)
    // coords without having to convert to lat/lon themselves.
    // -----------------------------------------------------------------------

    /// Sample the grid at a normalized 3D point on the unit sphere.
    ///
    /// Convention: `+y` is north pole, `-y` south pole, `+x` is longitude
    /// 0°, and longitude runs counter-clockwise looking down from +y. The
    /// input is automatically normalized.
    pub fn sample_xyz(&self, x: f32, y: f32, z: f32) -> usize {
        let len = (x * x + y * y + z * z).sqrt().max(1e-6);
        let (nx, ny, nz) = (x / len, y / len, z / len);
        let lat_deg = ny.clamp(-1.0, 1.0).asin().to_degrees();
        let lon_deg = nz.atan2(nx).to_degrees();
        self.idx_latlon(lat_deg, lon_deg)
    }

    /// Sample at a cube-face `(u, v)` position where `u, v ∈ [-1, 1]`.
    ///
    /// Projects the face point onto the sphere and samples there. Six
    /// faces cover the sphere with nearly-uniform area per face cell,
    /// which is why engines prefer this layout over equirectangular.
    pub fn sample_cube_face(&self, face: CubeFace, u: f32, v: f32) -> usize {
        let (x, y, z) = face.to_xyz(u, v);
        self.sample_xyz(x, y, z)
    }

    /// Physical area of one tile at latitude row `row` in km².
    ///
    /// Assumes a spherical body of radius `planet_radius_km`. Tile area
    /// drops with `cos(latitude)` on the equirectangular projection.
    pub fn tile_area_km2(&self, row: u16, planet_radius_km: f32) -> f32 {
        let lat_rad = self.row_latitude(row).to_radians();
        let dlat_rad = std::f32::consts::PI / self.height as f32;
        let dlon_rad = 2.0 * std::f32::consts::PI / self.width as f32;
        planet_radius_km * planet_radius_km * lat_rad.cos().abs() * dlat_rad * dlon_rad
    }

    /// Total surface area of the body in km², summed over all tiles.
    pub fn surface_area_km2(&self, planet_radius_km: f32) -> f32 {
        (0..self.height)
            .map(|r| self.tile_area_km2(r, planet_radius_km) * self.width as f32)
            .sum()
    }

    // -----------------------------------------------------------------------
    // Query helpers
    //
    // Area-weighted statistics and spatial lookups over an already-populated
    // grid. These are derived values — compute once and cache if you plan
    // to query repeatedly.
    // -----------------------------------------------------------------------

    /// Area-weighted biome fractions (summing to 1.0) across the whole grid.
    /// Weights each tile by `cos(lat)` so polar pinching in the
    /// equirectangular projection doesn't skew the result.
    pub fn biome_distribution(&self) -> std::collections::HashMap<crate::types::BiomeType, f32> {
        let mut totals: std::collections::HashMap<crate::types::BiomeType, f32> =
            std::collections::HashMap::new();
        let mut grand_total = 0.0f32;
        for r in 0..self.height {
            let lat = self.row_latitude(r);
            let weight = lat.to_radians().cos().abs();
            for c in 0..self.width {
                let idx = self.idx(c, r);
                *totals.entry(self.layers.biome[idx]).or_insert(0.0) += weight;
                grand_total += weight;
            }
        }
        if grand_total > 0.0 {
            for v in totals.values_mut() {
                *v /= grand_total;
            }
        }
        totals
    }

    /// Total land area in km² for a body with the given radius.
    pub fn total_land_area_km2(&self, planet_radius_km: f32) -> f32 {
        let mut total = 0.0f32;
        for r in 0..self.height {
            let tile_area = self.tile_area_km2(r, planet_radius_km);
            for c in 0..self.width {
                let idx = self.idx(c, r);
                if !self.layers.is_ocean[idx] {
                    total += tile_area;
                }
            }
        }
        total
    }

    /// `(col, row)` of the ocean tile closest to `(lat_deg, lon_deg)` by
    /// great-circle distance. Returns `None` if the world has no ocean.
    pub fn nearest_ocean_tile(&self, lat_deg: f32, lon_deg: f32) -> Option<(u16, u16)> {
        let lat1 = lat_deg.to_radians();
        let lon1 = lon_deg.to_radians();
        let mut best: Option<(u16, u16, f32)> = None;
        for r in 0..self.height {
            for c in 0..self.width {
                let idx = self.idx(c, r);
                if !self.layers.is_ocean[idx] {
                    continue;
                }
                let lat2 = self.row_latitude(r).to_radians();
                let lon2 = self.col_longitude(c).to_radians();
                let dlon = lon2 - lon1;
                let d = (lat1.sin() * lat2.sin() + lat1.cos() * lat2.cos() * dlon.cos())
                    .clamp(-1.0, 1.0)
                    .acos();
                match best {
                    None => best = Some((c, r, d)),
                    Some((_, _, bd)) if d < bd => best = Some((c, r, d)),
                    _ => {}
                }
            }
        }
        best.map(|(c, r, _)| (c, r))
    }

    /// The river with the highest mouth-discharge: its tiles (source
    /// first) and discharge in m³/s. `None` if no rivers exist.
    pub fn longest_river(&self) -> Option<(Vec<usize>, f32)> {
        let rivers = crate::features::detect_rivers(self);
        rivers
            .into_iter()
            .max_by(|a, b| {
                a.max_discharge_m3s
                    .partial_cmp(&b.max_discharge_m3s)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| (r.tiles, r.max_discharge_m3s))
    }

    /// Tiles of the largest contiguous mountain range, by tile count.
    pub fn largest_mountain_range(&self) -> Option<Vec<usize>> {
        crate::features::detect_mountain_ranges(self)
            .into_iter()
            .max_by_key(|r| r.tiles.len())
            .map(|r| r.tiles)
    }

    // -----------------------------------------------------------------------
    // LOD — region zoom
    //
    // Generates a higher-resolution sub-grid for a lat/lon bounding box
    // by bilinearly interpolating the parent grid's continuous layers and
    // nearest-neighbour copying discrete layers. Optionally adds extra
    // fractal noise to elevation for fine detail.
    // -----------------------------------------------------------------------

    /// Extract a zoomed sub-grid covering a rectangular region.
    ///
    /// - `lon_min`, `lon_max` in degrees (−180 to 180).
    /// - `lat_min`, `lat_max` in degrees (−90 to 90), `lat_min < lat_max`.
    /// - `factor`: resolution multiplier relative to the parent grid's
    ///   tile density (e.g., 4 means each parent tile becomes 4×4 sub-tiles).
    /// - `seed`: deterministic seed for extra elevation noise.
    ///
    /// The returned grid is a standalone `SurfaceGrid` whose coordinate
    /// system covers only the specified region. `row_latitude` / `col_longitude`
    /// on the sub-grid return positions within the parent's coordinate space.
    pub fn zoom_region(
        &self,
        lon_min: f32,
        lat_min: f32,
        lon_max: f32,
        lat_max: f32,
        factor: u16,
        seed: &str,
    ) -> SurfaceGrid {
        let lon_span = lon_max - lon_min;
        let lat_span = lat_max - lat_min;
        // Compute sub-grid dimensions from parent density × factor.
        let tiles_per_deg_lon = self.width as f32 / 360.0;
        let tiles_per_deg_lat = self.height as f32 / 180.0;
        let sub_w = ((lon_span * tiles_per_deg_lon * factor as f32).round() as u16).max(2);
        let sub_h = ((lat_span * tiles_per_deg_lat * factor as f32).round() as u16).max(2);
        let n = sub_w as usize * sub_h as usize;

        let mut sub = SurfaceGrid::empty(GridResolution::Custom(sub_w, sub_h));
        sub.sea_level_m = self.sea_level_m;

        // For each sub-tile, compute the lat/lon, then bilinear-sample the parent.
        for sr in 0..sub_h {
            // Latitude: sub row 0 = lat_max (north), last row = lat_min (south).
            let lat = lat_max - (sr as f32 + 0.5) / sub_h as f32 * lat_span;
            for sc in 0..sub_w {
                let lon = lon_min + (sc as f32 + 0.5) / sub_w as f32 * lon_span;
                let si = sr as usize * sub_w as usize + sc as usize;

                // Parent grid fractional coordinates.
                let frac_r = ((90.0 - lat) / 180.0) * self.height as f32 - 0.5;
                let frac_c = ((lon + 180.0) / 360.0).rem_euclid(1.0) * self.width as f32 - 0.5;

                // Four corner indices for bilinear interpolation.
                let r0 = (frac_r.floor() as i32).clamp(0, self.height as i32 - 1) as u16;
                let r1 = (r0 + 1).min(self.height - 1);
                let c0 = (frac_c.floor() as i32).rem_euclid(self.width as i32) as u16;
                let c1 = (c0 + 1) % self.width;
                let tr = frac_r - frac_r.floor(); // vertical blend
                let tc = frac_c - frac_c.floor(); // horizontal blend

                let i00 = self.idx(c0, r0);
                let i01 = self.idx(c1, r0);
                let i10 = self.idx(c0, r1);
                let i11 = self.idx(c1, r1);

                // Bilinear helper for f32 layers.
                macro_rules! bilerp {
                    ($layer:expr) => {{
                        let v00 = $layer[i00];
                        let v01 = $layer[i01];
                        let v10 = $layer[i10];
                        let v11 = $layer[i11];
                        let top = v00 + (v01 - v00) * tc;
                        let bot = v10 + (v11 - v10) * tc;
                        top + (bot - top) * tr
                    }};
                }

                // Nearest-neighbour index (whichever corner is closest).
                let nn = if tr < 0.5 {
                    if tc < 0.5 {
                        i00
                    } else {
                        i01
                    }
                } else if tc < 0.5 {
                    i10
                } else {
                    i11
                };

                sub.layers.elevation_m[si] = bilerp!(self.layers.elevation_m);
                sub.layers.temperature_c[si] = bilerp!(self.layers.temperature_c);
                sub.layers.temperature_summer_c[si] = bilerp!(self.layers.temperature_summer_c);
                sub.layers.temperature_winter_c[si] = bilerp!(self.layers.temperature_winter_c);
                sub.layers.precipitation_mm[si] = bilerp!(self.layers.precipitation_mm);
                sub.layers.humidity_relative[si] = bilerp!(self.layers.humidity_relative);
                sub.layers.wind_speed_ms[si] = bilerp!(self.layers.wind_speed_ms);
                sub.layers.wind_direction_deg[si] = bilerp!(self.layers.wind_direction_deg);
                sub.layers.sea_surface_temp_c[si] = bilerp!(self.layers.sea_surface_temp_c);
                sub.layers.river_discharge_m3s[si] = bilerp!(self.layers.river_discharge_m3s);

                // Discrete layers: nearest neighbour.
                sub.layers.plate_id[si] = self.layers.plate_id[nn];
                sub.layers.is_ocean[si] = self.layers.is_ocean[nn];
                sub.layers.tectonic_boundary[si] = self.layers.tectonic_boundary[nn];
                sub.layers.biome[si] = self.layers.biome[nn];
                sub.layers.koppen_class[si] = self.layers.koppen_class[nn];
                sub.layers.drainage_basin_id[si] = self.layers.drainage_basin_id[nn];
                sub.layers.flow_accumulation[si] = self.layers.flow_accumulation[nn];
                sub.layers.pet_ratio[si] = self.layers.pet_ratio[nn];
                sub.layers.ocean_current_direction_deg[si] =
                    self.layers.ocean_current_direction_deg[nn];
                sub.layers.ocean_current_speed_ms[si] = self.layers.ocean_current_speed_ms[nn];
                sub.layers.temperature_monthly_c[si] = self.layers.temperature_monthly_c[nn];
                sub.layers.precipitation_monthly_mm[si] = self.layers.precipitation_monthly_mm[nn];
            }
        }

        // Add extra fractal noise to elevation for fine detail.
        use noise::{NoiseFn, SuperSimplex};
        let seed_hash = seed
            .bytes()
            .fold(0u32, |a, b| a.wrapping_mul(31).wrapping_add(b as u32));
        let detail_noise = SuperSimplex::new(seed_hash);
        let detail_amplitude = 150.0f32; // subtle detail (metres)
        let detail_freq = 8.0f64 * factor as f64;
        for sr in 0..sub_h {
            let ny = sr as f64 / sub_h as f64;
            for sc in 0..sub_w {
                let nx = sc as f64 / sub_w as f64;
                let si = sr as usize * sub_w as usize + sc as usize;
                let noise_val = detail_noise.get([nx * detail_freq, ny * detail_freq]) as f32;
                sub.layers.elevation_m[si] += noise_val * detail_amplitude;
            }
        }

        sub
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
///
/// ```
/// use world::grid::{generate_surface_grid, GridResolution};
/// use world::types::{PlanetSimulationInput, StarContext, OrbitContext};
///
/// let input = PlanetSimulationInput {
///     body_id: 1,
///     body_radius_earth: 1.0,
///     blackbody_temp_k: 255,
///     star: StarContext { age_gyr: 4.6, ..Default::default() },
///     orbit: OrbitContext { axial_tilt_deg: 23.4, ..Default::default() },
///     ..Default::default()
/// };
/// let grid = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "earth");
/// assert!(grid.tile_count() > 0);
/// ```
pub fn generate_surface_grid(
    context: &crate::types::PlanetSimulationInput,
    greenhouse_delta_k: f32,
    atmospheric_pressure: f32,
    hydrosphere_pct: f32,
    resolution: GridResolution,
    seed: &str,
) -> SurfaceGrid {
    let mut grid = crate::geology::generate_geology(context, hydrosphere_pct, resolution, seed);
    #[cfg(feature = "erosion")]
    {
        crate::erosion::erode(&mut grid, crate::erosion::ErosionParams::default(), seed);
        grid.sea_level_m = crate::geology::find_sea_level(&mut grid, hydrosphere_pct);
    }
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
    crate::climate::generate_monthly_climate(context, &mut grid);
    crate::climate::generate_biomes(&mut grid);
    grid
}

#[cfg(test)]
mod sampling_tests {
    use super::*;

    fn empty_grid() -> SurfaceGrid {
        SurfaceGrid::empty(GridResolution::Custom(72, 36))
    }

    #[test]
    fn sample_xyz_equator_positive_x() {
        let g = empty_grid();
        // +x direction at equator → longitude 0°, latitude 0°.
        let idx = g.sample_xyz(1.0, 0.0, 0.0);
        let row = idx / g.width as usize;
        let lat = g.row_latitude(row as u16);
        assert!(lat.abs() < 5.0, "got lat {}", lat);
    }

    #[test]
    fn sample_xyz_north_pole() {
        let g = empty_grid();
        // +y direction → north pole.
        let idx = g.sample_xyz(0.0, 1.0, 0.0);
        let row = idx / g.width as usize;
        assert!(row <= 1, "north pole should be row 0 or 1, got {}", row);
    }

    #[test]
    fn sample_xyz_south_pole() {
        let g = empty_grid();
        // -y direction → south pole.
        let idx = g.sample_xyz(0.0, -1.0, 0.0);
        let row = idx / g.width as usize;
        assert!(row >= 34, "south pole should be near last row, got {}", row);
    }

    #[test]
    fn sample_xyz_normalizes_input() {
        let g = empty_grid();
        // Unnormalized input should give the same tile as normalized.
        let a = g.sample_xyz(5.0, 0.0, 0.0);
        let b = g.sample_xyz(1.0, 0.0, 0.0);
        assert_eq!(a, b);
    }

    #[test]
    fn all_cube_faces_map_to_valid_tiles() {
        let g = empty_grid();
        for &face in &CubeFace::ALL {
            for u in [-1.0, -0.5, 0.0, 0.5, 1.0] {
                for v in [-1.0, -0.5, 0.0, 0.5, 1.0] {
                    let idx = g.sample_cube_face(face, u, v);
                    assert!(idx < g.tile_count());
                }
            }
        }
    }

    #[test]
    fn cube_face_centres_match_face_axis() {
        let g = empty_grid();
        // PosY face centre → north pole.
        let posy = g.sample_cube_face(CubeFace::PosY, 0.0, 0.0);
        let row = posy / g.width as usize;
        assert!(row <= 1);
        // NegY face centre → south pole.
        let negy = g.sample_cube_face(CubeFace::NegY, 0.0, 0.0);
        let row = negy / g.width as usize;
        assert!(row >= 34);
    }

    #[test]
    fn tile_area_decreases_toward_poles() {
        let g = empty_grid();
        let equator = g.tile_area_km2(18, 6371.0);
        let pole = g.tile_area_km2(0, 6371.0);
        assert!(
            equator > pole * 2.0,
            "equator area {} should be ≫ pole area {}",
            equator,
            pole
        );
    }

    #[test]
    fn surface_area_sums_to_earth_value() {
        let g = empty_grid();
        // Earth's surface area is ~510.1 million km².
        let total = g.surface_area_km2(6371.0);
        let expected = 4.0 * std::f32::consts::PI * 6371.0 * 6371.0;
        let error = ((total - expected) / expected).abs();
        assert!(
            error < 0.01,
            "total {} differs from expected {} by {}%",
            total,
            expected,
            error * 100.0
        );
    }

    #[test]
    fn sampling_is_stable_for_identical_inputs() {
        let g = empty_grid();
        for (x, y, z) in [(1.0, 0.0, 0.0), (0.5, 0.5, 0.5), (-0.3, 0.7, 0.2)] {
            assert_eq!(g.sample_xyz(x, y, z), g.sample_xyz(x, y, z));
        }
    }

    #[test]
    fn cube_face_xyz_produces_normalizable_vectors() {
        for &face in &CubeFace::ALL {
            let (x, y, z) = face.to_xyz(0.0, 0.0);
            let len = (x * x + y * y + z * z).sqrt();
            assert!(len > 0.9);
        }
    }
}

#[cfg(test)]
mod query_tests {
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
        generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "query")
    }

    #[test]
    fn biome_distribution_sums_to_one() {
        let g = earth_grid();
        let dist = g.biome_distribution();
        let sum: f32 = dist.values().sum();
        assert!((sum - 1.0).abs() < 1e-3, "sum = {}", sum);
    }

    #[test]
    fn biome_distribution_ocean_matches_hydrosphere() {
        let g = earth_grid();
        let dist = g.biome_distribution();
        let ocean = dist
            .get(&crate::types::BiomeType::Ocean)
            .copied()
            .unwrap_or(0.0);
        // Target was 71% ocean; allow ±15% slop from simplex noise.
        assert!(
            (0.55..=0.85).contains(&ocean),
            "ocean fraction {} out of plausible range",
            ocean
        );
    }

    #[test]
    fn total_land_area_reasonable() {
        let g = earth_grid();
        let land = g.total_land_area_km2(6371.0);
        // Earth total 510M km² × 29% land ≈ 148M km².
        // Our Earth-like should be 15%-45% (~80-230M km²).
        assert!(
            (80e6..=230e6).contains(&land),
            "land area {} km² out of range",
            land
        );
    }

    #[test]
    fn land_and_ocean_sum_to_total_surface() {
        let g = earth_grid();
        let land = g.total_land_area_km2(6371.0);
        let total = g.surface_area_km2(6371.0);
        assert!(land < total);
        // land + ocean = total. Ocean area = total - land.
        let ocean_frac = 1.0 - land / total;
        assert!((0.5..=0.9).contains(&ocean_frac));
    }

    #[test]
    fn nearest_ocean_tile_is_ocean() {
        let g = earth_grid();
        let (c, r) = g.nearest_ocean_tile(0.0, 0.0).expect("earth has ocean");
        let idx = g.idx(c, r);
        assert!(g.layers.is_ocean[idx]);
    }

    #[test]
    fn nearest_ocean_from_ocean_is_itself_or_same_basin() {
        let g = earth_grid();
        // Find any ocean tile.
        let oceanic = g
            .layers
            .is_ocean
            .iter()
            .position(|&o| o)
            .expect("has ocean");
        let r = (oceanic / g.width as usize) as u16;
        let c = (oceanic % g.width as usize) as u16;
        let lat = g.row_latitude(r);
        let lon = g.col_longitude(c);
        let (nc, nr) = g.nearest_ocean_tile(lat, lon).unwrap();
        let ni = g.idx(nc, nr);
        // Could be this tile or any neighbour; just check it's ocean.
        assert!(g.layers.is_ocean[ni]);
    }

    #[test]
    fn longest_river_has_positive_discharge() {
        let g = earth_grid();
        if let Some((tiles, discharge)) = g.longest_river() {
            assert!(!tiles.is_empty());
            assert!(discharge > 0.0);
        }
    }

    #[test]
    fn largest_mountain_range_has_tiles() {
        let g = earth_grid();
        if let Some(tiles) = g.largest_mountain_range() {
            assert!(!tiles.is_empty());
        }
    }

    #[test]
    fn dry_world_has_no_nearest_ocean() {
        // An empty grid has is_ocean all false — nearest_ocean_tile must return None.
        let g = SurfaceGrid::empty(GridResolution::Fast);
        assert!(g.nearest_ocean_tile(0.0, 0.0).is_none());
    }
}

#[cfg(test)]
mod lod_tests {
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
        generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "lod")
    }

    #[test]
    fn zoom_region_has_higher_resolution() {
        let g = earth_grid();
        let sub = g.zoom_region(-10.0, -10.0, 10.0, 10.0, 4, "zoom");
        // 20° lon × 20° lat at 4× Fast (72/360 = 0.2 tiles/°)
        // sub_w ≈ 20 * 0.2 * 4 = 16, sub_h ≈ 20 * 0.2 * 4 = 16
        assert!(sub.tile_count() > 1);
        assert!(sub.width >= 4);
        assert!(sub.height >= 4);
    }

    #[test]
    fn zoom_preserves_climate_continuity() {
        let g = earth_grid();
        // Zoom into a tropical region.
        let sub = g.zoom_region(-30.0, -15.0, 30.0, 15.0, 2, "cont");
        // Centre of the parent at (0°, 0°):
        let parent_idx = g.idx_latlon(0.0, 0.0);
        let parent_temp = g.layers.temperature_c[parent_idx];
        // Centre of the sub-grid:
        let sub_centre = sub.idx(sub.width / 2, sub.height / 2);
        let sub_temp = sub.layers.temperature_c[sub_centre];
        // Should be close (bilinear interpolation + small noise).
        assert!(
            (parent_temp - sub_temp).abs() < 5.0,
            "parent temp {} vs sub temp {} differ too much",
            parent_temp,
            sub_temp
        );
    }

    #[test]
    fn zoom_is_deterministic() {
        let g = earth_grid();
        let a = g.zoom_region(0.0, 0.0, 30.0, 30.0, 3, "det");
        let b = g.zoom_region(0.0, 0.0, 30.0, 30.0, 3, "det");
        assert_eq!(a.layers.elevation_m, b.layers.elevation_m);
        assert_eq!(a.layers.temperature_c, b.layers.temperature_c);
    }

    #[test]
    fn zoom_different_seeds_differ() {
        let g = earth_grid();
        let a = g.zoom_region(0.0, 0.0, 30.0, 30.0, 3, "seed_a");
        let b = g.zoom_region(0.0, 0.0, 30.0, 30.0, 3, "seed_b");
        // Elevation should differ due to different detail noise.
        assert_ne!(a.layers.elevation_m, b.layers.elevation_m);
    }

    #[test]
    fn zoom_preserves_ocean_flag() {
        let g = earth_grid();
        // Zoom into a region that has mixed ocean/land.
        let sub = g.zoom_region(-180.0, -90.0, 180.0, 90.0, 1, "full");
        let has_ocean = sub.layers.is_ocean.iter().any(|&o| o);
        let has_land = sub.layers.is_ocean.iter().any(|&o| !o);
        assert!(has_ocean, "zoomed full grid should have ocean");
        assert!(has_land, "zoomed full grid should have land");
    }
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
