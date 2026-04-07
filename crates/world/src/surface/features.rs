//! Geographic feature detection on a SurfaceGrid.
//!
//! Clusters tiles into contiguous regions (mountain ranges, islands,
//! deserts, ocean basins) and traces river main-stems from source to
//! mouth. All features are indexed by flat tile indices into the grid.
//!
//! Names are NOT assigned here — that happens in the root adapter via
//! life's Markov generators. The output structures expose the raw
//! geography only.

use crate::grid::SurfaceGrid;
use crate::types::BiomeType;

/// Elevation (metres above sea level) above which a tile counts toward
/// a mountain range for the purposes of feature detection.
const MOUNTAIN_THRESHOLD_M: f32 = 1500.0;

/// Minimum tile count for a mountain range to be reported.
const MIN_RANGE_SIZE: usize = 3;

/// Minimum discharge (m³/s) for a tile to be considered a river mouth.
const RIVER_MOUTH_MIN_DISCHARGE: f32 = 500.0;

/// Minimum discharge along the traced main-stem.
const RIVER_MIN_DISCHARGE: f32 = 50.0;

/// A contiguous cluster of high-elevation tiles.
#[derive(Clone, Debug)]
pub struct MountainRange {
    pub tiles: Vec<usize>,
    pub highest_elevation_m: f32,
}

/// A river traced from source down to an ocean-adjacent mouth tile.
#[derive(Clone, Debug)]
pub struct River {
    /// Tiles along the main-stem, source first, mouth last.
    pub tiles: Vec<usize>,
    /// Ocean-adjacent land tile where the river meets the sea.
    pub mouth_tile: usize,
    /// Discharge at the mouth in m³/s.
    pub max_discharge_m3s: f32,
}

/// A contiguous ocean basin identified by flood-fill.
#[derive(Clone, Debug)]
pub struct OceanBasin {
    pub basin_id: u16,
    pub tiles: Vec<usize>,
}

/// A contiguous landmass.
#[derive(Clone, Debug)]
pub struct Island {
    pub tiles: Vec<usize>,
}

/// A contiguous region of desert tiles.
#[derive(Clone, Debug)]
pub struct Desert {
    pub tiles: Vec<usize>,
}

/// All detected geographic features for a surface grid.
#[derive(Clone, Debug, Default)]
pub struct Features {
    pub mountain_ranges: Vec<MountainRange>,
    pub rivers: Vec<River>,
    pub ocean_basins: Vec<OceanBasin>,
    pub islands: Vec<Island>,
    pub deserts: Vec<Desert>,
}

/// Detect all features on a fully-populated grid.
pub fn detect_features(grid: &SurfaceGrid) -> Features {
    Features {
        mountain_ranges: detect_mountain_ranges(grid),
        rivers: detect_rivers(grid),
        ocean_basins: detect_ocean_basins(grid),
        islands: detect_islands(grid),
        deserts: detect_deserts(grid),
    }
}

/// Flood-fill connected high-elevation land tiles.
pub fn detect_mountain_ranges(grid: &SurfaceGrid) -> Vec<MountainRange> {
    let sl = grid.sea_level_m;
    let predicate = |idx: usize| {
        !grid.layers.is_ocean[idx] && grid.layers.elevation_m[idx] - sl > MOUNTAIN_THRESHOLD_M
    };
    let clusters = flood_fill_clusters(grid, predicate);
    clusters
        .into_iter()
        .filter(|tiles| tiles.len() >= MIN_RANGE_SIZE)
        .map(|tiles| {
            let highest = tiles
                .iter()
                .map(|&i| grid.layers.elevation_m[i])
                .fold(f32::NEG_INFINITY, f32::max);
            MountainRange {
                tiles,
                highest_elevation_m: highest,
            }
        })
        .collect()
}

