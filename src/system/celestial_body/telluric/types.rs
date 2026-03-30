use crate::internal::*;
use crate::prelude::*;
use std::fmt;

/// A list of settings used to configure the the Telluric Bodies (like rocky planets) generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct TelluricBodySettings {}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricBodyComposition {
    Metallic,
    #[default]
    Rocky,
    Icy,
}

impl Display for TelluricBodyComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TelluricBodyComposition::Metallic => "Metallic",
                TelluricBodyComposition::Rocky => "Rocky",
                TelluricBodyComposition::Icy => "Icy",
            }
        )
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum ResourceType {
    #[default]
    CommonMetals,
    PreciousMetals,
    Radioactives,
    IndustrialMinerals,
    Volatiles,
    OrganicCompounds,
    ExoticMaterials,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ResourceType::CommonMetals => "Common Metals",
            ResourceType::PreciousMetals => "Precious Metals",
            ResourceType::Radioactives => "Radioactives",
            ResourceType::IndustrialMinerals => "Industrial Minerals",
            ResourceType::Volatiles => "Volatiles",
            ResourceType::OrganicCompounds => "Organic Compounds",
            ResourceType::ExoticMaterials => "Exotic Materials",
        })
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum ResourceAbundance {
    Absent,
    Trace,
    Poor,
    #[default]
    Average,
    Rich,
    Motherlode,
}

impl Display for ResourceAbundance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ResourceAbundance::Absent => "Absent",
            ResourceAbundance::Trace => "Trace",
            ResourceAbundance::Poor => "Poor",
            ResourceAbundance::Average => "Average",
            ResourceAbundance::Rich => "Rich",
            ResourceAbundance::Motherlode => "Motherlode",
        })
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum ResourceAccessibility {
    Inaccessible,
    Deep,
    #[default]
    Subsurface,
    Surface,
    Atmospheric,
}

impl Display for ResourceAccessibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ResourceAccessibility::Inaccessible => "Inaccessible",
            ResourceAccessibility::Deep => "Deep",
            ResourceAccessibility::Subsurface => "Subsurface",
            ResourceAccessibility::Surface => "Surface",
            ResourceAccessibility::Atmospheric => "Atmospheric",
        })
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PlanetaryResource {
    pub resource_type: ResourceType,
    pub abundance: ResourceAbundance,
    pub accessibility: ResourceAccessibility,
}
