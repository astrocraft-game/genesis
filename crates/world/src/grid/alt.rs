//! Alternative grid layouts for sphere surfaces.
//!
//! Provides `HexGrid` (icosahedron-based hex tiles) and `CubeSphereGrid`
//! (6-face cube-sphere), plus the `SurfaceSampler` trait that all grid
//! types implement. The equirectangular `SurfaceGrid` also implements the
//! trait, so callers can program against `dyn SurfaceSampler`.

use crate::grid::{CubeFace, SurfaceGrid};
use crate::types::BiomeType;

// ---------------------------------------------------------------------------
// TileData — lightweight snapshot returned by the sampler trait
// ---------------------------------------------------------------------------

/// A lightweight snapshot of one tile's physical properties. Returned by
/// `SurfaceSampler::sample_at` so callers don't need to know the backing
/// grid layout.
#[derive(Clone, Debug, Default)]
pub struct TileData {
    pub elevation_m: f32,
    pub temperature_c: f32,
    pub precipitation_mm: f32,
    pub biome: BiomeType,
    pub is_ocean: bool,
}

// ---------------------------------------------------------------------------
// SurfaceSampler trait
// ---------------------------------------------------------------------------

/// Uniform interface for sampling a planetary surface at a 3D direction.
///
/// Implementations convert the unit-sphere direction to whichever internal
/// coordinate system the grid uses, then return the tile data there.
pub trait SurfaceSampler {
    /// Number of tiles in this grid.
    fn tile_count(&self) -> usize;

    /// Sample the grid at a normalized 3D direction on the unit sphere.
    /// The input does NOT need to be pre-normalized.
    fn sample_at(&self, x: f32, y: f32, z: f32) -> TileData;
}

// ---------------------------------------------------------------------------
// SurfaceGrid implements the trait
// ---------------------------------------------------------------------------

impl SurfaceSampler for SurfaceGrid {
    fn tile_count(&self) -> usize {
        self.width as usize * self.height as usize
    }

    fn sample_at(&self, x: f32, y: f32, z: f32) -> TileData {
        let idx = self.sample_xyz(x, y, z);
        TileData {
            elevation_m: self.layers.elevation_m[idx],
            temperature_c: self.layers.temperature_c[idx],
            precipitation_mm: self.layers.precipitation_mm[idx],
            biome: self.layers.biome[idx],
            is_ocean: self.layers.is_ocean[idx],
        }
    }
}

// ---------------------------------------------------------------------------
// HexGrid — icosahedron-based hex tiles
// ---------------------------------------------------------------------------

/// A spherical grid based on subdivided icosahedron faces, producing
/// approximately uniform hexagonal tiles (plus 12 pentagonal vertices).
///
/// `subdivisions` controls resolution: total tiles ≈ 10 × 4^sub + 2.
/// Sub=0 → 12 vertices; sub=4 → ~2562 tiles; sub=6 → ~40962 tiles.
#[derive(Clone, Debug)]
pub struct HexGrid {
    /// Subdivision level (0 = icosahedron vertices only).
    pub subdivisions: u8,
    /// Per-tile 3D positions on the unit sphere.
    pub positions: Vec<(f32, f32, f32)>,
    /// Per-tile data.
    pub tiles: Vec<TileData>,
}

impl HexGrid {
    /// Create an empty hex grid at the given subdivision level.
    pub fn new(subdivisions: u8) -> Self {
        let positions = generate_icosphere_vertices(subdivisions);
        let n = positions.len();
        Self {
            subdivisions,
            positions,
            tiles: vec![TileData::default(); n],
        }
    }

    /// Populate this hex grid by sampling an existing equirectangular grid.
    pub fn from_surface_grid(grid: &SurfaceGrid, subdivisions: u8) -> Self {
        let mut hex = Self::new(subdivisions);
        for (i, &(x, y, z)) in hex.positions.iter().enumerate() {
            hex.tiles[i] = grid.sample_at(x, y, z);
        }
        hex
    }
}

