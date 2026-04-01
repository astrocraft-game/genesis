#![allow(dead_code)]
use crate::internal::*;
use crate::prelude::*;

pub const BROWN_DWARF_MIN_MASS: f64 = 0.015;
pub const RED_DWARF_POP_HYPERDWARF_MIN_MASS: f64 = 1.0 / 25.0;
pub const RED_DWARF_POP_SUPERDWARF_MIN_MASS: f64 = 1.0 / 16.0;
pub const RED_DWARF_POP_DWARF_MIN_MASS: f64 = 0.07;
pub const RED_DWARF_POP_SUBDWARF_MIN_MASS: f64 = 0.125;
pub const RED_DWARF_POP_PALEODWARF_MIN_MASS: f64 = 0.4;
pub const ORANGE_DWARF_MIN_MASS: f64 = 0.5;
pub const YELLOW_DWARF_MIN_MASS: f64 = 1.0;
pub const WHITE_DWARF_MIN_MASS: f64 = 2.0;
pub const WHITE_GIANT_MIN_MASS: f64 = 4.0;
pub const BLUE_GIANT_MIN_MASS: f64 = 8.0;
pub const BLUE_GIANT_POP_HYPERDWARF_MAX_MASS: f64 = 25.0;
pub const BLUE_GIANT_POP_SUPERDWARF_MAX_MASS: f64 = 50.0;
pub const BLUE_GIANT_POP_DWARF_MAX_MASS: f64 = 100.0;
pub const BLUE_GIANT_POP_SUBDWARF_MAX_MASS: f64 = 200.0;
pub const BLUE_GIANT_POP_PALEODWARF_MAX_MASS: f64 = 500.0;

/// The following array gives expected temperatures and luminosity (powers of 10) for solar masses at each stage of their lifespan
/// 1st entry is "birth", 3rd is beginning of subgiant, 5th beginning of giant/supergiant, 7th end of giant/supergiant, and inbetween are mid-points
pub const STAR_LIFECYCLE_DATASET: [[TemperatureAndLuminosity; 7]; 8] = [
    // 0.4 solar masses
    [
        TemperatureAndLuminosity(3375.0, -2.05),
        TemperatureAndLuminosity(4300.0, -0.8),
        TemperatureAndLuminosity(4100.0, -0.2),
        TemperatureAndLuminosity(3950.0, 0.7),
        TemperatureAndLuminosity(3800.0, 1.2),
        TemperatureAndLuminosity(3650.0, 1.75),
        TemperatureAndLuminosity(3200.0, 0.5),
    ],
    // 0.5 solar masses
    [
        TemperatureAndLuminosity(4200.0, -1.3),
        TemperatureAndLuminosity(4300.0, -0.8),
        TemperatureAndLuminosity(4100.0, -0.2),
        TemperatureAndLuminosity(3950.0, 0.7),
        TemperatureAndLuminosity(3800.0, 1.2),
        TemperatureAndLuminosity(3650.0, 1.75),
        TemperatureAndLuminosity(3300.0, 2.3),
    ],
    // 1 solar mass
    [
        TemperatureAndLuminosity(5400.0, -0.1455),
        TemperatureAndLuminosity(5805.0, 0.0126543),
        TemperatureAndLuminosity(5500.0, 0.6),
        TemperatureAndLuminosity(5200.0, 0.75),
        TemperatureAndLuminosity(4300.0, 0.75),
        TemperatureAndLuminosity(3900.0, 1.25),
        TemperatureAndLuminosity(3500.0, 2.6),
    ],
    // 2 solar masses
    [
        TemperatureAndLuminosity(8450.0, 0.8),
        TemperatureAndLuminosity(7800.0, 1.4),
        TemperatureAndLuminosity(6700.0, 1.4),
        TemperatureAndLuminosity(7500.0, 1.5),
        TemperatureAndLuminosity(5100.0, 1.7),
        TemperatureAndLuminosity(4500.0, 2.0),
        TemperatureAndLuminosity(3950.0, 2.9),
    ],
    // 5 solar masses
    [
        TemperatureAndLuminosity(17000.0, 2.75),
        TemperatureAndLuminosity(16000.0, 3.1),
        TemperatureAndLuminosity(13800.0, 3.1),
        TemperatureAndLuminosity(8000.0, 3.2),
        TemperatureAndLuminosity(3600.0, 3.5),
        TemperatureAndLuminosity(8600.0, 3.8),
        TemperatureAndLuminosity(5500.0, 3.9),
    ],
    // 15 solar masses
    [
        TemperatureAndLuminosity(31000.0, 4.4),
        TemperatureAndLuminosity(25000.0, 4.6),
        TemperatureAndLuminosity(27000.0, 4.7),
        TemperatureAndLuminosity(17000.0, 4.75),
        TemperatureAndLuminosity(12000.0, 4.8),
        TemperatureAndLuminosity(6000.0, 4.6),
        TemperatureAndLuminosity(3600.0, 4.8),
    ],
    // 60 solar masses
    [
        TemperatureAndLuminosity(43520.0, 5.75),
        TemperatureAndLuminosity(17000.0, 5.95),
        TemperatureAndLuminosity(6000.0, 6.0),
        TemperatureAndLuminosity(19000.0, 6.1),
        TemperatureAndLuminosity(46000.0, 6.0),
        TemperatureAndLuminosity(27000.0, 5.9),
        TemperatureAndLuminosity(62000.0, 5.4),
    ],
    // 500 solar masses
    [
        TemperatureAndLuminosity(53000.0, 6.7),
        TemperatureAndLuminosity(22000.0, 6.9),
        TemperatureAndLuminosity(7000.0, 6.8),
        TemperatureAndLuminosity(24000.0, 7.0),
        TemperatureAndLuminosity(48000.0, 6.9),
        TemperatureAndLuminosity(30000.0, 6.8),
        TemperatureAndLuminosity(70000.0, 6.2),
    ],
];