/// Trace each river main-stem from its ocean-adjacent mouth upstream.
pub fn detect_rivers(grid: &SurfaceGrid) -> Vec<River> {
    let w = grid.width as usize;
    let h = grid.height as usize;
    let mut rivers: Vec<River> = Vec::new();
    let mut claimed = vec![false; grid.tile_count()];

    for idx in 0..grid.tile_count() {
        if grid.layers.is_ocean[idx] || claimed[idx] {
            continue;
        }
        let discharge = grid.layers.river_discharge_m3s[idx];
        if discharge < RIVER_MOUTH_MIN_DISCHARGE {
            continue;
        }
        // Must be adjacent to an ocean tile.
        if !has_ocean_neighbour(grid, idx) {
            continue;
        }
        // Trace upstream: at each step go to the land neighbour with the
        // highest discharge strictly less than the current tile's.
        let mut path = vec![idx];
        let mut current = idx;
        claimed[idx] = true;
        loop {
            let cur_discharge = grid.layers.river_discharge_m3s[current];
            let mut best: Option<(usize, f32)> = None;
            for n_idx in neighbours_4(current, w, h) {
                if grid.layers.is_ocean[n_idx] || claimed[n_idx] {
                    continue;
                }
                let nd = grid.layers.river_discharge_m3s[n_idx];
                if nd >= cur_discharge || nd < RIVER_MIN_DISCHARGE {
                    continue;
                }
                if best.map(|(_, d)| nd > d).unwrap_or(true) {
                    best = Some((n_idx, nd));
                }
            }
            match best {
                Some((next, _)) => {
                    path.push(next);
                    claimed[next] = true;
                    current = next;
                }
                None => break,
            }
        }
        path.reverse(); // source first, mouth last
        rivers.push(River {
            mouth_tile: idx,
            max_discharge_m3s: discharge,
            tiles: path,
        });
    }
    rivers
}

/// Group ocean tiles by `drainage_basin_id` into contiguous basins.
pub fn detect_ocean_basins(grid: &SurfaceGrid) -> Vec<OceanBasin> {
    use std::collections::HashMap;
    let mut by_id: HashMap<u16, Vec<usize>> = HashMap::new();
    for idx in 0..grid.tile_count() {
        if !grid.layers.is_ocean[idx] {
            continue;
        }
        let bid = grid.layers.drainage_basin_id[idx];
        if bid == 0 {
            continue;
        }
        by_id.entry(bid).or_default().push(idx);
    }
    let mut basins: Vec<OceanBasin> = by_id
        .into_iter()
        .map(|(basin_id, tiles)| OceanBasin { basin_id, tiles })
        .collect();
    basins.sort_by_key(|b| std::cmp::Reverse(b.tiles.len()));
    basins
}

/// Flood-fill contiguous land tiles.
pub fn detect_islands(grid: &SurfaceGrid) -> Vec<Island> {
    let predicate = |idx: usize| !grid.layers.is_ocean[idx];
    let clusters = flood_fill_clusters(grid, predicate);
    let mut islands: Vec<Island> = clusters.into_iter().map(|tiles| Island { tiles }).collect();
    islands.sort_by_key(|i| std::cmp::Reverse(i.tiles.len()));
    islands
}

/// Flood-fill contiguous desert tiles.
pub fn detect_deserts(grid: &SurfaceGrid) -> Vec<Desert> {
    let predicate = |idx: usize| {
        !grid.layers.is_ocean[idx] && matches!(grid.layers.biome[idx], BiomeType::Desert)
    };
    let clusters = flood_fill_clusters(grid, predicate);
    clusters
        .into_iter()
        .filter(|tiles| tiles.len() >= 2)
        .map(|tiles| Desert { tiles })
        .collect()
}

/// Generic 4-connected flood fill: returns one Vec<tile_idx> per cluster.
fn flood_fill_clusters<F: Fn(usize) -> bool>(grid: &SurfaceGrid, pred: F) -> Vec<Vec<usize>> {
    let n = grid.tile_count();
    let w = grid.width as usize;
    let h = grid.height as usize;
    let mut visited = vec![false; n];
    let mut clusters: Vec<Vec<usize>> = Vec::new();
    for start in 0..n {
        if visited[start] || !pred(start) {
            continue;
        }
        let mut stack = vec![start];
        let mut cluster = Vec::new();
        while let Some(idx) = stack.pop() {
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            cluster.push(idx);
            for n_idx in neighbours_4(idx, w, h) {
                if !visited[n_idx] && pred(n_idx) {
                    stack.push(n_idx);
                }
            }
        }
        clusters.push(cluster);
    }
    clusters
}

fn neighbours_4(idx: usize, w: usize, h: usize) -> [usize; 4] {
    let r = idx / w;
    let c = idx % w;
    [
        r.saturating_sub(1) * w + c,
        ((r + 1).min(h - 1)) * w + c,
        r * w + (c + w - 1) % w,
        r * w + (c + 1) % w,
    ]
}

