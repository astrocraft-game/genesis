//! Zone-aware rare ore placement.
//!
//! Each geological zone has its own probability table for rare/strategic
//! ores. These are layered on top of the common ores from `strata.rs`,
//! giving each zone a unique resource signature that forces multi-base play.

use crate::resources::Resource;
use crate::strata::OreDeposit;
use crate::zones::{GeologicalZone, ZoneMap};
use seeded_dice_roller::SeededDiceRoller;

/// A rare ore deposit placed by geological zone.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZoneOreDeposit {
    pub resource: Resource,
    pub purity: f32,
    pub quantity_kt: f32,
    pub zone: GeologicalZone,
}

/// Per-tile rare ore deposits from zone classification.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZoneOreMap {
    pub deposits: Vec<Vec<ZoneOreDeposit>>,
}

impl ZoneOreMap {
    /// Total rare deposits across all tiles.
    pub fn total_deposits(&self) -> usize {
        self.deposits.iter().map(|d| d.len()).sum()
    }

    /// Tiles with at least one rare deposit.
    pub fn tiles_with_deposits(&self) -> usize {
        self.deposits.iter().filter(|d| !d.is_empty()).count()
    }

    /// All deposits of a specific resource across the map.
    pub fn deposits_of(&self, resource: Resource) -> Vec<(usize, &ZoneOreDeposit)> {
        self.deposits
            .iter()
            .enumerate()
            .flat_map(|(i, deps)| deps.iter().map(move |d| (i, d)))
            .filter(|(_, d)| d.resource == resource)
            .collect()
    }
}

/// Generate rare ore deposits based on geological zone classification.
///
/// Each zone rolls its own ore table. Common zone tiles get no rare ores.
pub fn generate_zone_ores(zone_map: &ZoneMap, seed: &str) -> ZoneOreMap {
    let mut rng = SeededDiceRoller::new(seed, "zone_ores");
    let n = zone_map.zones.len();
    let mut deposits = Vec::with_capacity(n);

    for &zone in &zone_map.zones {
        let mut tile_deps = Vec::new();
        for &(resource, chance, purity_range, qty_range) in zone_ore_table(zone) {
            if rng.gen_f64() as f32 <= chance {
                let purity =
                    purity_range.0 + rng.gen_f64() as f32 * (purity_range.1 - purity_range.0);
                let quantity = qty_range.0 + rng.gen_f64() as f32 * (qty_range.1 - qty_range.0);
                tile_deps.push(ZoneOreDeposit {
                    resource,
                    purity,
                    quantity_kt: quantity,
                    zone,
                });
            }
        }
        deposits.push(tile_deps);
    }

    ZoneOreMap { deposits }
}

/// Zone → rare ore probability table.
/// Each entry: (Resource, spawn_chance, (purity_min, purity_max), (qty_min_kt, qty_max_kt))
type OreEntry = (Resource, f32, (f32, f32), (f32, f32));

