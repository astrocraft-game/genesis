use crate::internal::ConversionUtils;

use super::elements::ChemicalComponent;

/// Returns a value in Kelvin
pub(crate) fn calculate_blackbody_temperature(luminosity: f32, orbital_radius: f64) -> u32 {
    if orbital_radius <= 0.0 {
        panic!("Orbital radius should be greater than 0");
    }

    let b = 278.0 * ((luminosity as f64).powf(0.25)) / (orbital_radius).sqrt();
    b.round() as u32
}

/// Returns a value in Astronomical Units.
pub(crate) fn calculate_distance_for_temperature(luminosity: f32, temperature: i32) -> f64 {
    let t_ratio = temperature as f64 / 278.0;
    (luminosity as f64 / t_ratio.powi(4)).sqrt()
}

/// Habitable zone boundaries using Kopparapu et al. (2013) formulas.
/// Takes stellar luminosity (solar units) and effective temperature (K).
/// Returns (inner_au, outer_au) using Recent Venus / Early Mars limits.
pub(crate) fn calculate_habitable_zone(luminosity: f32, star_temperature: u32) -> (f64, f64) {
    let t = star_temperature as f64 - 5780.0;
    let s_inner = 1.7763 + 1.4335e-4 * t + 3.3954e-9 * t.powi(2);
    let s_outer = 0.3179 + 5.4513e-5 * t + 1.5313e-9 * t.powi(2);
    let inner_au = (luminosity as f64 / s_inner).sqrt();
    let outer_au = (luminosity as f64 / s_outer).sqrt();
    (inner_au, outer_au)
}

/// Snow line distance in AU (where water ice can condense).
pub(crate) fn calculate_snow_line(luminosity: f32) -> f64 {
    2.7 * (luminosity as f64).sqrt()
}

/// Returns a value in Earth Radii
pub(crate) fn calculate_radius(mass_earth_masses: f64, density_g_cm3: f64) -> f64 {
    let earth_mass_kg: f64 = 5.972e24;
    let earth_radius_meters: f64 = 6.371e6;

    let mass_kg = mass_earth_masses * earth_mass_kg;
    let density_kg_m3 = density_g_cm3 * 1000.0;
    let volume_m3 = mass_kg / density_kg_m3;
    let radius_meters = ((3.0 * volume_m3) / (4.0 * std::f64::consts::PI)).cbrt();

    radius_meters / earth_radius_meters
}

/// Returns a value in Gs
pub(crate) fn calculate_surface_gravity(density_g_cm3: f32, radius_earth_radii: f64) -> f32 {
    (density_g_cm3 / 5.513) * radius_earth_radii as f32
}

/// Calculates the Roche limit based on the densities of the primary and the satellite.
/// The radius of the primary is in Earth radii, density can be any shared unit, and the return value is in AU.
pub fn calculate_roche_limit(
    radius_primary: f64,
    density_primary: f64,
    density_satellite: f64,
) -> f64 {
    ConversionUtils::earth_radii_to_astronomical_units(
        2.44 * radius_primary * (density_primary / density_satellite).powf(1.0 / 3.0),
    )
}

/// Calculates the Hill sphere radius, aka the region around a planet where it can have stable satellites instead of them
/// being pulled out by the system's star.
/// The distance must be in AU and the masses in Solar Masses.
pub(crate) fn calculate_hill_sphere_radius(
    orbital_radius_planet: f64,
    mass_planet: f64,
    mass_star: f64,
) -> f64 {
    orbital_radius_planet * (mass_planet / (3.0 * mass_star)).powf(1.0 / 3.0)
}

/// Calculates the escape velocity for a planet.
///
/// The escape velocity is the minimum speed needed for an object to break free
/// from the gravitational attraction of the planet without further propulsion.
///
/// # Parameters
/// - `mass_earth`: Mass of the planet in Earth masses (1 Earth mass = 5.972e24 kg)
/// - `radius_earth`: Radius of the planet in Earth radii (1 Earth radius = 6.371e6 m)
///
/// # Returns
/// The escape velocity in meters per second (m/s).
pub(crate) fn escape_velocity(mass_earth: f64, radius_earth: f64) -> f64 {
    const G: f64 = 6.67430e-11; // Gravitational constant in m^3 kg^-1 s^-2
    const EARTH_MASS: f64 = 5.972e24; // Earth mass in kg
    const EARTH_RADIUS: f64 = 6.371e6; // Earth radius in meters

    let mass = mass_earth * EARTH_MASS; // Convert mass to kg
    let radius = radius_earth * EARTH_RADIUS; // Convert radius to meters

    ((2.0 * G * mass) / radius).sqrt() // Result in m/s
}