fn has_ocean_neighbour(grid: &SurfaceGrid, idx: usize) -> bool {
    let w = grid.width as usize;
    let h = grid.height as usize;
    neighbours_4(idx, w, h)
        .iter()
        .any(|&n| grid.layers.is_ocean[n])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::climate::{generate_biomes, generate_temperature, generate_wind};
    use crate::geology::generate_geology;
    use crate::grid::GridResolution;
    use crate::hydrology::{generate_hydrology, generate_precipitation};
    use crate::ocean::generate_ocean_dynamics;
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
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "features");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        generate_biomes(&mut g);
        g
    }

    #[test]
    fn earth_grid_detects_mountain_ranges() {
        let g = earth_grid();
        let ranges = detect_mountain_ranges(&g);
        assert!(
            !ranges.is_empty(),
            "Earth-like world should have mountain ranges"
        );
        for r in &ranges {
            assert!(r.tiles.len() >= MIN_RANGE_SIZE);
            assert!(r.highest_elevation_m > g.sea_level_m + MOUNTAIN_THRESHOLD_M);
        }
    }

    #[test]
    fn rivers_reach_the_ocean() {
        let g = earth_grid();
        let rivers = detect_rivers(&g);
        for river in &rivers {
            assert!(has_ocean_neighbour(&g, river.mouth_tile));
            assert!(river.max_discharge_m3s >= RIVER_MOUTH_MIN_DISCHARGE);
            assert!(!river.tiles.is_empty());
        }
    }

    #[test]
    fn rivers_flow_downhill_in_order() {
        let g = earth_grid();
        let rivers = detect_rivers(&g);
        for river in &rivers {
            // Along the path source → mouth, discharge should monotonically
            // increase (weakly).
            for pair in river.tiles.windows(2) {
                let up = g.layers.river_discharge_m3s[pair[0]];
                let down = g.layers.river_discharge_m3s[pair[1]];
                assert!(
                    down >= up,
                    "river runs uphill in discharge: {} → {}",
                    up,
                    down
                );
            }
        }
    }

    #[test]
    fn ocean_basins_match_drainage_ids() {
        let g = earth_grid();
        let basins = detect_ocean_basins(&g);
        for basin in &basins {
            assert!(!basin.tiles.is_empty());
            for &idx in &basin.tiles {
                assert!(g.layers.is_ocean[idx]);
                assert_eq!(g.layers.drainage_basin_id[idx], basin.basin_id);
            }
        }
    }

    #[test]
    fn islands_are_all_land() {
        let g = earth_grid();
        let islands = detect_islands(&g);
        assert!(!islands.is_empty());
        for island in &islands {
            assert!(!island.tiles.is_empty());
            for &idx in &island.tiles {
                assert!(!g.layers.is_ocean[idx]);
            }
        }
        // Total land tiles should match the sum of all island tiles.
        let land_count: usize = islands.iter().map(|i| i.tiles.len()).sum();
        let expected = g.layers.is_ocean.iter().filter(|&&o| !o).count();
        assert_eq!(land_count, expected);
    }

    #[test]
    fn deserts_are_all_desert_biome() {
        let g = earth_grid();
        let deserts = detect_deserts(&g);
        for desert in &deserts {
            for &idx in &desert.tiles {
                assert_eq!(g.layers.biome[idx], BiomeType::Desert);
            }
        }
    }

    #[test]
    fn detect_features_returns_populated_set() {
        let g = earth_grid();
        let features = detect_features(&g);
        assert!(!features.mountain_ranges.is_empty());
        assert!(!features.ocean_basins.is_empty());
        assert!(!features.islands.is_empty());
    }

    #[test]
    fn detection_is_deterministic() {
        let g = earth_grid();
        let a = detect_features(&g);
        let b = detect_features(&g);
        assert_eq!(a.mountain_ranges.len(), b.mountain_ranges.len());
        assert_eq!(a.rivers.len(), b.rivers.len());
        assert_eq!(a.ocean_basins.len(), b.ocean_basins.len());
        assert_eq!(a.islands.len(), b.islands.len());
    }

    #[test]
    fn largest_island_first() {
        let g = earth_grid();
        let islands = detect_islands(&g);
        for w in islands.windows(2) {
            assert!(w[0].tiles.len() >= w[1].tiles.len());
        }
    }
}
