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

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum POIType {
    // Geological
    MassiveCanyon,
    SuperVolcano,
    ImpactCrater,
    CrystalFormation,
    LavaLake,
    GeyserField,
    CaveSystem,
    // Hydrological
    SubterraneanOcean,
    ThermalVents,
    IceGeysers,
    // Atmospheric
    PermanentStorm,
    AuroraField,
    // Biological
    FossilSite,
    ExtremeLifeColony,
    // Anomalous
    GravityAnomaly,
    MagneticAnomaly,
    RadioactiveZone,
    #[default]
    UnusualMineral,
}

impl Display for POIType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            POIType::MassiveCanyon => "Massive Canyon",
            POIType::SuperVolcano => "Super Volcano",
            POIType::ImpactCrater => "Impact Crater",
            POIType::CrystalFormation => "Crystal Formation",
            POIType::LavaLake => "Lava Lake",
            POIType::GeyserField => "Geyser Field",
            POIType::CaveSystem => "Cave System",
            POIType::SubterraneanOcean => "Subterranean Ocean",
            POIType::ThermalVents => "Thermal Vents",
            POIType::IceGeysers => "Ice Geysers",
            POIType::PermanentStorm => "Permanent Storm",
            POIType::AuroraField => "Aurora Field",
            POIType::FossilSite => "Fossil Site",
            POIType::ExtremeLifeColony => "Extreme Life Colony",
            POIType::GravityAnomaly => "Gravity Anomaly",
            POIType::MagneticAnomaly => "Magnetic Anomaly",
            POIType::RadioactiveZone => "Radioactive Zone",
            POIType::UnusualMineral => "Unusual Mineral Deposit",
        })
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum POISignificance {
    Minor,
    #[default]
    Notable,
    Major,
    Unique,
}

impl Display for POISignificance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            POISignificance::Minor => "Minor",
            POISignificance::Notable => "Notable",
            POISignificance::Major => "Major",
            POISignificance::Unique => "Unique",
        })
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PointOfInterest {
    pub poi_type: POIType,
    pub significance: POISignificance,
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum BiomeType {
    Tundra,
    Taiga,
    TemperateForest,
    TropicalForest,
    #[default]
    Grassland,
    Desert,
    Savanna,
    Wetland,
    Alpine,
    Volcanic,
    IceCap,
    Ocean,
    Barren,
}

impl Display for BiomeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            BiomeType::Tundra => "Tundra",
            BiomeType::Taiga => "Taiga",
            BiomeType::TemperateForest => "Temperate Forest",
            BiomeType::TropicalForest => "Tropical Forest",
            BiomeType::Grassland => "Grassland",
            BiomeType::Desert => "Desert",
            BiomeType::Savanna => "Savanna",
            BiomeType::Wetland => "Wetland",
            BiomeType::Alpine => "Alpine",
            BiomeType::Volcanic => "Volcanic",
            BiomeType::IceCap => "Ice Cap",
            BiomeType::Ocean => "Ocean",
            BiomeType::Barren => "Barren",
        })
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct AtmosphericCirculation {
    /// Number of Hadley-like cells per hemisphere (1 = Venus, 3 = Earth, 5+ = Jupiter).
    pub cells_per_hemisphere: u8,
    /// Number of jet streams (= 2 * cells - 1).
    pub jet_stream_count: u8,
    /// Dominant wind speed class.
    pub wind_intensity: WindIntensity,
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum WindIntensity {
    Calm,
    #[default]
    Light,
    Moderate,
    Strong,
    Extreme,
}

impl Display for WindIntensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            WindIntensity::Calm => "Calm",
            WindIntensity::Light => "Light",
            WindIntensity::Moderate => "Moderate",
            WindIntensity::Strong => "Strong",
            WindIntensity::Extreme => "Extreme",
        })
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum CraterDensity {
    Pristine,
    Light,
    #[default]
    Moderate,
    Heavy,
    Saturated,
}

impl Display for CraterDensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            CraterDensity::Pristine => "Pristine",
            CraterDensity::Light => "Light",
            CraterDensity::Moderate => "Moderate",
            CraterDensity::Heavy => "Heavy",
            CraterDensity::Saturated => "Saturated",
        })
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PlanetSurfaceMap {
    /// Number of distinct continents.
    pub continent_count: u8,
    /// Biome distribution: (biome_type, fraction of total surface 0.0-1.0).
    pub biome_distribution: Vec<(BiomeType, f32)>,
    /// Highest point above sea level in km.
    pub highest_elevation_km: f32,
    /// Deepest ocean point in km.
    pub deepest_ocean_km: f32,
    /// Number of tectonic plates.
    pub tectonic_plate_count: u8,
    /// Seasonal temperature swing in Kelvin (from axial tilt + eccentricity).
    pub temperature_range_k: f32,
    /// If a major atmospheric gas seasonally condenses as frost.
    pub seasonal_frost: bool,
    /// Crater density class based on surface age and resurfacing.
    pub crater_density: CraterDensity,
    /// Diameter of the largest impact crater in km.
    pub largest_crater_km: f32,
}
