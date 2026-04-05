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

// ---------------------------------------------------------------------------
// Grid-level temperature layer
//
// Produces `temperature_c`, `temperature_summer_c`, `temperature_winter_c`
// for a `SurfaceGrid` from latitude insolation (cos-weighted, modulated by
// axial tilt), elevation lapse rate, and continentality via distance-to-
// ocean BFS.
// ---------------------------------------------------------------------------

use crate::grid::SurfaceGrid;

/// Environmental lapse rate in °C per 1000 m of elevation gain.
const LAPSE_RATE_C_PER_KM: f32 = 6.5;

/// Populate the temperature layers on an already-geologised grid.
///
/// `greenhouse_delta_k` is the atmospheric warming above the blackbody
/// equilibrium (computed by `generate_greenhouse_effect`). Pass 0.0 for
/// airless bodies.
pub fn generate_temperature(
    context: &PlanetSimulationInput,
    greenhouse_delta_k: f32,
    grid: &mut SurfaceGrid,
) {
    let mean_c = context.blackbody_temp_k as f32 + greenhouse_delta_k - 273.15;
    let delta = pole_to_equator_delta(context.orbit.axial_tilt_deg);

    // Stage 1: latitude insolation + elevation lapse.
    for r in 0..grid.height {
        let lat_deg = grid.row_latitude(r);
        let lat_profile = latitude_profile(lat_deg);
        let base_c = mean_c + 0.5 * delta * lat_profile;
        for c in 0..grid.width {
            let idx = grid.idx(c, r);
            let is_ocean = grid.layers.is_ocean[idx];
            if is_ocean {
                // SST: latitude-driven only (ocean gyres added later).
                grid.layers.temperature_c[idx] = base_c;
                grid.layers.sea_surface_temp_c[idx] = base_c;
            } else {
                let elev = grid.layers.elevation_m[idx];
                let sea_level = grid.sea_level_m;
                let height_km = ((elev - sea_level) / 1000.0).max(0.0);
                grid.layers.temperature_c[idx] = base_c - height_km * LAPSE_RATE_C_PER_KM;
            }
        }
    }

    // Stage 2: continentality. Multi-source BFS from ocean tiles.
    let distance_to_ocean = compute_distance_to_ocean(grid);

    let max_dist = *distance_to_ocean.iter().max().unwrap_or(&1) as f32;
    for (idx, &dist) in distance_to_ocean.iter().enumerate() {
        let mean = grid.layers.temperature_c[idx];
        let swing = if grid.layers.is_ocean[idx] {
            // Ocean tiles have strong thermal inertia: minimal annual swing.
            2.0
        } else {
            // Land tiles: 3 °C at coast → 15 °C deep interior.
            let norm = dist as f32 / max_dist.max(1.0);
            3.0 + norm * 12.0
        };
        grid.layers.temperature_summer_c[idx] = mean + swing;
        grid.layers.temperature_winter_c[idx] = mean - swing;
    }
}

/// Pole-to-equator mean annual temperature delta, in °C.
///
/// Empirically calibrated so Earth (23° tilt) yields a ~55 °C gradient.
/// High-tilt worlds invert: at ~73° tilt the poles become warmer than the
/// equator in annual mean because they receive 24-hour sun during summer.
fn pole_to_equator_delta(tilt_deg: f32) -> f32 {
    let tilt = tilt_deg.abs();
    if tilt < 54.0 {
        // Near-linear decrease: 60 °C at tilt=0 → 49 °C at tilt=54°.
        60.0 - 0.2 * tilt
    } else {
        // Above ~54°, the summer pole receives more annual insolation than
        // the equator. Gradient crosses zero around 73° and inverts strongly
        // past that.
        49.0 - (tilt - 54.0) * 2.5
    }
}

/// Latitude profile returning +1 at equator and −1 at poles. Smooth.
fn latitude_profile(lat_deg: f32) -> f32 {
    ((2.0 * lat_deg).to_radians()).cos()
}

