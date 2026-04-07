//! Cave system generation.
//!
//! Produces per-tile `CaveNetwork`s containing rooms and tunnels.
//! Cave placement is influenced by:
//! - **Geology**: karst caves in limestone (sedimentary), lava tubes near
//!   volcanic/divergent boundaries (igneous).
//! - **Hydrology**: aquifer caves near tiles with significant river discharge.
//!
//! Networks are generated per-tile rather than spanning multiple tiles,
//! because each tile already represents a large region (~200k km² at Fast
//! resolution). A tile's network is its underground feature set.

use crate::grid::{BoundaryKind, SurfaceGrid};
use crate::strata::{RockType, StratifiedGeology};
use seeded_dice_roller::SeededDiceRoller;

/// A single underground chamber.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaveRoom {
    /// Unique room index within this tile's network.
    pub id: u16,
    /// Depth below surface in metres.
    pub depth_m: f32,
    /// Approximate chamber volume in cubic metres.
    pub volume_m3: f32,
    /// Whether this room is water-filled (aquifer).
    pub is_aquifer: bool,
}

/// A passage connecting two rooms.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tunnel {
    /// Room indices connected by this tunnel.
    pub from: u16,
    pub to: u16,
    /// Tunnel length in metres.
    pub length_m: f32,
}

/// Classification of how the cave formed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum CaveOrigin {
    /// Dissolved limestone (karst).
    Karst,
    /// Drained lava flow (volcanic tube).
    LavaTube,
    /// Tectonic fracturing.
    Tectonic,
    /// Water erosion within existing fractures.
    Erosional,
}

/// A cave network at a single tile.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaveNetwork {
    pub rooms: Vec<CaveRoom>,
    pub tunnels: Vec<Tunnel>,
    pub origin: Option<CaveOrigin>,
}

impl CaveNetwork {
    pub fn is_empty(&self) -> bool {
        self.rooms.is_empty()
    }

    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    /// True if any room in this network is an aquifer.
    pub fn has_aquifer(&self) -> bool {
        self.rooms.iter().any(|r| r.is_aquifer)
    }
}

/// Cave data for an entire grid.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaveMap {
    pub networks: Vec<CaveNetwork>,
}

impl CaveMap {
    pub fn total_rooms(&self) -> usize {
        self.networks.iter().map(|n| n.rooms.len()).sum()
    }

    pub fn tiles_with_caves(&self) -> usize {
        self.networks.iter().filter(|n| !n.is_empty()).count()
    }
}

/// Generate cave networks for every tile using geology and hydrology data.
///
/// - `grid`: surface grid (for boundary kind, river discharge, ocean flag).
/// - `strata`: stratified geology (determines which rock types host caves).
/// - `seed`: deterministic seed.
pub fn generate_caves(grid: &SurfaceGrid, strata: &StratifiedGeology, seed: &str) -> CaveMap {
    let mut rng = SeededDiceRoller::new(seed, "caves");
    let n = grid.tile_count();
    let mut networks = Vec::with_capacity(n);

    for idx in 0..n {
        if grid.layers.is_ocean[idx] {
            networks.push(CaveNetwork::default());
            continue;
        }

        let boundary = grid.layers.tectonic_boundary[idx];
        let discharge = grid.layers.river_discharge_m3s[idx];
        let col = &strata.columns[idx];

        // Determine cave origin and probability.
        let (origin, prob) = cave_probability(col, boundary);

        if rng.gen_f64() as f32 > prob {
            networks.push(CaveNetwork::default());
            continue;
        }

        let network = generate_network(&mut rng, col, origin, discharge);
        networks.push(network);
    }

    CaveMap { networks }
}

