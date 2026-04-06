//! Mutation and adaptation — species respond to environmental changes.
//!
//! When the environment shifts (pollution, climate change, disaster),
//! species in affected tiles may mutate. Mutations alter traits and
//! can be beneficial (species becomes more dangerous) or maladaptive
//! (species declines toward extinction).

use crate::species::{SizeClass, Species, SpeciesTrait};
use seeded_dice_roller::SeededDiceRoller;
use serde::{Deserialize, Serialize};

/// What triggered the mutation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MutationTrigger {
    Pollution,
    ClimateWarming,
    ClimateCooling,
    Disaster,
    RadiationExposure,
    HabitatLoss,
}

/// What changed in the species.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TraitChange {
    /// Gained a new special trait.
    GainedTrait(SpeciesTrait),
    /// Lost a special trait.
    LostTrait(SpeciesTrait),
    /// Size class shifted up.
    SizeIncrease,
    /// Size class shifted down.
    SizeDecrease,
    /// Temperature tolerance widened.
    TempToleranceWidened,
    /// Became more aggressive (increased danger to player).
    AggressionIncrease,
    /// Maladaptive: population decline, risk of extinction.
    Maladaptive,
}

/// A single mutation event applied to a species.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mutation {
    pub trigger: MutationTrigger,
    pub change: TraitChange,
    /// Which generation this mutation appeared in.
    pub generation: u32,
    /// Narrative description.
    pub description: String,
}

/// Accumulated mutations for a species.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MutationLog {
    pub mutations: Vec<Mutation>,
}

impl MutationLog {
    pub fn is_empty(&self) -> bool {
        self.mutations.is_empty()
    }

    /// Count of maladaptive mutations (extinction risk indicator).
    pub fn maladaptive_count(&self) -> usize {
        self.mutations
            .iter()
            .filter(|m| m.change == TraitChange::Maladaptive)
            .count()
    }

