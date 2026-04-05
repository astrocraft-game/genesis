use cosmos::prelude::ExternalBodyFacts;
use life::{Climate, Habitat, SpeciesGenerationInput, Temperature};
use world::prelude::{
    CelestialBodyWorldType, LifeLevel, MagneticFieldStrength, OrbitContext,
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
}
