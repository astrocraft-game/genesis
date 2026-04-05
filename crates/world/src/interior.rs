use crate::types::{
    AtmosphericCirculation, BiomeType, CelestialBodyWorldType, ChemicalComponent, CraterDensity,
    MagneticFieldStrength, PlanetGenerationProfile, PlanetInterior, PlanetSimulationInput,
    PlanetSurfaceMap, TelluricBodyComposition, WindIntensity, WorldClimateType,
    WorldTemperatureCategory,
};

pub fn generate_planet_interior(
    context: &PlanetSimulationInput,
    profile: &PlanetGenerationProfile,
) -> PlanetInterior {
    let temperature_category = classify_temperature(context.blackbody_temp_k);
    let atmospheric_pressure =
        generate_atmospheric_pressure(context, profile, temperature_category);
    let atmospheric_composition =
        generate_atmospheric_composition(profile, atmospheric_pressure, temperature_category);
    let hydrosphere = generate_hydrosphere(context, profile, temperature_category);
    let ice_over_water = generate_ice_over_water(temperature_category, hydrosphere);
    let land_area_percentage = generate_land_area_percentage(profile, hydrosphere);
    let ice_over_land =
        generate_ice_over_land(temperature_category, hydrosphere, land_area_percentage);
    let volcanism = generate_volcanism(context, profile);
    let tectonic_activity = generate_tectonics(context, volcanism);
    let humidity = generate_humidity(atmospheric_pressure, hydrosphere, context.blackbody_temp_k);
    let climate = classify_climate(
        profile,
        temperature_category,
        atmospheric_pressure,
        hydrosphere,
        context,
    );
    let surface_map = Some(generate_surface_map(
        context,
        climate,
        hydrosphere,
        ice_over_water,
        ice_over_land,
        volcanism,
        tectonic_activity,
    ));
    let atmospheric_circulation = generate_atmospheric_circulation(context, atmospheric_pressure);

    PlanetInterior {
        body_type: profile.body_type,
        world_type: profile.world_type,
        magnetic_field: profile.magnetic_field,
        atmospheric_pressure,
        atmospheric_composition,
        hydrosphere,
        ice_over_water,
        land_area_percentage,
        ice_over_land,
        volcanism,
        tectonic_activity,
        humidity,
        temperature_category,
        climate,
        life_level: profile.life_level,
        surface_map,
        atmospheric_circulation,
    }
}

fn classify_temperature(blackbody_temp_k: u32) -> WorldTemperatureCategory {
    match blackbody_temp_k {
        0..=120 => WorldTemperatureCategory::Frozen,
        121..=170 => WorldTemperatureCategory::VeryCold,
        171..=220 => WorldTemperatureCategory::Cold,
        221..=255 => WorldTemperatureCategory::Chilly,
        256..=285 => WorldTemperatureCategory::Cool,
        286..=305 => WorldTemperatureCategory::Temperate,
        306..=330 => WorldTemperatureCategory::Warm,
        331..=380 => WorldTemperatureCategory::Hot,
        381..=500 => WorldTemperatureCategory::VeryHot,
        501..=900 => WorldTemperatureCategory::Scorching,
        _ => WorldTemperatureCategory::Infernal,
    }
}

fn generate_atmospheric_pressure(
    context: &PlanetSimulationInput,
    profile: &PlanetGenerationProfile,
    temperature: WorldTemperatureCategory,
) -> f32 {
    let gravity_factor = context.gravity_g.clamp(0.1, 2.5);
    let base: f32 = match profile.world_type {
        CelestialBodyWorldType::Ocean | CelestialBodyWorldType::Terrestrial => 0.8,
        CelestialBodyWorldType::Greenhouse => 20.0,
        CelestialBodyWorldType::Ammonia => 3.0,
        CelestialBodyWorldType::Ice | CelestialBodyWorldType::DirtySnowball => 0.15,
        CelestialBodyWorldType::LavaWorld | CelestialBodyWorldType::Chthonian => 0.05,
        CelestialBodyWorldType::ProtoWorld => 0.2,
        _ => 0.35,
    };
    let temp_factor: f32 = match temperature {
        WorldTemperatureCategory::Frozen | WorldTemperatureCategory::VeryCold => 0.35,
        WorldTemperatureCategory::Cold | WorldTemperatureCategory::Chilly => 0.65,
        WorldTemperatureCategory::Cool | WorldTemperatureCategory::Temperate => 1.0,
        WorldTemperatureCategory::Warm | WorldTemperatureCategory::Hot => 1.15,
        WorldTemperatureCategory::VeryHot => 0.85,
        WorldTemperatureCategory::Scorching | WorldTemperatureCategory::Infernal => 0.25,
    };
    let tidal_factor = 1.0 + (context.tidal_heating as f32 / 60.0).clamp(0.0, 0.5);

    (base * gravity_factor * temp_factor * tidal_factor).clamp(0.0, 90.0)
}

