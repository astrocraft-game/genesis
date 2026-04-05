pub use crate::types::{
    CelestialBodyWorldType, ChemicalComponent, LifeLevel, LiquidType, NutrientRichness,
    OceanChemistry, OceanIronContent, OceanRedoxState, OceanStratification, OrbitContext,
    PlanetSimulationInput, TelluricBodyComposition,
};

pub fn generate_ocean_chemistry(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    atmospheric_composition: &[(f32, ChemicalComponent)],
    _body_type: TelluricBodyComposition,
    world_type: CelestialBodyWorldType,
    hydrosphere: f32,
    volcanism: f32,
    tectonic_activity: f32,
    land_fraction: f32,
    life_level: LifeLevel,
) -> Option<OceanChemistry> {
    if hydrosphere <= 5.0 {
        return None;
    }

    let has_oxygen = atmospheric_composition
        .iter()
        .any(|(f, c)| *c == ChemicalComponent::Oxygen && *f > 0.1);
    let has_h2s = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::HydrogenSulfide);
    let has_methane = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Methane);
    let has_ammonia = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Ammonia);

    let liquid = match world_type {
        CelestialBodyWorldType::Ammonia => LiquidType::Ammonia,
        CelestialBodyWorldType::LavaWorld => LiquidType::Magma,
        _ if context.blackbody_temp_k < 150 => LiquidType::MethaneEthane,
        _ if hydrosphere < 15.0 && context.blackbody_temp_k > 310 => LiquidType::Brine,
        _ => LiquidType::Water,
    };

    let salinity_g_per_kg = if liquid == LiquidType::Brine {
        140.0 + hydrosphere.min(20.0) * 2.0 + volcanism * 0.3
    } else {
        (18.0 + volcanism * 0.35 + tectonic_activity * 0.15 + land_fraction * 20.0)
            .clamp(10.0, 70.0)
    };

    let ph = if liquid == LiquidType::Water {
        let co2_pp = atmospheric_composition
            .iter()
            .filter(|(_, c)| *c == ChemicalComponent::CarbonDioxide)
            .map(|(f, _)| f * atmospheric_pressure)
            .sum::<f32>()
            .max(0.0001);
        (8.1 - 0.8 * (co2_pp / 0.0004).log10()).clamp(4.0, 10.0)
    } else {
        0.0
    };

    let alkalinity_meq_l = if liquid == LiquidType::Water || liquid == LiquidType::Brine {
        let carbonate_buffer = if tectonic_activity > 10.0 { 1.4 } else { 1.0 };
        let evaporative_boost = if liquid == LiquidType::Brine {
            2.0
        } else {
            1.0
        };
        (1.2 + salinity_g_per_kg / 40.0
            + hydrosphere / 140.0
            + carbonate_buffer * evaporative_boost)
            .clamp(0.5, 12.0)
    } else {
        0.0
    };

    let anoxic = !has_oxygen || life_level.as_u8() < LifeLevel::PlantLike.as_u8();
    let iron_content = if !anoxic {
        OceanIronContent::Negligible
    } else if life_level.as_u8() >= LifeLevel::UniCellular.as_u8() {
        OceanIronContent::Moderate
    } else if volcanism > 20.0 {
        OceanIronContent::High
    } else {
        OceanIronContent::Low
    };
    let hydrothermal_vents = volcanism > 10.0 && hydrosphere > 20.0;

    let redox_state = if !anoxic {
        OceanRedoxState::Oxic
    } else if has_h2s || (has_methane && volcanism > 15.0) {
        OceanRedoxState::Euxinic
    } else if life_level.as_u8() >= LifeLevel::UniCellular.as_u8() || hydrothermal_vents {
        OceanRedoxState::Reducing
    } else {
        OceanRedoxState::Dysoxic
    };

    let nutrient_index = volcanism * 0.4
        + tectonic_activity * 0.3
        + if hydrothermal_vents { 20.0 } else { 0.0 }
        + hydrosphere * 0.1
        + if land_fraction > 0.2 { 10.0 } else { 0.0 };
    let nutrient_richness = if nutrient_index > 70.0 {
        NutrientRichness::BloomProne
    } else if nutrient_index > 45.0 {
        NutrientRichness::Fertile
    } else if nutrient_index > 22.0 {
        NutrientRichness::Moderate
    } else if nutrient_index > 8.0 {
        NutrientRichness::Limited
    } else {
        NutrientRichness::Starved
    };

    let stratification = if liquid == LiquidType::Brine || salinity_g_per_kg > 80.0 {
        OceanStratification::StronglyStratified
    } else if context.orbit.tidally_locked && context.orbit.rotation_period_days > 10.0 {
        OceanStratification::Layered
    } else if hydrosphere > 40.0 && context.orbit.axial_tilt_deg > 10.0 {
        OceanStratification::Seasonal
    } else {
        OceanStratification::WellMixed
    };

    let dissolved_volatile_load = (atmospheric_pressure * 0.8
        + volcanism * 0.35
        + if liquid == LiquidType::Brine {
            8.0
        } else {
            0.0
        }
        + if has_methane { 4.0 } else { 0.0 }
        + if has_ammonia { 3.0 } else { 0.0 })
    .clamp(0.0, 100.0);

    Some(OceanChemistry {
        liquid_type: liquid,
        salinity_g_per_kg,
        ph,
        alkalinity_meq_l,
        anoxic,
        redox_state,
        iron_content,
        nutrient_richness,
        stratification,
        dissolved_volatile_load,
        hydrothermal_vents,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(temp: u32) -> PlanetSimulationInput {
        PlanetSimulationInput {
            blackbody_temp_k: temp,
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn earth_like_ocean_is_oxic_and_buffered() {
        let composition = vec![
            (0.78, ChemicalComponent::Nitrogen),
            (0.21, ChemicalComponent::Oxygen),
            (0.01, ChemicalComponent::Argon),
        ];
        let ocean = generate_ocean_chemistry(
            &context(288),
            1.0,
            &composition,
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Terrestrial,
            71.0,
            30.0,
            35.0,
            0.29,
            LifeLevel::Sentient,
        )
        .unwrap();
        assert_eq!(ocean.redox_state, OceanRedoxState::Oxic);
        assert!(ocean.alkalinity_meq_l > 1.0);
        assert!(matches!(
            ocean.nutrient_richness,
            NutrientRichness::Moderate | NutrientRichness::Fertile | NutrientRichness::BloomProne
        ));
    }

    #[test]
    fn reducing_volcanic_ocean_shows_iron_and_volatiles() {
        let composition = vec![
            (0.8, ChemicalComponent::Nitrogen),
            (0.12, ChemicalComponent::CarbonDioxide),
            (0.08, ChemicalComponent::Methane),
        ];
        let ocean = generate_ocean_chemistry(
            &context(305),
            2.5,
            &composition,
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Ocean,
            55.0,
            35.0,
            18.0,
            0.45,
            LifeLevel::UniCellular,
        )
        .unwrap();
        assert!(matches!(
            ocean.redox_state,
            OceanRedoxState::Reducing | OceanRedoxState::Euxinic
        ));
        assert!(ocean.iron_content >= OceanIronContent::Moderate);
        assert!(ocean.dissolved_volatile_load > 10.0);
    }
}
