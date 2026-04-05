//! Settlement placement: score tiles for civilisation suitability and
//! greedily drop the top-N candidates with a minimum-separation radius.
//!
//! This module stays independent of the `world` crate: callers pass in
//! per-tile scores (habitability, water access, resource density) built
//! via root adapters. The life crate owns only the scoring combination
//! function and the placement algorithm.

use crate::habitat::HabitatGrid;
use crate::types::Biome;
use std::rc::Rc;

/// A single settled location.
#[derive(Clone, Debug)]
pub struct Settlement {
    /// Species that founded this settlement.
    pub species: Rc<str>,
    /// Flat tile index into a HabitatGrid.
    pub tile_idx: usize,
    /// Combined suitability score (0.0 – 1.0).
    pub suitability: f32,
    /// Rough initial population (log10 scale: 3 = village, 6 = city).
    pub population_order: u8,
}

/// Compute per-tile settlement suitability by combining species
/// habitability, water access, resource density, climate moderation,
/// and elevation penalty.
///
/// All inputs are expected to be parallel to the habitat grid
/// (one value per tile). Scores clamp to [0, 1].
pub fn compute_settlement_suitability(
    habitat: &HabitatGrid,
    species_habitability: &[f32],
    water_access: &[f32],
    resource_density: &[f32],
) -> Vec<f32> {
    let n = habitat.tile_count();
    assert_eq!(species_habitability.len(), n);
    assert_eq!(water_access.len(), n);
    assert_eq!(resource_density.len(), n);

    (0..n)
        .map(|idx| {
            if habitat.is_ocean[idx] {
                return 0.0; // No ocean settlements in this coarse model.
            }
            let hab = species_habitability[idx].clamp(0.0, 1.0);
            if hab <= 0.01 {
                return 0.0;
            }
            let water = water_access[idx].clamp(0.0, 1.0);
            let resources = resource_density[idx].clamp(0.0, 1.0);
            let moderation = moderation_score(habitat.biome[idx]);
            let elev_penalty = elevation_penalty(habitat.elevation_m[idx]);

            // Multiplicative combination: any zero factor rules out the tile.
            // Resources and water are boosters so they use (0.5 + 0.5 × x)
            // to avoid forcing low-score tiles to 0.
            let score =
                hab * moderation * elev_penalty * (0.4 + 0.6 * water) * (0.5 + 0.5 * resources);
            score.clamp(0.0, 1.0)
        })
        .collect()
}

/// Preference multiplier for each biome's climate moderation.
fn moderation_score(biome: Biome) -> f32 {
    match biome {
        Biome::TemperateForest | Biome::Grassland => 1.0,
        Biome::Savanna | Biome::TropicalForest => 0.85,
        Biome::Taiga | Biome::Wetland => 0.6,
        Biome::Desert | Biome::Tundra => 0.35,
        Biome::Alpine => 0.25,
        Biome::IceCap | Biome::Volcanic | Biome::Barren | Biome::Ocean => 0.0,
    }
}

/// Elevation penalty: 1.0 at sea level, drops to 0 at 4500 m.
fn elevation_penalty(elevation_m: f32) -> f32 {
    let e = elevation_m.max(0.0);
    if e > 4500.0 {
        0.0
    } else {
        (1.0 - e / 4500.0).clamp(0.0, 1.0).powf(0.6)
    }
}

