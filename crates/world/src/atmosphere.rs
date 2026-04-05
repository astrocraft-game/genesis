pub use crate::types::{
    AtmosphericEscape, AtmosphericEscapeDriver, AtmosphericLayers, AtmosphericLossIntensity,
    AtmosphereBreathability, AtmosphereToxicity, ChemicalComponent, LifeLevel,
    MagneticFieldStrength, PlanetSimulationInput, StarContext,
};

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct AtmosphereProfile {
    pub atmospheric_layers: Option<AtmosphericLayers>,
    pub atmospheric_escape: Option<AtmosphericEscape>,
    pub breathability: AtmosphereBreathability,
    pub toxicity: AtmosphereToxicity,
}

pub fn generate_atmosphere_profile(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    atmospheric_composition: &[(f32, ChemicalComponent)],
    magnetic_field: MagneticFieldStrength,
    life_level: LifeLevel,
) -> AtmosphereProfile {
    let has_atmosphere = atmospheric_pressure > 0.01;
    let has_escape_atmosphere = atmospheric_pressure > 0.001;

    let has_oxygen = atmospheric_composition
        .iter()
        .any(|(f, c)| *c == ChemicalComponent::Oxygen && *f > 0.1);
    let has_h2s = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::HydrogenSulfide);
    let has_so2 = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::SulfurDioxide);
    let has_hcl = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Chlorine);
    let has_methane = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Methane);
    let has_nitrogen = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Nitrogen);

    let mean_molecular_mass = mean_molecular_mass(has_escape_atmosphere, atmospheric_composition);
    let atmospheric_layers = generate_atmospheric_layers(
        has_atmosphere,
        atmospheric_pressure,
        atmospheric_composition,
        context.blackbody_temp_k,
        context.gravity_g,
        mean_molecular_mass,
        has_oxygen,
        has_methane,
        has_nitrogen,
        life_level,
    );
    let atmospheric_escape = generate_atmospheric_escape(
        context,
        atmospheric_pressure,
        mean_molecular_mass,
        magnetic_field,
        has_escape_atmosphere,
    );
    let breathability = classify_breathability(atmospheric_pressure);
    let toxicity = classify_toxicity(
        has_atmosphere,
        atmospheric_pressure,
        atmospheric_composition,
        has_oxygen,
        has_h2s,
        has_so2,
        has_hcl,
    );

    AtmosphereProfile {
        atmospheric_layers,
        atmospheric_escape,
        breathability,
        toxicity,
    }
}

fn mean_molecular_mass(
    has_escape_atmosphere: bool,
    atmospheric_composition: &[(f32, ChemicalComponent)],
) -> f64 {
    if !has_escape_atmosphere {
        return 0.0;
    }

    atmospheric_composition
        .iter()
        .map(|(fraction, component)| {
            let mass = match component {
                ChemicalComponent::Hydrogen => 0.002,
                ChemicalComponent::Helium => 0.004,
                ChemicalComponent::Nitrogen => 0.028,
                ChemicalComponent::Oxygen => 0.032,
                ChemicalComponent::CarbonDioxide => 0.044,
                ChemicalComponent::Methane => 0.016,
                ChemicalComponent::Ammonia => 0.017,
                ChemicalComponent::Water => 0.018,
                ChemicalComponent::Argon => 0.040,
                ChemicalComponent::SulfurDioxide => 0.064,
                _ => 0.029,
            };
            *fraction as f64 * mass
        })
        .sum::<f64>()
        .max(0.002)
}

fn generate_atmospheric_layers(
    has_atmosphere: bool,
    atmospheric_pressure: f32,
    atmospheric_composition: &[(f32, ChemicalComponent)],
    blackbody_temperature: u32,
    gravity: f32,
    mean_molecular_mass: f64,
    has_oxygen: bool,
    has_methane: bool,
    has_nitrogen: bool,
    life_level: LifeLevel,
) -> Option<AtmosphericLayers> {
    if !has_atmosphere {
        return None;
    }

    let g_ms2 = gravity * 9.81;
    let scale_height_m =
        8.314 * blackbody_temperature as f64 / (mean_molecular_mass * g_ms2.max(0.1) as f64);
    let scale_height_km = (scale_height_m / 1000.0) as f32;
    let tropopause_km =
        (scale_height_km * 1.5 * atmospheric_pressure.powf(0.3)).clamp(1.0, 100.0);
    let has_stratosphere = (has_oxygen && life_level.as_u8() >= LifeLevel::PlantLike.as_u8())
        || (has_methane && has_nitrogen);
    let exobase_km = scale_height_km * (atmospheric_pressure.max(0.001).ln() + 15.0).max(5.0);

    Some(AtmosphericLayers {
        scale_height_km,
        tropopause_km,
        has_stratosphere,
        exobase_km,
    })
}