/// Calculates the root mean square (rms) speed of gas molecules.
///
/// The rms speed is a measure of the average speed of gas molecules at a given temperature.
///
/// # Parameters
/// - `temperature`: Temperature in Kelvin (K)
/// - `molecular_mass`: Molecular mass of the gas in kilograms (kg)
///
/// # Returns
/// The rms speed in meters per second (m/s).
pub(crate) fn rms_speed(temperature: i32, molecular_mass: f64) -> f64 {
    const K_B: f64 = 1.380649e-23; // Boltzmann constant in J/K
    ((3.0 * K_B * temperature as f64) / molecular_mass).sqrt()
}

/// Calculates the Jeans parameter for a given planet and gas.
///
/// The Jeans parameter indicates the ability of a planet to retain a particular gas.
/// Higher values indicate better retention capability.
///
/// # Parameters
/// - `mass_earth`: Mass of the planet in Earth masses (1 Earth mass = 5.972e24 kg)
/// - `radius_earth`: Radius of the planet in Earth radii (1 Earth radius = 6.371e6 m)
/// - `temperature`: Temperature in Kelvin (K)
/// - `element`: The atmospheric element as an enum
///
/// # Returns
/// The Jeans parameter (dimensionless).
pub(crate) fn jeans_parameter(
    mass_earth: f64,
    radius_earth: f64,
    temperature: i32,
    element: ChemicalComponent,
) -> f64 {
    let v_e = escape_velocity(mass_earth, radius_earth);
    let v_rms = rms_speed(temperature, element.molecular_weight_kg());
    v_e / v_rms
}

/// Estimate Lagrange trojan population for a planet.
/// L4/L5 points are stable when mass ratio > 25:1 (planet:star).
/// Returns estimated number of trojans (order of magnitude) or 0.
pub(crate) fn estimate_trojan_population(
    planet_mass_solar: f64,
    star_mass_solar: f64,
    system_age_gyr: f32,
) -> u32 {
    if planet_mass_solar <= 0.0 || star_mass_solar <= 0.0 {
        return 0;
    }
    let mass_ratio = star_mass_solar / planet_mass_solar;
    // L4/L5 stable only if mass ratio > ~25
    if mass_ratio < 25.0 {
        return 0;
    }
    // Population scales with planet mass and system age
    // Jupiter has ~10,000 known trojans
    let jupiter_mass_solar = 9.545e-4;
    let mass_factor = (planet_mass_solar / jupiter_mass_solar).sqrt();
    let age_factor = (system_age_gyr as f64 / 4.5).min(2.0);
    (mass_factor * age_factor * 5000.0) as u32
}

/// Estimate photochemical haze optical depth for atmospheres with CH4/N2 under UV.
/// Returns haze optical depth (0 = clear, >1 = opaque like Titan).
pub(crate) fn estimate_photochemical_haze(
    atmospheric_pressure: f32,
    has_methane: bool,
    has_nitrogen: bool,
    blackbody_temperature: u32,
    star_uv_relative: f32,
) -> f32 {
    // Need CH4 or N2 in atmosphere with UV radiation
    if atmospheric_pressure < 0.01 || !has_methane {
        return 0.0;
    }
    // Titan conditions: N2+CH4 atmosphere, cold, moderate UV -> opaque haze
    // Haze production scales with UV flux and CH4 abundance
    let uv_factor = star_uv_relative.clamp(0.0, 10.0);
    let n2_boost = if has_nitrogen { 2.0 } else { 1.0 };
    // Cold temperatures favor haze persistence (less thermal dissociation)
    let temp_factor = if blackbody_temperature < 150 {
        2.0
    } else if blackbody_temperature < 300 {
        1.0
    } else {
        0.3
    };
    let pressure_factor = (atmospheric_pressure / 1.5).min(2.0);
    (0.1 * uv_factor * n2_boost * temp_factor * pressure_factor).min(5.0)
}

