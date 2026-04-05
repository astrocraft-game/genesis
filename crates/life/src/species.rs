use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum Biochemistry {
    #[default]
    CarbonWater,
    Ammonia,
    Silicon,
    Methane,
    Exotic,
}

impl Display for Biochemistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Biochemistry::CarbonWater => "Carbon/Water",
                Biochemistry::Ammonia => "Ammonia",
                Biochemistry::Silicon => "Silicon",
                Biochemistry::Methane => "Methane",
                Biochemistry::Exotic => "Exotic",
            }
        )
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum BodyPlan {
    #[default]
    Vertebrate,
    Arthropod,
    Mollusk,
    PlantLike,
    Amorphous,
    Crystalline,
}

impl Display for BodyPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BodyPlan::Vertebrate => "Vertebrate",
                BodyPlan::Arthropod => "Arthropod",
                BodyPlan::Mollusk => "Mollusk",
                BodyPlan::PlantLike => "Plant-like",
                BodyPlan::Amorphous => "Amorphous",
                BodyPlan::Crystalline => "Crystalline",
            }
        )
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum LocomotionType {
    #[default]
    Walker,
    Swimmer,
    Flyer,
    Burrower,
    Sessile,
    Floater,
}

impl Display for LocomotionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LocomotionType::Walker => "Walker",
                LocomotionType::Swimmer => "Swimmer",
                LocomotionType::Flyer => "Flyer",
                LocomotionType::Burrower => "Burrower",
                LocomotionType::Sessile => "Sessile",
                LocomotionType::Floater => "Floater",
            }
        )
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum TrophicLevel {
    Autotroph,
    Herbivore,
    #[default]
    Omnivore,
    Carnivore,
    FilterFeeder,
    Parasite,
}

impl Display for TrophicLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TrophicLevel::Autotroph => "Autotroph",
                TrophicLevel::Herbivore => "Herbivore",
                TrophicLevel::Omnivore => "Omnivore",
                TrophicLevel::Carnivore => "Carnivore",
                TrophicLevel::FilterFeeder => "Filter Feeder",
                TrophicLevel::Parasite => "Parasite",
            }
        )
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum SizeClass {
    Microscopic,
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Huge,
    Colossal,
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum SocialStructure {
    Solitary,
    Pair,
    #[default]
    Pack,
    Herd,
    Hive,
    Collective,
}

impl Display for SocialStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SocialStructure::Solitary => "Solitary",
                SocialStructure::Pair => "Pair-bonded",
                SocialStructure::Pack => "Pack",
                SocialStructure::Herd => "Herd",
                SocialStructure::Hive => "Hive",
                SocialStructure::Collective => "Collective",
            }
        )
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum ReproductionType {
    #[default]
    Sexual,
    Asexual,
    Hermaphroditic,
    Budding,
    Spore,
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum SpeciesTrait {
    #[default]
    None,
    Psionic,
    HiveMind,
    Metamorphic,
    Amphibious,
    Bioluminescent,
    Venomous,
    Armored,
    Regenerative,
    LongLived,
    ShortLived,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct Species {
    /// Species name.
    #[default("Unnamed Species")]
    pub name: Rc<str>,
    /// Biochemical basis.
    pub biochemistry: Biochemistry,
    /// Body structure.
    pub body_plan: BodyPlan,
    /// Movement types.
    pub locomotion: Vec<LocomotionType>,
    /// Feeding strategy.
    pub trophic_level: TrophicLevel,
    /// Physical size category.
    pub size_class: SizeClass,
    /// How they reproduce.
    pub reproduction: ReproductionType,
    /// Social organization.
    pub social_structure: SocialStructure,
    /// Intelligence level (maps to LifeLevel).
    pub intelligence: u8,
    /// Technology level (0-15), None if non-sapient.
    pub tech_level: Option<u8>,
    /// Average lifespan in years.
    pub lifespan_years: f32,
    /// Preferred temperature range in Kelvin.
    pub preferred_temp_range: (f32, f32),
    /// Preferred gravity range in g.
    pub preferred_gravity_range: (f32, f32),
    /// Special biological traits.
    pub special_traits: Vec<SpeciesTrait>,
}