/// Multi-source BFS distance (in tile-steps) from any ocean tile. Land
/// tiles get positive distances; ocean tiles get 0.
fn compute_distance_to_ocean(grid: &SurfaceGrid) -> Vec<u16> {
    let w = grid.width as usize;
    let h = grid.height as usize;
    let n = w * h;
    let mut dist = vec![u16::MAX; n];
    let mut queue = std::collections::VecDeque::new();

    for (idx, &ocean) in grid.layers.is_ocean.iter().enumerate() {
        if ocean {
            dist[idx] = 0;
            queue.push_back(idx);
        }
    }

    while let Some(idx) = queue.pop_front() {
        let d = dist[idx];
        let r = idx / w;
        let c = idx % w;
        // 4-connected neighbours with longitude wrap.
        let neighbours = [
            (c, r.saturating_sub(1)),
            (c, (r + 1).min(h - 1)),
            ((c + w - 1) % w, r),
            ((c + 1) % w, r),
        ];
        for (nc, nr) in neighbours {
            let nidx = nr * w + nc;
            if dist[nidx] > d + 1 {
                dist[nidx] = d + 1;
                queue.push_back(nidx);
            }
        }
    }

    // Any land tile that couldn't reach an ocean gets a large finite value.
    for d in dist.iter_mut() {
        if *d == u16::MAX {
            *d = 0;
        }
    }
    dist
}

// ---------------------------------------------------------------------------
// Grid-level wind layer
//
// Assigns prevailing wind direction and speed per tile from Hadley cells
// (derived from axial tilt), latitude bands (trade easterlies, westerlies,
// polar easterlies), and atmospheric-pressure scaling. Direction follows
// meteorological convention: the bearing *from which* the wind blows.
// ---------------------------------------------------------------------------

/// Populate wind layers on an already-geologised grid.
pub fn generate_wind(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    grid: &mut SurfaceGrid,
) {
    let tilt = context.orbit.axial_tilt_deg.abs();
    let cells = hadley_cells_for_tilt(tilt);
    let pressure_factor = pressure_wind_factor(atmospheric_pressure);

    for r in 0..grid.height {
        let lat_deg = grid.row_latitude(r);
        let direction = band_direction(lat_deg, cells);
        let speed = band_speed(lat_deg, cells) * pressure_factor;
        for c in 0..grid.width {
            let idx = grid.idx(c, r);
            grid.layers.wind_direction_deg[idx] = direction;
            grid.layers.wind_speed_ms[idx] = speed;
        }
    }
}

/// Hadley-cell count per hemisphere, matching the hydrology module.
fn hadley_cells_for_tilt(tilt_deg: f32) -> u8 {
    if tilt_deg < 8.0 {
        1
    } else if tilt_deg < 40.0 {
        3
    } else if tilt_deg < 54.0 {
        2
    } else {
        1
    }
}

/// Prevailing wind direction (meteorological convention) at a given
/// latitude, for N hemisphere cells.
fn band_direction(lat_deg: f32, cells: u8) -> f32 {
    let abs_lat = lat_deg.abs();
    let is_nh = lat_deg >= 0.0;

    match cells {
        3 => {
            if abs_lat < 30.0 {
                // Trade winds: from NE (045°) in NH, from SE (135°) in SH.
                if is_nh {
                    45.0
                } else {
                    135.0
                }
            } else if abs_lat < 60.0 {
                // Westerlies: from SW (225°) in NH, from NW (315°) in SH.
                if is_nh {
                    225.0
                } else {
                    315.0
                }
            } else {
                // Polar easterlies: from NE (045°) in NH, from SE (135°) in SH.
                if is_nh {
                    45.0
                } else {
                    135.0
                }
            }
        }
        2 => {
            if abs_lat < 45.0 {
                if is_nh {
                    45.0
                } else {
                    135.0
                }
            } else if is_nh {
                45.0
            } else {
                135.0
            }
        }
        // 1-cell and chaotic (>54° tilt) — easterlies everywhere.
        _ => {
            if is_nh {
                45.0
            } else {
                135.0
            }
        }
    }
}

