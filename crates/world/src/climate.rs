pub use crate::types::{
    CelestialBodyWorldType, ChemicalComponent, ClimateRegime, ClimateRegulation, GlaciationState,
    GreenhouseEffect, IceCapLocation, LifeLevel, MagneticFieldStrength, SkyAppearance, SkyColor,
    TelluricBodyComposition, TerminatorHabitability, TidallyLockedClimate,
    TidallyLockedClimateRegime, WindProfile,
};

use crate::atmosphere::AtmosphereProfile;
use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct ClimateProfile {
    pub greenhouse: Option<GreenhouseEffect>,
    pub climate_regulation: Option<ClimateRegulation>,
    pub tidally_locked_climate: Option<TidallyLockedClimate>,
    pub sky: Option<SkyAppearance>,
    pub wind: Option<WindProfile>,
    pub glaciation: Option<GlaciationState>,
}

#[allow(clippy::too_many_arguments)]
pub fn generate_climate_profile(
    context: &PlanetSimulationInput,
    atmosphere: &AtmosphereProfile,
    atmospheric_pressure: f32,
    atmospheric_composition: &[(f32, ChemicalComponent)],
    hydrosphere: f32,
    ice_over_water: f32,
    ice_over_land: f32,
    volcanism: f32,
    tectonic_activity: f32,
    body_type: TelluricBodyComposition,
    _world_type: CelestialBodyWorldType,
    life_level: LifeLevel,
) -> ClimateProfile {
    let has_atmosphere = atmospheric_pressure > 0.01;
    let has_oxygen = atmospheric_composition
        .iter()
        .any(|(f, c)| *c == ChemicalComponent::Oxygen && *f > 0.1);
    let has_co2_high = atmospheric_composition
        .iter()
        .any(|(f, c)| *c == ChemicalComponent::CarbonDioxide && *f > 0.05);
    let has_so2 = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::SulfurDioxide);
    let has_methane = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Methane);
    let has_cl2 = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Chlorine);

    let land_fraction = (100.0 - hydrosphere).max(0.0) / 100.0;
    let greenhouse = generate_greenhouse(
        context.blackbody_temp_k,
        atmospheric_pressure,
        atmospheric_composition,
        hydrosphere,
        ice_over_land,
    );
    let wind = generate_wind_profile(context, atmosphere, atmospheric_pressure);
    let glaciation = generate_glaciation(context, hydrosphere, ice_over_water, ice_over_land);
    let climate_regulation = generate_climate_regulation(
        context,
        atmospheric_pressure,
        hydrosphere,
        land_fraction,
        volcanism,
        tectonic_activity,
        life_level,
        greenhouse.as_ref(),
        glaciation.as_ref(),
        wind.as_ref(),
    );
    let tidally_locked_climate = generate_tidally_locked_climate(
        context,
        has_atmosphere,
        atmospheric_pressure,
        hydrosphere,
        greenhouse.as_ref(),
        glaciation.as_ref(),
        wind.as_ref(),
    );
    let sky = generate_sky_appearance(
        atmospheric_pressure,
        body_type,
        has_oxygen,
        has_co2_high,
        has_so2,
        has_methane,
        has_cl2,
        volcanism,
    );

    ClimateProfile {
        greenhouse,
        climate_regulation,
        tidally_locked_climate,
        sky,
        wind,
        glaciation,
    }
}

