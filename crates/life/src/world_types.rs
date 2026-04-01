//! Minimal world types needed by the life crate.
//! These mirror the types from planet_generator to keep this crate standalone.

use serde::{Serialize, Deserialize};
use smart_default::SmartDefault;

#[derive(Clone, Copy, PartialEq, Eq, Debug, SmartDefault, Serialize, Deserialize)]
pub enum CelestialBodyWorldType {
    #[default] Rock, ProtoWorld, Ice, DirtySnowball, GeoActive, Hadean,
    Ammonia, Terrestrial, Ocean, Greenhouse, Chthonian, VolatilesGiant,
    CarbonWorld, LavaWorld, EyeballWorld, RoguePlanet, IronWorld, MiniNeptune,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, SmartDefault, Serialize, Deserialize)]
pub enum WorldClimateType {
    #[default] Dead, Desert, Steppe, Savanna, Terrestrial, Taiga, Tundra,
    MudBall, Jungle, Tropical, Ribbon, Arctic, Rainforest, Ocean,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, SmartDefault, Serialize, Deserialize)]
pub enum WorldTemperatureCategory {
    #[default] Frozen, VeryCold, Cold, Chilly, Cool, Temperate, Warm, Hot, VeryHot, Scorching, Infernal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, SmartDefault, Serialize, Deserialize)]
pub struct SpaceCoordinates {
    pub x: i64, pub y: i64, pub z: i64,
}

impl SpaceCoordinates {
    pub fn new(x: i64, y: i64, z: i64) -> Self { Self { x, y, z } }
}

impl std::fmt::Display for SpaceCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
