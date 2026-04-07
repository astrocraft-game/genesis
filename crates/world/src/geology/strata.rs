//! Stratified geological layers — vertical rock columns per tile.
//!
//! Each tile gets a stack of `RockLayer`s representing the subsurface
//! geology from surface to deep crust. Layer composition is driven by
//! plate type, boundary kind, and elevation. Ore deposits are placed
//! within geologically appropriate layers.

use crate::grid::{BoundaryKind, PlateKind, SurfaceGrid};
use crate::resources::Resource;
use seeded_dice_roller::SeededDiceRoller;

/// Type of rock in a geological layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum RockType {
    /// Deposited by water/wind (sandstone, shale, limestone).
    Sedimentary,
    /// Transformed by heat/pressure (marble, slate, quartzite).
    Metamorphic,
    /// Cooled from magma (granite, basalt, gabbro).
    Ignite,
    /// Top layer: loose material (soil, regolith, alluvium).
    Regolith,
    /// Ore-bearing vein within a host layer.
    OreVein,
}

/// A single rock layer in a tile's vertical column.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RockLayer {
    pub rock_type: RockType,
    /// Top of this layer in metres below surface.
    pub depth_top_m: f32,
    /// Thickness of this layer in metres.
    pub thickness_m: f32,
    /// Ore deposit within this layer (if any).
    pub ore: Option<OreDeposit>,
}

impl RockLayer {
    /// Bottom depth = top + thickness.
    pub fn depth_bottom_m(&self) -> f32 {
        self.depth_top_m + self.thickness_m
    }
}

/// An ore deposit embedded in a rock layer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OreDeposit {
    pub resource: Resource,
    /// Purity 0.0–1.0 (fraction of useful material per tonne).
    pub purity: f32,
    /// Total extractable quantity in kilotonnes.
    pub quantity_kt: f32,
}

/// Per-tile geological column.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeologicalColumn {
    /// Layers ordered from surface (index 0) to deep crust.
    pub layers: Vec<RockLayer>,
}

impl GeologicalColumn {
    /// Total depth of the column in metres.
    pub fn total_depth_m(&self) -> f32 {
        self.layers
            .last()
            .map(|l| l.depth_bottom_m())
            .unwrap_or(0.0)
    }

    /// All ore deposits in this column.
    pub fn ore_deposits(&self) -> Vec<&OreDeposit> {
        self.layers.iter().filter_map(|l| l.ore.as_ref()).collect()
    }
}

/// Stratified geology for an entire grid.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StratifiedGeology {
    pub columns: Vec<GeologicalColumn>,
}

/// Generate stratified geological columns for every tile in the grid.
///
/// Layer composition is driven by:
/// - **Plate type**: continental → thick sedimentary + granite;
///   oceanic → thin crust + basalt.
/// - **Boundary kind**: convergent → metamorphic + folded;
///   divergent → igneous intrusions; transform → fractured.
/// - **Elevation**: high elevations get thicker metamorphic; low get
///   thicker sedimentary.
///
/// Ore deposits are placed in geologically appropriate layers:
/// - Iron, copper, gold → igneous veins.
/// - Coal → sedimentary (fossil carbon).
/// - Gemstones → metamorphic (pressure-formed).
/// - Limestone → sedimentary (biogenic).
/// - Oil, gas → deep sedimentary (trapped).
pub fn generate_strata(grid: &SurfaceGrid, seed: &str) -> StratifiedGeology {
    let mut rng = SeededDiceRoller::new(seed, "strata");
    let n = grid.tile_count();
    let mut columns = Vec::with_capacity(n);

    for idx in 0..n {
        let plate_id = grid.layers.plate_id[idx] as usize;
        let plate_kind = if plate_id < grid.plates.len() {
            grid.plates[plate_id].kind
        } else {
            PlateKind::Continental
        };
        let boundary = grid.layers.tectonic_boundary[idx];
        let is_ocean = grid.layers.is_ocean[idx];
        let elev = grid.layers.elevation_m[idx];

        let col = build_column(plate_kind, boundary, is_ocean, elev, &mut rng);
        columns.push(col);
    }

    StratifiedGeology { columns }
}

