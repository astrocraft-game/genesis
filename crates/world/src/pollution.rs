//! Pollution layer — per-tile contamination with diffusion and effects.
//!
//! The factory simulation emits pollution at source tiles. Each tick,
//! pollution diffuses to neighbours, is absorbed by vegetation, and
//! degrades biomes when concentration exceeds thresholds.

use crate::grid::SurfaceGrid;
use crate::types::BiomeType;

/// Per-tile pollution state for an entire grid.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PollutionMap {
    /// Current pollution level per tile (0.0 = pristine, 1.0+ = severe).
    pub levels: Vec<f32>,
    width: u16,
    height: u16,
}

impl PollutionMap {
    /// Create a pristine (zero pollution) map.
    pub fn new(width: u16, height: u16) -> Self {
        let n = width as usize * height as usize;
        Self {
            levels: vec![0.0; n],
            width,
            height,
        }
    }

    /// Create from a surface grid's dimensions.
    pub fn from_grid(grid: &SurfaceGrid) -> Self {
        Self::new(grid.width, grid.height)
    }

    /// Emit pollution at a tile (additive).
    pub fn emit(&mut self, tile_idx: usize, amount: f32) {
        if let Some(level) = self.levels.get_mut(tile_idx) {
            *level += amount;
        }
    }

    /// Run one diffusion tick: pollution spreads to 4-neighbours, decays
    /// globally, and is absorbed by vegetation.
    ///
    /// - `diffusion_rate`: fraction of each tile's pollution that spreads
    ///   to neighbours per tick (0.0–0.25 recommended).
    /// - `decay_rate`: fraction that dissipates naturally per tick.
    /// - `vegetation_absorption`: per-tile absorption capacity (e.g., from
    ///   `life::generate_vegetation`). Tiles with more vegetation absorb
    ///   more pollution. Pass an empty slice to skip absorption.
    pub fn tick(&mut self, diffusion_rate: f32, decay_rate: f32, vegetation_absorption: &[f32]) {
        let w = self.width as usize;
        let h = self.height as usize;
        let n = w * h;

        // Diffusion: each tile gives `diffusion_rate / 4` of its pollution
        // to each 4-neighbour.
        let source = self.levels.clone();
        let spread = diffusion_rate / 4.0;
        for r in 0..h {
            for c in 0..w {
                let idx = r * w + c;
                let give = source[idx] * spread;
                if give <= 0.0 {
                    continue;
                }
                // Remove total given from this tile.
                self.levels[idx] -= give * 4.0;
                // Add to neighbours.
                let neighbours = [
                    r.saturating_sub(1) * w + c,
                    (r + 1).min(h - 1) * w + c,
                    r * w + (c + w - 1) % w,
                    r * w + (c + 1) % w,
                ];
                for &ni in &neighbours {
                    self.levels[ni] += give;
                }
            }
        }

        // Natural decay.
        for level in &mut self.levels {
            *level *= 1.0 - decay_rate;
        }

        // Vegetation absorption.
        if vegetation_absorption.len() == n {
            for (level, &veg) in self.levels.iter_mut().zip(vegetation_absorption.iter()) {
                // Higher vegetation absorbs more pollution.
                let absorb = *level * veg * 0.1; // 10% of pollution × vegetation density
                *level = (*level - absorb).max(0.0);
            }
        }

        // Clamp to non-negative.
        for level in &mut self.levels {
            *level = level.max(0.0);
        }
    }

    /// Maximum pollution level across all tiles.
    pub fn max_pollution(&self) -> f32 {
        self.levels.iter().copied().fold(0.0f32, f32::max)
    }

    /// Mean pollution level.
    pub fn mean_pollution(&self) -> f32 {
        let sum: f32 = self.levels.iter().sum();
        sum / self.levels.len().max(1) as f32
    }

    /// Tiles where pollution exceeds a threshold.
    pub fn polluted_tiles(&self, threshold: f32) -> Vec<usize> {
        self.levels
            .iter()
            .enumerate()
            .filter(|(_, &l)| l > threshold)
            .map(|(i, _)| i)
            .collect()
    }
}

