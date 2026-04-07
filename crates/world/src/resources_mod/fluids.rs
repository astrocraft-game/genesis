//! Fluid resource model — oil/gas reservoirs and geothermal vents.
//!
//! Fluid resources differ from solid ores: they have pressure and flow
//! rate, deplete differently (pressure drop), and geothermal vents are
//! permanent energy sources that never deplete.

use crate::grid::{BoundaryKind, SurfaceGrid};
use crate::strata::{RockType, StratifiedGeology};
use seeded_dice_roller::SeededDiceRoller;

/// Classification of a fluid resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum FluidKind {
    /// Crude oil reservoir (finite, sedimentary).
    Oil,
    /// Natural gas pocket (finite, sedimentary).
    NaturalGas,
    /// Geothermal vent (permanent, volcanic).
    Geothermal,
}

/// A single fluid resource deposit at a tile.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FluidNode {
    pub kind: FluidKind,
    /// Reservoir pressure in bar. Drops as fluid is extracted (except geothermal).
    pub pressure_bar: f32,
    /// Initial pressure for depletion tracking.
    pub initial_pressure_bar: f32,
    /// Flow rate in litres per second at current pressure.
    pub flow_rate_lps: f32,
    /// Maximum flow rate at full pressure.
    pub max_flow_rate_lps: f32,
    /// Depth below surface in metres.
    pub depth_m: f32,
    /// Remaining quantity in megalitres (0 = spent). Geothermal is `f64::INFINITY`.
    pub remaining_ml: f64,
    /// Whether this source is permanent (geothermal).
    pub permanent: bool,
}

impl FluidNode {
    /// Whether this reservoir is fully depleted.
    pub fn is_spent(&self) -> bool {
        !self.permanent && self.remaining_ml <= 0.0
    }

    /// Extract fluid. Reduces remaining quantity and pressure proportionally.
    /// Returns actual litres extracted. Geothermal always yields at max rate.
    pub fn extract(&mut self, seconds: f32) -> f64 {
        if self.is_spent() {
            return 0.0;
        }
        if self.permanent {
            return self.flow_rate_lps as f64 * seconds as f64;
        }
        let litres = self.flow_rate_lps as f64 * seconds as f64;
        let actual = litres.min(self.remaining_ml * 1_000_000.0); // ml → litres
        self.remaining_ml -= actual / 1_000_000.0;
        // Pressure drops proportionally to depletion.
        let frac = if self.remaining_ml > 0.0 {
            (self.remaining_ml / (self.initial_pressure_bar as f64 * 100.0)).min(1.0)
        // rough proxy
        } else {
            0.0
        };
        self.pressure_bar = self.initial_pressure_bar * frac as f32;
        self.flow_rate_lps =
            self.max_flow_rate_lps * (self.pressure_bar / self.initial_pressure_bar).sqrt();
        actual
    }
}

/// Per-tile fluid resources.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TileFluids {
    pub nodes: Vec<FluidNode>,
}

impl TileFluids {
    pub fn has_geothermal(&self) -> bool {
        self.nodes.iter().any(|n| n.kind == FluidKind::Geothermal)
    }

    pub fn has_oil(&self) -> bool {
        self.nodes
            .iter()
            .any(|n| n.kind == FluidKind::Oil && !n.is_spent())
    }

    pub fn has_gas(&self) -> bool {
        self.nodes
            .iter()
            .any(|n| n.kind == FluidKind::NaturalGas && !n.is_spent())
    }
}

/// Fluid resource map for an entire grid.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FluidMap {
    pub tiles: Vec<TileFluids>,
}

impl FluidMap {
    pub fn geothermal_tile_count(&self) -> usize {
        self.tiles.iter().filter(|t| t.has_geothermal()).count()
    }

    pub fn oil_tile_count(&self) -> usize {
        self.tiles.iter().filter(|t| t.has_oil()).count()
    }

    pub fn gas_tile_count(&self) -> usize {
        self.tiles.iter().filter(|t| t.has_gas()).count()
    }
}