fn build_column(
    plate: PlateKind,
    boundary: BoundaryKind,
    is_ocean: bool,
    elevation_m: f32,
    rng: &mut SeededDiceRoller,
) -> GeologicalColumn {
    let mut layers = Vec::new();
    let mut depth = 0.0f32;

    // Layer 0: Regolith / soil / seafloor sediment.
    let regolith_thick = if is_ocean {
        50.0 + rng.gen_f64() as f32 * 100.0
    } else {
        5.0 + rng.gen_f64() as f32 * 30.0
    };
    layers.push(RockLayer {
        rock_type: RockType::Regolith,
        depth_top_m: depth,
        thickness_m: regolith_thick,
        ore: None,
    });
    depth += regolith_thick;

    // Continental vs oceanic determines the main column structure.
    match plate {
        PlateKind::Continental => {
            // Sedimentary: thicker in lowlands, thinner at high elevation.
            let sed_thick =
                (2000.0 - elevation_m.max(0.0) * 0.3).max(200.0) + rng.gen_f64() as f32 * 500.0;
            layers.push(RockLayer {
                rock_type: RockType::Sedimentary,
                depth_top_m: depth,
                thickness_m: sed_thick,
                ore: sedimentary_ore(rng),
            });
            depth += sed_thick;

            // Metamorphic: thicker at convergent boundaries or high elevation.
            let meta_factor = match boundary {
                BoundaryKind::Convergent => 2.0,
                _ => 1.0,
            } + if elevation_m > 2000.0 { 1.0 } else { 0.0 };
            let meta_thick = 500.0 * meta_factor + rng.gen_f64() as f32 * 300.0;
            layers.push(RockLayer {
                rock_type: RockType::Metamorphic,
                depth_top_m: depth,
                thickness_m: meta_thick,
                ore: metamorphic_ore(rng),
            });
            depth += meta_thick;

            // Igneous basement (granite).
            let ign_thick = 15000.0 + rng.gen_f64() as f32 * 10000.0;
            layers.push(RockLayer {
                rock_type: RockType::Ignite,
                depth_top_m: depth,
                thickness_m: ign_thick,
                ore: igneous_ore(rng),
            });
        }
        PlateKind::Oceanic => {
            // Thin sedimentary veneer.
            let sed_thick = 200.0 + rng.gen_f64() as f32 * 300.0;
            layers.push(RockLayer {
                rock_type: RockType::Sedimentary,
                depth_top_m: depth,
                thickness_m: sed_thick,
                ore: None,
            });
            depth += sed_thick;

            // Basaltic crust (igneous).
            let basalt_thick = 5000.0 + rng.gen_f64() as f32 * 3000.0;
            layers.push(RockLayer {
                rock_type: RockType::Ignite,
                depth_top_m: depth,
                thickness_m: basalt_thick,
                ore: igneous_ore(rng),
            });
        }
    }

    // Divergent boundaries: add an extra igneous intrusion layer.
    if boundary == BoundaryKind::Divergent {
        let intrusion_thick = 500.0 + rng.gen_f64() as f32 * 1000.0;
        let intrusion_depth = depth * 0.3;
        layers.push(RockLayer {
            rock_type: RockType::Ignite,
            depth_top_m: intrusion_depth,
            thickness_m: intrusion_thick,
            ore: igneous_ore(rng),
        });
    }

    // Sort by depth_top_m for consistent ordering.
    layers.sort_by(|a, b| a.depth_top_m.partial_cmp(&b.depth_top_m).unwrap());

    GeologicalColumn { layers }
}

