use serde::{Serialize, Deserialize};
use std::rc::Rc;
use smart_default::SmartDefault;
use seeded_dice_roller::SeededDiceRoller;
use std::fmt::{self, Display};

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum SettlementType {
    #[default]
    Homeworld,
    Colony,
    Outpost,
    MiningStation,
    ResearchStation,
    MilitaryBase,
}

impl Display for SettlementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SettlementType::Homeworld => "Homeworld",
                SettlementType::Colony => "Colony",
                SettlementType::Outpost => "Outpost",
                SettlementType::MiningStation => "Mining Station",
                SettlementType::ResearchStation => "Research Station",
                SettlementType::MilitaryBase => "Military Base",
            }
        )
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum DevelopmentLevel {
    Frontier,
    #[default]
    Developing,
    Established,
    Thriving,
    Mature,
}

impl Display for DevelopmentLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DevelopmentLevel::Frontier => "Frontier",
                DevelopmentLevel::Developing => "Developing",
                DevelopmentLevel::Established => "Established",
                DevelopmentLevel::Thriving => "Thriving",
                DevelopmentLevel::Mature => "Mature",
            }
        )
    }
}

/// Represents a species presence on a body.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Population {
    /// Name of the species inhabiting this body.
    pub species_name: Rc<str>,
    /// Order of magnitude of population (10^n).
    pub population_order: u8,
    /// Type of settlement.
    pub settlement_type: SettlementType,
    /// How developed is this settlement.
    pub development_level: DevelopmentLevel,
}

/// Determines the expansion reach (in parsecs) of a species based on tech level.
pub fn expansion_reach_parsecs(tech_level: u8) -> u32 {
    match tech_level {
        0..=3 => 0,
        4..=5 => 0,
        6..=7 => 0,
        8 => 1,
        9 => 3,
        10 => 10,
        11 => 50,
        _ => 200,
    }
}

/// Generate a population entry for a colonized body.
pub fn generate_colony(
    species_name: &str,
    tech_level: u8,
    distance_parsecs: f64,
    seed: &str,
    body_id: u32,
) -> Population {
    let mut rng = SeededDiceRoller::new(seed, &format!("colony_{}_{}", species_name, body_id));

    let settlement_type = if distance_parsecs < 0.01 {
        SettlementType::Homeworld
    } else if distance_parsecs < 1.0 {
        match rng.roll(1, 4, 0) {
            1 => SettlementType::Colony,
            2 => SettlementType::MiningStation,
            3 => SettlementType::ResearchStation,
            _ => SettlementType::Outpost,
        }
    } else {
        match rng.roll(1, 6, 0) {
            1..=2 => SettlementType::Colony,
            3 => SettlementType::Outpost,
            4 => SettlementType::MiningStation,
            5 => SettlementType::ResearchStation,
            _ => SettlementType::MilitaryBase,
        }
    };

    let development_level = if distance_parsecs < 0.01 {
        DevelopmentLevel::Mature
    } else {
        let dev_mod = if tech_level > 10 { 2 } else { 0 };
        match rng.roll(1, 6, dev_mod) {
            i64::MIN..=2 => DevelopmentLevel::Frontier,
            3 => DevelopmentLevel::Developing,
            4..=5 => DevelopmentLevel::Established,
            6..=7 => DevelopmentLevel::Thriving,
            _ => DevelopmentLevel::Mature,
        }
    };

    let pop_base = match settlement_type {
        SettlementType::Homeworld => 9 + rng.roll(1, 3, 0) as u8,
        SettlementType::Colony => 5 + rng.roll(1, 4, 0) as u8,
        SettlementType::Outpost => 2 + rng.roll(1, 3, 0) as u8,
        SettlementType::MiningStation => 2 + rng.roll(1, 2, 0) as u8,
        SettlementType::ResearchStation => 1 + rng.roll(1, 2, 0) as u8,
        SettlementType::MilitaryBase => 3 + rng.roll(1, 2, 0) as u8,
    };

    Population {
        species_name: species_name.into(),
        population_order: pop_base.min(12),
        settlement_type,
        development_level,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expansion_reach_increases_with_tech() {
        assert_eq!(expansion_reach_parsecs(0), 0);
        assert!(expansion_reach_parsecs(10) > expansion_reach_parsecs(8));
        assert!(expansion_reach_parsecs(12) > expansion_reach_parsecs(10));
    }

    #[test]
    fn homeworld_is_mature() {
        let pop = generate_colony("TestSpecies", 8, 0.0, "seed", 0);
        assert_eq!(pop.settlement_type, SettlementType::Homeworld);
        assert_eq!(pop.development_level, DevelopmentLevel::Mature);
    }

    #[test]
    fn colony_is_deterministic() {
        let p1 = generate_colony("Sp", 10, 5.0, "seed42", 1);
        let p2 = generate_colony("Sp", 10, 5.0, "seed42", 1);
        assert_eq!(p1.settlement_type, p2.settlement_type);
        assert_eq!(p1.population_order, p2.population_order);
    }

    #[test]
    fn population_order_is_bounded() {
        for i in 0..20 {
            let pop = generate_colony("Sp", 12, 100.0, &format!("seed{}", i), i);
            assert!(pop.population_order <= 12);
        }
    }
}
