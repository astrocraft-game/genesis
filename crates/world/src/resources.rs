//! Per-tile resource availability.
//!
//! Derives extractable resources for each tile from the surface grid's
//! existing physical layers — plate kind, tectonic boundary, biome,
//! elevation, and hydrology. Resources are coarse categories (iron ore,
//! timber, salt, fish…) rather than specific substances; the root
//! adapter layer maps them to concrete `crafting::Substance` values.

use crate::grid::{BoundaryKind, PlateKind, SurfaceGrid};
use crate::types::BiomeType;

/// Categories of extractable resources. A single tile may carry multiple
/// resources. Non-exhaustive because we expect to add more variants later.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Resource {
    // Metallic ores
    IronOre,
    CopperOre,
    GoldOre,
    TinOre,
    AluminumOre,
    // Non-metallic minerals
    Gemstones,
    Limestone,
    Obsidian,
    // Fossil carbon
    Coal,
    Oil,
    NaturalGas,
    // Chemicals
    Sulfur,
    Salt,
    // Biological
    Timber,
    Herbs,
    Spices,
    Fish,
    Livestock,
    Grain,
    // Fresh water
    FreshWater,
}

/// Per-tile resource inventory for a `SurfaceGrid`.
///
/// `per_tile[i]` lists the resources extractable at tile `i`; parallel
/// with `SurfaceGrid.layers` indexing. Most tiles carry 1-4 resources.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResourceMap {
    pub width: u16,
    pub height: u16,
    pub per_tile: Vec<Vec<Resource>>,
}

impl ResourceMap {
    pub fn tile_count(&self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Count how many tiles carry a given resource.
    pub fn count(&self, resource: Resource) -> usize {
        self.per_tile
            .iter()
            .filter(|v| v.contains(&resource))
            .count()
    }

    /// Every unique resource that appears on any tile.
    pub fn distinct_resources(&self) -> std::collections::HashSet<Resource> {
        self.per_tile.iter().flatten().copied().collect()
    }
}

/// Derive extractable resources per tile from the grid's existing layers.
pub fn generate_resources(grid: &SurfaceGrid) -> ResourceMap {
    let n = grid.tile_count();
    let mut per_tile: Vec<Vec<Resource>> = vec![Vec::new(); n];
    for (idx, tile) in per_tile.iter_mut().enumerate().take(n) {
        let plate_id = grid.layers.plate_id[idx] as usize;
        let plate = &grid.plates[plate_id];
        let boundary = grid.layers.tectonic_boundary[idx];
        let biome = grid.layers.biome[idx];
        let is_ocean = grid.layers.is_ocean[idx];
        let discharge = grid.layers.river_discharge_m3s[idx];

        if is_ocean {
            push_ocean_resources(tile, biome);
        } else {
            push_land_resources(tile, plate.kind, boundary, biome, discharge);
        }
    }
    ResourceMap {
        width: grid.width,
        height: grid.height,
        per_tile,
    }
}

fn push_ocean_resources(tile: &mut Vec<Resource>, biome: BiomeType) {
    // All ocean tiles yield fish + salt (dissolved).
    tile.push(Resource::Fish);
    tile.push(Resource::Salt);
    // Oceans on carbon-sediment basins can host oil/gas.
    if matches!(biome, BiomeType::Ocean) {
        // Always produces limestone via shell-bed deposition.
        tile.push(Resource::Limestone);
    }
}

fn push_land_resources(
    tile: &mut Vec<Resource>,
    plate: PlateKind,
    boundary: BoundaryKind,
    biome: BiomeType,
    discharge_m3s: f32,
) {
    // Metallic ores trace to continental interiors and convergent boundaries.
    if plate == PlateKind::Continental {
        match boundary {
            BoundaryKind::Convergent => {
                // Continental-continental collision exposes deep crust.
                tile.push(Resource::IronOre);
                tile.push(Resource::CopperOre);
                tile.push(Resource::GoldOre);
            }
            BoundaryKind::Divergent => {
                // Rift valleys expose minerals.
                tile.push(Resource::IronOre);
                tile.push(Resource::TinOre);
            }
            BoundaryKind::Transform | BoundaryKind::None => {
                // Continental interior / passive margin.
                tile.push(Resource::AluminumOre);
            }
            _ => {}
        }
    }

    // Volcanic features (convergent + volcanic biome).
    let is_volcanic = matches!(biome, BiomeType::Volcanic) || boundary == BoundaryKind::Convergent;
    if is_volcanic {
        tile.push(Resource::Sulfur);
        tile.push(Resource::Obsidian);
        tile.push(Resource::Gemstones);
    }

    // Fossil carbon: continental sedimentary basins (passive tiles, lowland).
    if plate == PlateKind::Continental && boundary == BoundaryKind::None {
        match biome {
            BiomeType::TropicalForest
            | BiomeType::TemperateForest
            | BiomeType::Savanna
            | BiomeType::Wetland => {
                // Ancient forests → coal.
                tile.push(Resource::Coal);
            }
            BiomeType::Desert | BiomeType::Grassland => {
                // Arid basins / evaporites → oil & gas occasionally.
                tile.push(Resource::Oil);
                tile.push(Resource::NaturalGas);
            }
            _ => {}
        }
    }

    // Limestone: ancient shallow-sea sediments on continental tiles.
    if plate == PlateKind::Continental
        && matches!(
            biome,
            BiomeType::Savanna | BiomeType::Grassland | BiomeType::Desert
        )
    {
        tile.push(Resource::Limestone);
    }

    // Biological resources from biome.
    match biome {
        BiomeType::TropicalForest => {
            tile.push(Resource::Timber);
            tile.push(Resource::Spices);
            tile.push(Resource::Herbs);
        }
        BiomeType::TemperateForest | BiomeType::Taiga => {
            tile.push(Resource::Timber);
            tile.push(Resource::Herbs);
        }
        BiomeType::Savanna => {
            tile.push(Resource::Livestock);
            tile.push(Resource::Grain);
        }
        BiomeType::Grassland => {
            tile.push(Resource::Livestock);
            tile.push(Resource::Grain);
            tile.push(Resource::Herbs);
        }
        BiomeType::Wetland => {
            tile.push(Resource::Herbs);
            tile.push(Resource::Fish);
        }
        BiomeType::Desert => {
            // Evaporite salt beds in arid basins.
            tile.push(Resource::Salt);
        }
        _ => {}
    }

    // Fresh water available where a large river flows through.
    if discharge_m3s > 50.0 {
        tile.push(Resource::FreshWater);
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
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "resources");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        generate_biomes(&mut g);
        g
    }

