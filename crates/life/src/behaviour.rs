//! Creature behaviour tags — per-species profiles derived from biology.
//!
//! Tags influence factory gameplay: territorial species attack nearby
//! buildings, burrowing species block mining, swarming species require
//! area defences, nocturnal species are only dangerous at night, etc.

use crate::species::{BodyPlan, LocomotionType, SizeClass, Species, SpeciesTrait, TrophicLevel};
use serde::{Deserialize, Serialize};

/// Behaviour flags for a species.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BehaviourProfile {
    /// Defends a territory around its nest/den. Attacks intruders.
    pub territorial: bool,
    /// Moves seasonally between regions. Present only part of the year.
    pub migratory: bool,
    /// Active at night; dormant during the day.
    pub nocturnal: bool,
    /// Lives underground. Can block mining operations.
    pub burrowing: bool,
    /// Attacks in large coordinated groups.
    pub swarming: bool,
    /// Delivers venom on contact. Extra hazard for personnel.
    pub venomous: bool,
    /// Ambush predator — hard to detect until attack.
    pub ambush: bool,
    /// Passive unless provoked. Low threat to factory operations.
    pub docile: bool,
}

impl BehaviourProfile {
    /// Number of active danger-related tags (excludes docile/migratory).
    pub fn threat_level(&self) -> u8 {
        self.territorial as u8
            + self.swarming as u8
            + self.venomous as u8
            + self.ambush as u8
            + self.burrowing as u8
    }

    /// Whether this species poses a significant threat to factory operations.
    pub fn is_dangerous(&self) -> bool {
        self.threat_level() >= 2 || self.venomous
    }
}

