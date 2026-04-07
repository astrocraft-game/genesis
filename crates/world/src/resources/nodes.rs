//! Resource node model — finite, depletable deposits with purity.
//!
//! Built from `StratifiedGeology` ore deposits. Each node tracks remaining
//! quantity so the factory simulation can extract material over time.
//! When a node's quantity reaches zero it is considered spent.

use crate::resources::Resource;
use crate::strata::StratifiedGeology;

/// A single extractable resource deposit at a tile.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResourceNode {
    pub resource: Resource,
    /// Purity 0.0–1.0: fraction of useful material per tonne extracted.
    /// Higher purity = less waste per unit of output.
    pub purity: f32,
    /// Remaining extractable quantity in kilotonnes.
    pub quantity_kt: f64,
    /// Original quantity at generation time (for progress tracking).
    pub initial_quantity_kt: f64,
    /// Depth below surface in metres (affects extraction cost).
    pub depth_m: f32,
}

impl ResourceNode {
    /// Whether this node has been fully depleted.
    pub fn is_spent(&self) -> bool {
        self.quantity_kt <= 0.0
    }

    /// Fraction of original quantity remaining (1.0 = untouched, 0.0 = spent).
    pub fn remaining_fraction(&self) -> f64 {
        if self.initial_quantity_kt <= 0.0 {
            return 0.0;
        }
        (self.quantity_kt / self.initial_quantity_kt).clamp(0.0, 1.0)
    }

    /// Extract up to `amount_kt` from this node. Returns the actual amount
    /// extracted (may be less if the node is nearly spent). The extracted
    /// amount is reduced by `(1 - purity)` to give the usable yield.
    ///
    /// Returns `(usable_yield_kt, waste_kt)`.
    pub fn extract(&mut self, amount_kt: f64) -> (f64, f64) {
        let actual = amount_kt.min(self.quantity_kt).max(0.0);
        self.quantity_kt -= actual;
        let usable = actual * self.purity as f64;
        let waste = actual - usable;
        (usable, waste)
    }
}

/// Per-tile collection of resource nodes.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TileNodes {
    pub nodes: Vec<ResourceNode>,
}

impl TileNodes {
    /// All non-spent nodes.
    pub fn active_nodes(&self) -> impl Iterator<Item = &ResourceNode> {
        self.nodes.iter().filter(|n| !n.is_spent())
    }

    /// Find nodes of a specific resource type (including spent ones).
    pub fn nodes_of(&self, resource: Resource) -> impl Iterator<Item = &ResourceNode> {
        self.nodes.iter().filter(move |n| n.resource == resource)
    }

    /// Total remaining quantity across all nodes of a given resource.
    pub fn total_remaining_kt(&self, resource: Resource) -> f64 {
        self.nodes
            .iter()
            .filter(|n| n.resource == resource)
            .map(|n| n.quantity_kt)
            .sum()
    }
}

/// Resource node map for an entire grid.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResourceNodeMap {
    pub tiles: Vec<TileNodes>,
}

impl ResourceNodeMap {
    /// Total nodes across the entire map.
    pub fn total_node_count(&self) -> usize {
        self.tiles.iter().map(|t| t.nodes.len()).sum()
    }

    /// Total non-spent nodes.
    pub fn active_node_count(&self) -> usize {
        self.tiles
            .iter()
            .flat_map(|t| t.nodes.iter())
            .filter(|n| !n.is_spent())
            .count()
    }

    /// Global remaining quantity of a resource in kilotonnes.
    pub fn global_remaining_kt(&self, resource: Resource) -> f64 {
        self.tiles
            .iter()
            .map(|t| t.total_remaining_kt(resource))
            .sum()
    }

    /// Tiles that have at least one active node of the given resource.
    pub fn tiles_with_resource(&self, resource: Resource) -> Vec<usize> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| {
                t.nodes
                    .iter()
                    .any(|n| n.resource == resource && !n.is_spent())
            })
            .map(|(i, _)| i)
            .collect()
    }
}