/// Base surface wind speed in m/s by latitude. Peaks in the westerlies
/// (30-60°) and drops in the doldrums (ITCZ) and horse latitudes.
fn band_speed(lat_deg: f32, cells: u8) -> f32 {
    let abs_lat = lat_deg.abs();
    match cells {
        3 => {
            if abs_lat < 5.0 {
                2.0 // doldrums / ITCZ
            } else if abs_lat < 30.0 {
                7.0 // trade winds
            } else if abs_lat < 35.0 {
                3.0 // horse latitudes
            } else if abs_lat < 60.0 {
                12.0 // westerlies / roaring forties
            } else if abs_lat < 65.0 {
                8.0 // polar front (storm belt)
            } else if abs_lat < 85.0 {
                5.0 // polar easterlies
            } else {
                2.0 // polar high
            }
        }
        2 => {
            if abs_lat < 5.0 {
                2.0
            } else if abs_lat < 45.0 {
                8.0
            } else if abs_lat < 50.0 {
                4.0
            } else {
                5.0
            }
        }
        1 => 5.0,
        _ => 3.0,
    }
}

/// Wind speed modifier for atmospheric pressure. Thin atmospheres carry
/// less momentum; thick atmospheres modestly boost bulk winds.
fn pressure_wind_factor(atmospheric_pressure: f32) -> f32 {
    if atmospheric_pressure < 1.0 {
        atmospheric_pressure.clamp(0.05, 1.0)
    } else {
        1.0 + (atmospheric_pressure - 1.0).sqrt() * 0.3
    }
}

#[cfg(test)]
mod grid_tests {
    use super::*;
    use crate::geology::generate_geology;
    use crate::grid::GridResolution;

    fn earth_like_input() -> PlanetSimulationInput {
        PlanetSimulationInput {
            body_id: 7,
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
        }
    }

    fn earth_grid() -> SurfaceGrid {
        let mut g = generate_geology(&earth_like_input(), 71.0, GridResolution::Fast, "earth");
        generate_temperature(&earth_like_input(), 33.0, &mut g);
        g
    }

    #[test]
    fn equator_is_warmer_than_poles() {
        let g = earth_grid();
        let equator_row = g.height / 2;
        let pole_row = 1u16;
        let mut eq_sum = 0.0f32;
        let mut pole_sum = 0.0f32;
        for c in 0..g.width {
            eq_sum += g.layers.temperature_c[g.idx(c, equator_row)];
            pole_sum += g.layers.temperature_c[g.idx(c, pole_row)];
        }
        let eq_mean = eq_sum / g.width as f32;
        let pole_mean = pole_sum / g.width as f32;
        assert!(
            eq_mean > pole_mean + 20.0,
            "equator {} not > pole {} by 20°C",
            eq_mean,
            pole_mean
        );
    }

    #[test]
    fn earth_mean_surface_temp_is_reasonable() {
        let g = earth_grid();
        let mean: f32 = g.layers.temperature_c.iter().sum::<f32>() / g.tile_count() as f32;
        assert!(
            (-5.0..=30.0).contains(&mean),
            "mean surface temp {} out of range",
            mean
        );
    }

    #[test]
    fn low_tilt_has_stronger_gradient() {
        let delta_low = pole_to_equator_delta(1.0);
        let delta_earth = pole_to_equator_delta(23.4);
        assert!(delta_low > delta_earth);
    }

    #[test]
    fn high_tilt_inverts_gradient() {
        assert!(pole_to_equator_delta(80.0) < 0.0);
    }

