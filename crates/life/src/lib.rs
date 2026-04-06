//! # Life
//!
//! Biological simulation: species generation, ecosystems, settlements,
//! history, and expansion. Independent of the `world` crate — all
//! physical facts arrive via a lightweight `HabitatGrid` adapter.
//!
//! ## Key modules
//!
//! - `species` — body plan, trophic level, biochemistry, reproduction.
//! - `ecosystem` — food web with predator-prey links, keystone detection,
//!   extinction cascades.
//! - `habitat` — per-tile habitability, vegetation, species ranges.
//! - `settlement` — suitability scoring + greedy placement.
//! - `history` — planetary timeline (geological + biological eras),
//!   species evolution history, technology era → recipe gating.
//! - `naming` — order-2 Markov chain name generator (5 built-in styles).
//! - `expansion` — interstellar expansion footprint by tech level.

#![allow(dead_code, unused_imports)]

pub mod ecosystem;
pub mod evolution;
pub mod expansion;
pub mod generator;
pub mod habitat;
pub mod history;
pub mod input;
pub mod naming;
pub mod settlement;
pub mod species;
pub mod types;

pub use ecosystem::{apply_extinction, generate_ecosystem_from_world, Ecosystem};
pub use expansion::{generate_expansion_footprint, ExpansionFootprint};
pub use habitat::{
    compute_species_range, distribute_ecosystem, generate_vegetation, HabitatGrid,
    LifeDistribution, SpeciesRange,
};
pub use history::{generate_planetary_timeline, HistoricalEra, PlanetaryTimeline};
pub use input::SpeciesGenerationInput;
pub use naming::{MarkovNameGen, NameStyle};
pub use settlement::{compute_settlement_suitability, place_settlements, Settlement};
pub use species::Species;
pub use types::{Biome, Climate, Habitat, LifeLevel, Temperature};