impl SurfaceSampler for HexGrid {
    fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    fn sample_at(&self, x: f32, y: f32, z: f32) -> TileData {
        let len = (x * x + y * y + z * z).sqrt().max(1e-6);
        let (nx, ny, nz) = (x / len, y / len, z / len);
        // Find nearest tile by dot product (brute force for now).
        let mut best = 0;
        let mut best_dot = f32::NEG_INFINITY;
        for (i, &(px, py, pz)) in self.positions.iter().enumerate() {
            let dot = nx * px + ny * py + nz * pz;
            if dot > best_dot {
                best_dot = dot;
                best = i;
            }
        }
        self.tiles[best].clone()
    }
}

/// Generate approximately-uniform vertices on the unit sphere via
/// recursive icosahedron subdivision.
fn generate_icosphere_vertices(subdivisions: u8) -> Vec<(f32, f32, f32)> {
    // Start with the 12 icosahedron vertices.
    let t = (1.0 + 5.0f32.sqrt()) / 2.0;
    let base = vec![
        (-1.0, t, 0.0),
        (1.0, t, 0.0),
        (-1.0, -t, 0.0),
        (1.0, -t, 0.0),
        (0.0, -1.0, t),
        (0.0, 1.0, t),
        (0.0, -1.0, -t),
        (0.0, 1.0, -t),
        (t, 0.0, -1.0),
        (t, 0.0, 1.0),
        (-t, 0.0, -1.0),
        (-t, 0.0, 1.0),
    ];

    // 20 icosahedron faces (vertex index triples).
    let faces: Vec<[usize; 3]> = vec![
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];

    let mut verts = base;
    let mut tris = faces;

    // Cache for midpoint indices to avoid duplicates.
    for _ in 0..subdivisions {
        let mut midpoint_cache: std::collections::HashMap<(usize, usize), usize> =
            std::collections::HashMap::new();
        let mut new_tris = Vec::with_capacity(tris.len() * 4);

        for tri in &tris {
            let a = tri[0];
            let b = tri[1];
            let c = tri[2];
            let ab = get_midpoint(a, b, &mut verts, &mut midpoint_cache);
            let bc = get_midpoint(b, c, &mut verts, &mut midpoint_cache);
            let ca = get_midpoint(c, a, &mut verts, &mut midpoint_cache);
            new_tris.push([a, ab, ca]);
            new_tris.push([b, bc, ab]);
            new_tris.push([c, ca, bc]);
            new_tris.push([ab, bc, ca]);
        }
        tris = new_tris;
    }

    // Normalize all vertices to unit sphere.
    for v in &mut verts {
        let len = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt();
        v.0 /= len;
        v.1 /= len;
        v.2 /= len;
    }

    verts
}

fn get_midpoint(
    a: usize,
    b: usize,
    verts: &mut Vec<(f32, f32, f32)>,
    cache: &mut std::collections::HashMap<(usize, usize), usize>,
) -> usize {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(&idx) = cache.get(&key) {
        return idx;
    }
    let va = verts[a];
    let vb = verts[b];
    let mid = (
        (va.0 + vb.0) / 2.0,
        (va.1 + vb.1) / 2.0,
        (va.2 + vb.2) / 2.0,
    );
    let idx = verts.len();
    verts.push(mid);
    cache.insert(key, idx);
    idx
}

// ---------------------------------------------------------------------------
// CubeSphereGrid — 6-face cube projected onto a sphere
// ---------------------------------------------------------------------------

/// A cube-sphere grid with `resolution × resolution` tiles per face.
/// Total tiles = `6 × resolution²`.
#[derive(Clone, Debug)]
pub struct CubeSphereGrid {
    /// Tiles per face edge.
    pub resolution: u16,
    /// Per-tile data, laid out as `face * resolution² + row * resolution + col`.
    pub tiles: Vec<TileData>,
}

