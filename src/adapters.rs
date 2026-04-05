use cosmos::prelude::ExternalBodyFacts;
use life::{Biome, Climate, Habitat, SpeciesGenerationInput, Temperature};
use world::grid::SurfaceGrid;
use world::prelude::{
    BiomeType, CelestialBodyWorldType, LifeLevel, MagneticFieldStrength, OrbitContext,
    PlanetGenerationProfile, PlanetInterior, PlanetSimulationInput, PlanetaryDetail, StarContext,
    TelluricBodyComposition, WorldClimateType, WorldTemperatureCategory,
};

pub fn external_facts_to_world_input(facts: &ExternalBodyFacts) -> PlanetSimulationInput {
    PlanetSimulationInput {
        body_id: facts.body_id,
        body_mass_earth: facts.mass,
        body_radius_earth: facts.radius,
        density_g_cm3: facts.density,
        gravity_g: facts.gravity,
        blackbody_temp_k: facts.blackbody_temperature,
        tidal_heating: facts.tidal_heating,
        moon_count: facts.moon_count,
        has_rings: facts.has_rings,
        in_habitable_zone: false,
        star: StarContext {
            age_gyr: facts.star_age,
            ..Default::default()
        },
        orbit: OrbitContext {
            orbital_distance_au: facts.distance_from_star,
            eccentricity: facts.eccentricity,
            axial_tilt_deg: facts.axial_tilt,
            rotation_period_days: facts.rotation_days,
            day_length_days: facts.rotation_days,
            tidally_locked: facts.is_tidally_locked,
        },
    }
}

