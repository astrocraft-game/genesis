pub use crate::types::{
    ChemicalComponent, HazeRegime, LifeLevel, PhotochemicalActivity, Photochemistry,
    PlanetSimulationInput, SmogLevel, StarContext, TelluricBodyComposition,
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
    let ozone_column_relative = if has_oxygen && life_level.as_u8() >= LifeLevel::PlantLike.as_u8()
    {
        (0.5 + atmospheric_pressure.clamp(0.1, 2.0) * 0.35 + uv_driver * 0.1).clamp(0.2, 2.0)
    } else {
        0.0
    };
    let haze_regime =
        if has_methane && has_nitrogen && (uv_driver > 0.35 || atmospheric_pressure > 0.5) {
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
    // Stratospheric activity: dominated by UV photolysis and stellar XUV flux.
    let stratospheric_activity = if uv_driver > 3.0 {
        PhotochemicalActivity::Extreme
    } else if uv_driver > 1.8 {
        PhotochemicalActivity::Intense
    } else if uv_driver > 0.9 {
        PhotochemicalActivity::Active
    } else {
        PhotochemicalActivity::Quiescent
    };

    // Tropospheric activity: driven by surface temperature, pressure, and
    // reactive species (O2/H2S/SO2/CH4). A hot, high-pressure, oxidising
    // lower atmosphere pushes thermal chemistry harder.
    let thermal_driver = (context.blackbody_temp_k as f32 / 288.0).max(0.3)
        * atmospheric_pressure.clamp(0.05, 5.0).sqrt();
    let reactant_score = (has_oxygen as u8 as f32) * 1.0
        + (has_methane as u8 as f32) * 0.6
        + (has_so2 as u8 as f32) * 0.5
        + (has_h2s as u8 as f32) * 0.5;
    let tropo_driver = thermal_driver * (0.5 + reactant_score * 0.35);
    let tropospheric_activity = if tropo_driver > 3.0 {
        PhotochemicalActivity::Extreme
    } else if tropo_driver > 1.8 {
        PhotochemicalActivity::Intense
    } else if tropo_driver > 0.9 {
        PhotochemicalActivity::Active
    } else {
        PhotochemicalActivity::Quiescent
    };

    // Overall activity is the max of the two layers.
    let activity = stratospheric_activity.max(tropospheric_activity);

    let haze_shielding = match haze_regime {
        HazeRegime::Clear => 0.05,
        HazeRegime::OzoneShielded => 0.5,
        HazeRegime::OrganicHaze => 0.45,
        HazeRegime::SulfurHaze => 0.55,
        HazeRegime::DustLoaded => 0.2,
    };
    let uv_shielding_fraction = (haze_shielding + ozone_column_relative * 0.25).clamp(0.0, 0.95);

    // Ozone-equivalent shielding: the effective O3 column scaled by whether
    // the stratosphere supports long-lived ozone. Needs O2, a cool stratosphere
    // (strong UV destroys ozone faster than it forms when XUV is extreme), and
    // at least a moderate atmospheric column.
    let ozone_equivalent_shielding = if has_oxygen
        && life_level.as_u8() >= LifeLevel::PlantLike.as_u8()
    {
        // O2 + photosynthesis history. Hot young stars destroy ozone faster
        // than it forms, so apply an XUV penalty.
        let xuv_penalty = if uv_driver > 2.0 {
            0.5
        } else if uv_driver > 1.2 {
            0.75
        } else {
            1.0
        };
        (ozone_column_relative * xuv_penalty * atmospheric_pressure.clamp(0.1, 2.0)).clamp(0.0, 2.0)
    } else {
        0.0
    };
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
        stratospheric_activity,
        tropospheric_activity,
        ozone_column_relative,
        ozone_equivalent_shielding,
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
    fn earth_like_has_high_ozone_equivalent_shielding() {
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
        // Mature oxygenated atmosphere → strong ozone layer.
        assert!(
            photo.ozone_equivalent_shielding > 0.5,
            "got {}",
            photo.ozone_equivalent_shielding
        );
    }

    #[test]
    fn young_star_reduces_ozone_shielding() {
        let composition = vec![
            (0.78, ChemicalComponent::Nitrogen),
            (0.21, ChemicalComponent::Oxygen),
        ];
        // Same world, young (0.5 Gyr) vs. mature (4.6 Gyr) star.
        let young = generate_photochemistry(
            &context(288, 0.5),
            1.0,
            &composition,
            TelluricBodyComposition::Rocky,
            LifeLevel::Sentient,
        )
        .unwrap();
        let mature = generate_photochemistry(
            &context(288, 4.6),
            1.0,
            &composition,
            TelluricBodyComposition::Rocky,
            LifeLevel::Sentient,
        )
        .unwrap();
        assert!(
            young.ozone_equivalent_shielding < mature.ozone_equivalent_shielding,
            "young ozone {} should be < mature ozone {}",
            young.ozone_equivalent_shielding,
            mature.ozone_equivalent_shielding
        );
    }

    #[test]
    fn no_oxygen_means_no_ozone_shielding() {
        let composition = vec![
            (0.95, ChemicalComponent::CarbonDioxide),
            (0.05, ChemicalComponent::Nitrogen),
        ];
        let photo = generate_photochemistry(
            &context(300, 4.6),
            1.0,
            &composition,
            TelluricBodyComposition::Rocky,
            LifeLevel::UniCellular,
        )
        .unwrap();
        assert_eq!(photo.ozone_equivalent_shielding, 0.0);
    }

    #[test]
    fn hot_world_has_active_troposphere() {
        let composition = vec![
            (0.90, ChemicalComponent::CarbonDioxide),
            (0.10, ChemicalComponent::SulfurDioxide),
        ];
        // Venus-like: 737 K, 90 atm, SO2 present.
        let photo = generate_photochemistry(
            &context(737, 4.6),
            90.0,
            &composition,
            TelluricBodyComposition::Rocky,
            LifeLevel::None,
        )
        .unwrap();
        assert!(
            photo.tropospheric_activity >= PhotochemicalActivity::Active,
            "got {:?}",
            photo.tropospheric_activity
        );
    }

    #[test]
    fn cold_thin_atmosphere_has_quiescent_troposphere() {
        // Mars-like: 210 K, 0.006 atm.
        let composition = vec![(0.95, ChemicalComponent::CarbonDioxide)];
        let photo = generate_photochemistry(
            &context(210, 4.6),
            0.006,
            &composition,
            TelluricBodyComposition::Rocky,
            LifeLevel::None,
        );
        // May be None (pressure too low) or Quiescent troposphere.
        if let Some(p) = photo {
            assert_eq!(p.tropospheric_activity, PhotochemicalActivity::Quiescent);
        }
    }

    #[test]
    fn stratospheric_activity_exceeds_tropospheric_for_young_stars() {
        let composition = vec![
            (0.80, ChemicalComponent::Nitrogen),
            (0.20, ChemicalComponent::Oxygen),
        ];
        // Young active star → strong XUV → stratospheric dominance.
        let photo = generate_photochemistry(
            &context(280, 0.3),
            1.0,
            &composition,
            TelluricBodyComposition::Rocky,
            LifeLevel::PlantLike,
        )
        .unwrap();
        assert!(photo.stratospheric_activity >= photo.tropospheric_activity);
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
