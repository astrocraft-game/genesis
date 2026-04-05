pub use crate::types::{
    ChemicalComponent, HazeRegime, LifeLevel, PhotochemicalActivity, Photochemistry, SmogLevel,
    PlanetSimulationInput, StarContext, TelluricBodyComposition,
};

pub fn generate_photochemistry(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    atmospheric_composition: &[(f32, ChemicalComponent)],
    body_type: TelluricBodyComposition,
    life_level: LifeLevel,
) -> Option<Photochemistry> {
    if atmospheric_pressure <= 0.01 {
        return None;
    }

    let has_oxygen = atmospheric_composition
        .iter()
        .any(|(f, c)| *c == ChemicalComponent::Oxygen && *f > 0.1);
    let has_h2s = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::HydrogenSulfide);
    let has_so2 = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::SulfurDioxide);
    let has_methane = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Methane);
    let has_nitrogen = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Nitrogen);

    let stellar_activity = if context.star.age_gyr < 0.7 {
        1.9
    } else if context.star.age_gyr < 1.5 {
        1.4
    } else if context.star.age_gyr < 4.0 {
        1.0
    } else {
        0.75
    };
    let uv_driver =
        ((context.blackbody_temp_k as f32 / 278.0).max(0.25)).powf(1.4) * stellar_activity;
    let ozone_column_relative =
        if has_oxygen && life_level.as_u8() >= LifeLevel::PlantLike.as_u8() {
            (0.5 + atmospheric_pressure.clamp(0.1, 2.0) * 0.35 + uv_driver * 0.1).clamp(0.2, 2.0)
        } else {
            0.0
        };
    let haze_regime = if has_methane
        && has_nitrogen
        && (uv_driver > 0.35 || atmospheric_pressure > 0.5)
    {
        HazeRegime::OrganicHaze
    } else if (has_so2 || has_h2s) && uv_driver > 0.6 {
        HazeRegime::SulfurHaze
    } else if ozone_column_relative > 0.3 {
        HazeRegime::OzoneShielded
    } else if atmospheric_pressure < 0.08 && body_type == TelluricBodyComposition::Rocky {
        HazeRegime::DustLoaded
    } else {
        HazeRegime::Clear
    };
    let activity = if uv_driver > 3.0 {
        PhotochemicalActivity::Extreme
    } else if uv_driver > 1.8 {
        PhotochemicalActivity::Intense
    } else if uv_driver > 0.9 {
        PhotochemicalActivity::Active
    } else {
        PhotochemicalActivity::Quiescent
    };
    let haze_shielding = match haze_regime {
        HazeRegime::Clear => 0.05,
        HazeRegime::OzoneShielded => 0.5,
        HazeRegime::OrganicHaze => 0.45,
        HazeRegime::SulfurHaze => 0.55,
        HazeRegime::DustLoaded => 0.2,
    };
    let uv_shielding_fraction = (haze_shielding + ozone_column_relative * 0.25).clamp(0.0, 0.95);
    let smog_index = if has_methane { 0.35 } else { 0.0 }
        + if has_so2 || has_h2s { 0.45 } else { 0.0 }
        + if has_oxygen && life_level.as_u8() >= LifeLevel::PlantLike.as_u8() {
            0.15
        } else {
            0.0
        }
        + if uv_driver > 1.0 { 0.2 } else { 0.0 };
    let smog_level = if smog_index > 0.8 {
        SmogLevel::Severe
    } else if smog_index > 0.5 {
        SmogLevel::Moderate
    } else if smog_index > 0.2 {
        SmogLevel::Light
    } else {
        SmogLevel::None
    };

    Some(Photochemistry {
        haze_regime,
        activity,
        ozone_column_relative,
        uv_shielding_fraction,
        smog_level,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(temp: u32, star_age: f32) -> PlanetSimulationInput {
        PlanetSimulationInput {
            blackbody_temp_k: temp,
            star: StarContext {
                age_gyr: star_age,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn earth_like_is_ozone_shielded() {
        let composition = vec![
            (0.78, ChemicalComponent::Nitrogen),
            (0.21, ChemicalComponent::Oxygen),
            (0.01, ChemicalComponent::Argon),
        ];
        let photo = generate_photochemistry(
            &context(288, 4.6),
            1.0,
            &composition,
            TelluricBodyComposition::Rocky,
            LifeLevel::Sentient,
        )
        .unwrap();
        assert_eq!(photo.haze_regime, HazeRegime::OzoneShielded);
        assert!(photo.uv_shielding_fraction > 0.4);
    }

    #[test]
    fn titan_like_is_organic_haze() {
        let composition = vec![
            (0.92, ChemicalComponent::Nitrogen),
            (0.06, ChemicalComponent::Methane),
            (0.02, ChemicalComponent::Argon),
        ];
        let photo = generate_photochemistry(
            &context(94, 2.0),
            1.4,
            &composition,
            TelluricBodyComposition::Icy,
            LifeLevel::None,
        )
        .unwrap();
        assert_eq!(photo.haze_regime, HazeRegime::OrganicHaze);
        assert!(photo.smog_level >= SmogLevel::Light);
    }
}