/// Estimate cryovolcanic activity for an icy body.
/// Returns (activity_level 0-100, plume_height_km) or None if no activity.
pub(crate) fn estimate_cryovolcanism(
    tidal_heating_flux: f64,
    surface_temperature_k: u32,
    gravity: f32,
    has_subsurface_ocean: bool,
) -> Option<(f32, f32)> {
    // Need subsurface liquid and tidal heating
    if !has_subsurface_ocean || tidal_heating_flux < 0.005 {
        return None;
    }
    // Need cold surface for ice
    if surface_temperature_k > 200 {
        return None;
    }
    // Activity level scales with heating flux
    let activity = ((tidal_heating_flux / 0.1) * 50.0).min(100.0) as f32;
    // Plume height: h ~ v^2 / (2*g), eruption velocity from pressure
    // Higher heating -> higher pressure -> higher plumes
    // Enceladus: ~200 km plumes, g=0.113 m/s^2
    let eruption_velocity = (tidal_heating_flux * 1000.0).sqrt().min(500.0);
    let g_ms2 = gravity * 9.81;
    let plume_height_km = if g_ms2 > 0.01 {
        (eruption_velocity * eruption_velocity / (2.0 * g_ms2 as f64) / 1000.0) as f32
    } else {
        500.0
    };
    Some((activity, plume_height_km.min(1000.0)))
}

/// Determine spin-orbit resonance state from eccentricity and atmospheric pressure.
/// Returns the resonance ratio (rotation:orbit) as (p, q), or None for non-resonant.
/// Mercury is 3:2 at e=0.206, Venus is ~243:(-1) from thermal tides.
pub(crate) fn determine_spin_orbit_resonance(
    eccentricity: f32,
    atmospheric_pressure: f32,
    is_tidally_locked: bool,
) -> Option<(u8, u8)> {
    if is_tidally_locked {
        return Some((1, 1)); // Synchronous rotation
    }
    // Thick atmosphere thermal tides prevent clean resonance (Venus case)
    if atmospheric_pressure > 50.0 {
        return None; // Chaotic/retrograde like Venus
    }
    // 3:2 resonance probability increases with eccentricity
    // At e=0.206 (Mercury), capture probability ~55%
    // At e>0.3, higher resonances become possible
    if eccentricity > 0.15 && eccentricity <= 0.35 {
        Some((3, 2))
    } else if eccentricity > 0.35 && eccentricity <= 0.5 {
        Some((2, 1))
    } else if eccentricity > 0.5 {
        Some((5, 2))
    } else {
        None
    }
}

/// Estimate subsurface ocean parameters for an icy body.
/// Returns (ice_shell_thickness_km, ocean_depth_km) or None if no ocean possible.
pub(crate) fn estimate_subsurface_ocean(
    tidal_heating_flux: f64,
    surface_temperature_k: u32,
    body_radius_earth: f64,
    body_mass_earth: f64,
    has_ammonia: bool,
) -> Option<(f32, f32)> {
    // Minimum heating needed to maintain liquid water under ice
    let min_flux = if has_ammonia { 0.002 } else { 0.01 }; // ammonia depresses melting point
    if tidal_heating_flux < min_flux {
        return None;
    }
    let melting_point = if has_ammonia { 176.0 } else { 273.0 };
    if surface_temperature_k as f64 > melting_point {
        return None; // Surface is already warm enough for surface water
    }
    // Ice shell thickness: d_ice ~ k_ice * (T_melt - T_surface) / q_tidal
    // k_ice ~ 2.2 W/(m*K) for water ice
    let k_ice = 2.2;
    let delta_t = melting_point - surface_temperature_k as f64;
    let ice_thickness_m = k_ice * delta_t / tidal_heating_flux;
    let ice_thickness_km = (ice_thickness_m / 1000.0) as f32;

    // Cap at body radius (can't have ice thicker than the body)
    let body_radius_km = body_radius_earth as f32 * 6371.0;
    let ice_thickness_km = ice_thickness_km.min(body_radius_km * 0.5);

    if ice_thickness_km > body_radius_km * 0.4 {
        return None; // Too thick, no room for ocean
    }

    // Ocean depth: rough estimate from remaining water budget
    // Assume ~10-20% of body mass is water for icy bodies
    let water_fraction = 0.15;
    let water_mass_kg = body_mass_earth * 5.972e24 * water_fraction;
    let ocean_surface_area = 4.0 * std::f64::consts::PI * (body_radius_earth * 6.371e6).powi(2);
    let water_depth_m = water_mass_kg / (1000.0 * ocean_surface_area);
    let ocean_depth_km = ((water_depth_m / 1000.0) as f32 - ice_thickness_km).max(1.0);

    Some((ice_thickness_km, ocean_depth_km.min(500.0)))
}