    #[test]
    fn elevation_lowers_temperature() {
        let g = earth_grid();
        let mut high_idx = 0usize;
        let mut low_idx = 0usize;
        let mut high_e = f32::NEG_INFINITY;
        let mut low_e = f32::INFINITY;
        let equator_row = g.height / 2;
        for c in 0..g.width {
            let idx = g.idx(c, equator_row);
            if g.layers.is_ocean[idx] {
                continue;
            }
            let e = g.layers.elevation_m[idx];
            if e > high_e {
                high_e = e;
                high_idx = idx;
            }
            if e < low_e {
                low_e = e;
                low_idx = idx;
            }
        }
        if (high_e - low_e) < 500.0 {
            return;
        }
        assert!(g.layers.temperature_c[high_idx] < g.layers.temperature_c[low_idx]);
    }

    #[test]
    fn summer_exceeds_winter_everywhere() {
        let g = earth_grid();
        for idx in 0..g.tile_count() {
            assert!(g.layers.temperature_summer_c[idx] >= g.layers.temperature_winter_c[idx]);
        }
    }

    #[test]
    fn continental_interiors_have_bigger_seasonal_swing() {
        let g = earth_grid();
        let mut ocean_swings = Vec::new();
        let mut land_swings = Vec::new();
        for idx in 0..g.tile_count() {
            let swing = g.layers.temperature_summer_c[idx] - g.layers.temperature_winter_c[idx];
            if g.layers.is_ocean[idx] {
                ocean_swings.push(swing);
            } else {
                land_swings.push(swing);
            }
        }
        let ocean_mean: f32 = ocean_swings.iter().sum::<f32>() / ocean_swings.len() as f32;
        let land_mean: f32 = land_swings.iter().sum::<f32>() / land_swings.len() as f32;
        assert!(land_mean > ocean_mean);
    }

    #[test]
    fn temperature_is_deterministic() {
        let input = earth_like_input();
        let mut a = generate_geology(&input, 71.0, GridResolution::Fast, "det");
        let mut b = generate_geology(&input, 71.0, GridResolution::Fast, "det");
        generate_temperature(&input, 33.0, &mut a);
        generate_temperature(&input, 33.0, &mut b);
        assert_eq!(a.layers.temperature_c, b.layers.temperature_c);
    }

    #[test]
    fn airless_body_uses_raw_blackbody() {
        let mut input = earth_like_input();
        input.blackbody_temp_k = 210;
        let mut g = generate_geology(&input, 0.0, GridResolution::Fast, "mars");
        generate_temperature(&input, 0.0, &mut g);
        let mean: f32 = g.layers.temperature_c.iter().sum::<f32>() / g.tile_count() as f32;
        assert!(mean < -40.0);
    }

    #[test]
    fn hot_venus_like_body_is_hot() {
        let mut input = earth_like_input();
        input.blackbody_temp_k = 232;
        let mut g = generate_geology(&input, 0.0, GridResolution::Fast, "venus");
        generate_temperature(&input, 500.0, &mut g);
        let mean: f32 = g.layers.temperature_c.iter().sum::<f32>() / g.tile_count() as f32;
        assert!(mean > 300.0);
    }

    // --- Wind tests ---

    fn make_wind_grid(input: &PlanetSimulationInput, pressure: f32) -> SurfaceGrid {
        let mut g = generate_geology(input, 71.0, GridResolution::Fast, "wind_test");
        generate_wind(input, pressure, &mut g);
        g
    }

    #[test]
    fn earth_has_three_cells() {
        assert_eq!(hadley_cells_for_tilt(23.4), 3);
    }

    #[test]
    fn low_tilt_has_single_cell() {
        assert_eq!(hadley_cells_for_tilt(2.0), 1);
    }

    #[test]
    fn high_tilt_has_single_chaotic_cell() {
        assert_eq!(hadley_cells_for_tilt(70.0), 1);
    }

    #[test]
    fn nh_trade_winds_blow_from_ne() {
        assert_eq!(band_direction(15.0, 3), 45.0);
    }

    #[test]
    fn sh_trade_winds_blow_from_se() {
        assert_eq!(band_direction(-15.0, 3), 135.0);
    }

