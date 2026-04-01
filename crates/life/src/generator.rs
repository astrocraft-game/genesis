use serde::{Serialize, Deserialize};
use smart_default::SmartDefault;
use seeded_dice_roller::SeededDiceRoller;
use std::rc::Rc;
use crate::species::*;
use crate::types::LifeLevel;
use crate::world_types::*;

/// Generates a species based on the conditions of its homeworld.
pub fn generate_species_from_world(
    world_type: CelestialBodyWorldType,
    climate: WorldClimateType,
    temperature_category: WorldTemperatureCategory,
    gravity: f32,
    atmospheric_pressure: f32,
    hydrosphere: f32,
    life_level: LifeLevel,
    seed: &str,
    coord: SpaceCoordinates,
    system_index: u16,
    star_id: u32,
    body_id: u32,
) -> Option<Species> {
    if life_level.as_u8() < LifeLevel::AnimalLike.as_u8() {
        return None;
    }

    let mut rng = SeededDiceRoller::new(
        seed,
        &format!(
            "sys_{}_{}_str_{}_bdy{}_species",
            coord, system_index, star_id, body_id
        ),
    );

    let biochemistry = match world_type {
        CelestialBodyWorldType::Ammonia => Biochemistry::Ammonia,
        _ => {
            if atmospheric_pressure < 0.01 {
                Biochemistry::Silicon
            } else {
                Biochemistry::CarbonWater
            }
        }
    };

    let is_aquatic = hydrosphere > 80.0;
    let is_high_g = gravity > 1.5;

    let body_plan = if is_aquatic {
        match rng.roll(1, 3, 0) {
            1 => BodyPlan::Mollusk,
            2 => BodyPlan::Vertebrate,
            _ => BodyPlan::Amorphous,
        }
    } else if is_high_g {
        match rng.roll(1, 3, 0) {
            1 => BodyPlan::Arthropod,
            _ => BodyPlan::Vertebrate,
        }
    } else {
        match rng.roll(1, 5, 0) {
            1 => BodyPlan::Arthropod,
            2 => BodyPlan::Mollusk,
            3 => BodyPlan::PlantLike,
            _ => BodyPlan::Vertebrate,
        }
    };

    let mut locomotion = Vec::new();
    if is_aquatic {
        locomotion.push(LocomotionType::Swimmer);
        if rng.roll(1, 3, 0) == 1 {
            locomotion.push(LocomotionType::Walker);
        }
    } else {
        locomotion.push(LocomotionType::Walker);
        if gravity < 0.8 && rng.roll(1, 3, 0) == 1 {
            locomotion.push(LocomotionType::Flyer);
        }
        if hydrosphere > 20.0 && rng.roll(1, 4, 0) == 1 {
            locomotion.push(LocomotionType::Swimmer);
        }
    }

    let trophic_level = match rng.roll(1, 6, 0) {
        1 => TrophicLevel::Herbivore,
        2..=3 => TrophicLevel::Omnivore,
        4 => TrophicLevel::Carnivore,
        5 => TrophicLevel::FilterFeeder,
        _ => TrophicLevel::Autotroph,
    };

    let size_class = {
        let g_mod = if is_high_g { -1 } else if gravity < 0.5 { 1 } else { 0 };
        match rng.roll(1, 6, g_mod) {
            i64::MIN..=1 => SizeClass::Tiny,
            2 => SizeClass::Small,
            3..=4 => SizeClass::Medium,
            5 => SizeClass::Large,
            _ => SizeClass::Huge,
        }
    };

    let social_structure = match rng.roll(1, 6, 0) {
        1 => SocialStructure::Solitary,
        2 => SocialStructure::Pair,
        3 => SocialStructure::Pack,
        4 => SocialStructure::Herd,
        5 => SocialStructure::Hive,
        _ => SocialStructure::Collective,
    };

    let reproduction = match rng.roll(1, 4, 0) {
        1 => ReproductionType::Asexual,
        2 => ReproductionType::Hermaphroditic,
        _ => ReproductionType::Sexual,
    };

    let tech_level = if life_level == LifeLevel::Sentient {
        Some(rng.roll(1, 12, 0) as u8)
    } else {
        None
    };

    let lifespan_years = match size_class {
        SizeClass::Microscopic | SizeClass::Tiny => rng.roll(1, 10, 5) as f32,
        SizeClass::Small => rng.roll(1, 20, 10) as f32,
        SizeClass::Medium => rng.roll(1, 60, 20) as f32,
        SizeClass::Large => rng.roll(1, 100, 40) as f32,
        SizeClass::Huge | SizeClass::Colossal => rng.roll(1, 200, 60) as f32,
    };

    // Temperature preference from homeworld climate
    let (temp_low, temp_high) = match temperature_category {
        WorldTemperatureCategory::Frozen => (150.0, 240.0),
        WorldTemperatureCategory::VeryCold => (200.0, 260.0),
        WorldTemperatureCategory::Cold => (230.0, 280.0),
        WorldTemperatureCategory::Chilly => (250.0, 295.0),
        WorldTemperatureCategory::Cool => (265.0, 305.0),
        WorldTemperatureCategory::Temperate => (278.0, 318.0),
        WorldTemperatureCategory::Warm => (290.0, 330.0),
        WorldTemperatureCategory::Hot => (305.0, 350.0),
        WorldTemperatureCategory::VeryHot => (320.0, 380.0),
        WorldTemperatureCategory::Scorching => (350.0, 420.0),
        WorldTemperatureCategory::Infernal => (380.0, 500.0),
    };

    let grav_range = ((gravity * 0.7).max(0.05), gravity * 1.4);

    let mut special_traits = Vec::new();
    if rng.roll(1, 10, 0) == 1 {
        special_traits.push(SpeciesTrait::Psionic);
    }
    if social_structure == SocialStructure::Hive && rng.roll(1, 3, 0) == 1 {
        special_traits.push(SpeciesTrait::HiveMind);
    }
    if is_aquatic && rng.roll(1, 4, 0) == 1 {
        special_traits.push(SpeciesTrait::Bioluminescent);
    }
    if rng.roll(1, 6, 0) == 1 {
        special_traits.push(SpeciesTrait::Regenerative);
    }
    if is_high_g {
        special_traits.push(SpeciesTrait::Armored);
    }
    special_traits.truncate(3);

    // Generate name
    let name = generate_species_name(&mut rng);

    Some(Species {
        name: name.into(),
        biochemistry,
        body_plan,
        locomotion,
        trophic_level,
        size_class,
        reproduction,
        social_structure,
        intelligence: life_level.as_u8(),
        tech_level,
        lifespan_years,
        preferred_temp_range: (temp_low, temp_high),
        preferred_gravity_range: grav_range,
        special_traits,
    })
}

