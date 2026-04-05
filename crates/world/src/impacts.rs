pub use crate::types::{
    CelestialBodyWorldType, CraterDensity, GlaciationState, ImpactBasinClass, ImpactHistory,
    PlanetSimulationInput, PlanetSurfaceMap, ResurfacingDriver, SurfaceAgeClass,
    TelluricBodyComposition, WindProfile,
};

pub fn generate_impact_history(
    context: &PlanetSimulationInput,
    body_type: TelluricBodyComposition,
    _world_type: CelestialBodyWorldType,
    surface_map: Option<&PlanetSurfaceMap>,
    hydrosphere: f32,
    atmospheric_pressure: f32,
    volcanism: f32,
    tectonic_activity: f32,
    glaciation: Option<&GlaciationState>,
    wind: Option<&WindProfile>,
) -> ImpactHistory {
    let (crater_density, largest_crater_km) = surface_map
        .map(|map| (map.crater_density, map.largest_crater_km))
        .unwrap_or_else(|| {
            let resurfacing_rate = volcanism + tectonic_activity;
            let density = if resurfacing_rate > 80.0 {
                CraterDensity::Pristine
            } else if resurfacing_rate > 50.0 {
                CraterDensity::Light
            } else if resurfacing_rate > 20.0 {
                CraterDensity::Moderate
            } else if resurfacing_rate > 5.0 {
                CraterDensity::Heavy
            } else {
                CraterDensity::Saturated
            };
            let body_diameter_km = context.body_radius_earth as f32 * 6371.0 * 2.0;
            let largest = match density {
                CraterDensity::Pristine => body_diameter_km * 0.03,
                CraterDensity::Light => body_diameter_km * 0.07,
                CraterDensity::Moderate => body_diameter_km * 0.12,
                CraterDensity::Heavy => body_diameter_km * 0.2,
                CraterDensity::Saturated => body_diameter_km * 0.28,
            };
            (density, largest)
        });

    let surface_age_class = match crater_density {
        CraterDensity::Pristine => SurfaceAgeClass::VeryYoung,
        CraterDensity::Light => SurfaceAgeClass::Young,
        CraterDensity::Moderate => SurfaceAgeClass::Mature,
        CraterDensity::Heavy => SurfaceAgeClass::Ancient,
        CraterDensity::Saturated => SurfaceAgeClass::Primordial,
    };

    let resurfacing_driver =
        if body_type == TelluricBodyComposition::Icy && context.tidal_heating > 3 {
            ResurfacingDriver::Cryovolcanic
        } else if tectonic_activity > 35.0 {
            ResurfacingDriver::Tectonic
        } else if volcanism > 20.0 {
            ResurfacingDriver::Volcanic
        } else if glaciation.is_some_and(|g| g.ice_coverage_fraction > 0.2) {
            ResurfacingDriver::Glacial
        } else if atmospheric_pressure > 0.01
            && hydrosphere < 5.0
            && wind.is_some_and(|w| w.mean_surface_wind_ms > 15.0)
        {
            ResurfacingDriver::Aeolian
        } else if atmospheric_pressure <= 0.01 {
            ResurfacingDriver::ImpactOnly
        } else {
            ResurfacingDriver::None
        };

    let planet_radius_km = context.body_radius_earth as f32 * 6371.0;
    let largest_basin_class = if largest_crater_km > planet_radius_km * 0.35 {
        ImpactBasinClass::MegaBasin
    } else if largest_crater_km > 1000.0 {
        ImpactBasinClass::Basin
    } else if largest_crater_km > 50.0 {
        ImpactBasinClass::Crater
    } else {
        ImpactBasinClass::None
    };

    let major_basin_count = match crater_density {
        CraterDensity::Pristine => 0,
        CraterDensity::Light => 1,
        CraterDensity::Moderate => 2 + u8::from(largest_basin_class >= ImpactBasinClass::Basin),
        CraterDensity::Heavy => 4 + u8::from(largest_basin_class >= ImpactBasinClass::Basin),
        CraterDensity::Saturated => {
            6 + u8::from(largest_basin_class >= ImpactBasinClass::MegaBasin)
        }
    };

    let ejecta_blanket_fraction = match crater_density {
        CraterDensity::Pristine => 0.01,
        CraterDensity::Light => 0.04,
        CraterDensity::Moderate => 0.08,
        CraterDensity::Heavy => 0.14,
        CraterDensity::Saturated => 0.22,
    };

    ImpactHistory {
        surface_age_class,
        resurfacing_driver,
        major_basin_count,
        largest_basin_class,
        ejecta_blanket_fraction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::IceCapLocation;

    fn context(radius: f64, tidal_heating: u32) -> PlanetSimulationInput {
        PlanetSimulationInput {
            body_radius_earth: radius,
            tidal_heating,
            ..Default::default()
        }
    }

    #[test]
    fn old_and_young_surfaces_diverge() {
        let ancient = generate_impact_history(
            &context(0.27, 0),
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Rock,
            None,
            0.0,
            0.0,
            0.0,
            0.0,
            None,
            None,
        );
        let young = generate_impact_history(
            &context(1.0, 0),
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Terrestrial,
            None,
            10.0,
            1.0,
            60.0,
            45.0,
            None,
            None,
        );
        assert!(young.surface_age_class < ancient.surface_age_class);
    }

    #[test]
    fn icy_tidally_heated_world_prefers_cryovolcanic_resurfacing() {
        let glaciation = GlaciationState {
            ice_coverage_fraction: 0.8,
            in_glacial_period: true,
            snowball_state: false,
            ice_cap_location: IceCapLocation::Global,
        };
        let history = generate_impact_history(
            &context(0.35, 9),
            TelluricBodyComposition::Icy,
            CelestialBodyWorldType::Ice,
            None,
            20.0,
            0.0,
            5.0,
            8.0,
            Some(&glaciation),
            None,
        );
        assert_eq!(history.resurfacing_driver, ResurfacingDriver::Cryovolcanic);
    }
}
