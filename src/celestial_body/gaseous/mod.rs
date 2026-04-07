use crate::internal::*;
use crate::prelude::*;

pub mod constants;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct GaseousBodyDetails {
    pub special_traits: Vec<CelestialBodySpecialTrait>,
}

impl GaseousBodyDetails {
    pub fn new(special_traits: Vec<CelestialBodySpecialTrait>) -> Self {
        Self { special_traits }
    }
}