impl CubeSphereGrid {
    /// Create an empty cube-sphere grid.
    pub fn new(resolution: u16) -> Self {
        let n = 6 * resolution as usize * resolution as usize;
        Self {
            resolution,
            tiles: vec![TileData::default(); n],
        }
    }

    /// Populate by sampling an existing equirectangular grid.
    pub fn from_surface_grid(grid: &SurfaceGrid, resolution: u16) -> Self {
        let mut cs = Self::new(resolution);
        let res = resolution as usize;
        for (fi, face) in CubeFace::ALL.iter().enumerate() {
            for row in 0..res {
                for col in 0..res {
                    let u = -1.0 + (2.0 * col as f32 + 1.0) / res as f32;
                    let v = -1.0 + (2.0 * row as f32 + 1.0) / res as f32;
                    let (x, y, z) = face.to_xyz(u, v);
                    let idx = fi * res * res + row * res + col;
                    cs.tiles[idx] = grid.sample_at(x, y, z);
                }
            }
        }
        cs
    }

    /// Face, row, col from a flat index.
    fn decompose(&self, idx: usize) -> (usize, usize, usize) {
        let res = self.resolution as usize;
        let face = idx / (res * res);
        let rem = idx % (res * res);
        (face, rem / res, rem % res)
    }
}