/// Build a `ResourceNodeMap` from stratified geology.
///
/// Each `OreDeposit` in the strata becomes a `ResourceNode` with
/// depth set to the middle of its host rock layer.
pub fn generate_resource_nodes(strata: &StratifiedGeology) -> ResourceNodeMap {
    let mut tiles = Vec::with_capacity(strata.columns.len());
    for col in &strata.columns {
        let mut nodes = Vec::new();
        for layer in &col.layers {
            if let Some(ref ore) = layer.ore {
                let depth = layer.depth_top_m + layer.thickness_m / 2.0;
                let qty = ore.quantity_kt as f64;
                nodes.push(ResourceNode {
                    resource: ore.resource,
                    purity: ore.purity,
                    quantity_kt: qty,
                    initial_quantity_kt: qty,
                    depth_m: depth,
                });
            }
        }
        tiles.push(TileNodes { nodes });
    }
    ResourceNodeMap { tiles }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{generate_surface_grid, GridResolution};
    use crate::strata::generate_strata;
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};

    fn earth_nodes() -> ResourceNodeMap {
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
        let g = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "nodes");
        let s = generate_strata(&g, "nodes");
        generate_resource_nodes(&s)
    }

    #[test]
    fn node_count_matches_strata_ores() {
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
        let g = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "count");
        let s = generate_strata(&g, "count");
        let strata_ores: usize = s.columns.iter().map(|c| c.ore_deposits().len()).sum();
        let nm = generate_resource_nodes(&s);
        assert_eq!(nm.total_node_count(), strata_ores);
    }

    #[test]
    fn purity_in_unit_range() {
        let nm = earth_nodes();
        for tile in &nm.tiles {
            for node in &tile.nodes {
                assert!(
                    (0.0..=1.0).contains(&node.purity),
                    "purity {} out of range",
                    node.purity
                );
            }
        }
    }

    #[test]
    fn quantity_positive_for_all_nodes() {
        let nm = earth_nodes();
        for tile in &nm.tiles {
            for node in &tile.nodes {
                assert!(node.quantity_kt > 0.0);
                assert_eq!(node.quantity_kt, node.initial_quantity_kt);
            }
        }
    }

    #[test]
    fn extract_reduces_quantity() {
        let mut node = ResourceNode {
            resource: Resource::IronOre,
            purity: 0.6,
            quantity_kt: 100.0,
            initial_quantity_kt: 100.0,
            depth_m: 500.0,
        };
        let (usable, waste) = node.extract(30.0);
        assert!((usable - 18.0).abs() < 0.01); // 30 * 0.6
        assert!((waste - 12.0).abs() < 0.01); // 30 * 0.4
        assert!((node.quantity_kt - 70.0).abs() < 0.01);
        assert!((node.remaining_fraction() - 0.7).abs() < 0.01);
    }

    #[test]
    fn extract_clamps_at_zero() {
        let mut node = ResourceNode {
            resource: Resource::CopperOre,
            purity: 0.5,
            quantity_kt: 10.0,
            initial_quantity_kt: 10.0,
            depth_m: 200.0,
        };
        let (usable, _) = node.extract(50.0);
        assert!((usable - 5.0).abs() < 0.01); // only 10 available, 10*0.5
        assert!(node.is_spent());
        assert_eq!(node.remaining_fraction(), 0.0);
    }

    #[test]
    fn global_remaining_sums_correctly() {
        let nm = earth_nodes();
        let iron_total = nm.global_remaining_kt(Resource::IronOre);
        let manual_sum: f64 = nm
            .tiles
            .iter()
            .flat_map(|t| t.nodes.iter())
            .filter(|n| n.resource == Resource::IronOre)
            .map(|n| n.quantity_kt)
            .sum();
        assert!((iron_total - manual_sum).abs() < 0.001);
    }

    #[test]
    fn tiles_with_resource_returns_correct_indices() {
        let nm = earth_nodes();
        let iron_tiles = nm.tiles_with_resource(Resource::IronOre);
        for &idx in &iron_tiles {
            assert!(nm.tiles[idx]
                .nodes
                .iter()
                .any(|n| n.resource == Resource::IronOre && !n.is_spent()));
        }
    }

    #[test]
    fn total_planetary_ore_within_plausible_bounds() {
        let nm = earth_nodes();
        let iron = nm.global_remaining_kt(Resource::IronOre);
        // Earth-like Fast grid should have some iron (not zero, not infinite).
        // With ~2592 tiles and ~20% igneous ore chance at 100-1100 kt each,
        // expect on the order of 10k-500k kt total.
        assert!(iron > 0.0, "no iron ore on Earth-like planet");
        assert!(iron < 10_000_000.0, "iron ore {} kt implausibly high", iron);
    }
}