fn generate_atmospheric_escape(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    mean_molecular_mass: f64,
    magnetic_field: MagneticFieldStrength,
    has_escape_atmosphere: bool,
) -> Option<AtmosphericEscape> {
    if !has_escape_atmosphere {
        return None;
    }

    let escape_velocity_km_s =
        (11.186
            * (context.gravity_g.max(0.01) * context.body_radius_earth.max(0.05) as f32).sqrt())
        .max(1.0);
    let xuv_flux_relative = ((context.blackbody_temp_k as f32 / 278.0).max(0.2)).powi(4);
    let stellar_activity = if context.star.age_gyr < 0.7 {
        4.0
    } else if context.star.age_gyr < 1.5 {
        2.5
    } else if context.star.age_gyr < 4.0 {
        1.4
    } else {
        0.8
    };
    let magnetic_exposure = match magnetic_field {
        MagneticFieldStrength::None => 1.5,
        MagneticFieldStrength::Weak => 1.2,
        MagneticFieldStrength::Moderate => 0.85,
        MagneticFieldStrength::Strong => 0.65,
        MagneticFieldStrength::VeryStrong => 0.5,
        MagneticFieldStrength::Extreme => 0.35,
    };
    let molecular_retention = ((mean_molecular_mass as f32) / 0.029).clamp(0.2, 2.5);
    let erosion_pressure = xuv_flux_relative.powf(0.35) * stellar_activity * magnetic_exposure;
    let column_retention = atmospheric_pressure.clamp(0.001, 1.0).powf(0.35);
    let retention_score = ((escape_velocity_km_s * molecular_retention * column_retention)
        / (4.5 * erosion_pressure))
        .clamp(0.0, 4.0);
    let loss_intensity = if retention_score > 1.8 {
        AtmosphericLossIntensity::Negligible
    } else if retention_score > 1.15 {
        AtmosphericLossIntensity::Low
    } else if retention_score > 0.8 {
        AtmosphericLossIntensity::Moderate
    } else if retention_score > 0.45 {
        AtmosphericLossIntensity::High
    } else {
        AtmosphericLossIntensity::Extreme
    };
    let dominant_driver = if context.blackbody_temp_k > 900 && context.gravity_g < 0.6 {
        AtmosphericEscapeDriver::HydrodynamicEscape
    } else if matches!(
        magnetic_field,
        MagneticFieldStrength::None | MagneticFieldStrength::Weak
    ) && xuv_flux_relative > 1.5
    {
        AtmosphericEscapeDriver::StellarWindSputtering
    } else if escape_velocity_km_s < 6.0 {
        AtmosphericEscapeDriver::JeansEscape
    } else {
        AtmosphericEscapeDriver::Minimal
    };
    let atmosphere_retained =
        retention_score >= 0.8 || (atmospheric_pressure > 0.5 && context.gravity_g > 0.6);

    Some(AtmosphericEscape {
        dominant_driver,
        loss_intensity,
        xuv_flux_relative,
        escape_velocity_km_s,
        retention_score,
        atmosphere_retained,
    })
}

fn classify_breathability(atmospheric_pressure: f32) -> AtmosphereBreathability {
    match atmospheric_pressure {
        p if p < 0.001 => AtmosphereBreathability::Vacuum,
        p if p < 0.1 => AtmosphereBreathability::Trace,
        p if p < 0.43 => AtmosphereBreathability::VeryThin,
        p if p < 0.71 => AtmosphereBreathability::ThinBreathable,
        p if p < 1.5 => AtmosphereBreathability::Standard,
        p if p < 2.5 => AtmosphereBreathability::Dense,
        p if p < 10.0 => AtmosphereBreathability::VeryDense,
        _ => AtmosphereBreathability::Superdense,
    }
}

