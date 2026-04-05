//! A* pathfinding on a SurfaceGrid with a caller-supplied cost function.
//!
//! Used by downstream code to find trade routes, migration corridors, or
//! any tile-to-tile path on the planet. The cost function takes a flat
//! tile index and returns a non-negative movement cost for entering that
//! tile; A* finds the minimum-cost path between start and goal.

use crate::grid::SurfaceGrid;
use crate::types::BiomeType;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Result of a successful pathfind: ordered tiles from start to goal
/// and the total movement cost.
#[derive(Clone, Debug)]
pub struct PathResult {
    pub tiles: Vec<usize>,
    pub total_cost: f32,
}

/// A* pathfinding on an equirectangular grid with longitude wrap.
///
/// `cost_fn(idx)` returns the movement cost to enter tile `idx`. A cost
/// of `f32::INFINITY` blocks a tile entirely. Uses 4-connected neighbours
/// and a Chebyshev heuristic scaled to the minimum expected cost.
///
/// Returns `None` if no path exists or if `cost_fn` blocks every
/// reasonable route.
pub fn find_path(
    grid: &SurfaceGrid,
    start: usize,
    goal: usize,
    cost_fn: impl Fn(usize) -> f32,
) -> Option<PathResult> {
    if start == goal {
        return Some(PathResult {
            tiles: vec![start],
            total_cost: 0.0,
        });
    }
    let n = grid.tile_count();
    if start >= n || goal >= n {
        return None;
    }
    let w = grid.width as usize;
    let h = grid.height as usize;

    let mut g_score = vec![f32::INFINITY; n];
    let mut came_from: Vec<Option<usize>> = vec![None; n];
    let mut open = BinaryHeap::new();
    g_score[start] = 0.0;
    open.push(HeapEntry {
        f_score: heuristic(grid, start, goal),
        idx: start,
    });

    while let Some(HeapEntry { idx, .. }) = open.pop() {
        if idx == goal {
            // Reconstruct path.
            let mut path = vec![idx];
            let mut cur = idx;
            while let Some(prev) = came_from[cur] {
                path.push(prev);
                cur = prev;
            }
            path.reverse();
            return Some(PathResult {
                tiles: path,
                total_cost: g_score[goal],
            });
        }
        let neighbours = [
            (idx / w).saturating_sub(1) * w + (idx % w),
            ((idx / w + 1).min(h - 1)) * w + (idx % w),
            (idx / w) * w + ((idx % w) + w - 1) % w,
            (idx / w) * w + ((idx % w) + 1) % w,
        ];
        for n_idx in neighbours {
            if n_idx == idx {
                continue;
            }
            let step_cost = cost_fn(n_idx);
            if !step_cost.is_finite() {
                continue;
            }
            let tentative = g_score[idx] + step_cost;
            if tentative < g_score[n_idx] {
                came_from[n_idx] = Some(idx);
                g_score[n_idx] = tentative;
                open.push(HeapEntry {
                    f_score: tentative + heuristic(grid, n_idx, goal),
                    idx: n_idx,
                });
            }
        }
    }
    None
}

/// Standard terrain cost for trade routes: cheap over water and flatland,
/// expensive over mountains/deserts/ice, impossible at extreme elevation.
pub fn trade_cost(grid: &SurfaceGrid, idx: usize) -> f32 {
    let elev = grid.layers.elevation_m[idx] - grid.sea_level_m;
    if grid.layers.is_ocean[idx] {
        return 0.5;
    }
    let base = 1.0
        + match grid.layers.biome[idx] {
            BiomeType::Grassland | BiomeType::Savanna | BiomeType::TemperateForest => 0.0,
            BiomeType::TropicalForest | BiomeType::Taiga => 0.5,
            BiomeType::Wetland => 1.5,
            BiomeType::Desert => 1.8,
            BiomeType::Tundra => 1.3,
            BiomeType::Alpine => 3.0,
            BiomeType::Volcanic => 5.0,
            BiomeType::IceCap => 4.0,
            BiomeType::Barren => 1.5,
            _ => 1.0,
        };
    let elev_penalty = (elev / 1000.0).max(0.0) * 0.8;
    base + elev_penalty
}

