//! Species evolution timeline — explains how each species got its traits.
//!
//! Takes a species and the planet's geological timeline, and produces a
//! sequence of adaptation events that rationalise the species' current
//! body plan, size, locomotion, and special traits as responses to
//! environmental pressures over deep time.

use crate::history::{PlanetaryEventKind, PlanetaryTimeline};
use crate::species::{BodyPlan, LocomotionType, SizeClass, Species, SpeciesTrait, TrophicLevel};
use serde::{Deserialize, Serialize};

/// A single evolutionary adaptation event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdaptationEvent {
    /// Millions of years ago.
    pub mya: f64,
    /// What changed in the species.
    pub kind: AdaptationKind,
    /// Narrative explanation.
    pub description: String,
}

/// Categories of evolutionary adaptation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AdaptationKind {
    BodyPlanShift,
    SizeChange,
    NewLocomotion,
    TrophicShift,
    TraitGained,
    SpeciationBurst,
    NearExtinction,
}

/// Full evolutionary history for a single species.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EvolutionHistory {
    pub species_name: String,
    pub events: Vec<AdaptationEvent>,
}

/// Generate an evolution history for a species based on the planetary timeline.
///
/// The algorithm walks the planetary timeline and, for each relevant event,
/// generates an adaptation that rationalises one of the species' current traits.
/// Events without a matching planetary trigger are attributed to background
/// natural selection.
pub fn generate_evolution_history(
    species: &Species,
    timeline: &PlanetaryTimeline,
) -> EvolutionHistory {
    let mut events = Vec::new();

    // Find key planetary events to anchor adaptations.
    let mass_extinctions: Vec<f64> = timeline
        .events
        .iter()
        .filter(|e| e.kind == PlanetaryEventKind::MassExtinction)
        .map(|e| e.mya)
        .collect();

    let ice_ages: Vec<f64> = timeline
        .events
        .iter()
        .filter(|e| e.kind == PlanetaryEventKind::IceAge)
        .map(|e| e.mya)
        .collect();

    let first_land = timeline
        .events
        .iter()
        .find(|e| e.kind == PlanetaryEventKind::FirstLandLife)
        .map(|e| e.mya);

    let cambrian = timeline
        .events
        .iter()
        .find(|e| e.kind == PlanetaryEventKind::CambrianExplosion)
        .map(|e| e.mya);

    // 1. Origin: body plan established during or after Cambrian explosion.
    let origin_mya = cambrian.unwrap_or(500.0) - 10.0;
    events.push(AdaptationEvent {
        mya: origin_mya.max(1.0),
        kind: AdaptationKind::BodyPlanShift,
        description: format!(
            "Ancestral {} body plan emerged during rapid diversification.",
            body_plan_label(species.body_plan),
        ),
    });

    // 2. Post-extinction speciation burst (if any extinction happened).
    if let Some(&ext_mya) = mass_extinctions.first() {
        events.push(AdaptationEvent {
            mya: ext_mya - 5.0,
            kind: AdaptationKind::NearExtinction,
            description: "Population bottleneck during mass extinction event.".into(),
        });
        events.push(AdaptationEvent {
            mya: ext_mya - 10.0,
            kind: AdaptationKind::SpeciationBurst,
            description: "Rapid radiation into empty ecological niches after extinction.".into(),
        });
    }

    // 3. Size adaptation — ice ages drive size changes (Bergmann's rule).
    if let Some(&ice_mya) = ice_ages.first() {
        let desc = match species.size_class {
            SizeClass::Microscopic | SizeClass::Tiny => {
                "Miniaturised to survive resource scarcity during glaciation."
            }
            SizeClass::Small => "Evolved small body size for metabolic efficiency in cold.",
            SizeClass::Medium => {
                "Maintained moderate size balancing heat retention and food needs."
            }
            SizeClass::Large | SizeClass::Huge | SizeClass::Colossal => {
                "Evolved large body mass for heat retention during ice age (Bergmann's rule)."
            }
        };
        events.push(AdaptationEvent {
            mya: ice_mya - 2.0,
            kind: AdaptationKind::SizeChange,
            description: desc.into(),
        });
    }

    // 4. Locomotion — land colonisation drives new movement types.
    if let Some(land_mya) = first_land {
        for loco in &species.locomotion {
            let desc = match loco {
                LocomotionType::Walker => {
                    format!("Evolved terrestrial {} after land colonisation.", loco)
                }
                LocomotionType::Flyer => {
                    format!("Developed {} to exploit aerial niches.", loco)
                }
                LocomotionType::Swimmer => {
                    "Retained aquatic locomotion from marine ancestors.".into()
                }
                LocomotionType::Burrower => "Adapted burrowing to escape surface predators.".into(),
                LocomotionType::Floater => {
                    "Evolved gas-bladder flotation for atmospheric drift.".into()
                }
                LocomotionType::Sessile => {
                    "Anchored lifestyle evolved for filter-feeding or photosynthesis.".into()
                }
            };
            events.push(AdaptationEvent {
                mya: land_mya - 20.0,
                kind: AdaptationKind::NewLocomotion,
                description: desc,
            });
        }
    }

    // 5. Trophic level — rationalise feeding strategy.
    let trophic_mya = origin_mya * 0.5;
    let trophic_desc = match species.trophic_level {
        TrophicLevel::Autotroph => "Photosynthetic metabolism established.",
        TrophicLevel::Herbivore => "Specialised digestive system for plant matter evolved.",
        TrophicLevel::Omnivore => "Generalised diet enabled exploitation of diverse food sources.",
        TrophicLevel::Carnivore => "Predatory adaptations (speed, claws, venom) developed.",
        TrophicLevel::FilterFeeder => "Filter-feeding apparatus evolved for plankton harvesting.",
        TrophicLevel::Parasite => "Parasitic lifestyle emerged, co-evolving with host species.",
    };
    events.push(AdaptationEvent {
        mya: trophic_mya.max(1.0),
        kind: AdaptationKind::TrophicShift,
        description: trophic_desc.into(),
    });

    // 6. Special traits — each gets a narrative explanation.
    for trait_ in &species.special_traits {
        if *trait_ == SpeciesTrait::None {
            continue;
        }
        let (desc, mya_offset) = trait_explanation(*trait_);
        events.push(AdaptationEvent {
            mya: (trophic_mya * mya_offset).max(0.1),
            kind: AdaptationKind::TraitGained,
            description: desc,
        });
    }

    // Sort oldest first.
    events.sort_by(|a, b| {
        b.mya
            .partial_cmp(&a.mya)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    EvolutionHistory {
        species_name: species.name.to_string(),
        events,
    }
}

fn body_plan_label(bp: BodyPlan) -> &'static str {
    match bp {
        BodyPlan::Vertebrate => "vertebrate",
        BodyPlan::Arthropod => "arthropod",
        BodyPlan::Mollusk => "mollusk",
        BodyPlan::PlantLike => "plant-like",
        BodyPlan::Amorphous => "amorphous",
        BodyPlan::Crystalline => "crystalline",
    }
}

fn trait_explanation(t: SpeciesTrait) -> (String, f64) {
    match t {
        SpeciesTrait::Psionic => (
            "Bioelectric neural network developed psionic sensitivity.".into(),
            0.3,
        ),
        SpeciesTrait::HiveMind => (
            "Colonial superorganism structure emerged via pheromone networks.".into(),
            0.4,
        ),
        SpeciesTrait::Metamorphic => (
            "Multi-stage life cycle with radical metamorphosis evolved.".into(),
            0.5,
        ),
        SpeciesTrait::Amphibious => (
            "Dual respiratory system for land and water habitats.".into(),
            0.6,
        ),
        SpeciesTrait::Bioluminescent => (
            "Light-producing organs evolved for communication or luring prey.".into(),
            0.7,
        ),
        SpeciesTrait::Venomous => (
            "Venom delivery system evolved as predatory or defensive adaptation.".into(),
            0.45,
        ),
        SpeciesTrait::Armored => (
            "Exoskeletal or dermal armor developed against predation.".into(),
            0.55,
        ),
        SpeciesTrait::Regenerative => (
            "Enhanced cell regeneration enabled limb and tissue regrowth.".into(),
            0.35,
        ),
        SpeciesTrait::LongLived => (
            "Telomere maintenance and DNA repair extended natural lifespan.".into(),
            0.25,
        ),
        SpeciesTrait::ShortLived => (
            "Rapid reproduction cycle favoured over individual longevity.".into(),
            0.8,
        ),
        SpeciesTrait::None => ("No special adaptation.".into(), 0.5),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::generate_planetary_timeline;
    use std::rc::Rc;

    fn sample_species() -> Species {
        Species {
            name: Rc::from("Testoid"),
            body_plan: BodyPlan::Arthropod,
            locomotion: vec![LocomotionType::Walker, LocomotionType::Flyer],
            trophic_level: TrophicLevel::Carnivore,
            size_class: SizeClass::Medium,
            special_traits: vec![SpeciesTrait::Venomous, SpeciesTrait::Armored],
            ..Default::default()
        }
    }

    fn sample_timeline() -> PlanetaryTimeline {
        generate_planetary_timeline(4.6, true, true, 2592, "evo")
    }

    #[test]
    fn evolution_events_are_chronological() {
        let sp = sample_species();
        let tl = sample_timeline();
        let evo = generate_evolution_history(&sp, &tl);
        for w in evo.events.windows(2) {
            assert!(
                w[0].mya >= w[1].mya,
                "out of order: {:.1} then {:.1}",
                w[0].mya,
                w[1].mya
            );
        }
    }

    #[test]
    fn evolution_has_body_plan_event() {
        let sp = sample_species();
        let tl = sample_timeline();
        let evo = generate_evolution_history(&sp, &tl);
        assert!(evo
            .events
            .iter()
            .any(|e| e.kind == AdaptationKind::BodyPlanShift));
    }

    #[test]
    fn evolution_has_trophic_event() {
        let sp = sample_species();
        let tl = sample_timeline();
        let evo = generate_evolution_history(&sp, &tl);
        assert!(evo
            .events
            .iter()
            .any(|e| e.kind == AdaptationKind::TrophicShift));
    }

    #[test]
    fn evolution_has_locomotion_events() {
        let sp = sample_species();
        let tl = sample_timeline();
        let evo = generate_evolution_history(&sp, &tl);
        let loco_count = evo
            .events
            .iter()
            .filter(|e| e.kind == AdaptationKind::NewLocomotion)
            .count();
        assert!(
            loco_count >= 2,
            "expected 2 locomotion events, got {}",
            loco_count
        );
    }

    #[test]
    fn evolution_has_trait_events() {
        let sp = sample_species();
        let tl = sample_timeline();
        let evo = generate_evolution_history(&sp, &tl);
        let trait_count = evo
            .events
            .iter()
            .filter(|e| e.kind == AdaptationKind::TraitGained)
            .count();
        assert_eq!(
            trait_count, 2,
            "expected 2 trait events (Venomous + Armored)"
        );
    }

    #[test]
    fn all_events_have_descriptions() {
        let sp = sample_species();
        let tl = sample_timeline();
        let evo = generate_evolution_history(&sp, &tl);
        for e in &evo.events {
            assert!(!e.description.is_empty(), "{:?} has no description", e.kind);
        }
    }

    #[test]
    fn species_name_recorded() {
        let sp = sample_species();
        let tl = sample_timeline();
        let evo = generate_evolution_history(&sp, &tl);
        assert_eq!(evo.species_name, "Testoid");
    }

    #[test]
    fn plant_species_gets_autotroph_event() {
        let sp = Species {
            name: Rc::from("Florix"),
            body_plan: BodyPlan::PlantLike,
            trophic_level: TrophicLevel::Autotroph,
            ..Default::default()
        };
        let tl = sample_timeline();
        let evo = generate_evolution_history(&sp, &tl);
        let auto = evo
            .events
            .iter()
            .find(|e| e.kind == AdaptationKind::TrophicShift)
            .unwrap();
        assert!(auto.description.contains("Photosynthetic"));
    }
}