fn generate_atmospheric_composition(
    profile: &PlanetGenerationProfile,
    atmospheric_pressure: f32,
    temperature: WorldTemperatureCategory,
) -> Vec<(f32, ChemicalComponent)> {
    if atmospheric_pressure <= 0.01 {
        return Vec::new();
    }

    match profile.world_type {
        CelestialBodyWorldType::Greenhouse => vec![
            (0.92, ChemicalComponent::CarbonDioxide),
            (0.05, ChemicalComponent::Nitrogen),
            (0.03, ChemicalComponent::SulfurDioxide),
        ],
        CelestialBodyWorldType::Ammonia => vec![
            (0.65, ChemicalComponent::Nitrogen),
            (0.2, ChemicalComponent::Ammonia),
            (0.15, ChemicalComponent::Methane),
        ],
        CelestialBodyWorldType::Ice | CelestialBodyWorldType::DirtySnowball => vec![
            (0.85, ChemicalComponent::Nitrogen),
            (0.1, ChemicalComponent::Methane),
            (0.05, ChemicalComponent::Argon),
        ],
        CelestialBodyWorldType::LavaWorld | CelestialBodyWorldType::Chthonian => vec![
            (0.7, ChemicalComponent::CarbonDioxide),
            (0.2, ChemicalComponent::SulfurDioxide),
            (0.1, ChemicalComponent::CarbonMonoxide),
        ],
        _ if profile.life_level.as_u8() >= crate::types::LifeLevel::PlantLike.as_u8()
            && matches!(
                temperature,
                WorldTemperatureCategory::Cool
                    | WorldTemperatureCategory::Temperate
                    | WorldTemperatureCategory::Warm
            ) =>
        {
            vec![
                (0.78, ChemicalComponent::Nitrogen),
                (0.21, ChemicalComponent::Oxygen),
                (0.01, ChemicalComponent::Argon),
            ]
        }
        _ => vec![
            (0.82, ChemicalComponent::Nitrogen),
            (0.15, ChemicalComponent::CarbonDioxide),
            (0.03, ChemicalComponent::Argon),
        ],
    }
}

fn generate_hydrosphere(
    context: &PlanetSimulationInput,
    profile: &PlanetGenerationProfile,
    temperature: WorldTemperatureCategory,
) -> f32 {
    let base = match profile.world_type {
        CelestialBodyWorldType::Ocean => 85.0,
        CelestialBodyWorldType::Terrestrial => 55.0,
        CelestialBodyWorldType::Ammonia => 45.0,
        CelestialBodyWorldType::Ice | CelestialBodyWorldType::DirtySnowball => 35.0,
        CelestialBodyWorldType::Greenhouse | CelestialBodyWorldType::LavaWorld => 2.0,
        CelestialBodyWorldType::Hadean | CelestialBodyWorldType::ProtoWorld => 15.0,
        _ => 12.0,
    };
    let temp_factor = match temperature {
        WorldTemperatureCategory::Frozen | WorldTemperatureCategory::VeryCold => 0.7,
        WorldTemperatureCategory::Cold | WorldTemperatureCategory::Chilly => 0.9,
        WorldTemperatureCategory::Cool | WorldTemperatureCategory::Temperate => 1.0,
        WorldTemperatureCategory::Warm => 0.95,
        WorldTemperatureCategory::Hot => 0.7,
        WorldTemperatureCategory::VeryHot | WorldTemperatureCategory::Scorching => 0.25,
        WorldTemperatureCategory::Infernal => 0.0,
    };
    let gravity_bonus: f32 = if context.gravity_g > 0.8 { 1.0 } else { 0.75 };

    (base * temp_factor * gravity_bonus).clamp(0.0, 100.0)
}

fn generate_ice_over_water(temperature: WorldTemperatureCategory, hydrosphere: f32) -> f32 {
    let coverage: f32 = match temperature {
        WorldTemperatureCategory::Frozen => 100.0,
        WorldTemperatureCategory::VeryCold => 75.0,
        WorldTemperatureCategory::Cold => 45.0,
        WorldTemperatureCategory::Chilly => 20.0,
        WorldTemperatureCategory::Cool => 8.0,
        _ => 0.0,
    };

    coverage.min(hydrosphere)
}

