#![allow(dead_code)]

// =============================================================================
// IAU 2015 Nominal Solar Values
// =============================================================================
/// Solar effective temperature in Kelvin (IAU 2015 Resolution B3)
pub const SOLAR_EFFECTIVE_TEMP_K: f64 = 5772.0;
/// Solar luminosity in Watts (IAU 2015)
pub const SOLAR_LUMINOSITY_W: f64 = 3.828e26;
/// Solar radius in meters (IAU 2015)
pub const SOLAR_RADIUS_M: f64 = 6.957e8;
/// Solar radius in kilometers (IAU 2015)
pub const SOLAR_RADIUS_KM: f64 = 695_700.0;
/// Solar mass in kilograms
pub const SOLAR_MASS_KG: f64 = 1.989e30;
/// Solar absolute visual magnitude
pub const SOLAR_ABSOLUTE_MAG_V: f64 = 4.83;
/// Solar absolute bolometric magnitude
pub const SOLAR_ABSOLUTE_MAG_BOL: f64 = 4.74;

// =============================================================================
// Fundamental Physical Constants
// =============================================================================
/// Gravitational constant in m^3 kg^-1 s^-2
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;
/// Stefan-Boltzmann constant in W m^-2 K^-4
pub const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;
/// Speed of light in m/s (exact)
pub const SPEED_OF_LIGHT_MS: f64 = 299_792_458.0;

// =============================================================================
// Distance Units
// =============================================================================
/// 1 AU in meters (exact, IAU 2012)
pub const AU_M: f64 = 149_597_870_700.0;
/// 1 AU in kilometers
pub const AU_KM: f64 = 149_597_870.700;
/// 1 parsec in AU
pub const PARSEC_AU: f64 = 206_264.806_247;
/// 1 parsec in light-years
pub const PARSEC_LY: f64 = 3.261_563_777;
/// 1 light-year in meters
pub const LIGHT_YEAR_M: f64 = 9.460_730_472_58e15;

// =============================================================================
// Solar Radii / AU Conversion (derived from IAU values)
// =============================================================================
/// 1 solar radius in AU: SOLAR_RADIUS_KM / AU_KM
pub const SOLAR_RADIUS_AU: f64 = 0.004_650_467;
/// 1 AU in solar radii: AU_KM / SOLAR_RADIUS_KM
pub const AU_IN_SOLAR_RADII: f64 = 215.032;

// =============================================================================
// Planetary Reference Values
// =============================================================================
/// Earth mass in kilograms
pub const EARTH_MASS_KG: f64 = 5.972_17e24;
/// Earth mean radius in kilometers
pub const EARTH_RADIUS_KM: f64 = 6_371.0;
/// Earth mass in solar masses (1/332,946)
pub const EARTH_MASS_SOLAR: f64 = 3.003_46e-6;
/// Solar masses per Earth mass (332,946)
pub const SOLAR_MASS_IN_EARTH_MASSES: f64 = 332_946.0;
/// Jupiter mass in kilograms
pub const JUPITER_MASS_KG: f64 = 1.898_2e27;
/// Jupiter mean radius in kilometers
pub const JUPITER_RADIUS_KM: f64 = 69_911.0;
/// Jupiter mass in solar masses (1/1047.35)
pub const JUPITER_MASS_SOLAR: f64 = 9.545_8e-4;
/// Chandrasekhar mass limit in solar masses
pub const CHANDRASEKHAR_MASS: f64 = 1.44;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solar_radius_au_is_consistent() {
        let computed = SOLAR_RADIUS_KM / AU_KM;
        assert!(
            (computed - SOLAR_RADIUS_AU).abs() < 1e-6,
            "SOLAR_RADIUS_AU should equal SOLAR_RADIUS_KM / AU_KM: {} vs {}",
            computed,
            SOLAR_RADIUS_AU
        );
    }

    #[test]
    fn au_in_solar_radii_is_consistent() {
        let computed = AU_KM / SOLAR_RADIUS_KM;
        assert!(
            (computed - AU_IN_SOLAR_RADII).abs() < 0.001,
            "AU_IN_SOLAR_RADII should equal AU_KM / SOLAR_RADIUS_KM: {} vs {}",
            computed,
            AU_IN_SOLAR_RADII
        );
    }

    #[test]
    fn earth_mass_solar_is_consistent() {
        let computed = EARTH_MASS_KG / SOLAR_MASS_KG;
        assert!(
            (computed - EARTH_MASS_SOLAR).abs() / EARTH_MASS_SOLAR < 0.01,
            "EARTH_MASS_SOLAR ratio mismatch: {} vs {}",
            computed,
            EARTH_MASS_SOLAR
        );
    }

    #[test]
    fn solar_mass_in_earth_masses_is_inverse() {
        let computed = 1.0 / EARTH_MASS_SOLAR;
        assert!(
            (computed - SOLAR_MASS_IN_EARTH_MASSES).abs() / SOLAR_MASS_IN_EARTH_MASSES < 0.01,
            "SOLAR_MASS_IN_EARTH_MASSES should be inverse of EARTH_MASS_SOLAR: {} vs {}",
            computed,
            SOLAR_MASS_IN_EARTH_MASSES
        );
    }
}
