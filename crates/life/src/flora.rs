//! Alien flora detail — per-tile vegetation composition.
//!
//! Each land tile hosts a mix of plant types determined by biome.
//! Plants have growth rates and yield harvestable crafting inputs
//! (timber, fiber, resin, spores). Harvesting depletes density;
//! density regenerates over time via growth rate.

use crate::types::Biome;
use serde::{Deserialize, Serialize};

/// Classification of plant type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PlantType {
    /// Tall trees forming a canopy layer.
    CanopyTree,
    /// Low shrubs and ground-level vegetation.
    GroundCover,
    /// Submerged or floating aquatic plants.
    Aquatic,
    /// Fungal organisms (mushrooms, molds, mycorrhizae).
    Fungal,
    /// Mosses, lichens, and other pioneer species.
    MossLichen,
    /// Succulent / drought-resistant plants.
    Succulent,
}

/// What the player can harvest from this plant type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HarvestYield {
    Timber,
    Fiber,
    Resin,
    Spores,
    Fruit,
    Algae,
}

/// A single plant layer within a tile's vegetation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlantLayer {
    pub plant_type: PlantType,
    /// Current density 0.0–1.0 (1.0 = full natural coverage).
    pub density: f32,
    /// Growth rate: fraction of density recovered per tick (0.0–0.1).
    pub growth_rate: f32,
    /// What harvesting this plant yields.
    pub yields: Vec<HarvestYield>,
}

impl PlantLayer {
    /// Harvest a fraction of this plant layer. Returns the amount
    /// actually harvested (may be less if density is low). Depletes density.
    pub fn harvest(&mut self, fraction: f32) -> f32 {
        let amount = (self.density * fraction.clamp(0.0, 1.0)).max(0.0);
        self.density = (self.density - amount).max(0.0);
        amount
    }

    /// Regenerate density by one tick of growth.
    pub fn grow(&mut self) {
        self.density = (self.density + self.growth_rate * (1.0 - self.density)).min(1.0);
    }

    /// Whether this layer has been completely cleared.
    pub fn is_cleared(&self) -> bool {
        self.density < 0.01
    }
}

/// Per-tile vegetation composition.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TileFlora {
    pub layers: Vec<PlantLayer>,
}

impl TileFlora {
    /// Total vegetation density across all layers (0.0–N where N = layer count).
    pub fn total_density(&self) -> f32 {
        self.layers.iter().map(|l| l.density).sum()
    }

    /// Whether this tile has any significant vegetation.
    pub fn has_vegetation(&self) -> bool {
        self.layers.iter().any(|l| l.density > 0.05)
    }

    /// Grow all layers by one tick.
    pub fn grow_all(&mut self) {
        for layer in &mut self.layers {
            layer.grow();
        }
    }

    /// All harvest yields available on this tile (from non-cleared layers).
    pub fn available_yields(&self) -> Vec<HarvestYield> {
        self.layers
            .iter()
            .filter(|l| !l.is_cleared())
            .flat_map(|l| l.yields.iter().copied())
            .collect()
    }
}

/// Flora map for an entire grid.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FloraMap {
    pub tiles: Vec<TileFlora>,
}

impl FloraMap {
    /// Count of tiles with any vegetation.
    pub fn vegetated_tile_count(&self) -> usize {
        self.tiles.iter().filter(|t| t.has_vegetation()).count()
    }

    /// Grow all vegetation by one tick.
    pub fn tick_growth(&mut self) {
        for tile in &mut self.tiles {
            tile.grow_all();
        }
    }
}

/// Generate per-tile flora from biome data.
///
/// Each biome gets a characteristic mix of plant types with appropriate
/// growth rates and harvest yields.
pub fn generate_flora(biomes: &[Biome], is_ocean: &[bool]) -> FloraMap {
    let mut tiles = Vec::with_capacity(biomes.len());
    for (idx, &biome) in biomes.iter().enumerate() {
        if is_ocean[idx] {
            tiles.push(TileFlora::default());
            continue;
        }
        tiles.push(TileFlora {
            layers: flora_for_biome(biome),
        });
    }
    FloraMap { tiles }
}