/// Generate a behaviour profile from a species' biology.
///
/// Rules:
/// - Carnivores → territorial + ambush (large) or swarming (small arthropods).
/// - Herbivores → migratory (large) or docile (small).
/// - Burrowing locomotion → burrowing tag.
/// - Venomous trait → venomous tag.
/// - Arthropods + small → swarming.
/// - Nocturnal for certain body plans in warm biomes (heuristic).
pub fn generate_behaviour(species: &Species) -> BehaviourProfile {
    let mut profile = BehaviourProfile::default();

    // Venomous trait directly maps.
    if species.special_traits.contains(&SpeciesTrait::Venomous) {
        profile.venomous = true;
    }

    // Burrowing locomotion.
    if species.locomotion.contains(&LocomotionType::Burrower) {
        profile.burrowing = true;
    }

    // Trophic level drives core behaviour.
    match species.trophic_level {
        TrophicLevel::Carnivore => {
            profile.territorial = true;
            match species.size_class {
                SizeClass::Large | SizeClass::Huge | SizeClass::Colossal => {
                    profile.ambush = true;
                }
                SizeClass::Tiny | SizeClass::Small | SizeClass::Microscopic => {
                    if species.body_plan == BodyPlan::Arthropod {
                        profile.swarming = true;
                    }
                }
                _ => {}
            }
        }
        TrophicLevel::Herbivore => match species.size_class {
            SizeClass::Large | SizeClass::Huge | SizeClass::Colossal => {
                profile.migratory = true;
            }
            _ => {
                profile.docile = true;
            }
        },
        TrophicLevel::Omnivore => {
            // Omnivores are opportunistic — territorial if medium+.
            if species.size_class >= SizeClass::Medium {
                profile.territorial = true;
            } else {
                profile.docile = true;
            }
        }
        TrophicLevel::Parasite => {
            profile.ambush = true;
        }
        TrophicLevel::Autotroph | TrophicLevel::FilterFeeder => {
            profile.docile = true;
        }
    }

    // Arthropod small → swarming (even if herbivore, locusts are dangerous).
    if species.body_plan == BodyPlan::Arthropod
        && matches!(
            species.size_class,
            SizeClass::Tiny | SizeClass::Small | SizeClass::Microscopic
        )
        && !profile.docile
    {
        profile.swarming = true;
    }

    // Nocturnal heuristic: amorphous and mollusk body plans, or
    // species with bioluminescence.
    if matches!(species.body_plan, BodyPlan::Amorphous | BodyPlan::Mollusk)
        || species
            .special_traits
            .contains(&SpeciesTrait::Bioluminescent)
    {
        profile.nocturnal = true;
    }

    profile
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    fn make_species(
        body: BodyPlan,
        trophic: TrophicLevel,
        size: SizeClass,
        loco: Vec<LocomotionType>,
        traits: Vec<SpeciesTrait>,
    ) -> Species {
        Species {
            name: Rc::from("Test"),
            body_plan: body,
            trophic_level: trophic,
            size_class: size,
            locomotion: loco,
            special_traits: traits,
            ..Default::default()
        }
    }

    #[test]
    fn carnivore_is_territorial() {
        let sp = make_species(
            BodyPlan::Vertebrate,
            TrophicLevel::Carnivore,
            SizeClass::Large,
            vec![LocomotionType::Walker],
            vec![],
        );
        let b = generate_behaviour(&sp);
        assert!(b.territorial);
        assert!(b.ambush); // large carnivore
    }

    #[test]
    fn small_arthropod_carnivore_swarms() {
        let sp = make_species(
            BodyPlan::Arthropod,
            TrophicLevel::Carnivore,
            SizeClass::Tiny,
            vec![LocomotionType::Walker],
            vec![],
        );
        let b = generate_behaviour(&sp);
        assert!(b.swarming);
        assert!(b.territorial);
    }

    #[test]
    fn large_herbivore_is_migratory() {
        let sp = make_species(
            BodyPlan::Vertebrate,
            TrophicLevel::Herbivore,
            SizeClass::Large,
            vec![LocomotionType::Walker],
            vec![],
        );
        let b = generate_behaviour(&sp);
        assert!(b.migratory);
        assert!(!b.territorial);
    }

    #[test]
    fn small_herbivore_is_docile() {
        let sp = make_species(
            BodyPlan::Vertebrate,
            TrophicLevel::Herbivore,
            SizeClass::Small,
            vec![LocomotionType::Walker],
            vec![],
        );
        let b = generate_behaviour(&sp);
        assert!(b.docile);
        assert!(!b.is_dangerous());
    }

    #[test]
    fn venomous_trait_maps_to_tag() {
        let sp = make_species(
            BodyPlan::Arthropod,
            TrophicLevel::Carnivore,
            SizeClass::Small,
            vec![LocomotionType::Walker],
            vec![SpeciesTrait::Venomous],
        );
        let b = generate_behaviour(&sp);
        assert!(b.venomous);
        assert!(b.is_dangerous());
    }

    #[test]
    fn burrower_gets_burrowing_tag() {
        let sp = make_species(
            BodyPlan::Mollusk,
            TrophicLevel::Herbivore,
            SizeClass::Medium,
            vec![LocomotionType::Burrower],
            vec![],
        );
        let b = generate_behaviour(&sp);
        assert!(b.burrowing);
    }

    #[test]
    fn autotroph_is_docile() {
        let sp = make_species(
            BodyPlan::PlantLike,
            TrophicLevel::Autotroph,
            SizeClass::Medium,
            vec![LocomotionType::Sessile],
            vec![],
        );
        let b = generate_behaviour(&sp);
        assert!(b.docile);
        assert_eq!(b.threat_level(), 0);
    }

    #[test]
    fn amorphous_is_nocturnal() {
        let sp = make_species(
            BodyPlan::Amorphous,
            TrophicLevel::Omnivore,
            SizeClass::Medium,
            vec![LocomotionType::Walker],
            vec![],
        );
        let b = generate_behaviour(&sp);
        assert!(b.nocturnal);
    }

    #[test]
    fn bioluminescent_is_nocturnal() {
        let sp = make_species(
            BodyPlan::Vertebrate,
            TrophicLevel::Carnivore,
            SizeClass::Medium,
            vec![LocomotionType::Swimmer],
            vec![SpeciesTrait::Bioluminescent],
        );
        let b = generate_behaviour(&sp);
        assert!(b.nocturnal);
    }

    #[test]
    fn threat_level_counts_danger_tags() {
        let sp = make_species(
            BodyPlan::Arthropod,
            TrophicLevel::Carnivore,
            SizeClass::Tiny,
            vec![LocomotionType::Burrower],
            vec![SpeciesTrait::Venomous],
        );
        let b = generate_behaviour(&sp);
        // territorial + swarming + venomous + burrowing = 4
        assert!(b.threat_level() >= 3);
        assert!(b.is_dangerous());
    }
}
