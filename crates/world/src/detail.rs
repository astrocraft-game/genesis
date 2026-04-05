use crate::atmosphere::generate_atmosphere_profile;
use crate::climate::generate_climate_profile;
use crate::hydrology::generate_hydrography;
use crate::impacts::generate_impact_history;
use crate::interior::generate_planet_interior;
use crate::ocean::generate_ocean_chemistry;
use crate::photochemistry::generate_photochemistry;
use crate::subsurface::generate_subsurface_ocean;
pub use crate::types::PlanetaryDetail;
use crate::types::{
    AtmosphereBreathability, AtmosphereToxicity, ClimateRegime, HazeRegime, OrbitContext,
    PlanetGenerationProfile, PlanetInterior, PlanetSimulationInput, StarContext,
};

pub fn generate_planetary_detail(
    context: &PlanetSimulationInput,
    interior: &PlanetInterior,
) -> PlanetaryDetail {
    let atmosphere = generate_atmosphere_profile(
        context,
        interior.atmospheric_pressure,
        &interior.atmospheric_composition,
        interior.magnetic_field,
        interior.life_level,
    );
    let climate = generate_climate_profile(
        context,
        &atmosphere,
        interior.atmospheric_pressure,
        &interior.atmospheric_composition,
        interior.hydrosphere,
        interior.ice_over_water,
        interior.ice_over_land,
        interior.volcanism,
        interior.tectonic_activity,
        interior.body_type,
        interior.world_type,
        interior.life_level,
    );
    let land_fraction = (100.0 - interior.hydrosphere).max(0.0) / 100.0;
    let photochemistry = generate_photochemistry(
        context,
        interior.atmospheric_pressure,
        &interior.atmospheric_composition,
        interior.body_type,
        interior.life_level,
    );
    let ocean_chemistry = generate_ocean_chemistry(
        context,
        interior.atmospheric_pressure,
        &interior.atmospheric_composition,
        interior.body_type,
        interior.world_type,
        interior.hydrosphere,
        interior.volcanism,
        interior.tectonic_activity,
        land_fraction,
        interior.life_level,
    );
    let impact_history = Some(generate_impact_history(
        context,
        interior.body_type,
        interior.world_type,
        interior.surface_map.as_ref(),
        interior.hydrosphere,
        interior.atmospheric_pressure,
        interior.volcanism,
        interior.tectonic_activity,
        climate.glaciation.as_ref(),
        climate.wind.as_ref(),
    ));
    let subsurface_ocean = generate_subsurface_ocean(
        context,
        interior.body_type,
        interior.world_type,
        interior.hydrosphere,
        interior.volcanism,
        interior.tectonic_activity,
    );
    let hydrography =
        generate_hydrography(context, interior.atmospheric_pressure, interior.hydrosphere);

    PlanetaryDetail {
        atmospheric_layers: atmosphere.atmospheric_layers,
        atmospheric_escape: atmosphere.atmospheric_escape,
        breathability: atmosphere.breathability,
        toxicity: atmosphere.toxicity,
        greenhouse: climate.greenhouse,
        climate_regulation: climate.climate_regulation,
        tidally_locked_climate: climate.tidally_locked_climate,
        photochemistry,
        sky: climate.sky,
        wind: climate.wind,
        hydrography,
        glaciation: climate.glaciation,
        impact_history,
        subsurface_ocean,
        ocean_chemistry,
        ..Default::default()
    }
}

pub fn generate_complete_planet(
    context: &PlanetSimulationInput,
    profile: &PlanetGenerationProfile,
) -> (PlanetInterior, PlanetaryDetail) {
    let interior = generate_planet_interior(context, profile);
    let detail = generate_planetary_detail(context, &interior);
    (interior, detail)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn earth_context() -> PlanetSimulationInput {
        PlanetSimulationInput {
            blackbody_temp_k: 288,
            gravity_g: 1.0,
            body_radius_earth: 1.0,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                rotation_period_days: 1.0,
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn earth_profile() -> PlanetGenerationProfile {
        PlanetGenerationProfile {
            magnetic_field: crate::types::MagneticFieldStrength::Strong,
            world_type: crate::types::CelestialBodyWorldType::Terrestrial,
            life_level: crate::types::LifeLevel::Sentient,
            ..Default::default()
        }
    }

    #[test]
    fn earth_like_detail_pipeline_assembles_internal_systems() {
        let (_interior, detail) = generate_complete_planet(&earth_context(), &earth_profile());

        assert_eq!(detail.breathability, AtmosphereBreathability::Standard);
        assert_eq!(detail.toxicity, AtmosphereToxicity::Benign);
        assert_eq!(
            detail.photochemistry.unwrap().haze_regime,
            HazeRegime::OzoneShielded
        );
        assert!(matches!(
            detail.climate_regulation.unwrap().regime,
            ClimateRegime::CarbonateSilicate | ClimateRegime::WeatheringBalanced
        ));
        assert!(detail.ocean_chemistry.is_some());
        assert!(detail.impact_history.is_some());
    }
}
