pub use crate::types::{
    CelestialBodyWorldType, CryovolcanicTransport, EnclosedOceanHabitability, PlumeActivity,
    PlanetSimulationInput, SubsurfaceOcean, TelluricBodyComposition,
};

pub fn generate_subsurface_ocean(
    context: &PlanetSimulationInput,
    body_type: TelluricBodyComposition,
    world_type: CelestialBodyWorldType,
    hydrosphere: f32,
    volcanism: f32,
    tectonic_activity: f32,
) -> Option<SubsurfaceOcean> {
    let candidate_icy = body_type == TelluricBodyComposition::Icy
        || matches!(world_type, CelestialBodyWorldType::Ice | CelestialBodyWorldType::Ammonia);
    let enough_internal_heat = context.tidal_heating > 0 || volcanism > 5.0 || tectonic_activity > 8.0;
    let present =
        candidate_icy && context.blackbody_temp_k <= 240 && hydrosphere > 0.0 && enough_internal_heat;

    if !present {
        return None;
    }

    let hydrothermal_support = volcanism > 10.0 && hydrosphere > 20.0;
    let thermal_factor =
        (context.tidal_heating as f32 * 3.5 + volcanism * 0.4 + tectonic_activity * 0.25)
            .clamp(0.0, 100.0);
    let ice_shell_thickness_km = (2.0
        + (240.0 - context.blackbody_temp_k as f32).max(0.0) * 0.18
        + context.gravity_g * 10.0
        - thermal_factor * 0.22)
        .clamp(1.0, 80.0);
    let ocean_depth_km = (hydrosphere * 0.22
        + context.tidal_heating as f32 * 0.9
        + if world_type == CelestialBodyWorldType::Ammonia { 6.0 } else { 0.0 }
        + if context.blackbody_temp_k < 180 { 4.0 } else { 0.0 })
        .clamp(1.0, 120.0);

    let plume_activity = if context.tidal_heating > 12 || volcanism > 40.0 {
        PlumeActivity::Extreme
    } else if context.tidal_heating > 5 || volcanism > 20.0 {
        PlumeActivity::Persistent
    } else if context.tidal_heating > 1 || tectonic_activity > 10.0 {
        PlumeActivity::Occasional
    } else {
        PlumeActivity::None
    };

    let transport_efficiency =
        if plume_activity >= PlumeActivity::Persistent && ice_shell_thickness_km < 15.0 {
            CryovolcanicTransport::EfficientExchange
        } else if plume_activity >= PlumeActivity::Occasional && ice_shell_thickness_km < 30.0 {
            CryovolcanicTransport::EpisodicExchange
        } else if tectonic_activity > 8.0 || context.tidal_heating > 1 {
            CryovolcanicTransport::Fractured
        } else {
            CryovolcanicTransport::Trapped
        };

    let habitability = if hydrosphere > 20.0 {
        if context.tidal_heating > 4 && ocean_depth_km > 5.0 && hydrothermal_support {
            EnclosedOceanHabitability::HighPotential
        } else if context.tidal_heating > 1 || volcanism > 10.0 {
            EnclosedOceanHabitability::Chemotrophic
        } else {
            EnclosedOceanHabitability::Marginal
        }
    } else {
        EnclosedOceanHabitability::Sterile
    };

    Some(SubsurfaceOcean {
        present: true,
        ice_shell_thickness_km,
        ocean_depth_km,
        plume_activity,
        transport_efficiency,
        habitability,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(temp: u32, gravity: f32, tidal_heating: u32) -> PlanetSimulationInput {
        PlanetSimulationInput {
            blackbody_temp_k: temp,
            gravity_g: gravity,
            tidal_heating,
            ..Default::default()
        }
    }

    #[test]
    fn icy_moon_gets_subsurface_ocean_state() {
        let ocean = generate_subsurface_ocean(
            &context(115, 0.12, 9),
            TelluricBodyComposition::Icy,
            CelestialBodyWorldType::Ice,
            20.0,
            5.0,
            8.0,
        )
        .unwrap();
        assert!(ocean.present);
        assert!(ocean.ocean_depth_km > 1.0);
        assert!(ocean.plume_activity >= PlumeActivity::Persistent);
        assert!(ocean.transport_efficiency >= CryovolcanicTransport::EpisodicExchange);
    }

    #[test]
    fn frozen_dry_body_has_no_subsurface_ocean() {
        let ocean = generate_subsurface_ocean(
            &context(90, 0.08, 0),
            TelluricBodyComposition::Icy,
            CelestialBodyWorldType::Ice,
            0.0,
            0.0,
            0.0,
        );
        assert!(ocean.is_none());
    }
}