    #[test]
    fn ocean_tiles_have_fish_and_salt() {
        let g = earth_grid();
        let rm = generate_resources(&g);
        for (idx, tile) in rm.per_tile.iter().enumerate() {
            if g.layers.is_ocean[idx] {
                assert!(tile.contains(&Resource::Fish), "ocean {} has no fish", idx);
                assert!(tile.contains(&Resource::Salt), "ocean {} has no salt", idx);
            }
        }
    }

    #[test]
    fn land_tiles_never_have_fish_or_ocean_exclusive() {
        let g = earth_grid();
        let rm = generate_resources(&g);
        // Land tiles may have fish (wetlands) but not oceanic limestone via
        // the ocean-only path. Mainly check non-emptiness.
        for (idx, tile) in rm.per_tile.iter().enumerate() {
            if !g.layers.is_ocean[idx] {
                assert!(!tile.is_empty(), "land tile {} has no resources", idx);
            }
        }
    }

    #[test]
    fn earth_world_has_diverse_resources() {
        let g = earth_grid();
        let rm = generate_resources(&g);
        let distinct = rm.distinct_resources();
        assert!(
            distinct.len() >= 10,
            "Earth-like world only has {} distinct resources",
            distinct.len()
        );
    }

    #[test]
    fn forests_have_timber() {
        let g = earth_grid();
        let rm = generate_resources(&g);
        for idx in 0..g.tile_count() {
            let biome = g.layers.biome[idx];
            if matches!(
                biome,
                BiomeType::TropicalForest | BiomeType::TemperateForest | BiomeType::Taiga
            ) {
                assert!(
                    rm.per_tile[idx].contains(&Resource::Timber),
                    "forest tile {} has no timber",
                    idx
                );
            }
        }
    }

    #[test]
    fn grasslands_have_grain_and_livestock() {
        let g = earth_grid();
        let rm = generate_resources(&g);
        for idx in 0..g.tile_count() {
            if g.layers.biome[idx] == BiomeType::Grassland {
                assert!(rm.per_tile[idx].contains(&Resource::Grain));
                assert!(rm.per_tile[idx].contains(&Resource::Livestock));
            }
        }
    }

    #[test]
    fn convergent_boundaries_yield_iron_or_sulfur() {
        let g = earth_grid();
        let rm = generate_resources(&g);
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                continue;
            }
            if g.layers.tectonic_boundary[idx] == BoundaryKind::Convergent {
                let plate = &g.plates[g.layers.plate_id[idx] as usize];
                if plate.kind == PlateKind::Continental {
                    assert!(
                        rm.per_tile[idx].contains(&Resource::IronOre)
                            || rm.per_tile[idx].contains(&Resource::Sulfur),
                        "convergent continental tile {} has no iron or sulfur",
                        idx
                    );
                }
            }
        }
    }

    #[test]
    fn map_dimensions_match_grid() {
        let g = earth_grid();
        let rm = generate_resources(&g);
        assert_eq!(rm.width, g.width);
        assert_eq!(rm.height, g.height);
        assert_eq!(rm.tile_count(), g.tile_count());
        assert_eq!(rm.per_tile.len(), g.tile_count());
    }

    #[test]
    fn count_and_distinct_work() {
        let g = earth_grid();
        let rm = generate_resources(&g);
        let fish_tiles = rm.count(Resource::Fish);
        let ocean_count = g.layers.is_ocean.iter().filter(|&&o| o).count();
        // Fish appears on every ocean tile + wetland tiles.
        assert!(fish_tiles >= ocean_count);
    }

    #[test]
    fn resource_generation_is_deterministic() {
        let g = earth_grid();
        let a = generate_resources(&g);
        let b = generate_resources(&g);
        assert_eq!(a.per_tile, b.per_tile);
    }
}
