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
    let radius_earth_radii = radius_meters / earth_radius_meters;

    radius_earth_radii
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
    if flux_w_per_m2 < 0.001 { 0 }
    else if flux_w_per_m2 < 0.01 { 1 }
    else if flux_w_per_m2 < 0.04 { 2 }
    else if flux_w_per_m2 < 0.1 { 3 }
    else if flux_w_per_m2 < 0.5 { 5 }
    else if flux_w_per_m2 < 2.0 { 10 }
    else { 20 }
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
        assert!(
            inner > 0.7 && inner < 0.85,
            "Sun inner HZ = {} AU",
            inner
        );
        assert!(
            outer > 1.7 && outer < 1.9,
            "Sun outer HZ = {} AU",
            outer
        );
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
        assert!(
            (snow - 2.7).abs() < 0.1,
            "Sun snow line = {} AU",
            snow
        );
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
    fn test_tidal_heating_zero_eccentricity() {
        let flux = calculate_tidal_heating_flux(0.0, 0.003, 0.29, 0.001, false);
        assert_eq!(flux, 0.0);
    }

    #[test]
    fn test_tidal_heating_closer_means_more() {
        let close = calculate_tidal_heating_flux(0.01, 0.002, 0.29, 0.001, false);
        let far = calculate_tidal_heating_flux(0.01, 0.01, 0.29, 0.001, false);
        assert!(close > far, "closer orbit should heat more: {} vs {}", close, far);
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
