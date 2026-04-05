use seeded_dice_roller::SeededDiceRoller;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt::{self, Display};
use std::rc::Rc;

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

/// Returns `(max_temperature_celsius, max_pressure_atm)` that a civilisation
/// at the given tech level can realistically achieve. Callers compose these
/// with their own substance availability to filter accessible recipes.
///
/// Scale is calibrated to recipe prerequisites used in the `crafting` crate:
///   0-1: hand tools, campfires           → ~400 °C
///   2-3: ceramic kilns, bronze casting   → ~1200 °C
///   4-5: iron smelting                   → ~1600 °C
///   6-7: steam era, pressure vessels     → ~2000 °C, 20 atm
///   8-9: industrial revolution           → ~2500 °C, 200 atm
///   10-11: modern electrochemistry       → ~3500 °C, 1000 atm
///   12+:  plasma arcs, ultra-high-press  → ~5000 °C, 10000 atm
pub fn tech_level_capabilities(tech_level: u8) -> (i32, f32) {
    match tech_level {
        0..=1 => (400, 1.0),
        2..=3 => (1200, 1.0),
        4..=5 => (1600, 2.0),
        6..=7 => (2000, 20.0),
        8..=9 => (2500, 200.0),
        10..=11 => (3500, 1000.0),
        _ => (5000, 10_000.0),
    }
}

/// Summary of a species' expansion across its neighbourhood.
///
/// The homeworld is always entry 0; remaining entries are colonies/outposts
/// ordered by distance from the homeworld. Distances are in parsecs.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct ExpansionFootprint {
    /// All settled locations (homeworld first).
    pub settlements: Vec<(f64, Population)>,
    /// Total distinct bodies settled.
    pub settled_body_count: u32,
    /// Reach in parsecs (matches `expansion_reach_parsecs`).
    pub reach_parsecs: u32,
}

