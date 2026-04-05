use crate::internal::*;
use crate::prelude::*;

pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct IcyBodyDetails {
    pub world_type: CelestialBodyWorldType,
    pub special_traits: Vec<CelestialBodySpecialTrait>,
}

impl IcyBodyDetails {
    pub fn new(
        world_type: CelestialBodyWorldType,
        special_traits: Vec<CelestialBodySpecialTrait>,
    ) -> Self {
        Self {
            world_type,
            special_traits,
        }
    }
}