/// Greedy placement: select the top-scoring tiles, each blocking a
/// Chebyshev radius of `min_separation_tiles` (with longitude wrap) so
/// settlements don't cluster.
pub fn place_settlements(
    suitability: &[f32],
    habitat: &HabitatGrid,
    species: Rc<str>,
    max_settlements: usize,
    min_separation_tiles: u16,
) -> Vec<Settlement> {
    let n = habitat.tile_count();
    assert_eq!(suitability.len(), n);
    if max_settlements == 0 {
        return Vec::new();
    }

    // Sort candidate indices by suitability descending.
    let mut indices: Vec<usize> = (0..n).filter(|&i| suitability[i] > 0.05).collect();
    indices.sort_by(|&a, &b| {
        suitability[b]
            .partial_cmp(&suitability[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let w = habitat.width as i32;
    let h = habitat.height as i32;
    let sep = min_separation_tiles as i32;

    let mut blocked = vec![false; n];
    let mut settlements = Vec::with_capacity(max_settlements);

    for &idx in &indices {
        if blocked[idx] {
            continue;
        }
        let score = suitability[idx];
        let pop_order = ((score * 9.0) as u8 + 3).min(9);
        settlements.push(Settlement {
            species: species.clone(),
            tile_idx: idx,
            suitability: score,
            population_order: pop_order,
        });
        if settlements.len() >= max_settlements {
            break;
        }
        // Block all tiles within the Chebyshev separation radius.
        let r = (idx / w as usize) as i32;
        let c = (idx % w as usize) as i32;
        for dr in -sep..=sep {
            let nr = r + dr;
            if nr < 0 || nr >= h {
                continue;
            }
            for dc in -sep..=sep {
                let nc = (c + dc).rem_euclid(w);
                blocked[(nr * w + nc) as usize] = true;
            }
        }
    }
    settlements
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::habitat::HabitatGrid;
    use crate::types::Biome;

    fn grid_with_biomes(biomes: Vec<Biome>) -> HabitatGrid {
        let n = biomes.len();
        HabitatGrid {
            width: n as u16,
            height: 1,
            temperature_c: vec![15.0; n],
            humidity_relative: vec![0.5; n],
            biome: biomes,
            is_ocean: vec![false; n],
            elevation_m: vec![200.0; n],
        }
    }

    #[test]
    fn ocean_tiles_get_zero_suitability() {
        let mut habitat = grid_with_biomes(vec![Biome::Grassland, Biome::Ocean]);
        habitat.is_ocean = vec![false, true];
        let hab = vec![0.8f32; 2];
        let water = vec![1.0f32; 2];
        let res = vec![0.7f32; 2];
        let score = compute_settlement_suitability(&habitat, &hab, &water, &res);
        assert!(score[0] > 0.0);
        assert_eq!(score[1], 0.0);
    }

    #[test]
    fn temperate_outscores_tundra() {
        let habitat = grid_with_biomes(vec![Biome::TemperateForest, Biome::Tundra]);
        let hab = vec![0.8f32; 2];
        let water = vec![0.5f32; 2];
        let res = vec![0.5f32; 2];
        let score = compute_settlement_suitability(&habitat, &hab, &water, &res);
        assert!(score[0] > score[1]);
    }

    #[test]
    fn water_access_boosts_score() {
        let habitat = grid_with_biomes(vec![Biome::Grassland, Biome::Grassland]);
        let hab = vec![0.8f32; 2];
        let water = vec![0.0f32, 1.0f32]; // second tile has water
        let res = vec![0.5f32; 2];
        let score = compute_settlement_suitability(&habitat, &hab, &water, &res);
        assert!(score[1] > score[0]);
    }

    #[test]
    fn resources_boost_score() {
        let habitat = grid_with_biomes(vec![Biome::Grassland, Biome::Grassland]);
        let hab = vec![0.8f32; 2];
        let water = vec![0.5f32; 2];
        let res = vec![0.0f32, 1.0f32];
        let score = compute_settlement_suitability(&habitat, &hab, &water, &res);
        assert!(score[1] > score[0]);
    }

    #[test]
    fn extreme_elevation_penalised() {
        let mut habitat = grid_with_biomes(vec![Biome::Grassland, Biome::Grassland]);
        habitat.elevation_m = vec![0.0, 6000.0];
        let hab = vec![0.8f32; 2];
        let water = vec![0.5f32; 2];
        let res = vec![0.5f32; 2];
        let score = compute_settlement_suitability(&habitat, &hab, &water, &res);
        assert!(score[0] > score[1]);
        assert_eq!(score[1], 0.0); // >4500 m
    }

    #[test]
    fn zero_habitability_gives_zero_score() {
        let habitat = grid_with_biomes(vec![Biome::Grassland]);
        let hab = vec![0.0f32];
        let water = vec![1.0f32];
        let res = vec![1.0f32];
        let score = compute_settlement_suitability(&habitat, &hab, &water, &res);
        assert_eq!(score[0], 0.0);
    }

    #[test]
    fn greedy_placement_picks_top_scores() {
        let suitability = vec![0.1, 0.9, 0.5, 0.8, 0.2];
        let habitat = grid_with_biomes(vec![Biome::Grassland; 5]);
        let species: Rc<str> = "Testus".into();
        let s = place_settlements(&suitability, &habitat, species, 2, 0);
        assert_eq!(s.len(), 2);
        assert!(s[0].suitability >= s[1].suitability);
        assert_eq!(s[0].tile_idx, 1); // highest score
    }

    #[test]
    fn min_separation_prevents_clustering() {
        // 10 tiles, all scored high. With separation=2, expect ~3-4 settlements.
        let n = 10;
        let suitability: Vec<f32> = (0..n).map(|i| 1.0 - i as f32 * 0.01).collect();
        let habitat = grid_with_biomes(vec![Biome::Grassland; n]);
        let species: Rc<str> = "Testus".into();
        let s = place_settlements(&suitability, &habitat, species, 20, 2);
        assert!(s.len() <= 4, "too many settlements: {}", s.len());
        // Verify spacing.
        for i in 0..s.len() {
            for j in (i + 1)..s.len() {
                let delta = (s[i].tile_idx as i32 - s[j].tile_idx as i32).abs();
                assert!(delta > 2);
            }
        }
    }

    #[test]
    fn no_settlements_on_unsuitable_world() {
        let habitat = grid_with_biomes(vec![Biome::Ocean; 10]);
        let mut h = habitat.clone();
        h.is_ocean = vec![true; 10];
        let suit = vec![0.0f32; 10];
        let s = place_settlements(&suit, &h, "Testus".into(), 5, 1);
        assert!(s.is_empty());
    }

    #[test]
    fn population_scales_with_suitability() {
        let suitability = vec![0.9, 0.3];
        let habitat = grid_with_biomes(vec![Biome::Grassland; 2]);
        let s = place_settlements(&suitability, &habitat, "Folk".into(), 2, 0);
        assert_eq!(s.len(), 2);
        assert!(s[0].population_order > s[1].population_order);
    }

    #[test]
    fn longitude_wraps_for_separation() {
        // Width-8 row: if we place at col 0 with sep=3, col 5,6,7 should be
        // reachable (wrap) and only col 4+ should be available.
        let suitability = vec![0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2];
        let habitat = grid_with_biomes(vec![Biome::Grassland; 8]);
        let s = place_settlements(&suitability, &habitat, "Folk".into(), 8, 3);
        // With sep=3 across 8 tiles wrapped, separation should spread them.
        assert!(!s.is_empty() && s.len() <= 3);
    }
}