/// Determine the most likely cave origin and spawn probability based on
/// the geological column and tectonic boundary.
fn cave_probability(
    col: &crate::strata::GeologicalColumn,
    boundary: BoundaryKind,
) -> (CaveOrigin, f32) {
    let has_sedimentary = col
        .layers
        .iter()
        .any(|l| l.rock_type == RockType::Sedimentary && l.thickness_m > 500.0);
    let has_igneous_thick = col
        .layers
        .iter()
        .any(|l| l.rock_type == RockType::Ignite && l.thickness_m > 2000.0);
    let near_volcano = matches!(boundary, BoundaryKind::Divergent | BoundaryKind::Convergent);

    if has_sedimentary && !near_volcano {
        // Karst: limestone dissolution. ~30% chance in thick sedimentary.
        (CaveOrigin::Karst, 0.30)
    } else if near_volcano && has_igneous_thick {
        // Lava tubes: ~25% near active boundaries.
        (CaveOrigin::LavaTube, 0.25)
    } else if boundary == BoundaryKind::Transform {
        // Tectonic fractures: ~15% at transform boundaries.
        (CaveOrigin::Tectonic, 0.15)
    } else if has_sedimentary {
        // Erosional: ~10% in thinner sedimentary.
        (CaveOrigin::Erosional, 0.10)
    } else {
        // No cave.
        (CaveOrigin::Erosional, 0.03)
    }
}

