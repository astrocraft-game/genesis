#![allow(dead_code, unused_imports)]

pub mod ecosystem;
pub mod expansion;
pub mod generator;
pub mod history;
pub mod input;
pub mod species;
pub mod types;

pub use ecosystem::{generate_ecosystem_from_world, Ecosystem};
pub use expansion::{generate_expansion_footprint, ExpansionFootprint};
pub use history::HistoricalEra;
pub use input::SpeciesGenerationInput;
pub use species::Species;
pub use types::{Climate, Habitat, LifeLevel, Temperature};