/// Reference properties for main sequence stars by spectral type.
/// (spectral_type_code, T_eff_K, mass_solar, radius_solar, luminosity_solar, lifetime_Myr)
/// Sources: Pecaut & Mamajek (2013), updated 2022.
pub const MS_SPECTRAL_REFERENCE: &[(u8, u32, f64, f64, f64, f64)] = &[
    // O-type
    (12, 54000, 59.0, 15.0, 790000.0, 1.0),     // O2V
    (13, 45000, 37.0, 12.0, 316000.0, 1.6),      // O3V
    (15, 40600, 25.0, 9.4, 120000.0, 3.3),       // O5V
    (17, 37100, 18.0, 7.4, 55000.0, 5.7),        // O7V
    (19, 33300, 14.0, 5.7, 25000.0, 9.6),        // O9V
    // B-type
    (20, 29200, 11.0, 4.8, 13000.0, 14.0),       // B0V
    (22, 20600, 7.3, 3.6, 4000.0, 32.0),         // B2V
    (25, 15200, 4.5, 2.7, 830.0, 100.0),         // B5V
    (28, 12300, 3.4, 2.2, 280.0, 210.0),         // B8V
    // A-type
    (30, 9700, 2.8, 1.87, 80.0, 400.0),          // A0V
    (35, 8200, 2.0, 1.52, 25.0, 1000.0),         // A5V
    // F-type
    (40, 7220, 1.61, 1.35, 10.0, 2500.0),        // F0V
    (45, 6510, 1.33, 1.20, 4.0, 5000.0),         // F5V
    // G-type
    (50, 5930, 1.08, 1.06, 1.6, 9500.0),         // G0V
    (52, 5770, 1.00, 1.00, 1.0, 10000.0),        // G2V (Sun)
    (55, 5560, 0.93, 0.92, 0.73, 13000.0),       // G5V
    // K-type
    (60, 5240, 0.82, 0.81, 0.42, 22000.0),       // K0V
    (65, 4350, 0.68, 0.63, 0.15, 60000.0),       // K5V
    // M-type
    (70, 3850, 0.57, 0.55, 0.075, 100000.0),     // M0V
    (73, 3430, 0.37, 0.39, 0.018, 500000.0),     // M3V
    (75, 3060, 0.16, 0.20, 0.0032, 2000000.0),   // M5V
    (78, 2680, 0.09, 0.12, 0.0006, 10000000.0),  // M8V
    // Brown dwarfs
    (80, 2200, 0.07, 0.10, 0.0001, -1.0),        // L0
    (88, 1500, 0.06, 0.09, 0.00003, -1.0),       // L8
    (92, 1400, 0.05, 0.09, 0.00001, -1.0),       // T2
    (100, 450, 0.02, 0.08, 0.0000001, -1.0),     // Y0
];