fn generate_greenhouse(
    blackbody_temperature: u32,
    atmospheric_pressure: f32,
    atmospheric_composition: &[(f32, ChemicalComponent)],
    hydrosphere: f32,
    ice_over_land: f32,
) -> Option<GreenhouseEffect> {
    if atmospheric_pressure <= 0.01 {
        return None;
    }

    let co2_fraction: f32 = atmospheric_composition
        .iter()
        .filter(|(_, c)| *c == ChemicalComponent::CarbonDioxide)
        .map(|(f, _)| *f)
        .sum();
    let co2_pp = co2_fraction * atmospheric_pressure;
    let ch4_fraction: f32 = atmospheric_composition
        .iter()
        .filter(|(_, c)| *c == ChemicalComponent::Methane)
        .map(|(f, _)| *f)
        .sum();

    let delta_co2 = if co2_pp > 0.0004 {
        (10.0 * (co2_pp / 0.0004).ln()).max(0.0)
    } else {
        0.0
    };
    let ch4_ppm = ch4_fraction * 1_000_000.0;
    let delta_ch4 = if ch4_ppm > 2.0 {
        (0.5 * (ch4_ppm - 2.0).ln().max(0.0)).min(20.0)
    } else {
        0.0
    };
    let base_delta = delta_co2 + delta_ch4;
    let surface_temp_base = blackbody_temperature as f32 + base_delta;
    let h2o_feedback = if surface_temp_base > 300.0 && hydrosphere > 10.0 {
        ((surface_temp_base - 300.0) * 0.5).min(200.0)
    } else {
        0.0
    };
    let delta = base_delta + h2o_feedback;
    let is_runaway =
        (blackbody_temperature as f32 + delta > 500.0 && hydrosphere > 0.0) || h2o_feedback > 150.0;
    let bond_albedo = if hydrosphere > 50.0 {
        0.3
    } else if ice_over_land > 50.0 {
        0.6
    } else {
        0.25
    };

    Some(GreenhouseEffect {
        equilibrium_temp_k: blackbody_temperature as f32,
        surface_temp_k: blackbody_temperature as f32 + delta,
        greenhouse_delta_k: delta,
        bond_albedo,
        is_runaway,
    })
}

fn generate_wind_profile(
    context: &PlanetSimulationInput,
    atmosphere: &AtmosphereProfile,
    atmospheric_pressure: f32,
) -> Option<WindProfile> {
    if atmospheric_pressure <= 0.01 {
        return None;
    }

    let omega = if context.orbit.rotation_period_days.abs() > 0.01 {
        2.0 * std::f32::consts::PI / (context.orbit.rotation_period_days * 86_400.0)
    } else {
        0.0
    };
    let r_planet_m = context.body_radius_earth as f32 * 6.371e6;
    let scale_height_m = atmosphere
        .atmospheric_layers
        .as_ref()
        .map_or(8_500.0, |l| l.scale_height_km * 1_000.0);
    let delta_t = 50.0_f32;
    let rossby = if omega > 1e-8 && r_planet_m > 1_000.0 {
        (context.gravity_g * scale_height_m * delta_t) / (omega * omega * r_planet_m * r_planet_m)
    } else {
        100.0
    };
    let cells = if rossby > 10.0 {
        1_u8
    } else if rossby > 1.0 {
        2
    } else if rossby > 0.1 {
        3
    } else if rossby > 0.01 {
        5
    } else {
        8
    };
    let is_slow = rossby > 5.0;
    let base_wind =
        (cells as f32 * 3.0 + atmospheric_pressure.sqrt() * 5.0 + context.gravity_g * 2.0)
            .min(200.0);
    let internal_boost = if atmospheric_pressure > 10.0 && context.tidal_heating > 0 {
        50.0
    } else {
        0.0
    };
    let max_wind =
        (base_wind * (2.0 + if is_slow { 3.0 } else { 1.0 }) + internal_boost).min(600.0);

    Some(WindProfile {
        mean_surface_wind_ms: base_wind,
        max_wind_ms: max_wind,
        superrotation: is_slow && atmospheric_pressure > 0.5,
    })
}