    /// All gained traits from mutations.
    pub fn gained_traits(&self) -> Vec<SpeciesTrait> {
        self.mutations
            .iter()
            .filter_map(|m| {
                if let TraitChange::GainedTrait(t) = m.change {
                    Some(t)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Roll for mutations on a species given an environmental pressure.
///
/// - `species`: the species to potentially mutate.
/// - `trigger`: what environmental change is driving the pressure.
/// - `severity`: 0.0–1.0 intensity of the environmental change.
/// - `generation`: current generation counter.
/// - `seed`: deterministic seed.
///
/// Returns a list of mutations (may be empty if the species resists).
pub fn roll_mutations(
    species: &Species,
    trigger: MutationTrigger,
    severity: f32,
    generation: u32,
    seed: &str,
) -> Vec<Mutation> {
    let mut rng = SeededDiceRoller::new(seed, &format!("mut_{}_{}", species.name, generation));
    let mut results = Vec::new();

    // Base mutation chance scales with severity.
    let chance = severity.clamp(0.0, 1.0) * 0.6; // max 60% at severity 1.0
    if rng.gen_f64() as f32 > chance {
        return results;
    }

    // Determine mutation outcome based on trigger.
    let change = match trigger {
        MutationTrigger::Pollution => match rng.roll(1, 5, 0) {
            1 => TraitChange::GainedTrait(SpeciesTrait::Venomous),
            2 => TraitChange::GainedTrait(SpeciesTrait::Armored),
            3 => TraitChange::Maladaptive,
            4 => TraitChange::SizeDecrease,
            _ => TraitChange::GainedTrait(SpeciesTrait::Regenerative),
        },
        MutationTrigger::ClimateWarming => match rng.roll(1, 4, 0) {
            1 => TraitChange::SizeDecrease,
            2 => TraitChange::TempToleranceWidened,
            3 => TraitChange::Maladaptive,
            _ => TraitChange::GainedTrait(SpeciesTrait::ShortLived),
        },
        MutationTrigger::ClimateCooling => match rng.roll(1, 4, 0) {
            1 => TraitChange::SizeIncrease,
            2 => TraitChange::TempToleranceWidened,
            3 => TraitChange::GainedTrait(SpeciesTrait::Armored),
            _ => TraitChange::Maladaptive,
        },
        MutationTrigger::Disaster => match rng.roll(1, 4, 0) {
            1 => TraitChange::GainedTrait(SpeciesTrait::Regenerative),
            2 => TraitChange::AggressionIncrease,
            3 => TraitChange::Maladaptive,
            _ => TraitChange::SizeDecrease,
        },
        MutationTrigger::RadiationExposure => match rng.roll(1, 5, 0) {
            1 => TraitChange::GainedTrait(SpeciesTrait::Bioluminescent),
            2 => TraitChange::GainedTrait(SpeciesTrait::Metamorphic),
            3 => TraitChange::Maladaptive,
            4 => TraitChange::Maladaptive,
            _ => TraitChange::GainedTrait(SpeciesTrait::Psionic),
        },
        MutationTrigger::HabitatLoss => match rng.roll(1, 4, 0) {
            1 => TraitChange::SizeDecrease,
            2 => TraitChange::Maladaptive,
            3 => TraitChange::Maladaptive,
            _ => TraitChange::TempToleranceWidened,
        },
    };

    let desc = describe_mutation(species, trigger, &change);
    results.push(Mutation {
        trigger,
        change,
        generation,
        description: desc,
    });

    results
}

/// Apply mutations to a species in place. Returns the mutations applied.
pub fn apply_mutations(species: &mut Species, mutations: &[Mutation]) {
    for m in mutations {
        match m.change {
            TraitChange::GainedTrait(t) => {
                if !species.special_traits.contains(&t) {
                    species.special_traits.push(t);
                }
            }
            TraitChange::LostTrait(t) => {
                species.special_traits.retain(|&existing| existing != t);
            }
            TraitChange::SizeIncrease => {
                species.size_class = size_up(species.size_class);
            }
            TraitChange::SizeDecrease => {
                species.size_class = size_down(species.size_class);
            }
            TraitChange::TempToleranceWidened => {
                species.preferred_temp_range.0 -= 10.0;
                species.preferred_temp_range.1 += 10.0;
            }
            TraitChange::AggressionIncrease | TraitChange::Maladaptive => {
                // These are metadata — no direct trait change, but the
                // game engine can read the mutation log to adjust behaviour.
            }
        }
    }
}

fn size_up(s: SizeClass) -> SizeClass {
    match s {
        SizeClass::Microscopic => SizeClass::Tiny,
        SizeClass::Tiny => SizeClass::Small,
        SizeClass::Small => SizeClass::Medium,
        SizeClass::Medium => SizeClass::Large,
        SizeClass::Large => SizeClass::Huge,
        SizeClass::Huge | SizeClass::Colossal => SizeClass::Colossal,
    }
}

fn size_down(s: SizeClass) -> SizeClass {
    match s {
        SizeClass::Colossal => SizeClass::Huge,
        SizeClass::Huge => SizeClass::Large,
        SizeClass::Large => SizeClass::Medium,
        SizeClass::Medium => SizeClass::Small,
        SizeClass::Small => SizeClass::Tiny,
        SizeClass::Tiny | SizeClass::Microscopic => SizeClass::Microscopic,
    }
}

fn describe_mutation(species: &Species, trigger: MutationTrigger, change: &TraitChange) -> String {
    let trigger_str = match trigger {
        MutationTrigger::Pollution => "environmental pollution",
        MutationTrigger::ClimateWarming => "rising temperatures",
        MutationTrigger::ClimateCooling => "falling temperatures",
        MutationTrigger::Disaster => "a natural disaster",
        MutationTrigger::RadiationExposure => "radiation exposure",
        MutationTrigger::HabitatLoss => "habitat destruction",
    };
    let change_str = match change {
        TraitChange::GainedTrait(t) => format!("developed {:?} trait", t),
        TraitChange::LostTrait(t) => format!("lost {:?} trait", t),
        TraitChange::SizeIncrease => "grew larger".into(),
        TraitChange::SizeDecrease => "became smaller".into(),
        TraitChange::TempToleranceWidened => "widened temperature tolerance".into(),
        TraitChange::AggressionIncrease => "became more aggressive".into(),
        TraitChange::Maladaptive => "suffered maladaptive mutation".into(),
    };
    format!(
        "{} {} in response to {}.",
        species.name, change_str, trigger_str
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::species::{BodyPlan, SizeClass, TrophicLevel};
    use std::rc::Rc;

    fn sample_species() -> Species {
        Species {
            name: Rc::from("Testoid"),
            body_plan: BodyPlan::Arthropod,
            trophic_level: TrophicLevel::Carnivore,
            size_class: SizeClass::Medium,
            special_traits: vec![SpeciesTrait::Venomous],
            preferred_temp_range: (250.0, 310.0),
            ..Default::default()
        }
    }

    #[test]
    fn no_mutation_at_zero_severity() {
        let sp = sample_species();
        let muts = roll_mutations(&sp, MutationTrigger::Pollution, 0.0, 1, "zero");
        assert!(muts.is_empty());
    }

    #[test]
    fn high_severity_likely_produces_mutation() {
        let sp = sample_species();
        let mut any = false;
        for i in 0..20 {
            let muts = roll_mutations(&sp, MutationTrigger::Pollution, 1.0, i, "high");
            if !muts.is_empty() {
                any = true;
                break;
            }
        }
        assert!(
            any,
            "expected at least one mutation across 20 attempts at severity 1.0"
        );
    }

    #[test]
    fn mutation_has_description() {
        let sp = sample_species();
        for i in 0..50 {
            let muts = roll_mutations(&sp, MutationTrigger::Disaster, 0.9, i, "desc");
            for m in &muts {
                assert!(!m.description.is_empty());
                assert!(m.description.contains("Testoid"));
            }
        }
    }

    #[test]
    fn apply_gained_trait_adds_to_species() {
        let mut sp = sample_species();
        assert!(!sp.special_traits.contains(&SpeciesTrait::Armored));
        let m = Mutation {
            trigger: MutationTrigger::Pollution,
            change: TraitChange::GainedTrait(SpeciesTrait::Armored),
            generation: 1,
            description: String::new(),
        };
        apply_mutations(&mut sp, &[m]);
        assert!(sp.special_traits.contains(&SpeciesTrait::Armored));
    }

    #[test]
    fn apply_size_increase_shifts_class() {
        let mut sp = sample_species();
        assert_eq!(sp.size_class, SizeClass::Medium);
        let m = Mutation {
            trigger: MutationTrigger::ClimateCooling,
            change: TraitChange::SizeIncrease,
            generation: 1,
            description: String::new(),
        };
        apply_mutations(&mut sp, &[m]);
        assert_eq!(sp.size_class, SizeClass::Large);
    }

    #[test]
    fn apply_temp_tolerance_widens_range() {
        let mut sp = sample_species();
        let before = sp.preferred_temp_range;
        let m = Mutation {
            trigger: MutationTrigger::ClimateWarming,
            change: TraitChange::TempToleranceWidened,
            generation: 1,
            description: String::new(),
        };
        apply_mutations(&mut sp, &[m]);
        assert!(sp.preferred_temp_range.0 < before.0);
        assert!(sp.preferred_temp_range.1 > before.1);
    }

    #[test]
    fn maladaptive_count_tracks_decline() {
        let mut log = MutationLog::default();
        assert_eq!(log.maladaptive_count(), 0);
        log.mutations.push(Mutation {
            trigger: MutationTrigger::HabitatLoss,
            change: TraitChange::Maladaptive,
            generation: 1,
            description: String::new(),
        });
        log.mutations.push(Mutation {
            trigger: MutationTrigger::Pollution,
            change: TraitChange::GainedTrait(SpeciesTrait::Regenerative),
            generation: 2,
            description: String::new(),
        });
        log.mutations.push(Mutation {
            trigger: MutationTrigger::RadiationExposure,
            change: TraitChange::Maladaptive,
            generation: 3,
            description: String::new(),
        });
        assert_eq!(log.maladaptive_count(), 2);
    }

    #[test]
    fn size_bounds_dont_overflow() {
        assert_eq!(size_up(SizeClass::Colossal), SizeClass::Colossal);
        assert_eq!(size_down(SizeClass::Microscopic), SizeClass::Microscopic);
    }

    #[test]
    fn gained_traits_extracted_from_log() {
        let mut log = MutationLog::default();
        log.mutations.push(Mutation {
            trigger: MutationTrigger::Pollution,
            change: TraitChange::GainedTrait(SpeciesTrait::Armored),
            generation: 1,
            description: String::new(),
        });
        log.mutations.push(Mutation {
            trigger: MutationTrigger::Disaster,
            change: TraitChange::SizeDecrease,
            generation: 2,
            description: String::new(),
        });
        let gained = log.gained_traits();
        assert_eq!(gained, vec![SpeciesTrait::Armored]);
    }
}