pub fn telluric_details_to_world_profile(
    details: &cosmos::prelude::TelluricBodyDetails,
    life_level: LifeLevel,
) -> PlanetGenerationProfile {
    PlanetGenerationProfile {
        body_type: map_body_type(details.body_type),
        world_type: map_world_type(details.world_type),
        magnetic_field: map_magnetic_field(details.magnetic_field),
        life_level,
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct GeneratedTelluricWorld {
    pub input: PlanetSimulationInput,
    pub profile: PlanetGenerationProfile,
    pub interior: PlanetInterior,
    pub detail: PlanetaryDetail,
}

pub fn generate_world_from_cosmos_body(
    body: &cosmos::prelude::CelestialBody,
    star_age_gyr: f32,
    moon_count: u32,
    has_rings: bool,
    life_level: LifeLevel,
) -> Option<GeneratedTelluricWorld> {
    let cosmos::prelude::CelestialBodyDetails::Telluric(details) = &body.details else {
        return None;
    };

    let input =
        external_facts_to_world_input(&body.external_facts(star_age_gyr, moon_count, has_rings));
    let profile = telluric_details_to_world_profile(details, life_level);
    let (interior, detail) = world::prelude::generate_complete_planet(&input, &profile);

    Some(GeneratedTelluricWorld {
        input,
        profile,
        interior,
        detail,
    })
}

fn map_body_type(value: cosmos::prelude::TelluricBodyComposition) -> TelluricBodyComposition {
    match value {
        cosmos::prelude::TelluricBodyComposition::Metallic => TelluricBodyComposition::Metallic,
        cosmos::prelude::TelluricBodyComposition::Rocky => TelluricBodyComposition::Rocky,
        cosmos::prelude::TelluricBodyComposition::Icy => TelluricBodyComposition::Icy,
    }
}

fn map_world_type(value: cosmos::prelude::CelestialBodyWorldType) -> CelestialBodyWorldType {
    match value {
        cosmos::prelude::CelestialBodyWorldType::ProtoWorld => CelestialBodyWorldType::ProtoWorld,
        cosmos::prelude::CelestialBodyWorldType::Ice => CelestialBodyWorldType::Ice,
        cosmos::prelude::CelestialBodyWorldType::DirtySnowball => {
            CelestialBodyWorldType::DirtySnowball
        }
        cosmos::prelude::CelestialBodyWorldType::GeoActive => CelestialBodyWorldType::GeoActive,
        cosmos::prelude::CelestialBodyWorldType::Rock => CelestialBodyWorldType::Rock,
        cosmos::prelude::CelestialBodyWorldType::Hadean => CelestialBodyWorldType::Hadean,
        cosmos::prelude::CelestialBodyWorldType::Ammonia => CelestialBodyWorldType::Ammonia,
        cosmos::prelude::CelestialBodyWorldType::Ocean => CelestialBodyWorldType::Ocean,
        cosmos::prelude::CelestialBodyWorldType::Terrestrial => CelestialBodyWorldType::Terrestrial,
        cosmos::prelude::CelestialBodyWorldType::Greenhouse => CelestialBodyWorldType::Greenhouse,
        cosmos::prelude::CelestialBodyWorldType::Chthonian => CelestialBodyWorldType::Chthonian,
        cosmos::prelude::CelestialBodyWorldType::VolatilesGiant => {
            CelestialBodyWorldType::VolatilesGiant
        }
        cosmos::prelude::CelestialBodyWorldType::CarbonWorld => CelestialBodyWorldType::CarbonWorld,
        cosmos::prelude::CelestialBodyWorldType::LavaWorld => CelestialBodyWorldType::LavaWorld,
        cosmos::prelude::CelestialBodyWorldType::EyeballWorld => {
            CelestialBodyWorldType::EyeballWorld
        }
        cosmos::prelude::CelestialBodyWorldType::RoguePlanet => CelestialBodyWorldType::RoguePlanet,
        cosmos::prelude::CelestialBodyWorldType::IronWorld => CelestialBodyWorldType::IronWorld,
        cosmos::prelude::CelestialBodyWorldType::MiniNeptune => CelestialBodyWorldType::MiniNeptune,
    }
}

/// Build a life-crate species input from a generated world.
///
/// `scope_key` should uniquely identify this body within the universe seed
/// (e.g. `"sys_{coord}_{system}_str_{star}_bdy{body}"`) so species generation
/// stays deterministic across runs.
pub fn planetary_detail_to_species_input(
    input: &PlanetSimulationInput,
    interior: &PlanetInterior,
    seed: &str,
    scope_key: &str,
) -> SpeciesGenerationInput {
    SpeciesGenerationInput {
        habitat: map_world_type_to_habitat(interior.world_type),
        climate: map_climate(interior.climate),
        temperature: map_temperature(interior.temperature_category),
        gravity: input.gravity_g,
        atmospheric_pressure: interior.atmospheric_pressure,
        hydrosphere: interior.hydrosphere,
        life_level: map_life_level(interior.life_level),
        seed: seed.to_string(),
        scope_key: scope_key.to_string(),
    }
}

fn map_world_type_to_habitat(value: CelestialBodyWorldType) -> Habitat {
    match value {
        CelestialBodyWorldType::ProtoWorld => Habitat::ProtoWorld,
        CelestialBodyWorldType::Ice => Habitat::Ice,
        CelestialBodyWorldType::DirtySnowball => Habitat::DirtySnowball,
        CelestialBodyWorldType::GeoActive => Habitat::GeoActive,
        CelestialBodyWorldType::Rock => Habitat::Rock,
        CelestialBodyWorldType::Hadean => Habitat::Hadean,
        CelestialBodyWorldType::Ammonia => Habitat::Ammonia,
        CelestialBodyWorldType::Ocean => Habitat::Ocean,
        CelestialBodyWorldType::Terrestrial => Habitat::Terrestrial,
        CelestialBodyWorldType::Greenhouse => Habitat::Greenhouse,
        CelestialBodyWorldType::Chthonian => Habitat::Chthonian,
        CelestialBodyWorldType::VolatilesGiant => Habitat::VolatilesGiant,
        CelestialBodyWorldType::CarbonWorld => Habitat::CarbonWorld,
        CelestialBodyWorldType::LavaWorld => Habitat::LavaWorld,
        CelestialBodyWorldType::EyeballWorld => Habitat::EyeballWorld,
        CelestialBodyWorldType::RoguePlanet => Habitat::RoguePlanet,
        CelestialBodyWorldType::IronWorld => Habitat::IronWorld,
        CelestialBodyWorldType::MiniNeptune => Habitat::MiniNeptune,
    }
}

fn map_climate(value: WorldClimateType) -> Climate {
    match value {
        WorldClimateType::Terrestrial => Climate::Terrestrial,
        WorldClimateType::MudBall => Climate::MudBall,
        WorldClimateType::Ocean => Climate::Ocean,
        WorldClimateType::Arctic => Climate::Arctic,
        WorldClimateType::Rainforest => Climate::Rainforest,
        WorldClimateType::Tropical => Climate::Tropical,
        WorldClimateType::Jungle => Climate::Jungle,
        WorldClimateType::Tundra => Climate::Tundra,
        WorldClimateType::Taiga => Climate::Taiga,
        WorldClimateType::Savanna => Climate::Savanna,
        WorldClimateType::Steppe => Climate::Steppe,
        WorldClimateType::Desert => Climate::Desert,
        WorldClimateType::Ribbon => Climate::Ribbon,
        WorldClimateType::Dead => Climate::Dead,
    }
}

fn map_temperature(value: WorldTemperatureCategory) -> Temperature {
    match value {
        WorldTemperatureCategory::Frozen => Temperature::Frozen,
        WorldTemperatureCategory::VeryCold => Temperature::VeryCold,
        WorldTemperatureCategory::Cold => Temperature::Cold,
        WorldTemperatureCategory::Chilly => Temperature::Chilly,
        WorldTemperatureCategory::Cool => Temperature::Cool,
        WorldTemperatureCategory::Temperate => Temperature::Temperate,
        WorldTemperatureCategory::Warm => Temperature::Warm,
        WorldTemperatureCategory::Hot => Temperature::Hot,
        WorldTemperatureCategory::VeryHot => Temperature::VeryHot,
        WorldTemperatureCategory::Scorching => Temperature::Scorching,
        WorldTemperatureCategory::Infernal => Temperature::Infernal,
    }
}

fn map_life_level(value: LifeLevel) -> life::LifeLevel {
    match value {
        LifeLevel::None => life::LifeLevel::None,
        LifeLevel::UniCellular => life::LifeLevel::UniCellular,
        LifeLevel::PluriCellular => life::LifeLevel::PluriCellular,
        LifeLevel::PlantLike => life::LifeLevel::PlantLike,
        LifeLevel::AnimalLike => life::LifeLevel::AnimalLike,
        LifeLevel::Sentient => life::LifeLevel::Sentient,
    }
}

/// Convert a world `SurfaceGrid` into a life `HabitatGrid`.
///
/// Copies only the fields life needs (temperature, humidity, biome,
/// is_ocean, elevation) so life stays independent of world's full grid
/// type. The resulting HabitatGrid is owned by the caller.
pub fn surface_grid_to_habitat_grid(grid: &SurfaceGrid) -> life::HabitatGrid {
    life::HabitatGrid {
        width: grid.width,
        height: grid.height,
        temperature_c: grid.layers.temperature_c.clone(),
        humidity_relative: grid.layers.humidity_relative.clone(),
        biome: grid.layers.biome.iter().copied().map(map_biome).collect(),
        is_ocean: grid.layers.is_ocean.clone(),
        elevation_m: grid.layers.elevation_m.clone(),
    }
}

fn map_biome(b: BiomeType) -> Biome {
    match b {
        BiomeType::Tundra => Biome::Tundra,
        BiomeType::Taiga => Biome::Taiga,
        BiomeType::TemperateForest => Biome::TemperateForest,
        BiomeType::TropicalForest => Biome::TropicalForest,
        BiomeType::Grassland => Biome::Grassland,
        BiomeType::Desert => Biome::Desert,
        BiomeType::Savanna => Biome::Savanna,
        BiomeType::Wetland => Biome::Wetland,
        BiomeType::Alpine => Biome::Alpine,
        BiomeType::Volcanic => Biome::Volcanic,
        BiomeType::IceCap => Biome::IceCap,
        BiomeType::Ocean => Biome::Ocean,
        BiomeType::Barren => Biome::Barren,
        // Future variants in BiomeType default to a safe fallback.
        _ => Biome::Barren,
    }
}

/// Map a single world `Resource` to the nearest matching
/// `crafting::Substance` values (may produce multiple substances for broad
/// resource categories like "IronOre" → Hematite + Magnetite).
pub fn resource_to_substances(resource: world::resources::Resource) -> Vec<crafting::Substance> {
    use crafting::Substance as S;
    use world::resources::Resource as R;
    match resource {
        R::IronOre => vec![S::Hematite, S::Magnetite],
        R::CopperOre => vec![S::Copper],
        R::GoldOre => vec![S::Gold],
        R::TinOre => vec![S::Tin],
        R::AluminumOre => vec![S::Aluminum],
        R::Gemstones => vec![],
        R::Limestone => vec![S::Limestone],
        R::Obsidian => vec![],
        R::Coal => vec![],
        R::Oil => vec![S::CrudeOil],
        R::NaturalGas => vec![S::NaturalGas],
        R::Sulfur => vec![S::Sulfur],
        R::Salt => vec![S::Salt],
        R::Timber => vec![S::WoodLogs],
        R::Herbs => vec![],
        R::Spices => vec![],
        R::Fish => vec![],
        R::Livestock => vec![],
        R::Grain => vec![],
        R::FreshWater => vec![S::Water],
        _ => vec![],
    }
}

/// Collect all crafting substances harvestable across a world's tiles
/// given its resource map. The returned set is the union over all tiles.
pub fn resource_map_to_substance_set(
    map: &world::resources::ResourceMap,
) -> std::collections::HashSet<crafting::Substance> {
    let mut out = std::collections::HashSet::new();
    for tile in &map.per_tile {
        for &resource in tile {
            for sub in resource_to_substances(resource) {
                out.insert(sub);
            }
        }
    }
    out
}

/// Derive a per-tile water-access score from a `SurfaceGrid`. A tile
/// scores 1.0 if it borders the ocean or has significant river discharge,
/// tapering to 0.0 at dry interior tiles.
pub fn water_access_from_grid(grid: &SurfaceGrid) -> Vec<f32> {
    let w = grid.width as usize;
    let h = grid.height as usize;
    let n = w * h;
    let mut out = vec![0.0f32; n];
    for (idx, slot) in out.iter_mut().enumerate() {
        if grid.layers.is_ocean[idx] {
            continue;
        }
        // Start with a river score based on discharge.
        let discharge = grid.layers.river_discharge_m3s[idx];
        let river_score = (discharge / 100.0).min(1.0);
        // Coastal bonus: check 4 neighbours (with longitude wrap) for ocean.
        let r = idx / w;
        let c = idx % w;
        let neighbours = [
            (c, r.saturating_sub(1)),
            (c, (r + 1).min(h - 1)),
            ((c + w - 1) % w, r),
            ((c + 1) % w, r),
        ];
        let is_coastal = neighbours
            .iter()
            .any(|&(nc, nr)| grid.layers.is_ocean[nr * w + nc]);
        let coastal_score = if is_coastal { 1.0 } else { 0.0 };
        *slot = river_score.max(coastal_score);
    }
    out
}

/// Derive a per-tile resource density score (0.0 – 1.0) from a ResourceMap.
/// Tiles with many distinct resources score higher; normalised to the
/// maximum count observed in the map.
pub fn resource_density_from_map(map: &world::resources::ResourceMap) -> Vec<f32> {
    let max_count = map.per_tile.iter().map(|t| t.len()).max().unwrap_or(1) as f32;
    let max_count = max_count.max(1.0);
    map.per_tile
        .iter()
        .map(|t| (t.len() as f32 / max_count).clamp(0.0, 1.0))
        .collect()
}

/// Named geographic features: pairs each detected feature with a
/// Markov-generated name in the chosen style.
#[derive(Clone, Debug, Default)]
pub struct NamedFeatures {
    pub mountain_ranges: Vec<(String, world::features::MountainRange)>,
    pub rivers: Vec<(String, world::features::River)>,
    pub ocean_basins: Vec<(String, world::features::OceanBasin)>,
    pub islands: Vec<(String, world::features::Island)>,
    pub deserts: Vec<(String, world::features::Desert)>,
}

/// Pair each geographic feature with a generated name in the given style.
/// Names are deterministic from `seed` + a per-category scope key.
pub fn name_features(
    features: &world::features::Features,
    style: life::NameStyle,
    seed: &str,
) -> NamedFeatures {
    use seeded_dice_roller::SeededDiceRoller;
    let gen = life::MarkovNameGen::for_style(style);
    let name_one = |rng: &mut SeededDiceRoller, suffix: &str| -> String {
        let stem = gen.generate(rng, 4, 10);
        if suffix.is_empty() {
            stem
        } else {
            format!("{} {}", stem, suffix)
        }
    };

    let mut rng = SeededDiceRoller::new(seed, "features_ranges");
    let mountain_ranges = features
        .mountain_ranges
        .iter()
        .cloned()
        .map(|r| (name_one(&mut rng, "Mountains"), r))
        .collect();

    let mut rng = SeededDiceRoller::new(seed, "features_rivers");
    let rivers = features
        .rivers
        .iter()
        .cloned()
        .map(|r| (name_one(&mut rng, "River"), r))
        .collect();

    let mut rng = SeededDiceRoller::new(seed, "features_basins");
    let ocean_basins = features
        .ocean_basins
        .iter()
        .cloned()
        .map(|b| (name_one(&mut rng, "Ocean"), b))
        .collect();

    let mut rng = SeededDiceRoller::new(seed, "features_islands");
    let islands = features
        .islands
        .iter()
        .cloned()
        .map(|i| (name_one(&mut rng, ""), i))
        .collect();

    let mut rng = SeededDiceRoller::new(seed, "features_deserts");
    let deserts = features
        .deserts
        .iter()
        .cloned()
        .map(|d| (name_one(&mut rng, "Desert"), d))
        .collect();

    NamedFeatures {
        mountain_ranges,
        rivers,
        ocean_basins,
        islands,
        deserts,
    }
}

/// Build crafting planetary conditions from a species' tech level.
///
/// Returns `None` for non-sapient species (tech_level unset). Substance
/// availability defaults to unrestricted — pass `available_substances` into
/// `PlanetaryConditions` manually if you have a concrete inventory for a
/// species' homeworld.
pub fn species_tech_to_conditions(
    species: &life::Species,
) -> Option<crafting::recipes::PlanetaryConditions> {
    let tech = species.tech_level?;
    let (max_temperature_c, max_pressure_atm) = life::expansion::tech_level_capabilities(tech);
    Some(crafting::recipes::PlanetaryConditions {
        max_temperature_c,
        max_pressure_atm,
        available_substances: None,
    })
}

/// Returns the recipes a civilisation at this species' tech level could
/// realistically produce, ignoring substance availability. Pre-sapient
/// species return an empty vector.
pub fn recipes_accessible_to_species(
    species: &life::Species,
) -> Vec<&'static crafting::recipes::types::Recipe> {
    let Some(conditions) = species_tech_to_conditions(species) else {
        return Vec::new();
    };
    crafting::recipes::recipes_in_conditions(&conditions)
}

fn map_magnetic_field(value: cosmos::prelude::MagneticFieldStrength) -> MagneticFieldStrength {
    match value {
        cosmos::prelude::MagneticFieldStrength::None => MagneticFieldStrength::None,
        cosmos::prelude::MagneticFieldStrength::Weak => MagneticFieldStrength::Weak,
        cosmos::prelude::MagneticFieldStrength::Moderate => MagneticFieldStrength::Moderate,
        cosmos::prelude::MagneticFieldStrength::Strong => MagneticFieldStrength::Strong,
        cosmos::prelude::MagneticFieldStrength::VeryStrong => MagneticFieldStrength::VeryStrong,
        cosmos::prelude::MagneticFieldStrength::Extreme => MagneticFieldStrength::Extreme,
    }
}

/// A single trade route between two settlements: the tile path traced
/// by A*, its movement cost, and a complementarity-based value.
#[derive(Clone, Debug)]
pub struct TradeRoute {
    /// Index into the settlement list.
    pub from_settlement: usize,
    pub to_settlement: usize,
    /// Tile path from source to destination, inclusive.
    pub tiles: Vec<usize>,
    /// Total movement cost (sum of per-tile costs).
    pub total_cost: f32,
    /// Net trade value: how much each settlement has that the other lacks,
    /// divided by path cost. Higher = more worth trading.
    pub value: f32,
}

/// Compute trade routes between every pair of settlements. Routes with
/// `value < 0.01` are filtered out (not worth trading).
///
/// `routes_per_settlement` caps how many outbound routes each settlement
/// keeps (to prevent N² explosion for large networks).
pub fn compute_trade_routes(
    grid: &SurfaceGrid,
    resources: &world::resources::ResourceMap,
    settlements: &[life::Settlement],
    routes_per_settlement: usize,
) -> Vec<TradeRoute> {
    use std::collections::HashSet;
    let n = settlements.len();
    if n < 2 {
        return Vec::new();
    }
    let mut all_routes: Vec<TradeRoute> = Vec::new();
    // Precompute per-settlement resource sets.
    let resource_sets: Vec<HashSet<world::resources::Resource>> = settlements
        .iter()
        .map(|s| {
            resources
                .per_tile
                .get(s.tile_idx)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .collect()
        })
        .collect();

    for i in 0..n {
        let mut candidates: Vec<(usize, f32, Vec<usize>, f32)> = Vec::new();
        for j in 0..n {
            if i == j {
                continue;
            }
            let Some(path) = world::routing::find_path(
                grid,
                settlements[i].tile_idx,
                settlements[j].tile_idx,
                |idx| world::routing::trade_cost(grid, idx),
            ) else {
                continue;
            };
            let complementarity = {
                let a = &resource_sets[i];
                let b = &resource_sets[j];
                let a_only = a.difference(b).count();
                let b_only = b.difference(a).count();
                (a_only + b_only) as f32
            };
            let value = complementarity / (1.0 + path.total_cost);
            candidates.push((j, path.total_cost, path.tiles, value));
        }
        candidates.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
        for (j, cost, tiles, value) in candidates.into_iter().take(routes_per_settlement) {
            if value < 0.01 {
                continue;
            }
            all_routes.push(TradeRoute {
                from_settlement: i,
                to_settlement: j,
                tiles,
                total_cost: cost,
                value,
            });
        }
    }
    all_routes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_cosmos_facts_into_world_input() {
        let facts = ExternalBodyFacts {
            body_id: 7,
            mass: 1.0,
            radius: 1.0,
            density: 5.5,
            gravity: 1.0,
            blackbody_temperature: 288,
            star_age: 4.6,
            distance_from_star: 1.0,
            eccentricity: 0.0167,
            axial_tilt: 23.4,
            rotation_days: 1.0,
            is_tidally_locked: false,
            tidal_heating: 0,
            moon_count: 1,
            has_rings: false,
        };

        let input = external_facts_to_world_input(&facts);
        assert_eq!(input.body_id, 7);
        assert_eq!(input.blackbody_temp_k, 288);
        assert_eq!(input.star.age_gyr, 4.6);
        assert_eq!(input.orbit.orbital_distance_au, 1.0);
        assert_eq!(input.moon_count, 1);
    }

    #[test]
    fn maps_cosmos_telluric_details_into_world_interior() {
        let details = cosmos::prelude::TelluricBodyDetails::new(
            cosmos::prelude::TelluricBodyComposition::Rocky,
            cosmos::prelude::CelestialBodyWorldType::Terrestrial,
            Vec::new(),
            cosmos::prelude::CelestialBodyCoreHeat::ActiveCore,
            cosmos::prelude::MagneticFieldStrength::Strong,
            Vec::new(),
            Vec::new(),
            10.0,
            true,
            65.0,
        );

        let profile = telluric_details_to_world_profile(&details, LifeLevel::Sentient);
        assert_eq!(profile.life_level, LifeLevel::Sentient);
        assert_eq!(profile.world_type, CelestialBodyWorldType::Terrestrial);
        assert_eq!(profile.magnetic_field, MagneticFieldStrength::Strong);
    }

    #[test]
    fn generates_complete_world_from_cosmos_body() {
        let body = cosmos::prelude::CelestialBody::new(
            Some(cosmos::prelude::Orbit {
                average_distance: 1.0,
                average_distance_from_system_center: 1.0,
                eccentricity: 0.0167,
                axial_tilt: 23.4,
                rotation: 1.0,
                ..Default::default()
            }),
            7,
            "Gaia".into(),
            1.0,
            1.0,
            5.5,
            1.0,
            288,
            0,
            cosmos::prelude::CelestialBodySize::Standard,
            cosmos::prelude::CelestialBodyDetails::Telluric(
                cosmos::prelude::TelluricBodyDetails::new(
                    cosmos::prelude::TelluricBodyComposition::Rocky,
                    cosmos::prelude::CelestialBodyWorldType::Terrestrial,
                    Vec::new(),
                    cosmos::prelude::CelestialBodyCoreHeat::ActiveCore,
                    cosmos::prelude::MagneticFieldStrength::Strong,
                    Vec::new(),
                    Vec::new(),
                    10.0,
                    true,
                    65.0,
                ),
            ),
        );

        let generated =
            generate_world_from_cosmos_body(&body, 4.6, 1, false, LifeLevel::Sentient).unwrap();

        assert_eq!(generated.input.body_id, 7);
        assert_eq!(
            generated.profile.world_type,
            CelestialBodyWorldType::Terrestrial
        );
        assert!(generated.interior.atmospheric_pressure > 0.0);
        assert!(generated.detail.photochemistry.is_some());
    }

    fn sentient_species(tech_level: u8) -> life::Species {
        life::Species {
            name: "Testus".into(),
            tech_level: Some(tech_level),
            intelligence: 5,
            ..Default::default()
        }
    }

    #[test]
    fn presapient_species_has_no_tech_conditions() {
        let mut s = sentient_species(5);
        s.tech_level = None;
        assert!(species_tech_to_conditions(&s).is_none());
        assert!(recipes_accessible_to_species(&s).is_empty());
    }

    #[test]
    fn bronze_age_species_blocked_from_modern_recipes() {
        let bronze = sentient_species(3);
        let conditions = species_tech_to_conditions(&bronze).unwrap();
        assert!(conditions.max_temperature_c < 2000);
        let recipes = recipes_accessible_to_species(&bronze);
        // No recipe hotter than bronze-age furnace should appear.
        for r in &recipes {
            assert!(
                r.min_temp_c <= conditions.max_temperature_c,
                "bronze-age species accessed hot recipe {} at {}°C",
                r.name,
                r.min_temp_c
            );
        }
    }

    #[test]
    fn higher_tech_unlocks_more_recipes() {
        let bronze = sentient_species(3);
        let industrial = sentient_species(7);
        let modern = sentient_species(10);
        let bronze_count = recipes_accessible_to_species(&bronze).len();
        let industrial_count = recipes_accessible_to_species(&industrial).len();
        let modern_count = recipes_accessible_to_species(&modern).len();
        assert!(
            bronze_count <= industrial_count,
            "industrial ({}) ≥ bronze ({})",
            industrial_count,
            bronze_count
        );
        assert!(
            industrial_count <= modern_count,
            "modern ({}) ≥ industrial ({})",
            modern_count,
            industrial_count
        );
    }

    #[test]
    fn surface_grid_converts_to_habitat_grid() {
        use world::climate::{generate_biomes, generate_temperature, generate_wind};
        use world::geology::generate_geology;
        use world::grid::GridResolution;
        use world::hydrology::generate_precipitation;
        use world::ocean::generate_ocean_dynamics;
        use world::types::StarContext;

        let input = PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut sg = generate_geology(&input, 71.0, GridResolution::Fast, "adapt");
        generate_temperature(&input, 33.0, &mut sg);
        generate_wind(&input, 1.0, &mut sg);
        generate_precipitation(&input, 1.0, 71.0, &mut sg);
        generate_ocean_dynamics(&mut sg);
        generate_biomes(&mut sg);

        let habitat = surface_grid_to_habitat_grid(&sg);
        assert_eq!(habitat.width, sg.width);
        assert_eq!(habitat.height, sg.height);
        assert_eq!(habitat.tile_count(), sg.tile_count());
        assert_eq!(habitat.temperature_c.len(), sg.tile_count());
        assert_eq!(habitat.biome.len(), sg.tile_count());
        // Ocean tiles should map through unchanged.
        for idx in 0..sg.tile_count() {
            assert_eq!(habitat.is_ocean[idx], sg.layers.is_ocean[idx]);
        }
    }

    #[test]
    fn life_distribution_produces_species_ranges() {
        use life::types::{Climate, Habitat, Temperature};
        use life::{distribute_ecosystem, generate_ecosystem_from_world, SpeciesGenerationInput};
        use world::climate::{generate_biomes, generate_temperature, generate_wind};
        use world::geology::generate_geology;
        use world::grid::GridResolution;
        use world::hydrology::generate_precipitation;
        use world::ocean::generate_ocean_dynamics;
        use world::types::StarContext;

        // Build a world grid.
        let wi = PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut sg = generate_geology(&wi, 71.0, GridResolution::Fast, "combo");
        generate_temperature(&wi, 33.0, &mut sg);
        generate_wind(&wi, 1.0, &mut sg);
        generate_precipitation(&wi, 1.0, 71.0, &mut sg);
        generate_ocean_dynamics(&mut sg);
        generate_biomes(&mut sg);
        let habitat = surface_grid_to_habitat_grid(&sg);

        // Generate an ecosystem for an Earth-like world.
        let input = SpeciesGenerationInput {
            habitat: Habitat::Terrestrial,
            climate: Climate::Terrestrial,
            temperature: Temperature::Temperate,
            gravity: 1.0,
            atmospheric_pressure: 1.0,
            hydrosphere: 71.0,
            life_level: life::LifeLevel::AnimalLike,
            seed: "combo".into(),
            scope_key: "earth".into(),
        };
        let ecosystem = generate_ecosystem_from_world(&input);
        let distribution =
            distribute_ecosystem(&ecosystem, 1.0, &habitat, life::LifeLevel::AnimalLike);

        assert!(distribution.ranges.len() >= 3, "expect ≥3 species ranges");
        // Every range has the correct number of tiles.
        for range in &distribution.ranges {
            assert_eq!(range.habitability.len(), habitat.tile_count());
            assert_eq!(range.territory.len(), habitat.tile_count());
            assert_eq!(range.population_density.len(), habitat.tile_count());
        }
        // Vegetation density should have at least a few tiles above 0.
        let vege_count = distribution
            .vegetation_density
            .iter()
            .filter(|&&d| d > 0.0)
            .count();
        assert!(vege_count > 0, "no vegetation anywhere");
    }

    #[test]
    fn resource_map_bridges_to_crafting_substances() {
        use world::climate::{generate_biomes, generate_temperature, generate_wind};
        use world::geology::generate_geology;
        use world::grid::GridResolution;
        use world::hydrology::{generate_hydrology, generate_precipitation};
        use world::ocean::generate_ocean_dynamics;
        use world::resources::generate_resources;
        use world::types::StarContext;

        let input = PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "resources");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        generate_biomes(&mut g);
        let rm = generate_resources(&g);

        let substances = resource_map_to_substance_set(&rm);
        assert!(
            !substances.is_empty(),
            "Earth-like world should yield substances"
        );
        // Ocean tiles always produce Salt.
        assert!(substances.contains(&crafting::Substance::Salt));
    }

    #[test]
    fn settlements_placed_on_earth_like_world() {
        use life::{
            compute_settlement_suitability, place_settlements, Climate, Habitat, LifeLevel,
            SpeciesGenerationInput, Temperature,
        };
        use world::climate::{generate_biomes, generate_temperature, generate_wind};
        use world::geology::generate_geology;
        use world::grid::GridResolution;
        use world::hydrology::{generate_hydrology, generate_precipitation};
        use world::ocean::generate_ocean_dynamics;
        use world::resources::generate_resources;
        use world::types::StarContext;

        let input = PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "settle");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        generate_biomes(&mut g);
        let habitat = surface_grid_to_habitat_grid(&g);
        let resources = generate_resources(&g);

        // Generate a species and compute its ranges.
        let species_input = SpeciesGenerationInput {
            habitat: Habitat::Terrestrial,
            climate: Climate::Terrestrial,
            temperature: Temperature::Temperate,
            gravity: 1.0,
            atmospheric_pressure: 1.0,
            hydrosphere: 71.0,
            life_level: LifeLevel::Sentient,
            seed: "settle".into(),
            scope_key: "gaia".into(),
        };
        let species = life::generator::generate_species_from_world(&species_input).unwrap();
        let range = crate::generate_species_on_surface(&g, &species, 1.0, LifeLevel::Sentient);

        // Build the scoring inputs.
        let water = water_access_from_grid(&g);
        let res_score = resource_density_from_map(&resources);
        let suitability =
            compute_settlement_suitability(&habitat, &range.habitability, &water, &res_score);

        // Place up to 8 settlements with separation 5 tiles.
        let settlements = place_settlements(&suitability, &habitat, species.name.clone(), 8, 5);
        assert!(
            !settlements.is_empty(),
            "Earth-like world should support at least one settlement"
        );
        assert!(
            settlements.len() <= 8,
            "settlement cap exceeded: {}",
            settlements.len()
        );
        // Highest-scoring settlement should not be on the ocean.
        for s in &settlements {
            assert!(!g.layers.is_ocean[s.tile_idx]);
        }
        // Settlements should be ordered by suitability.
        for w in settlements.windows(2) {
            assert!(w[0].suitability >= w[1].suitability);
        }
    }

    #[test]
    fn water_access_flags_coasts_and_rivers() {
        use world::climate::{generate_biomes, generate_temperature, generate_wind};
        use world::geology::generate_geology;
        use world::grid::GridResolution;
        use world::hydrology::{generate_hydrology, generate_precipitation};
        use world::ocean::generate_ocean_dynamics;
        use world::types::StarContext;

        let input = PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "water");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        generate_biomes(&mut g);
        let water = water_access_from_grid(&g);
        assert_eq!(water.len(), g.tile_count());
        // Ocean tiles score 0 (no interior water needed).
        for (idx, &w_score) in water.iter().enumerate() {
            if g.layers.is_ocean[idx] {
                assert_eq!(w_score, 0.0);
            }
        }
        // At least a few land tiles should have water access.
        let access_count = water.iter().filter(|&&w| w > 0.5).count();
        assert!(access_count > 0);
    }

    #[test]
    fn named_features_use_markov_names() {
        use world::climate::{generate_biomes, generate_temperature, generate_wind};
        use world::features::detect_features;
        use world::geology::generate_geology;
        use world::grid::GridResolution;
        use world::hydrology::{generate_hydrology, generate_precipitation};
        use world::ocean::generate_ocean_dynamics;
        use world::types::StarContext;

        let input = PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "named");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        generate_biomes(&mut g);

        let features = detect_features(&g);
        let named = name_features(&features, life::NameStyle::FantasyHuman, "named");

        // Each named feature has a non-empty name.
        for (name, _) in &named.mountain_ranges {
            assert!(!name.is_empty());
            assert!(name.ends_with("Mountains"));
        }
        for (name, _) in &named.rivers {
            assert!(name.ends_with("River"));
        }
        for (name, _) in &named.ocean_basins {
            assert!(name.ends_with("Ocean"));
        }
        for (name, _) in &named.deserts {
            assert!(name.ends_with("Desert"));
        }
        assert_eq!(named.mountain_ranges.len(), features.mountain_ranges.len());
        assert_eq!(named.ocean_basins.len(), features.ocean_basins.len());

        // Determinism: same seed produces same names.
        let named2 = name_features(&features, life::NameStyle::FantasyHuman, "named");
        assert_eq!(
            named
                .mountain_ranges
                .iter()
                .map(|(n, _)| n.clone())
                .collect::<Vec<_>>(),
            named2
                .mountain_ranges
                .iter()
                .map(|(n, _)| n.clone())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn trade_routes_connect_settlements() {
        use life::{
            compute_settlement_suitability, place_settlements, Climate, Habitat, LifeLevel,
            SpeciesGenerationInput, Temperature,
        };
        use world::climate::{generate_biomes, generate_temperature, generate_wind};
        use world::geology::generate_geology;
        use world::grid::GridResolution;
        use world::hydrology::{generate_hydrology, generate_precipitation};
        use world::ocean::generate_ocean_dynamics;
        use world::resources::generate_resources;
        use world::types::StarContext;

        let input = PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "trade");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        generate_biomes(&mut g);
        let habitat = surface_grid_to_habitat_grid(&g);
        let rm = generate_resources(&g);

        // Place a few settlements.
        let species_input = SpeciesGenerationInput {
            habitat: Habitat::Terrestrial,
            climate: Climate::Terrestrial,
            temperature: Temperature::Temperate,
            gravity: 1.0,
            atmospheric_pressure: 1.0,
            hydrosphere: 71.0,
            life_level: LifeLevel::Sentient,
            seed: "trade".into(),
            scope_key: "gaia".into(),
        };
        let species = life::generator::generate_species_from_world(&species_input).unwrap();
        let range = crate::generate_species_on_surface(&g, &species, 1.0, LifeLevel::Sentient);
        let water = water_access_from_grid(&g);
        let res_score = resource_density_from_map(&rm);
        let suit =
            compute_settlement_suitability(&habitat, &range.habitability, &water, &res_score);
        let settlements = place_settlements(&suit, &habitat, species.name.clone(), 6, 5);
        assert!(settlements.len() >= 3);

        let routes = compute_trade_routes(&g, &rm, &settlements, 2);
        // Every settlement participates in at least one route as source.
        let mut has_outbound = vec![false; settlements.len()];
        for r in &routes {
            has_outbound[r.from_settlement] = true;
            assert!(!r.tiles.is_empty());
            assert_eq!(
                r.tiles.first(),
                Some(&settlements[r.from_settlement].tile_idx)
            );
            assert_eq!(r.tiles.last(), Some(&settlements[r.to_settlement].tile_idx));
            assert!(r.total_cost > 0.0);
            assert!(r.value >= 0.01);
        }
        // Expect at least most settlements to participate.
        let connected = has_outbound.iter().filter(|&&b| b).count();
        assert!(connected >= settlements.len() - 1);
    }
}