/// Roll for an ore deposit in sedimentary rock.
fn sedimentary_ore(rng: &mut SeededDiceRoller) -> Option<OreDeposit> {
    let roll = rng.gen_f64();
    if roll < 0.15 {
        Some(OreDeposit {
            resource: Resource::Coal,
            purity: 0.4 + rng.gen_f64() as f32 * 0.4,
            quantity_kt: 50.0 + rng.gen_f64() as f32 * 500.0,
        })
    } else if roll < 0.25 {
        Some(OreDeposit {
            resource: Resource::Limestone,
            purity: 0.7 + rng.gen_f64() as f32 * 0.3,
            quantity_kt: 100.0 + rng.gen_f64() as f32 * 1000.0,
        })
    } else if roll < 0.32 {
        Some(OreDeposit {
            resource: Resource::Oil,
            purity: 0.3 + rng.gen_f64() as f32 * 0.5,
            quantity_kt: 20.0 + rng.gen_f64() as f32 * 200.0,
        })
    } else if roll < 0.37 {
        Some(OreDeposit {
            resource: Resource::Salt,
            purity: 0.8 + rng.gen_f64() as f32 * 0.2,
            quantity_kt: 30.0 + rng.gen_f64() as f32 * 300.0,
        })
    } else {
        None
    }
}

/// Roll for an ore deposit in metamorphic rock.
fn metamorphic_ore(rng: &mut SeededDiceRoller) -> Option<OreDeposit> {
    let roll = rng.gen_f64();
    if roll < 0.12 {
        Some(OreDeposit {
            resource: Resource::Gemstones,
            purity: 0.01 + rng.gen_f64() as f32 * 0.05,
            quantity_kt: 0.5 + rng.gen_f64() as f32 * 5.0,
        })
    } else if roll < 0.20 {
        Some(OreDeposit {
            resource: Resource::GoldOre,
            purity: 0.005 + rng.gen_f64() as f32 * 0.02,
            quantity_kt: 1.0 + rng.gen_f64() as f32 * 10.0,
        })
    } else {
        None
    }
}