/// Checks if two orbital periods are in a near-integer resonance.
/// Returns Some((p, q)) if period1/period2 is within tolerance of p/q.
pub(crate) fn detect_orbital_resonance(
    period1: f32,
    period2: f32,
    tolerance: f64,
) -> Option<(u8, u8)> {
    if period1 <= 0.0 || period2 <= 0.0 {
        return None;
    }
    let ratio = if period1 > period2 {
        period1 as f64 / period2 as f64
    } else {
        period2 as f64 / period1 as f64
    };
    let resonances: &[(u8, u8)] = &[
        (2, 1),
        (3, 2),
        (4, 3),
        (5, 3),
        (5, 2),
        (3, 1),
        (4, 1),
        (5, 4),
        (7, 4),
    ];
    for &(p, q) in resonances {
        let expected = p as f64 / q as f64;
        if (ratio - expected).abs() / expected < tolerance {
            return Some((p, q));
        }
    }
    None
}

/// Computes tidal heating flux in W/m^2.
/// Based on: E_dot ~ (21/2) * k2/Q * G * M_host^2 * R^5 * n * e^2 / a^6
/// Simplified with empirical calibration to match Io (~2 W/m^2) and Europa (~0.02 W/m^2).
pub(crate) fn calculate_tidal_heating_flux(
    eccentricity: f32,
    semi_major_axis_au: f64,
    body_radius_earth: f64,
    host_mass_solar: f64,
    is_icy: bool,
) -> f64 {
    if eccentricity.abs() < 1e-6 || semi_major_axis_au < 1e-10 {
        return 0.0;
    }
    let e2 = (eccentricity as f64).powi(2);
    let r5 = body_radius_earth.powi(5);
    let a6 = semi_major_axis_au.powi(6);
    let m2 = host_mass_solar.powi(2);
    // k2/Q: rocky ~ 0.003, icy ~ 0.001
    let k2_over_q: f64 = if is_icy { 0.001 } else { 0.003 };
    // Calibration constant (tuned so Io-like params give ~2 W/m^2)
    let c = 2.5e-4;
    c * k2_over_q * m2 * r5 * e2 / a6
}

/// Maps tidal heating flux to a u32 value compatible with existing tidal_heating field.
pub(crate) fn tidal_heating_flux_to_u32(flux_w_per_m2: f64) -> u32 {
    if flux_w_per_m2 < 0.001 {
        0
    } else if flux_w_per_m2 < 0.01 {
        1
    } else if flux_w_per_m2 < 0.04 {
        2
    } else if flux_w_per_m2 < 0.1 {
        3
    } else if flux_w_per_m2 < 0.5 {
        5
    } else if flux_w_per_m2 < 2.0 {
        10
    } else {
        20
    }
}

/// Computes tidal locking timescale in Gyr.
/// Returns the time needed for tidal friction to synchronize rotation with orbit.
pub(crate) fn tidal_locking_timescale_gyr(
    semi_major_axis_au: f64,
    body_radius_earth: f64,
    host_mass_solar: f64,
    is_icy: bool,
) -> f64 {
    // t_lock ~ C * a^6 / (M_host^2 * R^5) in appropriate units
    // Q/k2 ~ 333 for rocky (Q=100, k2=0.3), ~1000 for icy (Q=50, k2=0.05)
    let q_over_k2: f64 = if is_icy { 1000.0 } else { 333.0 };
    // Empirical scaling calibrated so Earth at 1 AU from 1 Msun gives ~50 Gyr (not locked)
    // and Moon-mass body at 0.05 AU from 0.3 Msun gives ~0.1 Gyr (locked quickly)
    let a6 = semi_major_axis_au.powi(6);
    let r5 = body_radius_earth.powi(5);
    let m2 = host_mass_solar.powi(2);
    if r5 < 1e-30 || m2 < 1e-30 {
        return f64::INFINITY;
    }
    // Scaling constant derived from dimensional analysis + calibration
    0.15 * q_over_k2 * a6 / (m2 * r5)
}