fn generate_species_name(rng: &mut SeededDiceRoller) -> String {
    const PREFIXES: &[&str] = &[
        "Ax", "Br", "Ch", "Dr", "El", "Fr", "Gr", "Hy", "Ix", "Kr", "Ly", "Mn", "Nr", "Ox",
        "Pr", "Qu", "Rh", "Sk", "Th", "Vr", "Wr", "Xy", "Zr",
    ];
    const MIDDLES: &[&str] = &[
        "al", "en", "il", "or", "un", "ar", "el", "ir", "os", "ur", "an", "em", "in", "ov",
    ];
    const SUFFIXES: &[&str] = &[
        "id", "an", "ix", "us", "on", "ar", "is", "um", "ax", "os", "ek", "al",
    ];

    let prefix = PREFIXES[rng.gen_usize() % PREFIXES.len()];
    let middle = MIDDLES[rng.gen_usize() % MIDDLES.len()];
    let suffix = SUFFIXES[rng.gen_usize() % SUFFIXES.len()];
    format!("{}{}{}", prefix, middle, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_species_for_terrestrial_world() {
        let species = generate_species_from_world(
            CelestialBodyWorldType::Terrestrial,
            WorldClimateType::Terrestrial,
            WorldTemperatureCategory::Temperate,
            1.0,
            1.0,
            70.0,
            LifeLevel::Sentient,
            "test_seed",
            SpaceCoordinates::new(0, 0, 0),
            0,
            0,
            0,
        );
        assert!(species.is_some());
        let s = species.unwrap();
        assert_eq!(s.biochemistry, Biochemistry::CarbonWater);
        assert!(s.tech_level.is_some());
        assert!(s.name.len() >= 4);
    }

    #[test]
    fn no_species_for_low_life_level() {
        let species = generate_species_from_world(
            CelestialBodyWorldType::Rock,
            WorldClimateType::Dead,
            WorldTemperatureCategory::Frozen,
            0.3,
            0.0,
            0.0,
            LifeLevel::UniCellular,
            "test_seed",
            SpaceCoordinates::new(0, 0, 0),
            0,
            0,
            0,
        );
        assert!(species.is_none());
    }

    #[test]
    fn species_deterministic() {
        let s1 = generate_species_from_world(
            CelestialBodyWorldType::Terrestrial,
            WorldClimateType::Terrestrial,
            WorldTemperatureCategory::Temperate,
            1.0, 1.0, 70.0,
            LifeLevel::Sentient,
            "seed42",
            SpaceCoordinates::new(1, 2, 3),
            0, 0, 0,
        );
        let s2 = generate_species_from_world(
            CelestialBodyWorldType::Terrestrial,
            WorldClimateType::Terrestrial,
            WorldTemperatureCategory::Temperate,
            1.0, 1.0, 70.0,
            LifeLevel::Sentient,
            "seed42",
            SpaceCoordinates::new(1, 2, 3),
            0, 0, 0,
        );
        assert_eq!(s1.unwrap().name, s2.unwrap().name);
    }
}