    #[test]
    fn nh_westerlies_blow_from_sw() {
        assert_eq!(band_direction(45.0, 3), 225.0);
    }

    #[test]
    fn sh_westerlies_blow_from_nw() {
        assert_eq!(band_direction(-45.0, 3), 315.0);
    }

    #[test]
    fn nh_polar_easterlies_blow_from_ne() {
        assert_eq!(band_direction(75.0, 3), 45.0);
    }

    #[test]
    fn westerlies_are_the_fastest_band() {
        assert!(band_speed(45.0, 3) > band_speed(15.0, 3));
        assert!(band_speed(45.0, 3) > band_speed(75.0, 3));
        assert!(band_speed(45.0, 3) > band_speed(0.0, 3));
    }

    #[test]
    fn thin_atmosphere_weakens_wind() {
        assert!(pressure_wind_factor(0.006) < pressure_wind_factor(1.0));
        assert!(pressure_wind_factor(0.006) < 0.1);
    }

    #[test]
    fn thick_atmosphere_modestly_boosts_wind() {
        let venus = pressure_wind_factor(90.0);
        assert!(venus > 1.0 && venus < 5.0);
    }

    #[test]
    fn every_tile_has_wind_assigned() {
        let g = make_wind_grid(&earth_like_input(), 1.0);
        for idx in 0..g.tile_count() {
            let dir = g.layers.wind_direction_deg[idx];
            let speed = g.layers.wind_speed_ms[idx];
            assert!((0.0..360.0).contains(&dir));
            assert!(speed >= 0.0);
        }
    }

    #[test]
    fn wind_is_latitude_banded() {
        let g = make_wind_grid(&earth_like_input(), 1.0);
        for r in 0..g.height {
            let first = g.layers.wind_direction_deg[g.idx(0, r)];
            for c in 0..g.width {
                assert_eq!(g.layers.wind_direction_deg[g.idx(c, r)], first);
            }
        }
    }

    #[test]
    fn wind_is_deterministic() {
        let a = make_wind_grid(&earth_like_input(), 1.0);
        let b = make_wind_grid(&earth_like_input(), 1.0);
        assert_eq!(a.layers.wind_direction_deg, b.layers.wind_direction_deg);
        assert_eq!(a.layers.wind_speed_ms, b.layers.wind_speed_ms);
    }

    #[test]
    fn airless_body_has_zero_wind_speed() {
        let g = make_wind_grid(&earth_like_input(), 0.0);
        let max_speed = g
            .layers
            .wind_speed_ms
            .iter()
            .cloned()
            .fold(0.0f32, f32::max);
        assert!(max_speed < 1.0);
    }