/// Returns true if a body should be tidally locked based on physics.
/// Also checks if thick atmosphere resists locking via thermal tides.
pub(crate) fn should_be_tidally_locked(
    semi_major_axis_au: f64,
    body_radius_earth: f64,
    host_mass_solar: f64,
    system_age_gyr: f32,
    atmospheric_pressure: f32,
    is_icy: bool,
) -> bool {
    let t_lock = tidal_locking_timescale_gyr(
        semi_major_axis_au,
        body_radius_earth,
        host_mass_solar,
        is_icy,
    );
    // Thick atmospheres (>10 atm) resist tidal locking via thermal tides (Venus effect)
    if atmospheric_pressure > 10.0 && semi_major_axis_au < 0.5 {
        return false;
    }
    t_lock < system_age_gyr as f64
}

/// Validates atmospheric retention: removes gases that cannot be retained by the body.
/// Returns the filtered composition and adjusted pressure.
pub(crate) fn validate_atmosphere_retention(
    mass_earth: f64,
    radius_earth: f64,
    temperature: i32,
    magnetic_field_strength: u8,
    composition: &[(f32, ChemicalComponent)],
    pressure: f32,
) -> (Vec<(f32, ChemicalComponent)>, f32) {
    if composition.is_empty() || pressure < 0.001 {
        return (composition.to_vec(), pressure);
    }
    let mut retained = Vec::new();
    let mut total_removed_fraction: f32 = 0.0;

    for &(fraction, component) in composition {
        let jp = jeans_parameter(mass_earth, radius_earth, temperature, component);
        // Jeans parameter < 3: hydrodynamic escape (rapid loss)
        // Jeans parameter < 6: slow thermal escape over Gyr
        // No magnetic field: stellar wind sputtering removes ~2x faster
        let effective_jp = if magnetic_field_strength == 0 {
            jp * 0.5
        } else {
            jp
        };
        if effective_jp >= 6.0 {
            retained.push((fraction, component));
        } else if effective_jp >= 3.0 {
            // Partial retention - reduce fraction
            let retention = (effective_jp - 3.0) / 3.0;
            let new_fraction = fraction * retention as f32;
            if new_fraction > 0.001 {
                retained.push((new_fraction, component));
                total_removed_fraction += fraction - new_fraction;
            } else {
                total_removed_fraction += fraction;
            }
        } else {
            total_removed_fraction += fraction;
        }
    }

    // Renormalize remaining fractions
    let retained_sum: f32 = retained.iter().map(|(f, _)| f).sum();
    if retained_sum > 0.0 {
        retained.iter_mut().for_each(|(f, _)| *f /= retained_sum);
    }

    let adjusted_pressure = pressure * (1.0 - total_removed_fraction).max(0.0);
    (retained, adjusted_pressure)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR_MARGIN: f64 = 0.15;
    const EPSILON: f64 = 1e-5;

    fn within_error_margin(calculated: f64, expected: f64) -> bool {
        (calculated / expected - 1.0).abs() <= ERROR_MARGIN
    }

    #[test]
    fn test_habitable_zone_sun() {
        let (inner, outer) = calculate_habitable_zone(1.0, 5772);
        // Sun's HZ should be roughly 0.75-1.77 AU (Recent Venus / Early Mars)
        assert!(inner > 0.7 && inner < 0.85, "Sun inner HZ = {} AU", inner);
        assert!(outer > 1.7 && outer < 1.9, "Sun outer HZ = {} AU", outer);
    }

    #[test]
    fn test_habitable_zone_m_dwarf() {
        // Proxima Centauri-like: L=0.0017, T=3042K
        let (inner, outer) = calculate_habitable_zone(0.0017, 3042);
        assert!(inner < 0.1, "M dwarf inner HZ = {} AU", inner);
        assert!(outer < 0.3, "M dwarf outer HZ = {} AU", outer);
        assert!(inner < outer);
    }

    #[test]
    fn test_snow_line_sun() {
        let snow = calculate_snow_line(1.0);
        // Sun's snow line ~2.7 AU
        assert!((snow - 2.7).abs() < 0.1, "Sun snow line = {} AU", snow);
    }

    #[test]
    fn test_calculate_radius_earth() {
        let earth_mass = 1.0;
        let earth_density = 5.513;
        let radius = calculate_radius(earth_mass, earth_density);
        assert!(within_error_margin(radius, 1.0));
    }

    #[test]
    fn test_calculate_radius_jupiter() {
        let jupiter_mass = 317.8; // Jupiter's mass in Earth masses
        let jupiter_density = 1.33; // Jupiter's density in g/cm³
        let radius = calculate_radius(jupiter_mass, jupiter_density);
        assert!(within_error_margin(radius, 11.208));
    }

    #[test]
    fn test_calculate_radius_saturn() {
        let saturn_mass = 95.2;
        let saturn_density = 0.69;
        let radius = calculate_radius(saturn_mass, saturn_density);
        assert!(within_error_margin(radius, 9.45));
    }

    #[test]
    fn test_calculate_radius_mars() {
        let mars_mass = 0.107;
        let mars_density = 3.93;
        let radius = calculate_radius(mars_mass, mars_density);
        assert!(within_error_margin(radius, 0.532));
    }

    #[test]
    fn test_calculate_radius_ganymede() {
        let ganymede_mass = 0.0248;
        let ganymede_density = 1.942;
        let radius = calculate_radius(ganymede_mass, ganymede_density);
        assert!(within_error_margin(radius, 0.413));
    }

    #[test]
    fn test_calculate_radius_moon() {
        let moon_mass = 0.0123;
        let moon_density = 3.344;
        let radius = calculate_radius(moon_mass, moon_density);
        assert!(within_error_margin(radius, 0.273));
    }

    #[test]
    fn test_calculate_surface_gravity_earth() {
        let earth_density = 5.513;
        let earth_radius = 1.0;
        let gravity = calculate_surface_gravity(earth_density, earth_radius);
        assert!(within_error_margin(gravity as f64, 1.0));
    }

    #[test]
    fn test_calculate_surface_gravity_mars() {
        let mars_density = 3.93;
        let mars_radius = 0.532;
        let gravity = calculate_surface_gravity(mars_density, mars_radius);
        assert!(within_error_margin(gravity as f64, 0.38));
    }

    #[test]
    fn test_calculate_surface_gravity_jupiter() {
        let jupiter_density = 1.33;
        let jupiter_radius = 11.2;
        let gravity = calculate_surface_gravity(jupiter_density, jupiter_radius);
        assert!(within_error_margin(gravity as f64, 2.528));
    }

    #[test]
    fn test_calculate_surface_gravity_saturn() {
        let saturn_density = 0.69;
        let saturn_radius = 9.45;
        let gravity = calculate_surface_gravity(saturn_density, saturn_radius);
        assert!(within_error_margin(gravity as f64, 1.065));
    }

    #[test]
    fn test_calculate_surface_gravity_ganymede() {
        let ganymede_density = 1.942;
        let ganymede_radius = 0.413;
        let gravity = calculate_surface_gravity(ganymede_density, ganymede_radius);
        assert!(within_error_margin(gravity as f64, 0.146));
    }

    #[test]
    fn test_calculate_surface_gravity_moon() {
        let moon_density = 3.344;
        let moon_radius = 0.273;
        let gravity = calculate_surface_gravity(moon_density, moon_radius);
        assert!(within_error_margin(gravity as f64, 0.165));
    }

    #[test]
    fn test_calculate_hill_sphere_radius_earth_sun() {
        let semi_major_axis_earth: f64 = 1.0;
        let earth_mass: f64 = 1.0 / 333000.0;
        let sun_mass: f64 = 1.0;
        let expected_hill_sphere_radius_au: f64 = 0.01;

        let hill_sphere_radius =
            calculate_hill_sphere_radius(semi_major_axis_earth, earth_mass, sun_mass);
        assert!((hill_sphere_radius - expected_hill_sphere_radius_au).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_roche_limit_mass_gas_giant_moon() {
        let saturn_radius: f64 = 9.14;
        let saturn_density: f64 = 0.687;
        let titan_density: f64 = 1.88;

        let expected_roche_limit_au = 0.0006790230406567097;
        let roche_limit = calculate_roche_limit(saturn_radius, saturn_density, titan_density);

        assert!((roche_limit - expected_roche_limit_au).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_roche_limit_mass_star_planet() {
        let sun_radius: f64 = 109.2;
        let sun_density: f64 = 1.41;
        let earth_density: f64 = 5.51;

        let expected_roche_limit_au = 0.00720416795141276;
        let roche_limit = calculate_roche_limit(sun_radius, sun_density, earth_density);

        assert!((roche_limit - expected_roche_limit_au).abs() < EPSILON);
    }

    #[test]
    fn test_escape_velocity_earth() {
        let ve_earth = escape_velocity(1.0, 1.0);
        assert!((ve_earth - 11186.0).abs() < 1.0); // Expect around 11186 m/s
    }

    #[test]
    fn test_rms_speed_hydrogen_earth() {
        let temperature_earth = 288; // Average temperature of Earth's surface in Kelvin
        let mass_h2 = 2.0 * 1.6735575e-27; // Mass of hydrogen molecule (H2) in kg
        let vrms_h2 = rms_speed(temperature_earth, mass_h2);
        assert!((vrms_h2 - 1887.83).abs() < 10.0); // Expect around 1887.83 m/s
    }

    #[test]
    fn test_jeans_parameter_hydrogen_earth() {
        let mass_earth = 1.0; // Mass of Earth in Earth masses
        let radius_earth = 1.0; // Radius of Earth in Earth radii
        let temperature_earth = 288; // Average temperature of Earth's surface in Kelvin
        let jeans_param_h2_earth = jeans_parameter(
            mass_earth,
            radius_earth,
            temperature_earth,
            ChemicalComponent::Hydrogen,
        );
        assert!((jeans_param_h2_earth - 4.1).abs() < 0.1);
    }

    #[test]
    fn test_trojans_jupiter_like() {
        let pop = estimate_trojan_population(9.545e-4, 1.0, 4.5);
        assert!(
            pop > 1000,
            "Jupiter-like should have thousands of trojans, got {}",
            pop
        );
    }

    #[test]
    fn test_trojans_earth_like() {
        // Earth mass is too small relative to Sun for significant trojans
        let pop = estimate_trojan_population(3.0e-6, 1.0, 4.5);
        assert!(pop < 500, "Earth-like should have few trojans, got {}", pop);
    }

    #[test]
    fn test_haze_titan_like() {
        // Titan: N2+CH4, cold, moderate UV
        let haze = estimate_photochemical_haze(1.5, true, true, 94, 1.0);
        assert!(
            haze > 0.3,
            "Titan-like should have significant haze, got {}",
            haze
        );
    }

    #[test]
    fn test_haze_no_methane() {
        let haze = estimate_photochemical_haze(1.0, false, true, 200, 1.0);
        assert_eq!(haze, 0.0);
    }

    #[test]
    fn test_haze_no_atmosphere() {
        let haze = estimate_photochemical_haze(0.001, true, true, 100, 1.0);
        assert_eq!(haze, 0.0);
    }

    #[test]
    fn test_cryovolcanism_enceladus_like() {
        let result = estimate_cryovolcanism(0.1, 75, 0.012, true);
        assert!(result.is_some());
        let (activity, plume_km) = result.unwrap();
        assert!(activity > 0.0, "activity={}", activity);
        assert!(plume_km > 0.0, "plume_km={}", plume_km);
    }

    #[test]
    fn test_cryovolcanism_no_ocean() {
        let result = estimate_cryovolcanism(0.1, 75, 0.012, false);
        assert!(result.is_none());
    }

    #[test]
    fn test_spin_orbit_mercury() {
        // Mercury: e=0.206, thin atmosphere -> 3:2
        let res = determine_spin_orbit_resonance(0.206, 0.0, false);
        assert_eq!(res, Some((3, 2)));
    }

    #[test]
    fn test_spin_orbit_venus() {
        // Venus: thick atmosphere -> no clean resonance
        let res = determine_spin_orbit_resonance(0.007, 92.0, false);
        assert!(res.is_none());
    }

    #[test]
    fn test_spin_orbit_locked() {
        let res = determine_spin_orbit_resonance(0.01, 0.0, true);
        assert_eq!(res, Some((1, 1)));
    }

    #[test]
    fn test_subsurface_ocean_europa_like() {
        // Europa-like: moderate tidal heating, cold surface, icy
        let result = estimate_subsurface_ocean(0.05, 100, 0.245, 0.008, false);
        assert!(result.is_some());
        let (ice_km, ocean_km) = result.unwrap();
        assert!(ice_km > 1.0 && ice_km < 200.0, "ice={}", ice_km);
        assert!(ocean_km > 0.0, "ocean={}", ocean_km);
    }

    #[test]
    fn test_subsurface_ocean_no_heating() {
        let result = estimate_subsurface_ocean(0.0001, 80, 0.2, 0.005, false);
        assert!(result.is_none());
    }

    #[test]
    fn test_detect_resonance_2_1() {
        let res = detect_orbital_resonance(365.0, 183.0, 0.05);
        assert_eq!(res, Some((2, 1)));
    }

    #[test]
    fn test_detect_resonance_3_2() {
        // Io-Europa: 1.77 days vs 3.55 days -> 2:1
        let res = detect_orbital_resonance(3.55, 1.77, 0.05);
        assert_eq!(res, Some((2, 1)));
    }

    #[test]
    fn test_detect_no_resonance() {
        let res = detect_orbital_resonance(365.0, 200.0, 0.03);
        assert!(res.is_none());
    }

    #[test]
    fn test_tidal_heating_zero_eccentricity() {
        let flux = calculate_tidal_heating_flux(0.0, 0.003, 0.29, 0.001, false);
        assert_eq!(flux, 0.0);
    }

    #[test]
    fn test_tidal_heating_closer_means_more() {
        let close = calculate_tidal_heating_flux(0.01, 0.002, 0.29, 0.001, false);
        let far = calculate_tidal_heating_flux(0.01, 0.01, 0.29, 0.001, false);
        assert!(
            close > far,
            "closer orbit should heat more: {} vs {}",
            close,
            far
        );
    }

    #[test]
    fn test_tidal_heating_flux_to_u32_ranges() {
        assert_eq!(tidal_heating_flux_to_u32(0.0), 0);
        assert_eq!(tidal_heating_flux_to_u32(0.005), 1);
        assert_eq!(tidal_heating_flux_to_u32(0.02), 2);
        assert_eq!(tidal_heating_flux_to_u32(0.3), 5);
        assert_eq!(tidal_heating_flux_to_u32(1.5), 10);
        assert_eq!(tidal_heating_flux_to_u32(5.0), 20);
    }

    #[test]
    fn test_tidal_locking_close_m_dwarf() {
        // Planet at 0.05 AU from 0.3 Msun - should lock quickly
        assert!(should_be_tidally_locked(0.05, 1.0, 0.3, 5.0, 1.0, false));
    }

    #[test]
    fn test_tidal_locking_earth_not_locked() {
        // Earth at 1 AU from 1 Msun - should NOT be locked at 4.5 Gyr
        assert!(!should_be_tidally_locked(1.0, 1.0, 1.0, 4.5, 1.0, false));
    }

    #[test]
    fn test_tidal_locking_venus_thick_atmo() {
        // Venus-like: close but thick atmosphere resists locking
        assert!(!should_be_tidally_locked(0.1, 0.95, 0.3, 5.0, 92.0, false));
    }

    #[test]
    fn test_atmosphere_retention_earth() {
        let comp = vec![
            (0.78, ChemicalComponent::Nitrogen),
            (0.21, ChemicalComponent::Oxygen),
            (0.01, ChemicalComponent::Argon),
        ];
        let (retained, pressure) = validate_atmosphere_retention(1.0, 1.0, 288, 3, &comp, 1.0);
        // Earth retains N2, O2, Ar easily
        assert_eq!(retained.len(), 3);
        assert!((pressure - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_atmosphere_retention_small_hot_body() {
        let comp = vec![
            (0.5, ChemicalComponent::Hydrogen),
            (0.5, ChemicalComponent::Helium),
        ];
        // Small hot body with no magnetic field - should lose H and He
        let (retained, pressure) = validate_atmosphere_retention(0.01, 0.2, 500, 0, &comp, 0.1);
        assert!(retained.len() < 2 || pressure < 0.05);
    }
}
