use crate::utils::constants::*;

pub struct ConversionUtils {}
impl ConversionUtils {
    /// Converts a value from Solar radii to Astronomical units.
    pub fn solar_radii_to_astronomical_units(radius: f64) -> f64 {
        radius * SOLAR_RADIUS_AU
    }

    /// Converts a value from Astronomical units to Solar radii.
    pub fn astronomical_units_to_solar_radii(au: f64) -> f64 {
        au * AU_IN_SOLAR_RADII
    }

    /// Converts a value from Earth radii to Astronomical units.
    pub fn earth_radii_to_astronomical_units(radius: f64) -> f64 {
        radius * EARTH_RADIUS_KM / AU_KM
    }

    /// Converts a value from Astronomical units to Earth radii.
    pub fn astronomical_units_to_earth_radii(au: f64) -> f64 {
        au * AU_KM / EARTH_RADIUS_KM
    }

    /// Converts a value from Astronomical units to Earth diameters.
    pub fn astronomical_units_to_earth_diameters(au: f64) -> f64 {
        au * AU_KM / (EARTH_RADIUS_KM * 2.0)
    }

    /// Converts a temperature from Kelvin to Celsius.
    pub fn kelvin_to_celsius(temperature: u32) -> i32 {
        (temperature as f32 - 273.15) as i32
    }

    /// Converts a value expressed in Earth Masses into Solar Masses.
    pub fn earth_mass_to_solar_mass(mass: f64) -> f64 {
        mass / SOLAR_MASS_IN_EARTH_MASSES
    }

    /// Converts a value expressed in Solar Masses into Earth Masses.
    pub fn solar_mass_to_earth_mass(mass: f64) -> f64 {
        mass * SOLAR_MASS_IN_EARTH_MASSES
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_radii_to_astronomical_units() {
        let au = ConversionUtils::solar_radii_to_astronomical_units(1.0);
        assert!(
            (au - SOLAR_RADIUS_AU).abs() < 1e-9,
            "1 solar radius = {} AU",
            au
        );
    }

    #[test]
    fn test_astronomical_units_to_solar_radii() {
        let sr = ConversionUtils::astronomical_units_to_solar_radii(1.0);
        assert!(
            (sr - AU_IN_SOLAR_RADII).abs() < 0.001,
            "1 AU = {} solar radii",
            sr
        );
    }

    #[test]
    fn test_roundtrip_solar_radii_au() {
        let original = 1.0;
        let au = ConversionUtils::solar_radii_to_astronomical_units(original);
        let back = ConversionUtils::astronomical_units_to_solar_radii(au);
        assert!(
            (back - original).abs() < 0.01,
            "roundtrip: {} -> {} AU -> {}",
            original,
            au,
            back
        );
    }

    #[test]
    fn test_earth_radii_to_astronomical_units() {
        let au = ConversionUtils::earth_radii_to_astronomical_units(1.0);
        let expected = EARTH_RADIUS_KM / AU_KM;
        assert!((au - expected).abs() < 1e-12);
    }

    #[test]
    fn test_roundtrip_earth_radii_au() {
        let original = 1.0;
        let au = ConversionUtils::earth_radii_to_astronomical_units(original);
        let back = ConversionUtils::astronomical_units_to_earth_radii(au);
        assert!(
            (back - original).abs() < 1e-6,
            "roundtrip: {} -> {} AU -> {}",
            original,
            au,
            back
        );
    }

    #[test]
    fn test_kelvin_to_celsius() {
        assert_eq!(ConversionUtils::kelvin_to_celsius(273), 0);
        assert_eq!(ConversionUtils::kelvin_to_celsius(373), 99);
    }

    #[test]
    fn test_earth_mass_to_solar_mass() {
        let solar = ConversionUtils::earth_mass_to_solar_mass(SOLAR_MASS_IN_EARTH_MASSES);
        assert!((solar - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_solar_mass_to_earth_mass() {
        let earth = ConversionUtils::solar_mass_to_earth_mass(1.0);
        assert!((earth - SOLAR_MASS_IN_EARTH_MASSES).abs() < 1e-6);
    }

    #[test]
    fn test_roundtrip_mass() {
        let original = 5.0;
        let solar = ConversionUtils::earth_mass_to_solar_mass(original);
        let back = ConversionUtils::solar_mass_to_earth_mass(solar);
        assert!((back - original).abs() < 1e-9);
    }
}