/// Generate a species' interstellar expansion footprint: the homeworld plus a
/// set of colonies and outposts scattered within their tech-level reach.
///
/// The number of settlements scales with tech level:
/// - tech 0-7 (pre-interstellar): 1 homeworld + up to 3 in-system outposts
/// - tech 8-9 (early FTL): 3-12 colonies within 1-3 parsecs
/// - tech 10-11 (mature): 15-60 colonies out to 10-50 parsecs
/// - tech 12+ (advanced): 50-200 colonies across the reach
///
/// Deterministic from `(seed, species_name)`.
pub fn generate_expansion_footprint(
    species_name: &str,
    tech_level: u8,
    seed: &str,
) -> ExpansionFootprint {
    let reach = expansion_reach_parsecs(tech_level);
    let mut rng = SeededDiceRoller::new(seed, &format!("expansion_{}", species_name));

    // Always start with the homeworld.
    let mut settlements: Vec<(f64, Population)> = Vec::new();
    let homeworld = generate_colony(species_name, tech_level, 0.0, seed, 0);
    settlements.push((0.0, homeworld));

    // Determine colony count + distance ceiling from tech level.
    let (colony_count, min_dist, max_dist) = match tech_level {
        0..=7 => {
            let extras = rng.roll(1, 4, -1).max(0) as u32; // 0-3 outposts
            (extras, 0.0001, 0.01) // sub-parsec (in-system)
        }
        8 => (rng.roll(1, 6, 2) as u32, 0.5, 1.0),
        9 => (rng.roll(1, 10, 2) as u32, 0.5, 3.0),
        10 => (rng.roll(2, 10, 5) as u32, 0.5, 10.0),
        11 => (rng.roll(3, 20, 10) as u32, 1.0, 50.0),
        _ => (rng.roll(5, 40, 30) as u32, 1.0, 200.0),
    };

    for i in 0..colony_count {
        // Distance drawn with inverse-square bias toward the homeworld: most
        // colonies are closer, a handful push the frontier.
        let u = rng.gen_f64().clamp(0.001, 0.999);
        let dist = min_dist + (max_dist - min_dist) * u.powf(2.0);
        let colony = generate_colony(species_name, tech_level, dist, seed, i + 1);
        settlements.push((dist, colony));
    }

    // Sort by distance for predictable iteration order.
    settlements.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let settled_body_count = settlements.len() as u32;

    ExpansionFootprint {
        settlements,
        settled_body_count,
        reach_parsecs: reach,
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

    #[test]
    fn tech_capabilities_are_monotonic() {
        let mut prev = tech_level_capabilities(0);
        for tech in 1u8..=12 {
            let cur = tech_level_capabilities(tech);
            assert!(
                cur.0 >= prev.0 && cur.1 >= prev.1,
                "tech {}: ({}, {}) regressed from ({}, {})",
                tech,
                cur.0,
                cur.1,
                prev.0,
                prev.1
            );
            prev = cur;
        }
    }

    #[test]
    fn bronze_age_cannot_do_plasma_chemistry() {
        let (temp, _) = tech_level_capabilities(3);
        assert!(temp < 2000);
    }

    #[test]
    fn modern_era_reaches_steel_furnace() {
        let (temp, _) = tech_level_capabilities(9);
        assert!(temp >= 2000);
    }

    #[test]
    fn pre_interstellar_footprint_stays_in_system() {
        let f = generate_expansion_footprint("EarlyFolk", 5, "exp_seed");
        assert_eq!(f.reach_parsecs, 0);
        // First entry is homeworld; all others are in-system outposts.
        for (dist, _) in &f.settlements[1..] {
            assert!(
                *dist < 0.1,
                "pre-FTL species has interstellar colony at {} pc",
                dist
            );
        }
    }

    #[test]
    fn interstellar_footprint_has_colonies_beyond_homeworld() {
        let f = generate_expansion_footprint("StarFolk", 10, "exp_seed");
        assert!(f.reach_parsecs > 0);
        assert!(
            f.settled_body_count > 1,
            "interstellar species should settle multiple bodies"
        );
        // At least one colony past 1 parsec.
        assert!(f
            .settlements
            .iter()
            .any(|(d, _)| *d > 1.0 && *d <= f.reach_parsecs as f64));
    }

    #[test]
    fn footprint_distances_within_reach() {
        let f = generate_expansion_footprint("TestFolk", 12, "seed");
        for (dist, _) in &f.settlements {
            assert!(
                *dist <= f.reach_parsecs as f64 + 0.01,
                "colony at {} exceeds reach {}",
                dist,
                f.reach_parsecs
            );
        }
    }

    #[test]
    fn footprint_is_deterministic() {
        let f1 = generate_expansion_footprint("DetFolk", 10, "abc");
        let f2 = generate_expansion_footprint("DetFolk", 10, "abc");
        assert_eq!(f1.settled_body_count, f2.settled_body_count);
        assert_eq!(f1.settlements.len(), f2.settlements.len());
    }

    #[test]
    fn footprint_sorted_by_distance() {
        let f = generate_expansion_footprint("Sorted", 11, "seed");
        for w in f.settlements.windows(2) {
            assert!(
                w[0].0 <= w[1].0,
                "settlements not sorted: {} > {}",
                w[0].0,
                w[1].0
            );
        }
    }

    #[test]
    fn homeworld_always_at_distance_zero() {
        let f = generate_expansion_footprint("Homely", 10, "seed");
        assert_eq!(f.settlements[0].0, 0.0);
        assert_eq!(
            f.settlements[0].1.settlement_type,
            SettlementType::Homeworld
        );
    }

    #[test]
    fn mature_interstellar_has_many_colonies() {
        let f = generate_expansion_footprint("MatureFolk", 12, "seed");
        assert!(
            f.settled_body_count >= 30,
            "tech 12 should settle ≥30 bodies, got {}",
            f.settled_body_count
        );
    }
}