fn generate_land_area_percentage(profile: &PlanetGenerationProfile, hydrosphere: f32) -> f32 {
    let base = (100.0 - hydrosphere).clamp(0.0, 100.0);
    match profile.world_type {
        CelestialBodyWorldType::Ocean => base.min(20.0),
        CelestialBodyWorldType::LavaWorld => 95.0,
        _ => base,
    }
}

fn generate_ice_over_land(
    temperature: WorldTemperatureCategory,
    hydrosphere: f32,
    land_area_percentage: f32,
) -> f32 {
    let base: f32 = match temperature {
        WorldTemperatureCategory::Frozen => 95.0,
        WorldTemperatureCategory::VeryCold => 70.0,
        WorldTemperatureCategory::Cold => 35.0,
        WorldTemperatureCategory::Chilly => 12.0,
        _ => 0.0,
    };

    base.min(land_area_percentage.max(0.0))
        .min(100.0 - hydrosphere + land_area_percentage)
}

fn generate_volcanism(context: &PlanetSimulationInput, profile: &PlanetGenerationProfile) -> f32 {
    let body_factor = match profile.body_type {
        TelluricBodyComposition::Metallic => 18.0,
        TelluricBodyComposition::Rocky => 24.0,
        TelluricBodyComposition::Icy => 10.0,
    };
    let world_factor = match profile.world_type {
        CelestialBodyWorldType::GeoActive | CelestialBodyWorldType::LavaWorld => 25.0,
        CelestialBodyWorldType::ProtoWorld | CelestialBodyWorldType::Hadean => 15.0,
        CelestialBodyWorldType::Ice | CelestialBodyWorldType::DirtySnowball => 5.0,
        _ => 0.0,
    };

    (body_factor
        + world_factor
        + context.tidal_heating as f32 * 1.8
        + context.body_mass_earth as f32 * 6.0)
        .clamp(0.0, 100.0)
}

fn generate_tectonics(context: &PlanetSimulationInput, volcanism: f32) -> f32 {
    (volcanism * 0.75 + context.body_mass_earth as f32 * 8.0 + context.moon_count as f32 * 4.0)
        .clamp(0.0, 100.0)
}

fn generate_humidity(atmospheric_pressure: f32, hydrosphere: f32, blackbody_temp_k: u32) -> f32 {
    if atmospheric_pressure <= 0.01 || hydrosphere <= 0.0 {
        return 0.0;
    }

    let temp_factor = match blackbody_temp_k {
        0..=220 => 0.2,
        221..=260 => 0.45,
        261..=320 => 0.8,
        321..=380 => 0.55,
        _ => 0.15,
    };

    (hydrosphere * temp_factor * atmospheric_pressure.clamp(0.1, 2.0)).clamp(0.0, 100.0)
}

fn classify_climate(
    profile: &PlanetGenerationProfile,
    temperature: WorldTemperatureCategory,
    atmospheric_pressure: f32,
    hydrosphere: f32,
    context: &PlanetSimulationInput,
) -> WorldClimateType {
    if context.orbit.tidally_locked && hydrosphere > 10.0 && atmospheric_pressure > 0.1 {
        return WorldClimateType::Ribbon;
    }

    match profile.world_type {
        CelestialBodyWorldType::Ocean => WorldClimateType::Ocean,
        CelestialBodyWorldType::Greenhouse | CelestialBodyWorldType::LavaWorld => {
            WorldClimateType::Dead
        }
        _ => match temperature {
            WorldTemperatureCategory::Frozen | WorldTemperatureCategory::VeryCold => {
                WorldClimateType::Arctic
            }
            WorldTemperatureCategory::Cold | WorldTemperatureCategory::Chilly => {
                if hydrosphere > 40.0 {
                    WorldClimateType::Taiga
                } else {
                    WorldClimateType::Tundra
                }
            }
            WorldTemperatureCategory::Cool | WorldTemperatureCategory::Temperate => {
                if hydrosphere > 60.0 {
                    WorldClimateType::Ocean
                } else {
                    WorldClimateType::Terrestrial
                }
            }
            WorldTemperatureCategory::Warm => {
                if hydrosphere > 50.0 {
                    WorldClimateType::Rainforest
                } else {
                    WorldClimateType::Savanna
                }
            }
            WorldTemperatureCategory::Hot => {
                if hydrosphere > 30.0 && atmospheric_pressure > 0.3 {
                    WorldClimateType::Tropical
                } else {
                    WorldClimateType::Desert
                }
            }
            _ => WorldClimateType::Dead,
        },
    }
}

