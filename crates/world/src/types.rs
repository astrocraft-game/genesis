#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct StarContext {
    pub age_gyr: f32,
    pub habitable_zone_inner_au: f64,
    pub habitable_zone_outer_au: f64,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct OrbitContext {
    pub orbital_distance_au: f64,
    pub eccentricity: f32,
    pub axial_tilt_deg: f32,
    pub rotation_period_days: f32,
    pub day_length_days: f32,
    pub tidally_locked: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct PlanetSimulationInput {
    pub body_id: u32,
    pub body_mass_earth: f64,
    pub body_radius_earth: f64,
    pub density_g_cm3: f32,
    pub gravity_g: f32,
    pub blackbody_temp_k: u32,
    pub tidal_heating: u32,
    pub moon_count: u32,
    pub has_rings: bool,
    pub in_habitable_zone: bool,
    pub star: StarContext,
    pub orbit: OrbitContext,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct PlanetGenerationProfile {
    pub body_type: TelluricBodyComposition,
    pub world_type: CelestialBodyWorldType,
    pub magnetic_field: MagneticFieldStrength,
    pub life_level: LifeLevel,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct PlanetInterior {
    pub body_type: TelluricBodyComposition,
    pub world_type: CelestialBodyWorldType,
    pub magnetic_field: MagneticFieldStrength,
    pub atmospheric_pressure: f32,
    pub atmospheric_composition: Vec<(f32, ChemicalComponent)>,
    pub hydrosphere: f32,
    pub ice_over_water: f32,
    pub land_area_percentage: f32,
    pub ice_over_land: f32,
    pub volcanism: f32,
    pub tectonic_activity: f32,
    pub humidity: f32,
    pub temperature_category: WorldTemperatureCategory,
    pub climate: WorldClimateType,
    pub life_level: LifeLevel,
    pub surface_map: Option<PlanetSurfaceMap>,
    pub atmospheric_circulation: Option<AtmosphericCirculation>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum TelluricBodyComposition {
    Metallic,
    #[default]
    Rocky,
    Icy,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CelestialBodyWorldType {
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum MagneticFieldStrength {
    #[default]
    None,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
    Extreme,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum WorldTemperatureCategory {
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum WorldClimateType {
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ChemicalComponent {
    Hydrogen,
    Helium,
    #[default]
    Nitrogen,
    Oxygen,
    CarbonDioxide,
    CarbonMonoxide,
    Methane,
    Ammonia,
    Water,
    Argon,
    SulfurDioxide,
    HydrogenSulfide,
    Chlorine,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
#[non_exhaustive]
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

/// Köppen-Geiger climate classification. A simplified subset using the
/// yearly summer/winter temperature means and annual precipitation that
/// the grid computes; fine seasonal subtypes (e.g. Cfa vs Cwa) are not
/// distinguished since we lack per-month rainfall.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[non_exhaustive]
pub enum KoppenClass {
    #[default]
    Ocean,
    // Group A — Tropical (coldest month ≥ 18 °C)
    /// Tropical rainforest
    Af,
    /// Tropical monsoon
    Am,
    /// Tropical savanna (wet-dry)
    Aw,
    // Group B — Arid
    /// Hot desert
    BWh,
    /// Cold desert
    BWk,
    /// Hot steppe
    BSh,
    /// Cold steppe
    BSk,
    // Group C — Temperate (coldest month 0-18 °C)
    /// Humid subtropical
    Cfa,
    /// Oceanic / marine west coast
    Cfb,
    /// Subpolar oceanic
    Cfc,
    // Group D — Continental (coldest month < 0 °C, warmest > 10 °C)
    /// Hot-summer humid continental
    Dfa,
    /// Warm-summer humid continental
    Dfb,
    /// Subarctic
    Dfc,
    /// Extremely cold subarctic
    Dfd,
    // Group E — Polar (all months < 10 °C)
    /// Tundra
    ET,
    /// Ice cap
    EF,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CraterDensity {
    Pristine,
    Light,
    #[default]
    Moderate,
    Heavy,
    Saturated,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct AtmosphericCirculation {
    pub cells_per_hemisphere: u8,
    pub jet_stream_count: u8,
    pub wind_intensity: WindIntensity,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum WindIntensity {
    Calm,
    #[default]
    Light,
    Moderate,
    Strong,
    Extreme,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct PlanetSurfaceMap {
    pub continent_count: u8,
    pub biome_distribution: Vec<(BiomeType, f32)>,
    pub highest_elevation_km: f32,
    pub deepest_ocean_km: f32,
    pub tectonic_plate_count: u8,
    pub temperature_range_k: f32,
    pub seasonal_frost: bool,
    pub crater_density: CraterDensity,
    pub largest_crater_km: f32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct PlanetaryDetail {
    pub atmospheric_layers: Option<AtmosphericLayers>,
    pub atmospheric_escape: Option<AtmosphericEscape>,
    pub breathability: AtmosphereBreathability,
    pub toxicity: AtmosphereToxicity,
    pub cloud_decks: Vec<CloudDeck>,
    pub greenhouse: Option<GreenhouseEffect>,
    pub climate_regulation: Option<ClimateRegulation>,
    pub tidally_locked_climate: Option<TidallyLockedClimate>,
    pub photochemistry: Option<Photochemistry>,
    pub sky: Option<SkyAppearance>,
    pub wind: Option<WindProfile>,
    pub hydrography: Option<Hydrography>,
    pub lakes: Option<LakeDistribution>,
    pub glaciation: Option<GlaciationState>,
    pub impact_history: Option<ImpactHistory>,
    pub subsurface_ocean: Option<SubsurfaceOcean>,
    pub ocean_chemistry: Option<OceanChemistry>,
    pub volcanic_profile: Option<VolcanicProfile>,
    pub mineral_diversity: Option<MineralDiversity>,
    pub surface_material: Option<SurfaceMaterial>,
    pub radiation: Option<RadiationEnvironment>,
    pub seismic: Option<SeismicProfile>,
    pub dust_storms: Option<DustStormProfile>,
    pub lightning: Option<LightningProfile>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct AtmosphericLayers {
    pub scale_height_km: f32,
    pub tropopause_km: f32,
    pub has_stratosphere: bool,
    pub exobase_km: f32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct AtmosphericEscape {
    pub dominant_driver: AtmosphericEscapeDriver,
    pub loss_intensity: AtmosphericLossIntensity,
    pub xuv_flux_relative: f32,
    pub escape_velocity_km_s: f32,
    pub retention_score: f32,
    pub atmosphere_retained: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum AtmosphericEscapeDriver {
    #[default]
    Minimal,
    JeansEscape,
    HydrodynamicEscape,
    StellarWindSputtering,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum AtmosphericLossIntensity {
    #[default]
    Negligible,
    Low,
    Moderate,
    High,
    Extreme,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum AtmosphereBreathability {
    #[default]
    Vacuum,
    Trace,
    VeryThin,
    ThinBreathable,
    Standard,
    Dense,
    VeryDense,
    Superdense,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum AtmosphereToxicity {
    #[default]
    Benign,
    Marginal,
    Filterable,
    Suffocating,
    MildlyToxic,
    HighlyToxic,
    LethallyToxic,
    Corrosive,
    Insidious,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CloudComposition {
    #[default]
    Water,
    WaterIce,
    SulfuricAcid,
    Ammonia,
    AmmoniumHydrosulfide,
    Methane,
    OrganicHaze,
    SiliconDust,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct CloudDeck {
    pub composition: CloudComposition,
    pub base_altitude_km: f32,
    pub top_altitude_km: f32,
    pub optical_depth: f32,
    pub coverage_fraction: f32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct GreenhouseEffect {
    pub equilibrium_temp_k: f32,
    pub surface_temp_k: f32,
    pub greenhouse_delta_k: f32,
    pub bond_albedo: f32,
    pub is_runaway: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct ClimateRegulation {
    pub regime: ClimateRegime,
    pub volcanic_outgassing_index: f32,
    pub weathering_drawdown_index: f32,
    pub regulation_strength: f32,
    pub estimated_feedback_k: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ClimateRegime {
    #[default]
    Unbuffered,
    WeatheringBalanced,
    CarbonateSilicate,
    TidallyModerated,
    SnowballLocked,
    RunawayGreenhouse,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct TidallyLockedClimate {
    pub regime: TidallyLockedClimateRegime,
    pub heat_redistribution_efficiency: f32,
    pub day_night_temperature_delta_k: f32,
    pub terminator_habitability: TerminatorHabitability,
    pub nightside_cold_traps: bool,
    pub substellar_cloud_fraction: f32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Photochemistry {
    pub haze_regime: HazeRegime,
    /// Dominant activity level across the whole atmosphere (legacy).
    pub activity: PhotochemicalActivity,
    /// UV-driven photolysis in the upper atmosphere. Scales with stellar
    /// XUV flux and stellar age.
    pub stratospheric_activity: PhotochemicalActivity,
    /// Thermochemical reactions in the lower atmosphere. Driven by surface
    /// temperature, pressure, and oxidizer/reducer content.
    pub tropospheric_activity: PhotochemicalActivity,
    pub ozone_column_relative: f32,
    /// Ozone-equivalent shielding: effective O3 column scaled by whether the
    /// atmosphere can actually support long-lived ozone (O2-rich + cold
    /// stratosphere). For non-O2 atmospheres this is zero.
    pub ozone_equivalent_shielding: f32,
    pub uv_shielding_fraction: f32,
    pub smog_level: SmogLevel,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum HazeRegime {
    #[default]
    Clear,
    OzoneShielded,
    OrganicHaze,
    SulfurHaze,
    DustLoaded,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum PhotochemicalActivity {
    #[default]
    Quiescent,
    Active,
    Intense,
    Extreme,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SmogLevel {
    #[default]
    None,
    Light,
    Moderate,
    Severe,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum TidallyLockedClimateRegime {
    #[default]
    AtmosphereCollapsed,
    NightsideColdTrap,
    TerminatorBelt,
    EyeballWorld,
    UniformSuperrotating,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum TerminatorHabitability {
    #[default]
    None,
    Marginal,
    Local,
    Broad,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SkyColor {
    #[default]
    Black,
    DeepBlue,
    Blue,
    PaleBlue,
    White,
    Yellow,
    Amber,
    Orange,
    Butterscotch,
    Red,
    Green,
    Pink,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct SkyAppearance {
    pub daytime_color: SkyColor,
    pub sunset_color: SkyColor,
    pub daytime_stars_visible: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct WindProfile {
    pub mean_surface_wind_ms: f32,
    pub max_wind_ms: f32,
    pub superrotation: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DeltaType {
    #[default]
    None,
    Arcuate,
    BirdFoot,
    Cuspate,
    Estuarine,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Hydrography {
    pub major_river_count: u32,
    pub longest_river_km: f32,
    pub mean_precipitation_mm: f32,
    /// Zonal precipitation bands: [equatorial, mid-latitude, polar] in mm/yr.
    /// Derived from a simple Hadley-cell redistribution of the global mean
    /// precipitation, scaled by axial tilt. The sum of these three values
    /// (weighted by zonal area) approximates `mean_precipitation_mm`.
    pub zonal_precipitation_mm: [f32; 3],
    /// Number of atmospheric circulation cells per hemisphere (Hadley +
    /// Ferrel + Polar). Drops to 1 on low-tilt, high-insolation worlds and
    /// becomes chaotic above ~54° tilt.
    pub hadley_cells_per_hemisphere: u8,
    pub dominant_delta_type: DeltaType,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum LakeFormationType {
    #[default]
    None,
    Glacial,
    Tectonic,
    Volcanic,
    Impact,
    Fluvial,
    Endorheic,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum LiquidType {
    #[default]
    Water,
    Brine,
    MethaneEthane,
    Ammonia,
    Magma,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct LakeDistribution {
    pub lake_count: u32,
    pub dominant_type: LakeFormationType,
    pub largest_lake_km2: f32,
    pub liquid_type: LiquidType,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum IceCapLocation {
    #[default]
    None,
    Polar,
    Equatorial,
    DarkSide,
    Global,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct GlaciationState {
    pub ice_coverage_fraction: f32,
    pub in_glacial_period: bool,
    pub snowball_state: bool,
    pub ice_cap_location: IceCapLocation,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct ImpactHistory {
    pub surface_age_class: SurfaceAgeClass,
    pub resurfacing_driver: ResurfacingDriver,
    pub major_basin_count: u8,
    pub largest_basin_class: ImpactBasinClass,
    pub ejecta_blanket_fraction: f32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct SubsurfaceOcean {
    pub present: bool,
    pub ice_shell_thickness_km: f32,
    pub ocean_depth_km: f32,
    pub plume_activity: PlumeActivity,
    pub transport_efficiency: CryovolcanicTransport,
    pub habitability: EnclosedOceanHabitability,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum PlumeActivity {
    #[default]
    None,
    Occasional,
    Persistent,
    Extreme,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CryovolcanicTransport {
    #[default]
    Trapped,
    Fractured,
    EpisodicExchange,
    EfficientExchange,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum EnclosedOceanHabitability {
    #[default]
    Sterile,
    Marginal,
    Chemotrophic,
    HighPotential,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SurfaceAgeClass {
    #[default]
    VeryYoung,
    Young,
    Mature,
    Ancient,
    Primordial,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ResurfacingDriver {
    #[default]
    None,
    Tectonic,
    Volcanic,
    Cryovolcanic,
    Glacial,
    Aeolian,
    ImpactOnly,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ImpactBasinClass {
    #[default]
    None,
    Crater,
    Basin,
    MegaBasin,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct OceanChemistry {
    pub liquid_type: LiquidType,
    pub salinity_g_per_kg: f32,
    pub ph: f32,
    pub alkalinity_meq_l: f32,
    pub anoxic: bool,
    pub redox_state: OceanRedoxState,
    pub iron_content: OceanIronContent,
    pub nutrient_richness: NutrientRichness,
    pub stratification: OceanStratification,
    pub dissolved_volatile_load: f32,
    pub hydrothermal_vents: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum OceanRedoxState {
    #[default]
    Oxic,
    Dysoxic,
    Reducing,
    Euxinic,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum OceanIronContent {
    #[default]
    Negligible,
    Low,
    Moderate,
    High,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum NutrientRichness {
    #[default]
    Starved,
    Limited,
    Moderate,
    Fertile,
    BloomProne,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum OceanStratification {
    #[default]
    WellMixed,
    Seasonal,
    Layered,
    StronglyStratified,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ResourceAbundance {
    Absent,
    Trace,
    Poor,
    #[default]
    Average,
    Rich,
    Motherlode,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum Mineral {
    #[default]
    Silicates,
    Carbonates,
    Sulfides,
    Organics,
    Ices,
    Metals,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct MineralDeposit {
    pub mineral: Mineral,
    pub abundance: ResourceAbundance,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum MineralEvolutionStage {
    #[default]
    Primitive,
    Aqueous,
    Igneous,
    Metamorphic,
    Biogenic,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct MineralDiversity {
    pub stage: MineralEvolutionStage,
    pub distinct_mineral_count: u16,
    pub deposits: Vec<MineralDeposit>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum VolcanoType {
    #[default]
    Shield,
    Stratovolcano,
    Caldera,
    FloodBasalt,
    Cryovolcano,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct VolcanicProfile {
    pub volcano_type: VolcanoType,
    pub active_volcano_count: u32,
    pub tallest_volcano_km: f32,
    pub has_supervolcano: bool,
    pub flood_basalt_province: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SeismicitySource {
    #[default]
    None,
    TectonicModerate,
    TectonicExtreme,
    Tidal,
    Volcanic,
    Residual,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct SeismicProfile {
    pub mean_quake_magnitude: f32,
    pub max_quake_magnitude: f32,
    pub dominant_source: SeismicitySource,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SurfaceMaterialType {
    #[default]
    BarrenRock,
    SandDunes,
    IceSheet,
    Soil,
    OrganicSediment,
    SulfurDeposits,
    EvaporiteDeposits,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct SurfaceMaterial {
    pub primary: SurfaceMaterialType,
    pub secondary: Vec<SurfaceMaterialType>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct RadiationEnvironment {
    pub surface_dose_relative: f32,
    pub auroral_activity: f32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct DustStormProfile {
    pub global_storms_possible: bool,
    pub global_storm_interval_years: f32,
    pub peak_wind_ms: f32,
    pub dust_devils_active: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum LightningMechanism {
    #[default]
    WaterCloud,
    AcidCloud,
    VolcanicPlume,
    DustTriboelectric,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct LightningProfile {
    pub present: bool,
    pub flash_rate_relative: f32,
    pub mechanism: LightningMechanism,
}