fn flora_for_biome(biome: Biome) -> Vec<PlantLayer> {
    match biome {
        Biome::TropicalForest => vec![
            PlantLayer {
                plant_type: PlantType::CanopyTree,
                density: 1.0,
                growth_rate: 0.08,
                yields: vec![HarvestYield::Timber, HarvestYield::Resin],
            },
            PlantLayer {
                plant_type: PlantType::GroundCover,
                density: 0.7,
                growth_rate: 0.10,
                yields: vec![HarvestYield::Fiber, HarvestYield::Fruit],
            },
            PlantLayer {
                plant_type: PlantType::Fungal,
                density: 0.5,
                growth_rate: 0.06,
                yields: vec![HarvestYield::Spores],
            },
        ],
        Biome::TemperateForest => vec![
            PlantLayer {
                plant_type: PlantType::CanopyTree,
                density: 0.9,
                growth_rate: 0.05,
                yields: vec![HarvestYield::Timber, HarvestYield::Resin],
            },
            PlantLayer {
                plant_type: PlantType::GroundCover,
                density: 0.6,
                growth_rate: 0.07,
                yields: vec![HarvestYield::Fiber],
            },
        ],
        Biome::Taiga => vec![
            PlantLayer {
                plant_type: PlantType::CanopyTree,
                density: 0.7,
                growth_rate: 0.03,
                yields: vec![HarvestYield::Timber, HarvestYield::Resin],
            },
            PlantLayer {
                plant_type: PlantType::MossLichen,
                density: 0.5,
                growth_rate: 0.02,
                yields: vec![HarvestYield::Fiber],
            },
        ],
        Biome::Grassland | Biome::Steppe | Biome::Savanna => vec![PlantLayer {
            plant_type: PlantType::GroundCover,
            density: 0.8,
            growth_rate: 0.06,
            yields: vec![HarvestYield::Fiber, HarvestYield::Fruit],
        }],
        Biome::MediterraneanShrubland | Biome::Chaparral => vec![PlantLayer {
            plant_type: PlantType::GroundCover,
            density: 0.6,
            growth_rate: 0.04,
            yields: vec![HarvestYield::Fiber, HarvestYield::Resin],
        }],
        Biome::XericShrubland => vec![PlantLayer {
            plant_type: PlantType::Succulent,
            density: 0.3,
            growth_rate: 0.02,
            yields: vec![HarvestYield::Resin],
        }],
        Biome::Desert | Biome::ColdDesert => vec![PlantLayer {
            plant_type: PlantType::Succulent,
            density: 0.05,
            growth_rate: 0.01,
            yields: vec![],
        }],
        Biome::Wetland | Biome::Mangrove => vec![
            PlantLayer {
                plant_type: PlantType::Aquatic,
                density: 0.9,
                growth_rate: 0.08,
                yields: vec![HarvestYield::Algae, HarvestYield::Fiber],
            },
            PlantLayer {
                plant_type: PlantType::Fungal,
                density: 0.4,
                growth_rate: 0.05,
                yields: vec![HarvestYield::Spores],
            },
        ],
        Biome::Tundra => vec![PlantLayer {
            plant_type: PlantType::MossLichen,
            density: 0.3,
            growth_rate: 0.01,
            yields: vec![],
        }],
        Biome::Alpine => vec![PlantLayer {
            plant_type: PlantType::MossLichen,
            density: 0.2,
            growth_rate: 0.01,
            yields: vec![],
        }],
        Biome::Ocean | Biome::IceCap | Biome::Volcanic | Biome::Barren => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tropical_forest_has_canopy_and_ground() {
        let layers = flora_for_biome(Biome::TropicalForest);
        assert!(layers.iter().any(|l| l.plant_type == PlantType::CanopyTree));
        assert!(layers
            .iter()
            .any(|l| l.plant_type == PlantType::GroundCover));
    }

    #[test]
    fn desert_has_minimal_vegetation() {
        let layers = flora_for_biome(Biome::Desert);
        let total: f32 = layers.iter().map(|l| l.density).sum();
        assert!(
            total < 0.2,
            "desert should have very low density: {}",
            total
        );
    }

    #[test]
    fn ocean_tiles_have_no_flora() {
        let biomes = vec![Biome::Ocean, Biome::TropicalForest];
        let ocean = vec![true, false];
        let fm = generate_flora(&biomes, &ocean);
        assert!(fm.tiles[0].layers.is_empty());
        assert!(!fm.tiles[1].layers.is_empty());
    }

    #[test]
    fn harvest_depletes_density() {
        let mut layer = PlantLayer {
            plant_type: PlantType::CanopyTree,
            density: 1.0,
            growth_rate: 0.05,
            yields: vec![HarvestYield::Timber],
        };
        let got = layer.harvest(0.5);
        assert!((got - 0.5).abs() < 0.01);
        assert!((layer.density - 0.5).abs() < 0.01);
    }

    #[test]
    fn growth_regenerates_density() {
        let mut layer = PlantLayer {
            plant_type: PlantType::GroundCover,
            density: 0.3,
            growth_rate: 0.10,
            yields: vec![],
        };
        layer.grow();
        assert!(layer.density > 0.3);
        assert!(layer.density <= 1.0);
    }

    #[test]
    fn growth_caps_at_one() {
        let mut layer = PlantLayer {
            plant_type: PlantType::Fungal,
            density: 0.99,
            growth_rate: 0.10,
            yields: vec![],
        };
        layer.grow();
        assert!(layer.density <= 1.0);
    }

    #[test]
    fn available_yields_from_tile() {
        let biomes = vec![Biome::TropicalForest];
        let ocean = vec![false];
        let fm = generate_flora(&biomes, &ocean);
        let yields = fm.tiles[0].available_yields();
        assert!(yields.contains(&HarvestYield::Timber));
        assert!(yields.contains(&HarvestYield::Fiber));
    }

    #[test]
    fn cleared_layer_not_in_yields() {
        let tile = TileFlora {
            layers: vec![PlantLayer {
                plant_type: PlantType::CanopyTree,
                density: 0.005,
                growth_rate: 0.05,
                yields: vec![HarvestYield::Timber],
            }],
        };
        assert!(tile.available_yields().is_empty());
        assert!(!tile.has_vegetation());
    }

    #[test]
    fn tick_growth_regenerates_all() {
        let biomes = vec![Biome::Grassland, Biome::Taiga];
        let ocean = vec![false, false];
        let mut fm = generate_flora(&biomes, &ocean);
        // Deplete some.
        for tile in &mut fm.tiles {
            for layer in &mut tile.layers {
                layer.density *= 0.5;
            }
        }
        let before: f32 = fm.tiles.iter().map(|t| t.total_density()).sum();
        fm.tick_growth();
        let after: f32 = fm.tiles.iter().map(|t| t.total_density()).sum();
        assert!(after > before, "growth should increase total density");
    }
}