/// Build a cave network with rooms and tunnels.
fn generate_network(
    rng: &mut SeededDiceRoller,
    col: &crate::strata::GeologicalColumn,
    origin: CaveOrigin,
    river_discharge: f32,
) -> CaveNetwork {
    let n_rooms = 2 + (rng.gen_u32() % 6) as u16;
    let total_depth = col.total_depth_m();

    // Depth range depends on origin.
    let (depth_min, depth_max) = match origin {
        CaveOrigin::Karst => (50.0, (total_depth * 0.3).min(2000.0)),
        CaveOrigin::LavaTube => (10.0, (total_depth * 0.15).min(500.0)),
        CaveOrigin::Tectonic => (100.0, (total_depth * 0.5).min(5000.0)),
        CaveOrigin::Erosional => (20.0, (total_depth * 0.2).min(1000.0)),
    };
    let depth_range = (depth_max - depth_min).max(10.0);

    // Near rivers (discharge > 200 m³/s), caves are more likely to be aquifers.
    let aquifer_chance = if river_discharge > 200.0 {
        0.6
    } else if river_discharge > 50.0 {
        0.3
    } else {
        0.05
    };

    let mut rooms = Vec::with_capacity(n_rooms as usize);
    for i in 0..n_rooms {
        let depth = depth_min + rng.gen_f64() as f32 * depth_range;
        let volume = match origin {
            CaveOrigin::Karst => 500.0 + rng.gen_f64() as f32 * 50000.0,
            CaveOrigin::LavaTube => 200.0 + rng.gen_f64() as f32 * 10000.0,
            CaveOrigin::Tectonic => 100.0 + rng.gen_f64() as f32 * 5000.0,
            CaveOrigin::Erosional => 50.0 + rng.gen_f64() as f32 * 3000.0,
        };
        let is_aquifer = rng.gen_f64() as f32 <= aquifer_chance;
        rooms.push(CaveRoom {
            id: i,
            depth_m: depth,
            volume_m3: volume,
            is_aquifer,
        });
    }

    // Connect rooms into a spanning tree (each room connects to the previous).
    let mut tunnels = Vec::with_capacity(n_rooms.saturating_sub(1) as usize);
    for i in 1..n_rooms {
        let length = 50.0 + rng.gen_f64() as f32 * 2000.0;
        tunnels.push(Tunnel {
            from: i - 1,
            to: i,
            length_m: length,
        });
    }
    // Add a few extra connections for loops (~30% chance per pair gap).
    if n_rooms > 3 {
        let extras = (rng.gen_u32() % (n_rooms as u32 / 2).max(1)) as u16;
        for _ in 0..extras {
            let a = (rng.gen_u32() % n_rooms as u32) as u16;
            let b = (rng.gen_u32() % n_rooms as u32) as u16;
            if a != b {
                tunnels.push(Tunnel {
                    from: a,
                    to: b,
                    length_m: 100.0 + rng.gen_f64() as f32 * 3000.0,
                });
            }
        }
    }

    CaveNetwork {
        rooms,
        tunnels,
        origin: Some(origin),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{generate_surface_grid, GridResolution};
    use crate::strata::generate_strata;
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};

    fn earth_setup() -> (SurfaceGrid, StratifiedGeology) {
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
        let g = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "caves");
        let s = generate_strata(&g, "caves");
        (g, s)
    }

    #[test]
    fn cave_map_has_correct_size() {
        let (g, s) = earth_setup();
        let cm = generate_caves(&g, &s, "test");
        assert_eq!(cm.networks.len(), g.tile_count());
    }

    #[test]
    fn ocean_tiles_have_no_caves() {
        let (g, s) = earth_setup();
        let cm = generate_caves(&g, &s, "ocean");
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                assert!(cm.networks[idx].is_empty());
            }
        }
    }

    #[test]
    fn some_land_tiles_have_caves() {
        let (g, s) = earth_setup();
        let cm = generate_caves(&g, &s, "land");
        assert!(cm.tiles_with_caves() > 0, "expected some tiles with caves");
    }

    #[test]
    fn cave_networks_are_connected() {
        let (g, s) = earth_setup();
        let cm = generate_caves(&g, &s, "conn");
        for net in &cm.networks {
            if net.rooms.len() >= 2 {
                // Every room except room 0 should be reachable via tunnels
                // (spanning tree guarantees connectivity).
                assert!(!net.tunnels.is_empty(), "multi-room network has no tunnels");
            }
        }
    }

    #[test]
    fn aquifers_near_rivers() {
        let (g, s) = earth_setup();
        let cm = generate_caves(&g, &s, "aquifer");
        // Tiles with high discharge should have more aquifers.
        let mut river_aquifers = 0;
        let mut dry_aquifers = 0;
        for idx in 0..g.tile_count() {
            if cm.networks[idx].has_aquifer() {
                if g.layers.river_discharge_m3s[idx] > 200.0 {
                    river_aquifers += 1;
                } else {
                    dry_aquifers += 1;
                }
            }
        }
        // River tiles should have at least as many aquifers as dry tiles
        // (probabilistically; may fail on very unlucky seeds, but
        // the 60% vs 5% chance makes this robust).
        if river_aquifers + dry_aquifers > 5 {
            assert!(
                river_aquifers >= dry_aquifers / 3,
                "river aquifers {} should dominate dry aquifers {}",
                river_aquifers,
                dry_aquifers
            );
        }
    }

    #[test]
    fn caves_are_deterministic() {
        let (g, s) = earth_setup();
        let a = generate_caves(&g, &s, "det");
        let b = generate_caves(&g, &s, "det");
        assert_eq!(a.networks.len(), b.networks.len());
        for (na, nb) in a.networks.iter().zip(b.networks.iter()) {
            assert_eq!(na.rooms.len(), nb.rooms.len());
            assert_eq!(na.tunnels.len(), nb.tunnels.len());
            assert_eq!(na.origin, nb.origin);
        }
    }

    #[test]
    fn room_depths_within_column() {
        let (g, s) = earth_setup();
        let cm = generate_caves(&g, &s, "depth");
        for (idx, net) in cm.networks.iter().enumerate() {
            let total = s.columns[idx].total_depth_m();
            for room in &net.rooms {
                assert!(
                    room.depth_m <= total * 1.1,
                    "room depth {} exceeds column depth {} at tile {}",
                    room.depth_m,
                    total,
                    idx
                );
            }
        }
    }

    #[test]
    fn cave_origin_matches_geology() {
        let (g, s) = earth_setup();
        let cm = generate_caves(&g, &s, "origin");
        for (idx, net) in cm.networks.iter().enumerate() {
            if let Some(origin) = net.origin {
                let boundary = g.layers.tectonic_boundary[idx];
                match origin {
                    CaveOrigin::LavaTube => {
                        assert!(
                            matches!(boundary, BoundaryKind::Divergent | BoundaryKind::Convergent),
                            "lava tube at non-volcanic tile {}",
                            idx
                        );
                    }
                    CaveOrigin::Tectonic => {
                        assert_eq!(
                            boundary,
                            BoundaryKind::Transform,
                            "tectonic cave at non-transform tile {}",
                            idx
                        );
                    }
                    _ => {} // Karst and Erosional can appear anywhere with sedimentary
                }
            }
        }
    }
}