/// The following array contains equivalencies between temperatures and spectral types.
pub const TEMPERATURE_TO_SPECTRAL_TYPE_DATASET: &[(u32, u32); 69] = &[
    (u32::MAX, 0), // WR
    (1500000, 0),
    (500000, 1),
    (380000, 2),
    (170000, 3),
    (117000, 4),
    (54000, 12), // O
    (45000, 13),
    (43300, 14),
    (40600, 15),
    (39500, 16),
    (37100, 17),
    (35100, 18),
    (33300, 19),
    (29200, 20), // B
    (23000, 21),
    (21000, 22),
    (17600, 23),
    (15200, 25),
    (14300, 26),
    (13500, 27),
    (12300, 28),
    (11400, 29),
    (9600, 30), // A
    (9330, 31),
    (9040, 32),
    (8750, 33),
    (8480, 34),
    (8310, 35),
    (7920, 37),
    (7350, 40), // F
    (7050, 42),
    (6850, 43),
    (6700, 45),
    (6550, 46),
    (6400, 47),
    (6300, 48),
    (6050, 50), // G
    (5930, 51),
    (5800, 52),
    (5660, 55),
    (5440, 58),
    (5240, 60), // K
    (5110, 61),
    (4960, 62),
    (4800, 63),
    (4600, 64),
    (4400, 65),
    (4000, 67),
    (3750, 70), // M
    (3700, 71),
    (3600, 72),
    (3500, 73),
    (3400, 74),
    (3200, 75),
    (3100, 76),
    (2900, 77),
    (2700, 78),
    (2600, 80), // L
    (2200, 83),
    (1500, 88),
    (1400, 92), // T
    (1000, 96),
    (800, 98),
    (370, 100), // Y
    (350, 101),
    (320, 102),
    (250, 104),
    (0, 109),
];

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct TemperatureAndLuminosity(pub f32, pub f32);