/// Chebyshev grid distance between two tiles (with longitude wrap),
/// scaled by the minimum-cost floor so the heuristic stays admissible.
fn heuristic(grid: &SurfaceGrid, a: usize, b: usize) -> f32 {
    let w = grid.width as i32;
    let ar = (a / grid.width as usize) as i32;
    let ac = (a % grid.width as usize) as i32;
    let br = (b / grid.width as usize) as i32;
    let bc = (b % grid.width as usize) as i32;
    let dlat = (ar - br).abs();
    let dlon_plain = (ac - bc).abs();
    let dlon_wrap = w - dlon_plain;
    let dlon = dlon_plain.min(dlon_wrap);
    (dlat.max(dlon) as f32) * 0.5 // 0.5 is the ocean floor cost
}

#[derive(Clone, Copy, Debug)]
struct HeapEntry {
    f_score: f32,
    idx: usize,
}

impl Eq for HeapEntry {}
impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score && self.idx == other.idx
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap; we want the lowest f_score first.
        other
            .f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.idx.cmp(&other.idx))
    }
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
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "routing");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        generate_biomes(&mut g);
        g
    }

    #[test]
    fn path_to_self_is_trivial() {
        let g = earth_grid();
        let p = find_path(&g, 100, 100, |i| trade_cost(&g, i)).unwrap();
        assert_eq!(p.tiles, vec![100]);
        assert_eq!(p.total_cost, 0.0);
    }

    #[test]
    fn finds_path_between_two_tiles() {
        let g = earth_grid();
        let p = find_path(&g, 0, g.tile_count() - 1, |i| trade_cost(&g, i)).unwrap();
        assert!(p.tiles.len() >= 2);
        assert_eq!(p.tiles.first(), Some(&0));
        assert_eq!(p.tiles.last(), Some(&(g.tile_count() - 1)));
        assert!(p.total_cost > 0.0);
    }

    #[test]
    fn ocean_is_cheapest() {
        let g = earth_grid();
        let ocean_idx = g
            .layers
            .is_ocean
            .iter()
            .position(|&o| o)
            .expect("earth has ocean");
        let land_idx = g
            .layers
            .is_ocean
            .iter()
            .position(|&o| !o)
            .expect("earth has land");
        assert!(trade_cost(&g, ocean_idx) < trade_cost(&g, land_idx));
    }

    #[test]
    fn mountains_cost_more_than_grassland() {
        let g = earth_grid();
        // Find a high-elevation tile and a low-elevation grassland tile.
        let high = g
            .layers
            .elevation_m
            .iter()
            .enumerate()
            .filter(|(i, _)| !g.layers.is_ocean[*i])
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        let grassland = g
            .layers
            .biome
            .iter()
            .enumerate()
            .find(|(i, b)| !g.layers.is_ocean[*i] && **b == BiomeType::Grassland);
        if let Some((low, _)) = grassland {
            assert!(trade_cost(&g, high) > trade_cost(&g, low));
        }
    }

    #[test]
    fn impossible_path_returns_none() {
        let g = earth_grid();
        // Block every tile.
        let p = find_path(&g, 0, 100, |_| f32::INFINITY);
        assert!(p.is_none());
    }

    #[test]
    fn heuristic_respects_longitude_wrap() {
        let g = earth_grid();
        let w = g.width as usize;
        // Col 0 to col w-1 should be ~1 cell apart via wrap, not w-1 cells.
        let h0 = heuristic(&g, 0, w - 1);
        let h_half = heuristic(&g, 0, w / 2);
        assert!(h0 < h_half);
    }

    #[test]
    fn routing_is_deterministic() {
        let g = earth_grid();
        let start = 10;
        let goal = g.tile_count() - 10;
        let a = find_path(&g, start, goal, |i| trade_cost(&g, i));
        let b = find_path(&g, start, goal, |i| trade_cost(&g, i));
        match (a, b) {
            (Some(x), Some(y)) => {
                assert_eq!(x.tiles, y.tiles);
                assert!((x.total_cost - y.total_cost).abs() < 1e-4);
            }
            (None, None) => {}
            _ => panic!("mismatched results"),
        }
    }
}
