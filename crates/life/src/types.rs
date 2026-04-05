use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt::{self, Display};

/// How advanced life is on a given world. Mirrors the ladder used by `world`
/// but owned here so `life` has no upstream dependency.
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum LifeLevel {
    #[default]
    None,
    UniCellular,
    PluriCellular,
    PlantLike,
    AnimalLike,
    Sentient,
}

impl LifeLevel {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::UniCellular => 1,
            Self::PluriCellular => 2,
            Self::PlantLike => 3,
            Self::AnimalLike => 4,
            Self::Sentient => 5,
        }
    }
}

/// Climate bucket describing the homeworld's prevailing surface conditions.
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum Climate {
    #[default]
    Terrestrial,
    MudBall,
    Ocean,
    Arctic,
    Rainforest,
    Tropical,
    Jungle,
    Tundra,
    Taiga,
    Savanna,
    Steppe,
    Desert,
    Ribbon,
    Dead,
}

impl Display for Climate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Climate::Terrestrial => "Terrestrial",
                Climate::MudBall => "Mud Ball",
                Climate::Ocean => "Ocean",
                Climate::Arctic => "Arctic",
                Climate::Rainforest => "Rainforest",
                Climate::Tropical => "Tropical",
                Climate::Jungle => "Jungle",
                Climate::Tundra => "Tundra",
                Climate::Taiga => "Taiga",
                Climate::Savanna => "Savanna",
                Climate::Steppe => "Steppe",
                Climate::Desert => "Desert",
                Climate::Ribbon => "Ribbon",
                Climate::Dead => "Dead",
            }
        )
    }
}

/// Coarse temperature band used to derive a species' preferred thermal range.
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum Temperature {
    #[default]
    Frozen,
    VeryCold,
    Cold,
    Chilly,
    Cool,
    Temperate,
    Warm,
    Hot,
    VeryHot,
    Scorching,
    Infernal,
}

/// High-level classification of a homeworld. Only the broad class matters for
/// species generation — detailed geology/atmosphere live in the `world` crate.
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum Habitat {
    ProtoWorld,
    Ice,
    DirtySnowball,
    GeoActive,
    #[default]
    Rock,
    Hadean,
    Ammonia,
    Ocean,
    Terrestrial,
    Greenhouse,
    Chthonian,
    VolatilesGiant,
    CarbonWorld,
    LavaWorld,
    EyeballWorld,
    RoguePlanet,
    IronWorld,
    MiniNeptune,
}