fn generate_surface_map(
    context: &PlanetSimulationInput,
    climate: WorldClimateType,
    hydrosphere: f32,
    ice_over_water: f32,
    ice_over_land: f32,
    volcanism: f32,
    tectonic_activity: f32,
) -> PlanetSurfaceMap {
    let continent_count = if hydrosphere > 80.0 {
        2
    } else if hydrosphere > 50.0 {
        4
    } else {
        6
    };
    let biome_distribution = match climate {
        WorldClimateType::Ocean => vec![(BiomeType::Ocean, hydrosphere / 100.0)],
        WorldClimateType::Rainforest => vec![
            (BiomeType::TropicalForest, 0.35),
            (BiomeType::Wetland, 0.15),
            (BiomeType::Ocean, hydrosphere / 100.0),
        ],
        WorldClimateType::Taiga => vec![
            (BiomeType::Taiga, 0.3),
            (BiomeType::Tundra, 0.15),
            (BiomeType::Ocean, hydrosphere / 100.0),
        ],
        WorldClimateType::Desert => vec![
            (BiomeType::Desert, 0.4),
            (BiomeType::Savanna, 0.1),
            (BiomeType::Ocean, hydrosphere / 100.0),
        ],
        WorldClimateType::Arctic => vec![
            (
                BiomeType::IceCap,
                ((ice_over_water + ice_over_land) / 100.0).clamp(0.2, 0.8),
            ),
            (BiomeType::Tundra, 0.15),
        ],
        WorldClimateType::Dead => vec![(BiomeType::Barren, 0.8)],
        _ => vec![
            (BiomeType::Grassland, 0.25),
            (BiomeType::TemperateForest, 0.2),
            (BiomeType::Ocean, hydrosphere / 100.0),
        ],
    };

    PlanetSurfaceMap {
        continent_count,
        biome_distribution,
        highest_elevation_km: (4.0 + tectonic_activity * 0.09 + volcanism * 0.04).clamp(1.0, 18.0),
        deepest_ocean_km: (2.0 + hydrosphere * 0.06).clamp(0.0, 12.0),
        tectonic_plate_count: (3.0 + tectonic_activity / 10.0).round().clamp(1.0, 16.0) as u8,
        temperature_range_k: (context.orbit.axial_tilt_deg * 0.8
            + context.orbit.eccentricity * 120.0)
            .clamp(2.0, 90.0),
        seasonal_frost: matches!(
            climate,
            WorldClimateType::Arctic | WorldClimateType::Tundra | WorldClimateType::Taiga
        ),
        crater_density: if volcanism > 40.0 || tectonic_activity > 45.0 {
            CraterDensity::Light
        } else if hydrosphere < 5.0 {
            CraterDensity::Heavy
        } else {
            CraterDensity::Moderate
        },
        largest_crater_km: (80.0 + (100.0 - hydrosphere) * 2.5).clamp(50.0, 1500.0),
    }
}

fn generate_atmospheric_circulation(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
) -> Option<AtmosphericCirculation> {
    if atmospheric_pressure <= 0.01 {
        return None;
    }

    let cells = if context.orbit.rotation_period_days <= 2.0 {
        3
    } else if context.orbit.rotation_period_days <= 10.0 {
        2
    } else {
        1
    };
    let wind_intensity = if atmospheric_pressure > 10.0 {
        WindIntensity::Extreme
    } else if atmospheric_pressure > 2.0 {
        WindIntensity::Strong
    } else if atmospheric_pressure > 0.5 {
        WindIntensity::Moderate
    } else {
        WindIntensity::Light
    };

    Some(AtmosphericCirculation {
        cells_per_hemisphere: cells,
        jet_stream_count: cells * 2 - 1,
        wind_intensity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn earth_like_profile_generates_temperate_interior() {
        let context = PlanetSimulationInput {
            gravity_g: 1.0,
            body_mass_earth: 1.0,
            body_radius_earth: 1.0,
            blackbody_temp_k: 288,
            moon_count: 1,
            orbit: crate::types::OrbitContext {
                axial_tilt_deg: 23.4,
                rotation_period_days: 1.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let profile = PlanetGenerationProfile {
            world_type: CelestialBodyWorldType::Terrestrial,
            magnetic_field: MagneticFieldStrength::Strong,
            life_level: crate::types::LifeLevel::Sentient,
            ..Default::default()
        };

        let interior = generate_planet_interior(&context, &profile);
        assert!(interior.atmospheric_pressure > 0.5);
        assert!(interior.hydrosphere > 20.0);
        assert_eq!(
            interior.temperature_category,
            WorldTemperatureCategory::Temperate
        );
    }
}