/// Generate fluid resources from the surface grid and stratified geology.
///
/// - **Geothermal vents**: placed at convergent/divergent boundary tiles
///   (volcanic activity). Permanent, never deplete.
/// - **Oil reservoirs**: placed in thick sedimentary layers on continental
///   plates. Finite, deplete under extraction.
/// - **Natural gas**: co-located with oil or in thinner sedimentary pockets.
pub fn generate_fluids(grid: &SurfaceGrid, strata: &StratifiedGeology, seed: &str) -> FluidMap {
    let mut rng = SeededDiceRoller::new(seed, "fluids");
    let n = grid.tile_count();
    let mut tiles = Vec::with_capacity(n);

    for idx in 0..n {
        let mut nodes = Vec::new();

        if grid.layers.is_ocean[idx] {
            tiles.push(TileFluids { nodes });
            continue;
        }

        let boundary = grid.layers.tectonic_boundary[idx];
        let col = &strata.columns[idx];

        // Geothermal: ~40% at convergent/divergent boundaries.
        if matches!(boundary, BoundaryKind::Convergent | BoundaryKind::Divergent)
            && rng.gen_f64() < 0.40
        {
            let depth = 500.0 + rng.gen_f64() as f32 * 3000.0;
            let flow = 5.0 + rng.gen_f64() as f32 * 50.0;
            nodes.push(FluidNode {
                kind: FluidKind::Geothermal,
                pressure_bar: 100.0 + rng.gen_f64() as f32 * 300.0,
                initial_pressure_bar: 0.0, // not used for permanent
                flow_rate_lps: flow,
                max_flow_rate_lps: flow,
                depth_m: depth,
                remaining_ml: f64::INFINITY,
                permanent: true,
            });
        }

        // Oil/gas: in thick sedimentary layers.
        let thick_sed = col
            .layers
            .iter()
            .find(|l| l.rock_type == RockType::Sedimentary && l.thickness_m > 800.0);

        if let Some(sed) = thick_sed {
            let depth = sed.depth_top_m + sed.thickness_m * 0.6;

            // Oil: ~20% in thick sedimentary.
            if rng.gen_f64() < 0.20 {
                let pressure = 50.0 + rng.gen_f64() as f32 * 250.0;
                let flow = 2.0 + rng.gen_f64() as f32 * 30.0;
                let quantity = 1.0 + rng.gen_f64() * 50.0; // megalitres
                nodes.push(FluidNode {
                    kind: FluidKind::Oil,
                    pressure_bar: pressure,
                    initial_pressure_bar: pressure,
                    flow_rate_lps: flow,
                    max_flow_rate_lps: flow,
                    depth_m: depth,
                    remaining_ml: quantity,
                    permanent: false,
                });
            }

            // Natural gas: ~25% (can co-occur with oil).
            if rng.gen_f64() < 0.25 {
                let pressure = 30.0 + rng.gen_f64() as f32 * 200.0;
                let flow = 5.0 + rng.gen_f64() as f32 * 40.0;
                let quantity = 0.5 + rng.gen_f64() * 30.0;
                nodes.push(FluidNode {
                    kind: FluidKind::NaturalGas,
                    pressure_bar: pressure,
                    initial_pressure_bar: pressure,
                    flow_rate_lps: flow,
                    max_flow_rate_lps: flow,
                    depth_m: depth + 100.0,
                    remaining_ml: quantity,
                    permanent: false,
                });
            }
        }

        tiles.push(TileFluids { nodes });
    }

    FluidMap { tiles }
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
        let g = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "fluids");
        let s = generate_strata(&g, "fluids");
        (g, s)
    }

    #[test]
    fn fluid_map_has_correct_size() {
        let (g, s) = earth_setup();
        let fm = generate_fluids(&g, &s, "sz");
        assert_eq!(fm.tiles.len(), g.tile_count());
    }

    #[test]
    fn geothermal_only_near_volcanics() {
        let (g, s) = earth_setup();
        let fm = generate_fluids(&g, &s, "geo");
        for (idx, tile) in fm.tiles.iter().enumerate() {
            if tile.has_geothermal() {
                let b = g.layers.tectonic_boundary[idx];
                assert!(
                    matches!(b, BoundaryKind::Convergent | BoundaryKind::Divergent),
                    "geothermal at non-volcanic tile {} ({:?})",
                    idx,
                    b
                );
            }
        }
    }

    #[test]
    fn oil_only_in_sedimentary() {
        let (g, s) = earth_setup();
        let fm = generate_fluids(&g, &s, "oil");
        for (idx, tile) in fm.tiles.iter().enumerate() {
            if tile.has_oil() {
                let has_sed = s.columns[idx]
                    .layers
                    .iter()
                    .any(|l| l.rock_type == RockType::Sedimentary && l.thickness_m > 800.0);
                assert!(
                    has_sed,
                    "oil at tile {} without thick sedimentary layer",
                    idx
                );
            }
        }
    }

    #[test]
    fn geothermal_is_permanent() {
        let (g, s) = earth_setup();
        let fm = generate_fluids(&g, &s, "perm");
        for tile in &fm.tiles {
            for node in &tile.nodes {
                if node.kind == FluidKind::Geothermal {
                    assert!(node.permanent);
                    assert!(!node.is_spent());
                    assert!(node.remaining_ml.is_infinite());
                }
            }
        }
    }

    #[test]
    fn oil_depletes_under_extraction() {
        let mut node = FluidNode {
            kind: FluidKind::Oil,
            pressure_bar: 200.0,
            initial_pressure_bar: 200.0,
            flow_rate_lps: 10.0,
            max_flow_rate_lps: 10.0,
            depth_m: 2000.0,
            remaining_ml: 1.0, // 1 megalitre = 1,000,000 litres
            permanent: false,
        };
        let extracted = node.extract(1.0); // 1 second at 10 l/s = 10 litres
        assert!(extracted > 0.0);
        assert!(node.remaining_ml < 1.0);
        assert!(!node.is_spent());
    }

    #[test]
    fn geothermal_extract_never_depletes() {
        let mut node = FluidNode {
            kind: FluidKind::Geothermal,
            pressure_bar: 300.0,
            initial_pressure_bar: 0.0,
            flow_rate_lps: 20.0,
            max_flow_rate_lps: 20.0,
            depth_m: 1000.0,
            remaining_ml: f64::INFINITY,
            permanent: true,
        };
        let extracted = node.extract(3600.0); // 1 hour
        assert!((extracted - 72000.0).abs() < 1.0); // 20 * 3600
        assert!(!node.is_spent());
    }

    #[test]
    fn some_tiles_have_fluids() {
        let (g, s) = earth_setup();
        let fm = generate_fluids(&g, &s, "some");
        let total: usize = fm.tiles.iter().map(|t| t.nodes.len()).sum();
        assert!(total > 5, "expected some fluid nodes, got {}", total);
    }

    #[test]
    fn fluids_are_deterministic() {
        let (g, s) = earth_setup();
        let a = generate_fluids(&g, &s, "det");
        let b = generate_fluids(&g, &s, "det");
        assert_eq!(a.tiles.len(), b.tiles.len());
        for (ta, tb) in a.tiles.iter().zip(b.tiles.iter()) {
            assert_eq!(ta.nodes.len(), tb.nodes.len());
            for (na, nb) in ta.nodes.iter().zip(tb.nodes.iter()) {
                assert_eq!(na.kind, nb.kind);
                assert_eq!(na.depth_m, nb.depth_m);
            }
        }
    }

    #[test]
    fn ocean_tiles_have_no_fluids() {
        let (g, s) = earth_setup();
        let fm = generate_fluids(&g, &s, "ocean");
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                assert!(
                    fm.tiles[idx].nodes.is_empty(),
                    "ocean tile {} has fluids",
                    idx
                );
            }
        }
    }
}