    #[test]
    fn low_tilt_world_has_uniform_easterlies() {
        let mut input = earth_like_input();
        input.orbit.axial_tilt_deg = 2.0;
        let g = make_wind_grid(&input, 1.0);
        for r in 0..g.height {
            let lat = g.row_latitude(r);
            let dir = g.layers.wind_direction_deg[g.idx(0, r)];
            if lat >= 0.0 {
                assert_eq!(dir, 45.0);
            } else {
                assert_eq!(dir, 135.0);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Grid-level biome classification (Phase 7)
//
// Assigns `BiomeType` via Whittaker lookup (temperature × precipitation) and
// `KoppenClass` via the Köppen-Geiger rule ladder using the grid's summer/
// winter temperatures and annual precipitation.
//
// Elevation overrides apply AFTER climate-based classification:
//   - >4500 m → IceCap / EF (permanent glaciation)
//   - >2500 m → Alpine (thin atmosphere, cold temperatures)
//
// This produces *climate envelopes* — the tile type is a description of
// what kind of vegetation it could support, not an assertion that any
// vegetation actually exists. Life occupancy is computed in the `life`
// crate from these envelopes.
// ---------------------------------------------------------------------------

use crate::types::{BiomeType, KoppenClass};

/// Populate biome + koppen_class layers on a grid that has temperature,
/// precipitation, and geology populated.
pub fn generate_biomes(grid: &mut SurfaceGrid) {
    for idx in 0..grid.tile_count() {
        if grid.layers.is_ocean[idx] {
            grid.layers.biome[idx] = BiomeType::Ocean;
            grid.layers.koppen_class[idx] = KoppenClass::Ocean;
            continue;
        }
        let temp_c = grid.layers.temperature_c[idx];
        let temp_summer = grid.layers.temperature_summer_c[idx];
        let temp_winter = grid.layers.temperature_winter_c[idx];
        let precip = grid.layers.precipitation_mm[idx];
        let elev = grid.layers.elevation_m[idx];
        let sea_level = grid.sea_level_m;
        let height_m = elev - sea_level;

        // Köppen classification first (pure climate).
        grid.layers.koppen_class[idx] = classify_koppen(temp_summer, temp_winter, precip);

        // Whittaker biome.
        let mut biome = classify_biome(temp_c, precip);

        // Elevation overrides.
        if height_m > 4500.0 {
            biome = BiomeType::IceCap;
        } else if height_m > 2500.0 {
            biome = BiomeType::Alpine;
        }
        grid.layers.biome[idx] = biome;
    }
}

/// Whittaker biome classification from annual mean temperature (°C)
/// and annual precipitation (mm).
pub fn classify_biome(temp_c: f32, precipitation_mm: f32) -> BiomeType {
    if temp_c > 20.0 {
        // Tropical
        if precipitation_mm > 2000.0 {
            BiomeType::TropicalForest
        } else if precipitation_mm > 500.0 {
            BiomeType::Savanna
        } else {
            BiomeType::Desert
        }
    } else if temp_c > 5.0 {
        // Temperate
        if precipitation_mm > 750.0 {
            BiomeType::TemperateForest
        } else if precipitation_mm > 250.0 {
            BiomeType::Grassland
        } else {
            BiomeType::Desert
        }
    } else if temp_c > -5.0 {
        // Boreal
        if precipitation_mm > 200.0 {
            BiomeType::Taiga
        } else {
            BiomeType::Grassland
        }
    } else if temp_c > -15.0 {
        BiomeType::Tundra
    } else {
        BiomeType::IceCap
    }
}

/// Simplified Köppen-Geiger classification using summer/winter means and
/// annual precipitation. Ignores seasonal precipitation subtypes since the
/// grid does not currently model monthly rainfall.
pub fn classify_koppen(
    temp_summer_c: f32,
    temp_winter_c: f32,
    precipitation_mm: f32,
) -> KoppenClass {
    // Group A — Tropical: coldest month ≥ 18 °C.
    if temp_winter_c >= 18.0 {
        return if precipitation_mm >= 2000.0 {
            KoppenClass::Af
        } else if precipitation_mm >= 1500.0 {
            KoppenClass::Am
        } else {
            KoppenClass::Aw
        };
    }

    // Group B — Arid: check BEFORE C/D because it overrides them.
    let mean_temp = (temp_summer_c + temp_winter_c) / 2.0;
    // Threshold: 20 × mean_annual_T + 140 (assumes balanced wet/dry seasons).
    let b_threshold = 20.0 * mean_temp + 140.0;
    if b_threshold > 0.0 && precipitation_mm < b_threshold {
        let is_hot = mean_temp > 18.0;
        return if precipitation_mm < b_threshold * 0.5 {
            if is_hot {
                KoppenClass::BWh
            } else {
                KoppenClass::BWk
            }
        } else if is_hot {
            KoppenClass::BSh
        } else {
            KoppenClass::BSk
        };
    }

    // Group E — Polar: warmest month < 10 °C.
    if temp_summer_c < 10.0 {
        return if temp_summer_c < 0.0 {
            KoppenClass::EF
        } else {
            KoppenClass::ET
        };
    }

    // Group C — Temperate: coldest month 0 to 18 °C.
    if (0.0..18.0).contains(&temp_winter_c) {
        return if temp_summer_c >= 22.0 {
            KoppenClass::Cfa
        } else if temp_summer_c >= 10.0 {
            KoppenClass::Cfb
        } else {
            KoppenClass::Cfc
        };
    }

    // Group D — Continental: coldest < 0 °C, warmest ≥ 10 °C.
    // Dfb requires a long warm season (summer ≥ 18 °C as a proxy for
    // ≥4 months > 10 °C); shorter warm seasons fall into Dfc (subarctic).
    if temp_winter_c < -38.0 {
        KoppenClass::Dfd
    } else if temp_summer_c >= 22.0 {
        KoppenClass::Dfa
    } else if temp_summer_c >= 18.0 {
        KoppenClass::Dfb
    } else {
        KoppenClass::Dfc
    }
}

#[cfg(test)]
mod biome_tests {
    use super::*;
    use crate::geology::generate_geology;
    use crate::grid::GridResolution;

    fn earth_like_input() -> PlanetSimulationInput {
        PlanetSimulationInput {
            body_id: 7,
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
        }
    }

    #[test]
    fn tropical_rainforest_classification() {
        assert_eq!(classify_biome(26.0, 2500.0), BiomeType::TropicalForest);
    }

    #[test]
    fn hot_desert_classification() {
        assert_eq!(classify_biome(28.0, 100.0), BiomeType::Desert);
    }

    #[test]
    fn savanna_classification() {
        assert_eq!(classify_biome(25.0, 800.0), BiomeType::Savanna);
    }

    #[test]
    fn temperate_forest_classification() {
        assert_eq!(classify_biome(12.0, 1100.0), BiomeType::TemperateForest);
    }

    #[test]
    fn grassland_classification() {
        assert_eq!(classify_biome(10.0, 400.0), BiomeType::Grassland);
    }

    #[test]
    fn taiga_classification() {
        assert_eq!(classify_biome(-2.0, 500.0), BiomeType::Taiga);
    }

    #[test]
    fn tundra_classification() {
        assert_eq!(classify_biome(-10.0, 200.0), BiomeType::Tundra);
    }

    #[test]
    fn ice_cap_classification() {
        assert_eq!(classify_biome(-25.0, 100.0), BiomeType::IceCap);
    }

    #[test]
    fn koppen_rainforest() {
        assert_eq!(classify_koppen(27.0, 24.0, 2500.0), KoppenClass::Af);
    }

    #[test]
    fn koppen_savanna() {
        assert_eq!(classify_koppen(29.0, 22.0, 900.0), KoppenClass::Aw);
    }

    #[test]
    fn koppen_hot_desert() {
        // Mean 25°C, P < threshold → BWh
        assert_eq!(classify_koppen(35.0, 15.0, 80.0), KoppenClass::BWh);
    }

    #[test]
    fn koppen_cold_desert() {
        // Mean 10°C, low precipitation → BWk
        assert_eq!(classify_koppen(20.0, 0.0, 50.0), KoppenClass::BWk);
    }

    #[test]
    fn koppen_humid_subtropical() {
        assert_eq!(classify_koppen(27.0, 5.0, 1200.0), KoppenClass::Cfa);
    }

    #[test]
    fn koppen_oceanic() {
        assert_eq!(classify_koppen(18.0, 5.0, 1000.0), KoppenClass::Cfb);
    }

    #[test]
    fn koppen_subarctic() {
        assert_eq!(classify_koppen(15.0, -20.0, 400.0), KoppenClass::Dfc);
    }

    #[test]
    fn koppen_tundra() {
        assert_eq!(classify_koppen(5.0, -20.0, 250.0), KoppenClass::ET);
    }

    #[test]
    fn koppen_ice_cap() {
        assert_eq!(classify_koppen(-10.0, -40.0, 200.0), KoppenClass::EF);
    }

    #[test]
    fn ocean_tiles_get_ocean_biome() {
        let input = earth_like_input();
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "biome");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        // Set minimal precipitation for biome classification to run.
        for p in g.layers.precipitation_mm.iter_mut() {
            *p = 800.0;
        }
        generate_biomes(&mut g);
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                assert_eq!(g.layers.biome[idx], BiomeType::Ocean);
                assert_eq!(g.layers.koppen_class[idx], KoppenClass::Ocean);
            }
        }
    }

    #[test]
    fn high_elevation_overrides_to_alpine() {
        let input = earth_like_input();
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "alpine");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        // Force one land tile to high elevation.
        let mut target: Option<usize> = None;
        for idx in 0..g.tile_count() {
            if !g.layers.is_ocean[idx] {
                target = Some(idx);
                break;
            }
        }
        let idx = target.unwrap();
        g.layers.elevation_m[idx] = g.sea_level_m + 3000.0;
        g.layers.temperature_c[idx] = 5.0;
        g.layers.precipitation_mm[idx] = 800.0;
        generate_biomes(&mut g);
        assert_eq!(g.layers.biome[idx], BiomeType::Alpine);
    }