/// Apply biome degradation to a surface grid based on pollution levels.
///
/// High pollution degrades biomes:
/// - `> 0.7`: forest → grassland, savanna → xeric shrubland
/// - `> 0.9`: any vegetated biome → barren
///
/// Ocean tiles are not affected.
pub fn apply_pollution_degradation(grid: &mut SurfaceGrid, pollution: &PollutionMap) {
    for (idx, &level) in pollution.levels.iter().enumerate() {
        if grid.layers.is_ocean[idx] {
            continue;
        }
        if level > 0.9 {
            match grid.layers.biome[idx] {
                BiomeType::Ocean | BiomeType::Barren | BiomeType::IceCap => {}
                _ => grid.layers.biome[idx] = BiomeType::Barren,
            }
        } else if level > 0.7 {
            grid.layers.biome[idx] = match grid.layers.biome[idx] {
                BiomeType::TropicalForest | BiomeType::TemperateForest | BiomeType::Taiga => {
                    BiomeType::Grassland
                }
                BiomeType::Savanna | BiomeType::MediterraneanShrubland => BiomeType::XericShrubland,
                BiomeType::Grassland | BiomeType::Steppe => BiomeType::Desert,
                other => other,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_map_is_pristine() {
        let pm = PollutionMap::new(10, 10);
        assert_eq!(pm.max_pollution(), 0.0);
        assert_eq!(pm.mean_pollution(), 0.0);
        assert!(pm.polluted_tiles(0.01).is_empty());
    }

    #[test]
    fn emit_increases_level() {
        let mut pm = PollutionMap::new(10, 10);
        pm.emit(55, 0.5);
        assert!((pm.levels[55] - 0.5).abs() < 1e-6);
        pm.emit(55, 0.3);
        assert!((pm.levels[55] - 0.8).abs() < 1e-6);
    }

    #[test]
    fn pollution_spreads_to_neighbours() {
        let mut pm = PollutionMap::new(10, 10);
        pm.emit(55, 1.0); // row 5, col 5
        pm.tick(0.20, 0.0, &[]);
        // Neighbours should have gained some pollution.
        let up = 45; // row 4, col 5
        let down = 65; // row 6, col 5
        let left = 54;
        let right = 56;
        assert!(pm.levels[up] > 0.0, "up neighbour should receive pollution");
        assert!(pm.levels[down] > 0.0);
        assert!(pm.levels[left] > 0.0);
        assert!(pm.levels[right] > 0.0);
        // Source should have decreased.
        assert!(pm.levels[55] < 1.0);
    }

    #[test]
    fn pollution_decays_over_time() {
        let mut pm = PollutionMap::new(5, 5);
        pm.emit(12, 1.0);
        let before = pm.levels[12];
        pm.tick(0.0, 0.1, &[]); // 10% decay, no diffusion
        assert!(
            pm.levels[12] < before,
            "pollution should decay: {} -> {}",
            before,
            pm.levels[12]
        );
    }

    #[test]
    fn vegetation_absorbs_pollution() {
        let mut pm = PollutionMap::new(5, 5);
        pm.emit(12, 1.0);
        let veg = vec![0.8; 25]; // high vegetation
        pm.tick(0.0, 0.0, &veg); // no diffusion, no decay, just absorption
        assert!(
            pm.levels[12] < 1.0,
            "vegetation should absorb: {}",
            pm.levels[12]
        );
    }

    #[test]
    fn high_pollution_degrades_forest_to_grassland() {
        use crate::grid::{GridResolution, SurfaceGrid};
        let mut g = SurfaceGrid::empty(GridResolution::Custom(5, 5));
        g.layers.biome[12] = BiomeType::TemperateForest;
        let mut pm = PollutionMap::new(5, 5);
        pm.levels[12] = 0.8; // above 0.7 threshold
        apply_pollution_degradation(&mut g, &pm);
        assert_eq!(g.layers.biome[12], BiomeType::Grassland);
    }

    #[test]
    fn severe_pollution_degrades_to_barren() {
        use crate::grid::{GridResolution, SurfaceGrid};
        let mut g = SurfaceGrid::empty(GridResolution::Custom(5, 5));
        g.layers.biome[12] = BiomeType::Grassland;
        let mut pm = PollutionMap::new(5, 5);
        pm.levels[12] = 0.95; // above 0.9 threshold
        apply_pollution_degradation(&mut g, &pm);
        assert_eq!(g.layers.biome[12], BiomeType::Barren);
    }

    #[test]
    fn ocean_tiles_unaffected_by_pollution() {
        use crate::grid::{GridResolution, SurfaceGrid};
        let mut g = SurfaceGrid::empty(GridResolution::Custom(5, 5));
        g.layers.is_ocean[12] = true;
        g.layers.biome[12] = BiomeType::Ocean;
        let mut pm = PollutionMap::new(5, 5);
        pm.levels[12] = 1.0;
        apply_pollution_degradation(&mut g, &pm);
        assert_eq!(g.layers.biome[12], BiomeType::Ocean);
    }

    #[test]
    fn polluted_tiles_returns_correct_indices() {
        let mut pm = PollutionMap::new(10, 10);
        pm.emit(3, 0.5);
        pm.emit(7, 0.8);
        pm.emit(50, 0.2);
        let high = pm.polluted_tiles(0.4);
        assert!(high.contains(&3));
        assert!(high.contains(&7));
        assert!(!high.contains(&50));
    }

    #[test]
    fn emit_out_of_bounds_is_safe() {
        let mut pm = PollutionMap::new(5, 5);
        pm.emit(999, 1.0);
        assert_eq!(pm.max_pollution(), 0.0);
    }
}
