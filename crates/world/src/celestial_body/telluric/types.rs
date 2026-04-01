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

// =============================================================================
// Planetary Detail Types (V3)
// =============================================================================

/// Comprehensive planetary detail computed from base world parameters.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PlanetaryDetail {
    pub atmospheric_layers: Option<AtmosphericLayers>,
    pub breathability: AtmosphereBreathability,
    pub toxicity: AtmosphereToxicity,
    pub cloud_decks: Vec<CloudDeck>,
    pub greenhouse: Option<GreenhouseEffect>,
    pub sky: Option<SkyAppearance>,
    pub wind: Option<WindProfile>,
    pub hydrography: Option<Hydrography>,
    pub lakes: Option<LakeDistribution>,
    pub glaciation: Option<GlaciationState>,
    pub ocean_chemistry: Option<OceanChemistry>,
    pub volcanic_profile: Option<VolcanicProfile>,
    pub mineral_diversity: Option<MineralDiversity>,
    pub surface_material: Option<SurfaceMaterial>,
    pub radiation: Option<RadiationEnvironment>,
    pub seismic: Option<SeismicProfile>,
    pub dust_storms: Option<DustStormProfile>,
    pub lightning: Option<LightningProfile>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct AtmosphericLayers {
    pub scale_height_km: f32,
    pub tropopause_km: f32,
    pub has_stratosphere: bool,
    pub exobase_km: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum AtmosphereBreathability {
    #[default] Vacuum, Trace, VeryThin, ThinBreathable, Standard, Dense, VeryDense, Superdense,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum AtmosphereToxicity {
    #[default] Benign, Marginal, Filterable, Suffocating, MildlyToxic, HighlyToxic, LethallyToxic, Corrosive, Insidious,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum CloudComposition {
    #[default] Water, WaterIce, SulfuricAcid, Ammonia, AmmoniumHydrosulfide, Methane, OrganicHaze, SiliconDust,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct CloudDeck {
    pub composition: CloudComposition,
    pub base_altitude_km: f32,
    pub top_altitude_km: f32,
    pub optical_depth: f32,
    pub coverage_fraction: f32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GreenhouseEffect {
    pub equilibrium_temp_k: f32,
    pub surface_temp_k: f32,
    pub greenhouse_delta_k: f32,
    pub bond_albedo: f32,
    pub is_runaway: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum SkyColor {
    #[default] Black, DeepBlue, Blue, PaleBlue, White, Yellow, Amber, Orange, Butterscotch, Red, Green, Pink,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct SkyAppearance {
    pub daytime_color: SkyColor,
    pub sunset_color: SkyColor,
    pub daytime_stars_visible: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct WindProfile {
    pub mean_surface_wind_ms: f32,
    pub max_wind_ms: f32,
    pub superrotation: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum DeltaType {
    #[default] None, Arcuate, BirdFoot, Cuspate, Estuarine,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Hydrography {
    pub major_river_count: u32,
    pub longest_river_km: f32,
    pub mean_precipitation_mm: f32,
    pub dominant_delta_type: DeltaType,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum LakeFormationType {
    #[default] None, Glacial, Tectonic, Volcanic, Impact, Fluvial, Endorheic,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum LiquidType {
    #[default] Water, Brine, MethaneEthane, Ammonia, Magma,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct LakeDistribution {
    pub lake_count: u32,
    pub dominant_type: LakeFormationType,
    pub largest_lake_km2: f32,
    pub liquid_type: LiquidType,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum IceCapLocation {
    #[default] None, Polar, Equatorial, DarkSide, Global,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GlaciationState {
    pub ice_coverage_fraction: f32,
    pub in_glacial_period: bool,
    pub snowball_state: bool,
    pub ice_cap_location: IceCapLocation,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum OceanIronContent {
    #[default] Negligible, Low, Moderate, High,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct OceanChemistry {
    pub liquid_type: LiquidType,
    pub salinity_g_per_kg: f32,
    pub ph: f32,
    pub anoxic: bool,
    pub iron_content: OceanIronContent,
    pub hydrothermal_vents: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum VolcanoType {
    #[default] Shield, Stratovolcano, Caldera, Fissure, FloodBasalt, Cryovolcano,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct VolcanicProfile {
    pub active_count: u32,
    pub dominant_type: VolcanoType,
    pub flood_basalt_history: bool,
    pub tallest_volcano_km: f32,
    pub supervolcano_present: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum MineralEvolutionStage {
    #[default] Primordial, Differentiated, Hydrated, TectonicallyActive, Oxidized, Biogenic,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct MineralDiversity {
    pub mineral_count: u32,
    pub evolution_stage: MineralEvolutionStage,
    pub deposits: Vec<MineralDeposit>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct MineralDeposit {
    pub mineral: Mineral,
    pub abundance: ResourceAbundance,
}

/// 90 real mineral species from IMA, Hazen et al. 2008, USGS, and planetary missions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, SmartDefault, Serialize, Deserialize)]
pub enum Mineral {
    // === Native Elements (8) ===
    NativeIron,
    NativeCopper,
    Gold,
    Silver,
    Platinum,
    #[default]
    NativeSulfur,
    Diamond,
    Graphite,
    // === Carbides & Nitrides (3) - presolar ===
    Moissanite,   // SiC
    Cohenite,     // Fe3C
    Osbornite,    // TiN
    // === Sulfides (12) ===
    Troilite,     // FeS
    Pyrite,       // FeS2
    Chalcopyrite, // CuFeS2
    Galena,       // PbS
    Sphalerite,   // ZnS
    Cinnabar,     // HgS
    Molybdenite,  // MoS2
    Pentlandite,  // (Ni,Fe)9S8
    Pyrrhotite,   // Fe(1-x)S
    Chalcocite,   // Cu2S
    Stibnite,     // Sb2S3
    Cobaltite,    // CoAsS
    // === Oxides & Hydroxides (12) ===
    Hematite,     // Fe2O3
    Magnetite,    // Fe3O4
    Corundum,     // Al2O3 (ruby/sapphire)
    Rutile,       // TiO2
    Cassiterite,  // SnO2
    Chromite,     // FeCr2O4
    Ilmenite,     // FeTiO3
    Uraninite,    // UO2
    Spinel,       // MgAl2O4
    Goethite,     // FeOOH
    Pyrolusite,   // MnO2
    Cuprite,      // Cu2O
    // === Silicates - Framework (6) ===
    Quartz,       // SiO2
    Plagioclase,  // (Na,Ca)(Al,Si)4O8
    Orthoclase,   // KAlSi3O8
    Nepheline,    // (Na,K)AlSiO4
    Sodalite,     // Na8(Al6Si6O24)Cl2
    Analcime,     // NaAlSi2O6·H2O
    // === Silicates - Chain/Sheet/Island (16) ===
    Olivine,      // (Mg,Fe)2SiO4
    Augite,       // (Ca,Mg,Fe)2Si2O6
    Enstatite,    // MgSiO3
    Hornblende,   // Ca2(Mg,Fe)4Al(Si7Al)O22(OH)2
    Muscovite,    // KAl2(AlSi3O10)(OH)2
    Biotite,      // K(Mg,Fe)3(AlSi3O10)(OH)2
    Garnet,       // (Ca,Mg,Fe,Mn)3(Al,Fe,Cr)2(SiO4)3
    Tourmaline,   // complex borosilicate
    Zircon,       // ZrSiO4
    Beryl,        // Be3Al2Si6O18 (emerald)
    Topaz,        // Al2SiO4(F,OH)2
    Kyanite,      // Al2SiO5
    Talc,         // Mg3Si4O10(OH)2
    Serpentine,   // Mg3Si2O5(OH)4
    Kaolinite,    // Al2Si2O5(OH)4
    Montmorillonite, // (Na,Ca)0.33(Al,Mg)2Si4O10(OH)2·nH2O
    // === Carbonates (6) ===
    Calcite,      // CaCO3
    Aragonite,    // CaCO3 (polymorph)
    Dolomite,     // CaMg(CO3)2
    Magnesite,    // MgCO3
    Siderite,     // FeCO3
    Malachite,    // Cu2CO3(OH)2
    // === Sulfates (5) ===
    Gypsum,       // CaSO4·2H2O
    Barite,       // BaSO4
    Anhydrite,    // CaSO4
    Jarosite,     // KFe3(SO4)2(OH)6
    Epsomite,     // MgSO4·7H2O
    // === Phosphates (3) ===
    Apatite,      // Ca5(PO4)3(F,Cl,OH)
    Monazite,     // (Ce,La)PO4
    Turquoise,    // CuAl6(PO4)4(OH)8·4H2O
    // === Halides (3) ===
    Halite,       // NaCl
    Fluorite,     // CaF2
    Sylvite,      // KCl
    // === Volatile Ices (5) ===
    WaterIce,
    CarbonDioxideIce,
    MethaneIce,
    AmmoniaIce,
    NitrogenIce,
    // === Hydrated Salts (4) ===
    Mirabilite,   // Na2SO4·10H2O (Europa)
    Hydrohalite,  // NaCl·2H2O (Europa)
    Kieserite,    // MgSO4·H2O (Mars)
    Hexahydrite,  // MgSO4·6H2O (Mars)
    // === Organic/Biogenic (4) ===
    BiogenicCalcite, // shells, coral
    HydrocarbonDeposit, // oil, gas, coal
    Tholin,       // organic polymer (Titan)
    Opal,         // SiO2·nH2O (biogenic silica)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum SurfaceMaterialType {
    #[default] BarrenRock, Regolith, IronOxideFines, Soil, SulfurDeposits, IceCrust, OrganicSediment, SandDunes, EvaporiteDeposits,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct SurfaceMaterial {
    pub primary_type: SurfaceMaterialType,
    pub depth_m: f32,
    pub perchlorates: bool,
    pub oxidized: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum RadiationHazard {
    #[default] Negligible, Low, Moderate, High, Extreme,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct RadiationEnvironment {
    pub surface_dose_msv_yr: f32,
    pub uv_index_peak: f32,
    pub radiation_hazard: RadiationHazard,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum SeismicitySource {
    #[default] None, Residual, TidalOnly, TectonicModerate, TectonicExtreme, TidalExtreme,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct SeismicProfile {
    pub max_magnitude: f32,
    pub quakes_per_year_m4: u32,
    pub seismicity_source: SeismicitySource,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct DustStormProfile {
    pub global_storms_possible: bool,
    pub global_storm_interval_years: f32,
    pub peak_wind_ms: f32,
    pub dust_devils_active: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize)]
pub enum LightningMechanism {
    #[default] None, WaterCloud, VolcanicPlume, DustTriboelectric, AcidCloud,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct LightningProfile {
    pub present: bool,
    pub flash_rate_relative: f32,
    pub mechanism: LightningMechanism,
}