impl SurfaceSampler for CubeSphereGrid {
    fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    fn sample_at(&self, x: f32, y: f32, z: f32) -> TileData {
        let len = (x * x + y * y + z * z).sqrt().max(1e-6);
        let (nx, ny, nz) = (x / len, y / len, z / len);
        let ax = nx.abs();
        let ay = ny.abs();
        let az = nz.abs();

        // Determine dominant face.
        let (face, u, v) = if ax >= ay && ax >= az {
            if nx > 0.0 {
                (CubeFace::PosX, nz / ax, ny / ax)
            } else {
                (CubeFace::NegX, -nz / ax, ny / ax)
            }
        } else if ay >= ax && ay >= az {
            if ny > 0.0 {
                (CubeFace::PosY, nx / ay, nz / ay)
            } else {
                (CubeFace::NegY, nx / ay, -nz / ay)
            }
        } else if nz > 0.0 {
            (CubeFace::PosZ, -nx / az, ny / az)
        } else {
            (CubeFace::NegZ, nx / az, ny / az)
        };

        let fi = CubeFace::ALL.iter().position(|&f| f == face).unwrap_or(0);
        let res = self.resolution as usize;
        let col = ((u + 1.0) / 2.0 * res as f32).clamp(0.0, res as f32 - 1.0) as usize;
        let row = ((v + 1.0) / 2.0 * res as f32).clamp(0.0, res as f32 - 1.0) as usize;
        let idx = fi * res * res + row * res + col;
        self.tiles[idx].clone()
    }
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

/// Convert an equirectangular `SurfaceGrid` to a `HexGrid` at the given
/// subdivision level.
pub fn equirect_to_hex(grid: &SurfaceGrid, subdivisions: u8) -> HexGrid {
    HexGrid::from_surface_grid(grid, subdivisions)
}

/// Convert an equirectangular `SurfaceGrid` to a `CubeSphereGrid` at the
/// given per-face resolution.
pub fn equirect_to_cube(grid: &SurfaceGrid, resolution: u16) -> CubeSphereGrid {
    CubeSphereGrid::from_surface_grid(grid, resolution)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::GridResolution;

    fn earth_grid() -> SurfaceGrid {
        use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};
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
        crate::grid::generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "grids")
    }

    #[test]
    fn hex_grid_tile_count_matches_formula() {
        // 10 × 4^sub + 2 vertices for an icosphere.
        let h = HexGrid::new(0);
        assert_eq!(h.tile_count(), 12); // icosahedron vertices
        let h2 = HexGrid::new(2);
        assert_eq!(h2.tile_count(), 162); // 10*16 + 2
        let h4 = HexGrid::new(4);
        assert_eq!(h4.tile_count(), 2562); // 10*256 + 2
    }

    #[test]
    fn cube_sphere_tile_count_is_6_res_sq() {
        let cs = CubeSphereGrid::new(10);
        assert_eq!(cs.tile_count(), 600);
    }

    #[test]
    fn surface_grid_implements_sampler() {
        let g = earth_grid();
        let sampler: &dyn SurfaceSampler = &g;
        let data = sampler.sample_at(1.0, 0.0, 0.0);
        // Equator at lon=0 should return valid data.
        assert!(data.temperature_c.is_finite());
    }

    #[test]
    fn hex_from_surface_preserves_biome() {
        let g = earth_grid();
        let hex = equirect_to_hex(&g, 3);
        // Every hex tile should have a biome assigned from the source.
        let has_ocean = hex.tiles.iter().any(|t| t.is_ocean);
        let has_land = hex.tiles.iter().any(|t| !t.is_ocean);
        assert!(has_ocean, "hex should contain ocean tiles");
        assert!(has_land, "hex should contain land tiles");
    }

    #[test]
    fn cube_from_surface_preserves_biome() {
        let g = earth_grid();
        let cs = equirect_to_cube(&g, 12);
        let has_ocean = cs.tiles.iter().any(|t| t.is_ocean);
        let has_land = cs.tiles.iter().any(|t| !t.is_ocean);
        assert!(has_ocean, "cube should contain ocean tiles");
        assert!(has_land, "cube should contain land tiles");
    }

    #[test]
    fn hex_sampler_returns_valid_data() {
        let g = earth_grid();
        let hex = equirect_to_hex(&g, 3);
        let sampler: &dyn SurfaceSampler = &hex;
        let data = sampler.sample_at(0.0, 1.0, 0.0); // north pole
        assert!(data.temperature_c.is_finite());
    }

    #[test]
    fn cube_sampler_returns_valid_data() {
        let g = earth_grid();
        let cs = equirect_to_cube(&g, 10);
        let sampler: &dyn SurfaceSampler = &cs;
        let data = sampler.sample_at(1.0, 0.0, 0.0);
        assert!(data.temperature_c.is_finite());
    }

    #[test]
    fn same_seed_produces_equivalent_coverage() {
        // Sampling the same direction on all three grid types from the
        // same source should give the same biome.
        let g = earth_grid();
        let hex = equirect_to_hex(&g, 4);
        let cs = equirect_to_cube(&g, 18);

        let dir = (0.5f32, 0.3, 0.7);
        let equirect_data = g.sample_at(dir.0, dir.1, dir.2);
        let hex_data = hex.sample_at(dir.0, dir.1, dir.2);
        let cs_data = cs.sample_at(dir.0, dir.1, dir.2);

        // Due to resolution differences the biome might differ at edge
        // tiles, but elevation should be roughly consistent.
        assert!(
            (equirect_data.elevation_m - hex_data.elevation_m).abs() < 1000.0,
            "hex elevation {} too far from equirect {}",
            hex_data.elevation_m,
            equirect_data.elevation_m,
        );
        assert!(
            (equirect_data.elevation_m - cs_data.elevation_m).abs() < 1000.0,
            "cube elevation {} too far from equirect {}",
            cs_data.elevation_m,
            equirect_data.elevation_m,
        );
    }

    #[test]
    fn hex_positions_are_on_unit_sphere() {
        let hex = HexGrid::new(3);
        for &(x, y, z) in &hex.positions {
            let len = (x * x + y * y + z * z).sqrt();
            assert!(
                (len - 1.0).abs() < 1e-4,
                "vertex not on unit sphere: len={}",
                len
            );
        }
    }

    #[test]
    fn cube_decompose_round_trips() {
        let cs = CubeSphereGrid::new(8);
        for idx in 0..cs.tile_count() {
            let (face, row, col) = cs.decompose(idx);
            let res = cs.resolution as usize;
            let rebuilt = face * res * res + row * res + col;
            assert_eq!(idx, rebuilt);
        }
    }
}