    #[test]
    fn extreme_elevation_overrides_to_icecap() {
        let input = earth_like_input();
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "icecap");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        let mut target: Option<usize> = None;
        for idx in 0..g.tile_count() {
            if !g.layers.is_ocean[idx] {
                target = Some(idx);
                break;
            }
        }
        let idx = target.unwrap();
        g.layers.elevation_m[idx] = g.sea_level_m + 5000.0;
        g.layers.temperature_c[idx] = 5.0;
        g.layers.precipitation_mm[idx] = 800.0;
        generate_biomes(&mut g);
        assert_eq!(g.layers.biome[idx], BiomeType::IceCap);
    }

    #[test]
    fn earth_grid_produces_diverse_biomes() {
        use crate::hydrology::generate_precipitation;
        use crate::ocean::generate_ocean_dynamics;
        let input = earth_like_input();
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "diverse");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_biomes(&mut g);

        // Count distinct biomes on land.
        let mut biomes: std::collections::HashSet<BiomeType> = std::collections::HashSet::new();
        for idx in 0..g.tile_count() {
            if !g.layers.is_ocean[idx] {
                biomes.insert(g.layers.biome[idx]);
            }
        }
        assert!(
            biomes.len() >= 4,
            "Earth-like world should have diverse biomes, got {:?}",
            biomes
        );
    }

    #[test]
    fn earth_grid_has_polar_ice_and_tropical_forest() {
        use crate::hydrology::generate_precipitation;
        use crate::ocean::generate_ocean_dynamics;
        let input = earth_like_input();
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "earth");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_biomes(&mut g);

        let mut has_tropical = false;
        let mut has_cold = false;
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                continue;
            }
            match g.layers.biome[idx] {
                BiomeType::TropicalForest | BiomeType::Savanna => has_tropical = true,
                BiomeType::Tundra | BiomeType::IceCap | BiomeType::Taiga => has_cold = true,
                _ => {}
            }
        }
        assert!(has_tropical, "no tropical/savanna biome found");
        assert!(has_cold, "no cold biome found");
    }

    #[test]
    fn biome_is_deterministic() {
        use crate::hydrology::generate_precipitation;
        use crate::ocean::generate_ocean_dynamics;
        let input = earth_like_input();
        let make = || {
            let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "det");
            generate_temperature(&input, 33.0, &mut g);
            generate_wind(&input, 1.0, &mut g);
            generate_precipitation(&input, 1.0, 71.0, &mut g);
            generate_ocean_dynamics(&mut g);
            generate_biomes(&mut g);
            g
        };
        let a = make();
        let b = make();
        assert_eq!(a.layers.biome, b.layers.biome);
        assert_eq!(a.layers.koppen_class, b.layers.koppen_class);
    }
}
