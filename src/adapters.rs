use cosmos::prelude::ExternalBodyFacts;
use world::grid::SurfaceGrid;
use world::prelude::{
    CelestialBodyWorldType, LifeLevel, MagneticFieldStrength, OrbitContext,
    PlanetGenerationProfile, PlanetInterior, PlanetSimulationInput, PlanetaryDetail, StarContext,
    TelluricBodyComposition,
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

/// Derive a per-tile water-access score from a `SurfaceGrid`.
pub fn water_access_from_grid(grid: &SurfaceGrid) -> Vec<f32> {
    let w = grid.width as usize;
    let h = grid.height as usize;
    let n = w * h;
    let mut out = vec![0.0f32; n];
    for (idx, slot) in out.iter_mut().enumerate() {
        if grid.layers.is_ocean[idx] {
            continue;
        }
        let discharge = grid.layers.river_discharge_m3s[idx];
        let river_score = (discharge / 100.0).min(1.0);
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

/// Derive a per-tile resource density score from a ResourceMap.
pub fn resource_density_from_map(map: &world::resources::ResourceMap) -> Vec<f32> {
    let max_count = map.per_tile.iter().map(|t| t.len()).max().unwrap_or(1) as f32;
    let max_count = max_count.max(1.0);
    map.per_tile
        .iter()
        .map(|t| (t.len() as f32 / max_count).clamp(0.0, 1.0))
        .collect()
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
            "TestWorld".into(),
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

        let generated = generate_world_from_cosmos_body(&body, 4.6, 1, false, LifeLevel::Sentient);
        assert!(generated.is_some());
        let g = generated.unwrap();
        assert_eq!(g.profile.world_type, CelestialBodyWorldType::Terrestrial);
        assert!(g.detail.photochemistry.is_some());
    }
}