fn generate_glaciation(
    context: &PlanetSimulationInput,
    hydrosphere: f32,
    ice_over_water: f32,
    ice_over_land: f32,
) -> Option<GlaciationState> {
    let ice_fraction = (ice_over_water * hydrosphere / 100.0
        + ice_over_land * (100.0 - hydrosphere) / 100.0)
        / 100.0;
    let snowball = ice_fraction > 0.9;
    let ice_cap_location = if context.orbit.tidally_locked {
        IceCapLocation::DarkSide
    } else if snowball {
        IceCapLocation::Global
    } else if context.orbit.axial_tilt_deg > 40.0 {
        IceCapLocation::Equatorial
    } else if ice_fraction > 0.01 {
        IceCapLocation::Polar
    } else {
        IceCapLocation::None
    };

    if ice_fraction > 0.01 || snowball {
        Some(GlaciationState {
            ice_coverage_fraction: ice_fraction,
            in_glacial_period: ice_fraction > 0.2,
            snowball_state: snowball,
            ice_cap_location,
        })
    } else {
        None
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_climate_regulation(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    hydrosphere: f32,
    land_fraction: f32,
    volcanism: f32,
    tectonic_activity: f32,
    life_level: LifeLevel,
    greenhouse: Option<&GreenhouseEffect>,
    glaciation: Option<&GlaciationState>,
    wind: Option<&WindProfile>,
) -> Option<ClimateRegulation> {
    let outgassing =
        (volcanism * 0.8 + tectonic_activity * 0.5 + context.tidal_heating as f32 * 1.4)
            .clamp(0.0, 100.0);
    let hydrology_factor = if hydrosphere > 0.0 {
        (hydrosphere / 100.0).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let weathering_temp_factor = if (250..=330).contains(&context.blackbody_temp_k) {
        1.0
    } else if context.blackbody_temp_k < 250 {
        (context.blackbody_temp_k as f32 / 250.0).clamp(0.1, 0.95)
    } else {
        (330.0 / context.blackbody_temp_k as f32).clamp(0.2, 0.95)
    };
    let biology_factor = if life_level.as_u8() >= LifeLevel::PlantLike.as_u8() {
        1.15
    } else {
        1.0
    };
    let weathering_drawdown = (100.0
        * hydrology_factor
        * (0.25 + land_fraction)
        * atmospheric_pressure.clamp(0.05, 3.0).sqrt()
        * weathering_temp_factor
        * (1.0 + tectonic_activity / 120.0)
        * biology_factor)
        .clamp(0.0, 100.0);
    let regulation_strength = if outgassing + weathering_drawdown > 0.1 {
        (1.0 - ((outgassing - weathering_drawdown).abs() / (outgassing + weathering_drawdown)))
            .clamp(0.0, 1.0)
    } else {
        0.0
    };
    let estimated_feedback_k = ((outgassing - weathering_drawdown) * 0.35).clamp(-35.0, 45.0);

    let regime = if greenhouse.is_some_and(|g| g.is_runaway) {
        ClimateRegime::RunawayGreenhouse
    } else if glaciation.is_some_and(|g| g.snowball_state) {
        ClimateRegime::SnowballLocked
    } else if context.orbit.tidally_locked && wind.is_some_and(|w| w.mean_surface_wind_ms > 20.0) {
        ClimateRegime::TidallyModerated
    } else if hydrosphere > 5.0
        && atmospheric_pressure > 0.1
        && volcanism > 5.0
        && tectonic_activity > 8.0
        && weathering_drawdown > 5.0
    {
        ClimateRegime::CarbonateSilicate
    } else if regulation_strength > 0.55 {
        ClimateRegime::WeatheringBalanced
    } else {
        ClimateRegime::Unbuffered
    };

    Some(ClimateRegulation {
        regime,
        volcanic_outgassing_index: outgassing,
        weathering_drawdown_index: weathering_drawdown,
        regulation_strength,
        estimated_feedback_k,
    })
}

fn generate_tidally_locked_climate(
    context: &PlanetSimulationInput,
    has_atmosphere: bool,
    atmospheric_pressure: f32,
    hydrosphere: f32,
    greenhouse: Option<&GreenhouseEffect>,
    glaciation: Option<&GlaciationState>,
    wind: Option<&WindProfile>,
) -> Option<TidallyLockedClimate> {
    if !context.orbit.tidally_locked {
        return None;
    }

    let atmospheric_coupling = atmospheric_pressure.clamp(0.0, 3.0) / 3.0;
    let ocean_coupling = (hydrosphere / 100.0).clamp(0.0, 1.0);
    let wind_coupling = wind
        .as_ref()
        .map_or(0.0, |w| (w.mean_surface_wind_ms / 80.0).clamp(0.0, 1.0));
    let superrotation_bonus = wind
        .as_ref()
        .map_or(0.0, |w| if w.superrotation { 0.15 } else { 0.0 });
    let greenhouse_buffer = greenhouse
        .as_ref()
        .map_or(0.0, |g| (g.greenhouse_delta_k / 120.0).clamp(0.0, 0.2));

    let heat_redistribution_efficiency = (atmospheric_coupling * 0.45
        + ocean_coupling * 0.2
        + wind_coupling * 0.2
        + superrotation_bonus
        + greenhouse_buffer)
        .clamp(0.0, 1.0);
    let day_night_temperature_delta_k = ((context.blackbody_temp_k as f32 * 0.85)
        * (1.0 - 0.82 * heat_redistribution_efficiency))
        .clamp(8.0, 500.0);
    let substellar_cloud_fraction =
        if has_atmosphere && hydrosphere > 5.0 && (220..=400).contains(&context.blackbody_temp_k) {
            (0.2 + ocean_coupling * 0.35 + wind_coupling * 0.25 + atmospheric_coupling * 0.2)
                .clamp(0.0, 1.0)
        } else {
            0.0
        };
    let nightside_cold_traps = !has_atmosphere
        || atmospheric_pressure < 0.08
        || heat_redistribution_efficiency < 0.32
        || glaciation
            .as_ref()
            .is_some_and(|g| g.ice_cap_location == IceCapLocation::DarkSide);
    let terminator_habitability = if !has_atmosphere
        || greenhouse.as_ref().is_some_and(|g| g.is_runaway)
        || context.blackbody_temp_k > 420
    {
        TerminatorHabitability::None
    } else if (240..=320).contains(&context.blackbody_temp_k)
        && (0.3..=0.78).contains(&heat_redistribution_efficiency)
    {
        TerminatorHabitability::Broad
    } else if (220..=360).contains(&context.blackbody_temp_k)
        && heat_redistribution_efficiency >= 0.18
    {
        TerminatorHabitability::Local
    } else if heat_redistribution_efficiency >= 0.1 {
        TerminatorHabitability::Marginal
    } else {
        TerminatorHabitability::None
    };
    let regime = if !has_atmosphere || atmospheric_pressure < 0.02 {
        TidallyLockedClimateRegime::AtmosphereCollapsed
    } else if heat_redistribution_efficiency > 0.72
        && wind.as_ref().is_some_and(|w| w.superrotation)
    {
        TidallyLockedClimateRegime::UniformSuperrotating
    } else if hydrosphere > 15.0
        && substellar_cloud_fraction > 0.45
        && terminator_habitability >= TerminatorHabitability::Local
    {
        TidallyLockedClimateRegime::EyeballWorld
    } else if nightside_cold_traps {
        TidallyLockedClimateRegime::NightsideColdTrap
    } else {
        TidallyLockedClimateRegime::TerminatorBelt
    };

    Some(TidallyLockedClimate {
        regime,
        heat_redistribution_efficiency,
        day_night_temperature_delta_k,
        terminator_habitability,
        nightside_cold_traps,
        substellar_cloud_fraction,
    })
}

fn generate_sky_appearance(
    atmospheric_pressure: f32,
    body_type: TelluricBodyComposition,
    has_oxygen: bool,
    has_co2_high: bool,
    has_so2: bool,
    has_methane: bool,
    has_cl2: bool,
    volcanism: f32,
) -> Option<SkyAppearance> {
    if atmospheric_pressure <= 0.001 {
        return Some(SkyAppearance {
            daytime_color: SkyColor::Black,
            sunset_color: SkyColor::Black,
            daytime_stars_visible: true,
        });
    }

    let has_dust = body_type == TelluricBodyComposition::Rocky && atmospheric_pressure < 0.05;
    let has_ch4_thick = has_methane && atmospheric_pressure > 5.0;
    let very_thin_dust = has_dust && atmospheric_pressure < 0.003;
    let daytime_color = if has_ch4_thick {
        SkyColor::DeepBlue
    } else if has_dust && volcanism > 50.0 {
        SkyColor::Red
    } else if has_dust {
        SkyColor::Butterscotch
    } else if very_thin_dust {
        SkyColor::Pink
    } else if has_cl2 {
        SkyColor::Green
    } else if has_co2_high && atmospheric_pressure > 10.0 {
        SkyColor::Amber
    } else if has_so2 {
        SkyColor::Yellow
    } else if has_oxygen && atmospheric_pressure > 0.3 {
        SkyColor::Blue
    } else if atmospheric_pressure < 0.2 {
        SkyColor::PaleBlue
    } else if atmospheric_pressure > 5.0 {
        SkyColor::White
    } else {
        SkyColor::PaleBlue
    };
    let sunset_color = if has_dust {
        SkyColor::Blue
    } else if daytime_color == SkyColor::Blue {
        SkyColor::Red
    } else if daytime_color == SkyColor::DeepBlue {
        SkyColor::Blue
    } else {
        SkyColor::Yellow
    };

    Some(SkyAppearance {
        daytime_color,
        sunset_color,
        daytime_stars_visible: atmospheric_pressure < 0.05,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atmosphere::generate_atmosphere_profile;
    fn context(temp: u32) -> PlanetSimulationInput {
        PlanetSimulationInput {
            orbit: OrbitContext {
                rotation_period_days: 1.0,
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            gravity_g: 1.0,
            body_radius_earth: 1.0,
            blackbody_temp_k: temp,
            ..Default::default()
        }
    }

    #[test]
    fn earth_like_climate_is_buffered() {
        let composition = vec![
            (0.78, ChemicalComponent::Nitrogen),
            (0.21, ChemicalComponent::Oxygen),
            (0.01, ChemicalComponent::Argon),
        ];
        let ctx = context(288);
        let atmosphere = generate_atmosphere_profile(
            &ctx,
            1.0,
            &composition,
            MagneticFieldStrength::Strong,
            LifeLevel::Sentient,
        );
        let climate = generate_climate_profile(
            &ctx,
            &atmosphere,
            1.0,
            &composition,
            71.0,
            5.0,
            8.0,
            30.0,
            35.0,
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Terrestrial,
            LifeLevel::Sentient,
        );

        assert!(climate.greenhouse.is_some());
        assert!(matches!(
            climate.climate_regulation.unwrap().regime,
            ClimateRegime::CarbonateSilicate | ClimateRegime::WeatheringBalanced
        ));
        assert_eq!(climate.sky.unwrap().daytime_color, SkyColor::Blue);
    }

    #[test]
    fn tidally_locked_ocean_world_gets_terminator_state() {
        let composition = vec![
            (0.8, ChemicalComponent::Nitrogen),
            (0.15, ChemicalComponent::CarbonDioxide),
            (0.05, ChemicalComponent::Methane),
        ];
        let mut ctx = context(290);
        ctx.orbit.tidally_locked = true;
        ctx.orbit.rotation_period_days = 24.0;
        let atmosphere = generate_atmosphere_profile(
            &ctx,
            1.4,
            &composition,
            MagneticFieldStrength::Moderate,
            LifeLevel::None,
        );
        let climate = generate_climate_profile(
            &ctx,
            &atmosphere,
            1.4,
            &composition,
            65.0,
            0.0,
            5.0,
            12.0,
            18.0,
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Ocean,
            LifeLevel::None,
        );

        let tidal = climate.tidally_locked_climate.unwrap();
        assert!(matches!(
            tidal.regime,
            TidallyLockedClimateRegime::EyeballWorld
                | TidallyLockedClimateRegime::TerminatorBelt
                | TidallyLockedClimateRegime::UniformSuperrotating
        ));
        assert!(tidal.heat_redistribution_efficiency > 0.18);
    }

    #[test]
    fn airless_tidally_locked_world_collapses() {
        let composition = vec![(1.0, ChemicalComponent::CarbonDioxide)];
        let mut ctx = context(260);
        ctx.orbit.tidally_locked = true;
        ctx.orbit.rotation_period_days = 58.0;
        ctx.gravity_g = 0.3;
        ctx.body_radius_earth = 0.4;
        let atmosphere = generate_atmosphere_profile(
            &ctx,
            0.005,
            &composition,
            MagneticFieldStrength::None,
            LifeLevel::None,
        );
        let climate = generate_climate_profile(
            &ctx,
            &atmosphere,
            0.005,
            &composition,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Terrestrial,
            LifeLevel::None,
        );

        assert_eq!(
            climate.tidally_locked_climate.unwrap().regime,
            TidallyLockedClimateRegime::AtmosphereCollapsed
        );
    }
}