fn classify_toxicity(
    has_atmosphere: bool,
    atmospheric_pressure: f32,
    atmospheric_composition: &[(f32, ChemicalComponent)],
    has_oxygen: bool,
    has_h2s: bool,
    has_so2: bool,
    has_hcl: bool,
) -> AtmosphereToxicity {
    let partial_pressure = |component: ChemicalComponent| -> f32 {
        atmospheric_composition
            .iter()
            .filter(|(_, c)| *c == component)
            .map(|(fraction, _)| fraction * atmospheric_pressure)
            .sum::<f32>()
    };
    let pp_co2 = partial_pressure(ChemicalComponent::CarbonDioxide);
    let pp_co = partial_pressure(ChemicalComponent::CarbonMonoxide);

    if !has_atmosphere {
        AtmosphereToxicity::Benign
    } else if has_hcl {
        AtmosphereToxicity::Insidious
    } else if has_so2 && atmospheric_pressure > 1.0 {
        AtmosphereToxicity::Corrosive
    } else if has_h2s && partial_pressure(ChemicalComponent::HydrogenSulfide) > 0.01 {
        AtmosphereToxicity::LethallyToxic
    } else if pp_co > 0.01 {
        AtmosphereToxicity::HighlyToxic
    } else if has_h2s {
        AtmosphereToxicity::HighlyToxic
    } else if pp_co2 > 0.1 {
        AtmosphereToxicity::MildlyToxic
    } else if pp_co2 > 0.05 {
        AtmosphereToxicity::Filterable
    } else if !has_oxygen && atmospheric_pressure > 0.1 {
        AtmosphereToxicity::Suffocating
    } else if has_oxygen && atmospheric_pressure > 0.4 && atmospheric_pressure < 2.0 {
        let o2_fraction: f32 = atmospheric_composition
            .iter()
            .filter(|(_, c)| *c == ChemicalComponent::Oxygen)
            .map(|(fraction, _)| *fraction)
            .sum();
        if o2_fraction > 0.16 && o2_fraction < 0.50 {
            AtmosphereToxicity::Benign
        } else if o2_fraction > 0.50 {
            AtmosphereToxicity::MildlyToxic
        } else {
            AtmosphereToxicity::Suffocating
        }
    } else {
        AtmosphereToxicity::Marginal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(temp: u32, star_age: f32, gravity: f32, radius: f64) -> PlanetSimulationInput {
        PlanetSimulationInput {
            blackbody_temp_k: temp,
            gravity_g: gravity,
            body_radius_earth: radius,
            star: StarContext {
                age_gyr: star_age,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn earth_like_atmosphere_is_retained_and_benign() {
        let composition = vec![
            (0.78, ChemicalComponent::Nitrogen),
            (0.21, ChemicalComponent::Oxygen),
            (0.01, ChemicalComponent::Argon),
        ];
        let profile = generate_atmosphere_profile(
            &context(288, 4.6, 1.0, 1.0),
            1.0,
            &composition,
            MagneticFieldStrength::Strong,
            LifeLevel::Sentient,
        );

        assert_eq!(profile.breathability, AtmosphereBreathability::Standard);
        assert_eq!(profile.toxicity, AtmosphereToxicity::Benign);
        assert!(profile.atmospheric_layers.unwrap().has_stratosphere);
        assert!(profile.atmospheric_escape.unwrap().atmosphere_retained);
    }

    #[test]
    fn thin_airless_body_is_vacuum_with_extreme_loss() {
        let composition = vec![(1.0, ChemicalComponent::CarbonDioxide)];
        let profile = generate_atmosphere_profile(
            &context(340, 0.4, 0.22, 0.38),
            0.0005,
            &composition,
            MagneticFieldStrength::None,
            LifeLevel::None,
        );

        assert_eq!(profile.breathability, AtmosphereBreathability::Vacuum);
        assert_eq!(profile.toxicity, AtmosphereToxicity::Benign);
        assert!(profile.atmospheric_layers.is_none());
        assert!(profile.atmospheric_escape.is_none());
    }

    #[test]
    fn sulfurous_dense_atmosphere_is_corrosive() {
        let composition = vec![
            (0.92, ChemicalComponent::CarbonDioxide),
            (0.05, ChemicalComponent::Nitrogen),
            (0.03, ChemicalComponent::SulfurDioxide),
        ];
        let profile = generate_atmosphere_profile(
            &context(735, 4.0, 0.9, 0.95),
            15.0,
            &composition,
            MagneticFieldStrength::Weak,
            LifeLevel::None,
        );

        assert_eq!(profile.breathability, AtmosphereBreathability::Superdense);
        assert_eq!(profile.toxicity, AtmosphereToxicity::Corrosive);
    }
}
