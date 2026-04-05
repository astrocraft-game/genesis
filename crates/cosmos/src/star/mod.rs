use crate::internal::*;
use crate::prelude::*;

pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct Star {
    #[default("default")]
    pub name: Rc<str>,
    pub mass: f64,
    pub luminosity: f32,
    pub radius: f64,
    pub age: f32,
    pub temperature: u32,
    pub population: StellarEvolution,
    pub spectral_type: StarSpectralType,
    pub luminosity_class: StarLuminosityClass,
    pub orbital_point_id: u32,
    pub orbit: Option<Orbit>,
    pub zones: Vec<StarZone>,
    pub special_traits: Vec<StarPeculiarity>,
    pub absolute_magnitude: f32,
    pub color_bv: f32,
    pub flare_activity: FlareActivity,
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum FlareActivity {
    VeryQuiet,
    #[default]
    Quiet,
    Moderate,
    Active,
    Hyperactive,
}

impl Star {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Rc<str>,
        mass: f64,
        luminosity: f32,
        radius: f64,
        age: f32,
        temperature: u32,
        population: StellarEvolution,
        spectral_type: StarSpectralType,
        luminosity_class: StarLuminosityClass,
        special_traits: Vec<StarPeculiarity>,
        orbital_point_id: u32,
        orbit: Option<Orbit>,
        zones: Vec<StarZone>,
    ) -> Self {
        let absolute_magnitude = absolute_magnitude_from_luminosity(luminosity);
        let color_bv = temperature_to_bv(temperature);
        let flare_activity = compute_flare_activity(&spectral_type, age);
        Self {
            name,
            mass,
            luminosity,
            radius,
            age,
            temperature,
            population,
            spectral_type,
            luminosity_class,
            special_traits,
            orbital_point_id,
            orbit,
            zones,
            absolute_magnitude,
            color_bv,
            flare_activity,
        }
    }

    pub fn is_main_sequence_dwarf(&self) -> bool {
        (self.luminosity_class == StarLuminosityClass::V
            || self.luminosity_class == StarLuminosityClass::IV)
            && self.is_more_luminous_than_brown_dwarf()
    }

    pub fn is_main_sequence_or_giant(&self) -> bool {
        (self.luminosity_class == StarLuminosityClass::O
            || self.luminosity_class == StarLuminosityClass::Ia
            || self.luminosity_class == StarLuminosityClass::Ib
            || self.luminosity_class == StarLuminosityClass::II
            || self.luminosity_class == StarLuminosityClass::III
            || self.luminosity_class == StarLuminosityClass::IV
            || self.luminosity_class == StarLuminosityClass::V
            || self.luminosity_class == StarLuminosityClass::IV)
            && self.is_more_luminous_than_brown_dwarf()
    }

    pub fn is_more_luminous_than_brown_dwarf(&self) -> bool {
        discriminant(&self.spectral_type) == discriminant(&StarSpectralType::WR(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::O(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::B(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::A(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::F(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::G(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::K(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::M(0))
    }

    pub fn get_minimum_orbital_separation(&self) -> f64 {
        ((1.0
            - self
                .orbit
                .clone()
                .unwrap_or(Orbit {
                    ..Default::default()
                })
                .eccentricity) as f64
            * self.radius) as f64
    }

    pub fn get_maximum_orbital_separation(&self) -> f64 {
        ((1.0
            + self
                .orbit
                .clone()
                .unwrap_or(Orbit {
                    ..Default::default()
                })
                .eccentricity) as f64
            * self.radius) as f64
    }
}

pub fn absolute_magnitude_from_luminosity(luminosity_solar: f32) -> f32 {
    if luminosity_solar <= 0.0 {
        return 99.0;
    }
    4.83 - 2.5 * (luminosity_solar as f64).log10() as f32
}

pub fn luminosity_from_absolute_magnitude(abs_mag: f32) -> f32 {
    10.0_f64.powf(((4.83 - abs_mag) / 2.5) as f64) as f32
}

impl Display for FlareActivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlareActivity::VeryQuiet => "Very Quiet",
                FlareActivity::Quiet => "Quiet",
                FlareActivity::Moderate => "Moderate",
                FlareActivity::Active => "Active",
                FlareActivity::Hyperactive => "Hyperactive",
            }
        )
    }
}

pub fn compute_flare_activity(spectral_type: &StarSpectralType, age_gyr: f32) -> FlareActivity {
    let is_young = age_gyr < 1.0;
    match spectral_type {
        StarSpectralType::M(_) => {
            if is_young {
                FlareActivity::Hyperactive
            } else {
                FlareActivity::Active
            }
        }
        StarSpectralType::K(_) => {
            if is_young {
                FlareActivity::Active
            } else {
                FlareActivity::Moderate
            }
        }
        StarSpectralType::G(_) => {
            if is_young {
                FlareActivity::Moderate
            } else {
                FlareActivity::Quiet
            }
        }
        StarSpectralType::F(_) => FlareActivity::Quiet,
        StarSpectralType::A(_) | StarSpectralType::B(_) | StarSpectralType::O(_) => {
            FlareActivity::VeryQuiet
        }
        _ => FlareActivity::Quiet,
    }
}

pub fn temperature_to_bv(temperature: u32) -> f32 {
    let t = temperature as f64;
    if t < 3000.0 {
        2.0
    } else if t > 30000.0 {
        -0.33
    } else {
        (0.865 * (5601.0 / t).powf(1.7) - 0.396) as f32
    }
}

pub fn bv_to_rgb(bv: f32) -> (u8, u8, u8) {
    let bv = (bv as f64).clamp(-0.4, 2.0);
    let temp = 4600.0 * (1.0 / (0.92 * bv + 1.7) + 1.0 / (0.92 * bv + 0.62));

    let r = if temp <= 6600.0 {
        255.0
    } else {
        (329.698_727_446 * (temp / 100.0 - 60.0).powf(-0.133_204_759_2)).clamp(0.0, 255.0)
    };

    let g = if temp <= 6600.0 {
        (99.470_802_586_1 * (temp / 100.0 - 2.0).ln() - 161.119_568_166_1).clamp(0.0, 255.0)
    } else {
        (288.122_169_528_3 * (temp / 100.0 - 60.0).powf(-0.075_514_849_2)).clamp(0.0, 255.0)
    };

    let b = if temp >= 6600.0 {
        255.0
    } else if temp <= 1900.0 {
        0.0
    } else {
        (138.517_731_223_1 * (temp / 100.0 - 10.0).ln() - 305.044_792_730_7).clamp(0.0, 255.0)
    };

    (r as u8, g as u8, b as u8)
}

pub fn get_star_color_code(star: &Star) -> &'static str {
    match star.spectral_type {
        StarSpectralType::WR(_) | StarSpectralType::O(_) => "\x1b[34m",
        StarSpectralType::B(_) => "\x1b[1;34m",
        StarSpectralType::A(_) => "\x1b[1;37m",
        StarSpectralType::F(_) => "\x1b[1;33m",
        StarSpectralType::G(_) => "\x1b[33m",
        StarSpectralType::K(_) => "\x1b[1;31m",
        StarSpectralType::M(_) => "\x1b[31m",
        StarSpectralType::L(_) | StarSpectralType::T(_) | StarSpectralType::Y(_) => "\x1b[31m",
        StarSpectralType::DA
        | StarSpectralType::DB
        | StarSpectralType::DC
        | StarSpectralType::DO
        | StarSpectralType::DZ
        | StarSpectralType::DQ
        | StarSpectralType::DX => "\x1b[1;37m",
        StarSpectralType::XNS => "\x1b[1;34m",
        _ => "",
    }
}