fn zone_ore_table(zone: GeologicalZone) -> &'static [OreEntry] {
    match zone {
        GeologicalZone::CarbonatitePipe => &[
            (Resource::IronOre, 0.30, (0.3, 0.6), (50.0, 500.0)), // niobium host (iron proxy)
            (Resource::AluminumOre, 0.20, (0.2, 0.5), (20.0, 200.0)), // REE host
            (Resource::Limestone, 0.40, (0.5, 0.9), (100.0, 1000.0)),
        ],
        GeologicalZone::MaficIntrusion => &[
            (Resource::IronOre, 0.50, (0.4, 0.7), (100.0, 1000.0)), // Ni-Cu-PGM host
            (Resource::CopperOre, 0.40, (0.2, 0.5), (50.0, 500.0)),
            (Resource::GoldOre, 0.15, (0.01, 0.05), (1.0, 20.0)),
        ],
        GeologicalZone::PegmatiteField => &[
            (Resource::TinOre, 0.35, (0.1, 0.4), (10.0, 100.0)),
            (Resource::GoldOre, 0.10, (0.005, 0.02), (0.5, 10.0)),
            (Resource::Gemstones, 0.20, (0.01, 0.05), (0.2, 5.0)),
        ],
        GeologicalZone::PorphyrySubduction => &[
            (Resource::CopperOre, 0.60, (0.3, 0.7), (200.0, 2000.0)),
            (Resource::GoldOre, 0.25, (0.01, 0.04), (5.0, 50.0)),
            (Resource::IronOre, 0.20, (0.2, 0.4), (50.0, 300.0)), // Mo-Re host
        ],
        GeologicalZone::LateriteTropical => &[
            (Resource::IronOre, 0.50, (0.4, 0.8), (200.0, 1500.0)), // lateritic Ni-Co
            (Resource::AluminumOre, 0.60, (0.3, 0.6), (100.0, 800.0)), // bauxite
        ],
        GeologicalZone::SedimentaryBasin => &[
            (Resource::Coal, 0.40, (0.5, 0.9), (100.0, 2000.0)),
            (Resource::Oil, 0.25, (0.3, 0.7), (20.0, 500.0)),
            (Resource::NaturalGas, 0.20, (0.4, 0.8), (10.0, 300.0)),
            (Resource::Limestone, 0.50, (0.6, 0.95), (200.0, 3000.0)),
            (Resource::Salt, 0.30, (0.8, 0.98), (50.0, 500.0)),
        ],
        GeologicalZone::HeavyMineralSands => &[
            (Resource::IronOre, 0.30, (0.2, 0.5), (30.0, 200.0)), // Ti-Zr-Hf host
            (Resource::GoldOre, 0.10, (0.005, 0.02), (0.5, 5.0)), // placer gold
        ],
        GeologicalZone::BrineFlat => &[
            (Resource::Salt, 0.70, (0.8, 0.99), (100.0, 1000.0)),
            (Resource::Sulfur, 0.15, (0.3, 0.6), (10.0, 100.0)),
        ],
        GeologicalZone::VolcanicVent => &[
            (Resource::Sulfur, 0.60, (0.5, 0.9), (20.0, 200.0)),
            (Resource::CopperOre, 0.20, (0.1, 0.3), (10.0, 100.0)),
            (Resource::GoldOre, 0.10, (0.01, 0.03), (1.0, 10.0)),
        ],
        GeologicalZone::ImpactCrater => &[
            (Resource::IronOre, 0.40, (0.5, 0.9), (50.0, 500.0)), // Ni-Fe meteoritic
            (Resource::GoldOre, 0.15, (0.01, 0.05), (1.0, 20.0)),
            (Resource::Gemstones, 0.10, (0.01, 0.03), (0.1, 2.0)), // shocked quartz/diamonds
        ],
        GeologicalZone::Common => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{generate_surface_grid, GridResolution};
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};
    use crate::zones::classify_zones;

    fn earth_setup() -> ZoneMap {
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
        let g = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "zore");
        classify_zones(&g)
    }

    #[test]
    fn zone_ore_map_has_correct_size() {
        let zm = earth_setup();
        let om = generate_zone_ores(&zm, "test");
        assert_eq!(om.deposits.len(), zm.zones.len());
    }

    #[test]
    fn common_tiles_have_no_zone_ores() {
        let zm = earth_setup();
        let om = generate_zone_ores(&zm, "common");
        for (idx, deps) in om.deposits.iter().enumerate() {
            if zm.zones[idx] == GeologicalZone::Common {
                assert!(
                    deps.is_empty(),
                    "Common tile {} should have no zone ores",
                    idx
                );
            }
        }
    }

    #[test]
    fn some_tiles_have_rare_deposits() {
        let zm = earth_setup();
        let om = generate_zone_ores(&zm, "rare");
        assert!(om.total_deposits() > 0, "expected some rare ore deposits");
    }

    #[test]
    fn deposits_match_their_zone() {
        let zm = earth_setup();
        let om = generate_zone_ores(&zm, "match");
        for (idx, deps) in om.deposits.iter().enumerate() {
            for dep in deps {
                assert_eq!(
                    dep.zone, zm.zones[idx],
                    "deposit zone {:?} doesn't match tile zone {:?} at {}",
                    dep.zone, zm.zones[idx], idx
                );
            }
        }
    }

    #[test]
    fn purity_in_unit_range() {
        let zm = earth_setup();
        let om = generate_zone_ores(&zm, "purity");
        for deps in &om.deposits {
            for dep in deps {
                assert!(
                    (0.0..=1.0).contains(&dep.purity),
                    "purity {} out of range",
                    dep.purity
                );
            }
        }
    }

    #[test]
    fn zone_ores_are_deterministic() {
        let zm = earth_setup();
        let a = generate_zone_ores(&zm, "det");
        let b = generate_zone_ores(&zm, "det");
        assert_eq!(a.total_deposits(), b.total_deposits());
    }

    #[test]
    fn deposits_of_filters_correctly() {
        let zm = earth_setup();
        let om = generate_zone_ores(&zm, "filter");
        let copper = om.deposits_of(Resource::CopperOre);
        for (idx, dep) in &copper {
            assert_eq!(dep.resource, Resource::CopperOre);
            assert_ne!(zm.zones[*idx], GeologicalZone::Common);
        }
    }

    #[test]
    fn sedimentary_basin_has_fossil_fuels() {
        let zm = earth_setup();
        let om = generate_zone_ores(&zm, "fossil");
        let sed_tiles = zm.tiles_in_zone(GeologicalZone::SedimentaryBasin);
        if sed_tiles.len() > 10 {
            let has_coal = sed_tiles.iter().any(|&idx| {
                om.deposits[idx]
                    .iter()
                    .any(|d| d.resource == Resource::Coal)
            });
            assert!(has_coal, "sedimentary basin should have coal deposits");
        }
    }
}