pub fn get_test_stars() -> Vec<Star> {
    vec![
        Star::new(
            "40 Eridani B".into(),
            0.573, // Mass
            0.013, // Luminosity
            0.014, // Radius
            9.0,   // Age
            16500, // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::DA,
            StarLuminosityClass::VII,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Sirius B".into(),
            1.018,  // Mass
            0.056,  // Luminosity
            0.0084, // Radius
            0.228,  // Age
            25200,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::DA,
            StarLuminosityClass::VII,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Cygnus OB2-12".into(),
            110.0,     // Mass
            1660000.0, // Luminosity
            246.0,     // Radius
            0.003,     // Age
            13700,     // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::B(3),
            StarLuminosityClass::Ia,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Rigel A".into(),
            21.0,     // Mass
            120000.0, // Luminosity
            78.9,     // Radius
            0.008,    // Age
            12100,    // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::F(8),
            StarLuminosityClass::Ia,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Epsilon Canis Majoris".into(),
            12.6,    // Mass
            38700.0, // Luminosity
            13.9,    // Radius
            0.022,   // Age
            22900,   // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::B(2),
            StarLuminosityClass::II,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Canopus".into(),
            8.0,     // Mass
            10700.0, // Luminosity
            71.0,    // Radius
            0.025,   // Age
            7400,    // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::A(9),
            StarLuminosityClass::II,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Beta Draconis".into(),
            6.0,   // Mass
            996.0, // Luminosity
            40.0,  // Radius
            0.095, // Age
            5160,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::G(2),
            StarLuminosityClass::Ib,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Theta Scorpii".into(),
            3.1,    // Mass
            1400.0, // Luminosity
            26.3,   // Radius
            0.5,    // Age
            6294,   // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::F(0),
            StarLuminosityClass::II,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "WISE 0855-0714".into(),
            0.004772,  // Mass
            0.0000011, // Luminosity
            0.021,     // Radius
            -1.0,      // Age
            240,       // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::Y(4),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Luhman 16".into(),
            0.03197,   // Mass
            0.0000219, // Luminosity
            0.08734,   // Radius
            -1.0,      // Age
            1350,      // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::L(7),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Wolf 359".into(),
            0.11,    // Mass
            0.00106, // Luminosity
            0.144,   // Radius
            0.25,    // Age
            2749,    // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::M(6),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Proxima Centauri".into(),
            0.1221,   // Mass
            0.001567, // Luminosity
            0.1542,   // Radius
            4.85,     // Age
            2992,     // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::M(5),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Barnard's Star".into(),
            0.144,  // Mass
            0.0035, // Luminosity
            0.196,  // Radius
            10.0,   // Age
            3134,   // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::M(4),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Ross 154".into(),
            0.177,    // Mass
            0.004015, // Luminosity
            0.2,      // Radius
            0.9,      // Age
            3248,     // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::M(3),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "40 Eridani C".into(),
            0.2036, // Mass
            0.008,  // Luminosity
            0.31,   // Radius
            5.6,    // Age
            3100,   // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::M(4),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Lalande 21185".into(),
            0.389,  // Mass
            0.0195, // Luminosity
            0.392,  // Radius
            7.5,    // Age
            3547,   // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::M(2),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Lacaille 9352".into(),
            0.479,  // Mass
            0.0368, // Luminosity
            0.474,  // Radius
            4.57,   // Age
            3672,   // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::M(0),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "TW Piscis Austrini".into(),
            0.725, // Mass
            0.19,  // Luminosity
            0.629, // Radius
            0.44,  // Age
            4711,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::K(5),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "40 Eridani A".into(),
            0.78,  // Mass
            0.457, // Luminosity
            0.812, // Radius
            7.0,   // Age
            5072,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::K(0),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Ran".into(),
            0.82,  // Mass
            0.34,  // Luminosity
            0.735, // Radius
            0.6,   // Age
            5084,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::K(2),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Alpha Centauri B".into(),
            0.9092, // Mass
            0.4981, // Luminosity
            0.8591, // Radius
            5.3,    // Age
            5260,   // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::K(1),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "61 Ursae Majoris".into(),
            0.93,  // Mass
            0.609, // Luminosity
            0.86,  // Radius
            2.1,   // Age
            5488,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::G(8),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Beta Canum Venaticorum".into(),
            0.97,  // Mass
            1.151, // Luminosity
            1.123, // Radius
            3.4,   // Age
            6043,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::G(0),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Sun".into(),
            1.0,  // Mass
            1.0,  // Luminosity
            1.0,  // Radius
            4.6,  // Age
            5772, // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::G(2),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Kappa Ceti".into(),
            1.037, // Mass
            0.85,  // Luminosity
            0.95,  // Radius
            0.3,   // Age
            5708,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::G(5),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Alpha Centauri A".into(),
            1.0788, // Mass
            1.5059, // Luminosity
            1.2175, // Radius
            5.3,    // Age
            5790,   // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::G(2),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Upsilon Andromedae".into(),
            1.27, // Mass
            3.57, // Luminosity
            1.48, // Radius
            3.12, // Age
            6213, // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::F(8),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Tau Boötis".into(),
            1.39, // Mass
            3.06, // Luminosity
            1.42, // Radius
            1.6,  // Age
            6399, // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::F(7),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Procyon A".into(),
            1.42,  // Mass
            7.73,  // Luminosity
            2.048, // Radius
            2.4,   // Age
            6550,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::F(5),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Altair".into(),
            1.86, // Mass
            10.6, // Luminosity
            1.57, // Radius
            0.1,  // Age
            7760, // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::A(7),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Fomalhaut".into(),
            1.92,  // Mass
            16.63, // Luminosity
            1.842, // Radius
            0.44,  // Age
            8590,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::A(3),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Delta Capricorni".into(),
            2.0,   // Mass
            8.5,   // Luminosity
            1.91,  // Radius
            0.228, // Age
            7301,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::A(7),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Sirius A".into(),
            2.063, // Mass
            25.4,  // Luminosity
            1.711, // Radius
            0.228, // Age
            9940,  // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::A(0),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "HD 21071".into(),
            3.69,  // Mass
            278.0, // Luminosity
            2.21,  // Radius
            0.009, // Age
            14768, // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::B(7),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "Eta Aurigae".into(),
            5.4,   // Mass
            955.0, // Luminosity
            3.25,  // Radius
            0.022, // Age
            17201, // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::B(3),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
        Star::new(
            "10 Lacertae".into(),
            26.9,     // Mass
            102000.0, // Luminosity
            8.27,     // Radius
            0.028,    // Age
            36000,    // Temperature
            StellarEvolution::Dwarf,
            StarSpectralType::O(9),
            StarLuminosityClass::V,
            vec![],
            0,
            Some(Orbit {
                ..Default::default()
            }),
            vec![],
        ),
    ]
}
