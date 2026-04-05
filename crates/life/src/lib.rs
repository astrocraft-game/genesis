#![allow(dead_code, unused_imports)]

pub mod ecosystem;
pub mod expansion;
pub mod generator;
pub mod habitat;
pub mod history;
pub mod input;
pub mod naming;
pub mod settlement;
pub mod species;
pub mod types;

pub use ecosystem::{generate_ecosystem_from_world, Ecosystem};
pub use expansion::{generate_expansion_footprint, ExpansionFootprint};
pub use habitat::{
    compute_species_range, distribute_ecosystem, generate_vegetation, HabitatGrid,
    LifeDistribution, SpeciesRange,
};
pub use history::HistoricalEra;
pub use input::SpeciesGenerationInput;
pub use naming::{MarkovNameGen, NameStyle};
pub use settlement::{compute_settlement_suitability, place_settlements, Settlement};
pub use species::Species;
pub use types::{Biome, Climate, Habitat, LifeLevel, Temperature};