/// Roll for an ore deposit in igneous rock.
fn igneous_ore(rng: &mut SeededDiceRoller) -> Option<OreDeposit> {
    let roll = rng.gen_f64();
    if roll < 0.20 {
        Some(OreDeposit {
            resource: Resource::IronOre,
            purity: 0.3 + rng.gen_f64() as f32 * 0.5,
            quantity_kt: 100.0 + rng.gen_f64() as f32 * 1000.0,
        })
    } else if roll < 0.30 {
        Some(OreDeposit {
            resource: Resource::CopperOre,
            purity: 0.1 + rng.gen_f64() as f32 * 0.3,
            quantity_kt: 20.0 + rng.gen_f64() as f32 * 200.0,
        })
    } else if roll < 0.37 {
        Some(OreDeposit {
            resource: Resource::TinOre,
            purity: 0.05 + rng.gen_f64() as f32 * 0.2,
            quantity_kt: 10.0 + rng.gen_f64() as f32 * 100.0,
        })
    } else if roll < 0.42 {
        Some(OreDeposit {
            resource: Resource::AluminumOre,
            purity: 0.2 + rng.gen_f64() as f32 * 0.4,
            quantity_kt: 50.0 + rng.gen_f64() as f32 * 500.0,
        })
    } else if roll < 0.46 {
        Some(OreDeposit {
            resource: Resource::Sulfur,
            purity: 0.5 + rng.gen_f64() as f32 * 0.4,
            quantity_kt: 30.0 + rng.gen_f64() as f32 * 200.0,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{generate_surface_grid, GridResolution};
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
        generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "strata")
    }

    #[test]
    fn every_tile_has_a_column() {
        let g = earth_grid();
        let s = generate_strata(&g, "test");
        assert_eq!(s.columns.len(), g.tile_count());
    }

    #[test]
    fn columns_have_positive_depth() {
        let g = earth_grid();
        let s = generate_strata(&g, "depth");
        for col in &s.columns {
            assert!(col.total_depth_m() > 0.0, "column has zero depth");
        }
    }

    #[test]
    fn continental_tiles_thicker_than_oceanic() {
        let g = earth_grid();
        let s = generate_strata(&g, "thick");
        let mut cont_depths = Vec::new();
        let mut ocean_depths = Vec::new();
        for idx in 0..g.tile_count() {
            let pid = g.layers.plate_id[idx] as usize;
            if pid >= g.plates.len() {
                continue;
            }
            let depth = s.columns[idx].total_depth_m();
            match g.plates[pid].kind {
                PlateKind::Continental => cont_depths.push(depth),
                PlateKind::Oceanic => ocean_depths.push(depth),
            }
        }
        if !cont_depths.is_empty() && !ocean_depths.is_empty() {
            let cont_mean: f32 = cont_depths.iter().sum::<f32>() / cont_depths.len() as f32;
            let ocean_mean: f32 = ocean_depths.iter().sum::<f32>() / ocean_depths.len() as f32;
            assert!(
                cont_mean > ocean_mean,
                "continental mean depth {} should exceed oceanic {}",
                cont_mean,
                ocean_mean
            );
        }
    }

    #[test]
    fn ore_deposits_in_correct_layers() {
        let g = earth_grid();
        let s = generate_strata(&g, "ore");
        for col in &s.columns {
            for layer in &col.layers {
                if let Some(ref ore) = layer.ore {
                    match layer.rock_type {
                        RockType::Sedimentary => {
                            assert!(
                                matches!(
                                    ore.resource,
                                    Resource::Coal
                                        | Resource::Limestone
                                        | Resource::Oil
                                        | Resource::Salt
                                ),
                                "sedimentary layer has {:?}",
                                ore.resource
                            );
                        }
                        RockType::Metamorphic => {
                            assert!(
                                matches!(ore.resource, Resource::Gemstones | Resource::GoldOre),
                                "metamorphic layer has {:?}",
                                ore.resource
                            );
                        }
                        RockType::Ignite => {
                            assert!(
                                matches!(
                                    ore.resource,
                                    Resource::IronOre
                                        | Resource::CopperOre
                                        | Resource::TinOre
                                        | Resource::AluminumOre
                                        | Resource::Sulfur
                                ),
                                "igneous layer has {:?}",
                                ore.resource
                            );
                        }
                        _ => panic!("ore in unexpected layer type {:?}", layer.rock_type),
                    }
                }
            }
        }
    }

    #[test]
    fn strata_is_deterministic() {
        let g = earth_grid();
        let a = generate_strata(&g, "det");
        let b = generate_strata(&g, "det");
        assert_eq!(a.columns.len(), b.columns.len());
        for (ca, cb) in a.columns.iter().zip(b.columns.iter()) {
            assert_eq!(ca.layers.len(), cb.layers.len());
            for (la, lb) in ca.layers.iter().zip(cb.layers.iter()) {
                assert_eq!(la.rock_type, lb.rock_type);
                assert_eq!(la.depth_top_m, lb.depth_top_m);
                assert_eq!(la.thickness_m, lb.thickness_m);
            }
        }
    }

    #[test]
    fn layers_ordered_by_depth() {
        let g = earth_grid();
        let s = generate_strata(&g, "order");
        for col in &s.columns {
            for w in col.layers.windows(2) {
                assert!(
                    w[0].depth_top_m <= w[1].depth_top_m,
                    "layers out of order: {} > {}",
                    w[0].depth_top_m,
                    w[1].depth_top_m
                );
            }
        }
    }

    #[test]
    fn ore_purity_in_unit_range() {
        let g = earth_grid();
        let s = generate_strata(&g, "purity");
        for col in &s.columns {
            for ore in col.ore_deposits() {
                assert!(
                    (0.0..=1.0).contains(&ore.purity),
                    "purity {} out of range",
                    ore.purity
                );
                assert!(ore.quantity_kt > 0.0);
            }
        }
    }

    #[test]
    fn some_tiles_have_ore() {
        let g = earth_grid();
        let s = generate_strata(&g, "some_ore");
        let total_ores: usize = s.columns.iter().map(|c| c.ore_deposits().len()).sum();
        assert!(
            total_ores > 10,
            "expected >10 ore deposits across grid, got {}",
            total_ores
        );
    }
}
