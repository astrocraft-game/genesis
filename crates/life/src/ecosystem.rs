//! Multi-species ecosystem generation.
//!
//! Produces a small, internally-consistent food web for a world: typically
//! one autotroph (producer), one or two herbivores, and one carnivore. The
//! intent is enough diversity to model predator/prey relationships without
//! the overhead of a full-blown ecology simulation.

use crate::generator::generate_species_with_role;
use crate::input::SpeciesGenerationInput;
use crate::species::{Species, TrophicLevel};
use crate::types::LifeLevel;

/// A minimal ecosystem: producer + consumers + predator.
///
/// `producer` is `None` if the planet's life level is below `PlantLike`;
/// the full chain requires at least `AnimalLike` life.
#[derive(Clone, Debug, Default)]
pub struct Ecosystem {
    /// Autotroph / photosynthetic base of the food web.
    pub producer: Option<Species>,
    /// Primary consumers (herbivores).
    pub herbivores: Vec<Species>,
    /// Secondary consumers (carnivores/omnivores).
    pub predators: Vec<Species>,
    /// Detritivores or filter-feeders at the ecosystem margins.
    pub detritivores: Vec<Species>,
    /// Predator-prey links as `(predator_idx, prey_idx)` into the
    /// flat species list returned by `all_species()`.
    pub predator_prey_links: Vec<(usize, usize)>,
    /// True if the trophic pyramid is valid: every carnivore has prey,
    /// every herbivore has a producer, and there are no dangling links.
    pub trophic_pyramid_valid: bool,
    /// Indices (into `all_species()`) of keystone species whose removal
    /// would collapse the food web (disconnect a trophic level).
    pub keystone_species: Vec<usize>,
    /// Competition links: `(species_a, species_b, overlap)` where overlap
    /// is 0.0–1.0 (higher = more direct competition for the same niche).
    /// Only between species at the same trophic level.
    pub competition_links: Vec<(usize, usize, f32)>,
    /// Parasitism links: `(parasite_idx, host_idx)`.
    pub parasitism_links: Vec<(usize, usize)>,
}

impl Ecosystem {
    /// Total distinct species count across all trophic levels.
    pub fn species_count(&self) -> usize {
        self.producer.iter().len()
            + self.herbivores.len()
            + self.predators.len()
            + self.detritivores.len()
    }

    /// Iterate all species in trophic order (producers first).
    pub fn all_species(&self) -> impl Iterator<Item = &Species> {
        self.producer
            .iter()
            .chain(self.herbivores.iter())
            .chain(self.predators.iter())
            .chain(self.detritivores.iter())
    }
}

/// Generate a small ecosystem from world conditions.
///
/// The input's `scope_key` is used as a prefix; each species is generated
/// with a distinct suffix so all members stay deterministic. Returns an
/// empty `Ecosystem` if the life level is below `PluriCellular`.
pub fn generate_ecosystem_from_world(input: &SpeciesGenerationInput) -> Ecosystem {
    if input.life_level.as_u8() < LifeLevel::PluriCellular.as_u8() {
        return Ecosystem::default();
    }

    let producer = generate_with_suffix(input, "_producer", TrophicLevel::Autotroph);

    // Herbivore / omnivore layer: two species at AnimalLike+ life levels.
    let mut herbivores = Vec::new();
    if input.life_level.as_u8() >= LifeLevel::AnimalLike.as_u8() {
        if let Some(h) = generate_with_suffix(input, "_herbivore_1", TrophicLevel::Herbivore) {
            herbivores.push(h);
        }
        if let Some(h) = generate_with_suffix(input, "_herbivore_2", TrophicLevel::Omnivore) {
            herbivores.push(h);
        }
    }

    // Carnivore: one top predator when the food web supports it.
    let mut predators = Vec::new();
    if input.life_level.as_u8() >= LifeLevel::AnimalLike.as_u8() && !herbivores.is_empty() {
        if let Some(c) = generate_with_suffix(input, "_carnivore", TrophicLevel::Carnivore) {
            predators.push(c);
        }
    }

    // Detritivore / filter-feeder: ocean/hydrosphere worlds support one.
    let mut detritivores = Vec::new();
    if input.hydrosphere > 30.0 && input.life_level.as_u8() >= LifeLevel::AnimalLike.as_u8() {
        if let Some(d) = generate_with_suffix(input, "_filter", TrophicLevel::FilterFeeder) {
            detritivores.push(d);
        }
    }

    let mut eco = Ecosystem {
        producer,
        herbivores,
        predators,
        detritivores,
        ..Default::default()
    };
    build_food_web(&mut eco);
    eco
}

/// Build predator-prey links, validate the trophic pyramid, and identify
/// keystone species. Mutates the ecosystem in place.
fn build_food_web(eco: &mut Ecosystem) {
    // Assign flat indices: producer(0), herbivores, predators, detritivores.
    let species: Vec<TrophicLevel> = eco.all_species().map(|s| s.trophic_level).collect();
    let n = species.len();
    let mut links: Vec<(usize, usize)> = Vec::new();

    for (i, &level) in species.iter().enumerate() {
        match level {
            TrophicLevel::Herbivore | TrophicLevel::Omnivore => {
                // Herbivores/omnivores eat the producer.
                for (j, &prey_level) in species.iter().enumerate() {
                    if prey_level == TrophicLevel::Autotroph {
                        links.push((i, j));
                    }
                }
                // Omnivores can also eat other herbivores.
                if level == TrophicLevel::Omnivore {
                    for (j, &prey_level) in species.iter().enumerate() {
                        if j != i && prey_level == TrophicLevel::Herbivore {
                            links.push((i, j));
                        }
                    }
                }
            }
            TrophicLevel::Carnivore => {
                // Carnivores eat herbivores and omnivores.
                for (j, &prey_level) in species.iter().enumerate() {
                    if matches!(prey_level, TrophicLevel::Herbivore | TrophicLevel::Omnivore) {
                        links.push((i, j));
                    }
                }
            }
            TrophicLevel::FilterFeeder => {
                // Filter-feeders consume producers (plankton-like).
                for (j, &prey_level) in species.iter().enumerate() {
                    if prey_level == TrophicLevel::Autotroph {
                        links.push((i, j));
                    }
                }
            }
            _ => {}
        }
    }

    // Trophic pyramid validity: every non-producer, non-detritivore has
    // at least one prey link.
    let valid = species.iter().enumerate().all(|(i, &level)| {
        if matches!(level, TrophicLevel::Autotroph) {
            return true;
        }
        links.iter().any(|&(pred, _)| pred == i)
    });

    // Keystone species: a species whose removal disconnects a consumer
    // from all its food sources.
    let mut keystones = Vec::new();
    for candidate in 0..n {
        // Simulate removing this species.
        let remaining_links: Vec<(usize, usize)> = links
            .iter()
            .copied()
            .filter(|&(p, q)| p != candidate && q != candidate)
            .collect();
        // Check if any surviving consumer has zero prey links.
        for (i, &level) in species.iter().enumerate() {
            if i == candidate {
                continue;
            }
            if matches!(level, TrophicLevel::Autotroph) {
                continue;
            }
            if !remaining_links.iter().any(|&(pred, _)| pred == i) {
                keystones.push(candidate);
                break;
            }
        }
    }

    // Competition: species at the same trophic level compete. Niche overlap
    // is reduced when species differ in size class or locomotion type.
    let all_species: Vec<_> = eco.all_species().cloned().collect();
    let mut competition = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            // Species compete if they occupy the same trophic category.
            let same_category = matches!(
                (species[i], species[j]),
                (
                    TrophicLevel::Herbivore | TrophicLevel::Omnivore,
                    TrophicLevel::Herbivore | TrophicLevel::Omnivore
                ) | (TrophicLevel::Carnivore, TrophicLevel::Carnivore)
                    | (TrophicLevel::FilterFeeder, TrophicLevel::FilterFeeder)
            );
            if same_category {
                let overlap = niche_overlap(&all_species[i], &all_species[j]);
                if overlap > 0.0 {
                    competition.push((i, j, overlap));
                }
            }
        }
    }

    // Parasitism: Parasite trophic level species target the closest
    // trophic level above them (herbivores/omnivores).
    let mut parasitism = Vec::new();
    for (i, &level) in species.iter().enumerate() {
        if level == TrophicLevel::Parasite {
            for (j, &host_level) in species.iter().enumerate() {
                if matches!(
                    host_level,
                    TrophicLevel::Herbivore | TrophicLevel::Omnivore | TrophicLevel::Carnivore
                ) {
                    parasitism.push((i, j));
                }
            }
        }
    }

    eco.predator_prey_links = links;
    eco.trophic_pyramid_valid = valid;
    eco.keystone_species = keystones;
    eco.competition_links = competition;
    eco.parasitism_links = parasitism;
}

/// Compute niche overlap (0.0–1.0) between two species. Same size and
/// locomotion = full overlap; differences reduce it.
fn niche_overlap(a: &Species, b: &Species) -> f32 {
    let mut overlap = 1.0f32;
    // Size difference reduces overlap.
    let size_diff = (a.size_class as i32 - b.size_class as i32).unsigned_abs();
    overlap -= size_diff as f32 * 0.2;
    // Different locomotion reduces overlap.
    let shared_loco = a.locomotion.iter().any(|la| b.locomotion.contains(la));
    if !shared_loco {
        overlap -= 0.4;
    }
    overlap.clamp(0.0, 1.0)
}

/// Simulate an extinction event: remove species whose habitability drops
/// below the threshold, then rebuild the food web. Returns the names of
/// extinct species.
///
/// `habitability` maps each species index (in `all_species()` order) to
/// its post-event habitability score (0.0 – 1.0).
pub fn apply_extinction(eco: &mut Ecosystem, habitability: &[f32], threshold: f32) -> Vec<String> {
    let names: Vec<String> = eco.all_species().map(|s| s.name.to_string()).collect();
    let mut extinct = Vec::new();

    // Walk the flat index list in reverse so removal doesn't invalidate
    // earlier indices within the same category.
    let mut idx = 0;
    if let Some(ref prod) = eco.producer {
        if idx < habitability.len() && habitability[idx] < threshold {
            extinct.push(prod.name.to_string());
            eco.producer = None;
        }
        idx += 1;
    }

    // Helper: drain items from a vec where habitability < threshold.
    fn drain_extinct(
        vec: &mut Vec<Species>,
        hab: &[f32],
        start: &mut usize,
        threshold: f32,
        extinct: &mut Vec<String>,
    ) {
        let mut i = 0;
        while i < vec.len() {
            if *start < hab.len() && hab[*start] < threshold {
                extinct.push(vec[i].name.to_string());
                vec.remove(i);
            } else {
                i += 1;
            }
            *start += 1;
        }
    }

    drain_extinct(
        &mut eco.herbivores,
        habitability,
        &mut idx,
        threshold,
        &mut extinct,
    );
    drain_extinct(
        &mut eco.predators,
        habitability,
        &mut idx,
        threshold,
        &mut extinct,
    );
    drain_extinct(
        &mut eco.detritivores,
        habitability,
        &mut idx,
        threshold,
        &mut extinct,
    );

    // Cascade: carnivores with no remaining prey go extinct too.
    if eco.herbivores.is_empty() {
        for p in &eco.predators {
            if !extinct.contains(&p.name.to_string()) {
                extinct.push(p.name.to_string());
            }
        }
        eco.predators.clear();
    }

    // Rebuild food web.
    build_food_web(eco);

    // Return only names that were not already in the extinct list.
    let _ = names; // consumed above via extinct
    extinct
}

fn generate_with_suffix(
    base: &SpeciesGenerationInput,
    suffix: &str,
    role: TrophicLevel,
) -> Option<Species> {
    let mut input = base.clone();
    input.scope_key = format!("{}{}", base.scope_key, suffix);
    generate_species_with_role(&input, Some(role))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Climate, Habitat, Temperature};

    fn input_for(life_level: LifeLevel, hydrosphere: f32) -> SpeciesGenerationInput {
        SpeciesGenerationInput {
            habitat: Habitat::Terrestrial,
            climate: Climate::Terrestrial,
            temperature: Temperature::Temperate,
            gravity: 1.0,
            atmospheric_pressure: 1.0,
            hydrosphere,
            life_level,
            seed: "ecosystem_test".into(),
            scope_key: "world_alpha".into(),
        }
    }

    #[test]
    fn low_life_level_yields_empty_ecosystem() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::UniCellular, 70.0));
        assert_eq!(e.species_count(), 0);
    }

    #[test]
    fn plantlike_world_has_producer_only() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::PlantLike, 70.0));
        assert!(e.producer.is_some());
        assert!(e.herbivores.is_empty());
        assert!(e.predators.is_empty());
    }

    #[test]
    fn animal_world_has_full_food_web() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        assert!(e.producer.is_some());
        assert!(!e.herbivores.is_empty());
        assert!(!e.predators.is_empty());
        assert!(!e.detritivores.is_empty()); // hydrosphere 70% > 30%
    }

    #[test]
    fn dry_world_has_no_filter_feeders() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 5.0));
        assert!(e.detritivores.is_empty());
        assert!(e.producer.is_some());
        assert!(!e.herbivores.is_empty());
    }

    #[test]
    fn ecosystem_is_deterministic() {
        let input = input_for(LifeLevel::AnimalLike, 70.0);
        let a = generate_ecosystem_from_world(&input);
        let b = generate_ecosystem_from_world(&input);
        assert_eq!(a.species_count(), b.species_count());
        assert_eq!(
            a.producer.as_ref().map(|s| s.name.clone()),
            b.producer.as_ref().map(|s| s.name.clone())
        );
        assert_eq!(
            a.predators.first().map(|s| s.name.clone()),
            b.predators.first().map(|s| s.name.clone())
        );
    }

    #[test]
    fn roles_are_enforced() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        assert_eq!(
            e.producer.as_ref().unwrap().trophic_level,
            TrophicLevel::Autotroph
        );
        for h in &e.herbivores {
            assert!(matches!(
                h.trophic_level,
                TrophicLevel::Herbivore | TrophicLevel::Omnivore
            ));
        }
        for p in &e.predators {
            assert_eq!(p.trophic_level, TrophicLevel::Carnivore);
        }
        for d in &e.detritivores {
            assert_eq!(d.trophic_level, TrophicLevel::FilterFeeder);
        }
    }

    #[test]
    fn species_are_distinct() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        let names: std::collections::HashSet<_> = e.all_species().map(|s| s.name.clone()).collect();
        // Different scope keys produce different names (almost always).
        assert_eq!(names.len(), e.species_count());
    }

    #[test]
    fn every_carnivore_has_prey() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        let species: Vec<_> = e.all_species().map(|s| s.trophic_level).collect();
        for (i, &level) in species.iter().enumerate() {
            if level == TrophicLevel::Carnivore {
                let has_prey = e.predator_prey_links.iter().any(|&(pred, _)| pred == i);
                assert!(has_prey, "carnivore at index {} has no prey", i);
            }
        }
    }

    #[test]
    fn trophic_pyramid_is_valid_for_animal_world() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        assert!(
            e.trophic_pyramid_valid,
            "animal-level ecosystem should have a valid trophic pyramid"
        );
    }

    #[test]
    fn producer_is_keystone() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        // Producer (index 0) should be keystone — removing it starves herbivores.
        assert!(
            e.keystone_species.contains(&0),
            "producer should be keystone, keystones = {:?}",
            e.keystone_species
        );
    }

    #[test]
    fn removal_of_keystone_collapses_web() {
        let mut e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        let n = e.species_count();
        // Remove the producer by setting its habitability to 0.
        let mut hab = vec![1.0; n];
        hab[0] = 0.0; // producer
        let extinct = apply_extinction(&mut e, &hab, 0.2);
        assert!(
            !extinct.is_empty(),
            "removing producer should cause extinctions"
        );
        assert!(
            e.species_count() < n,
            "species count should drop after keystone removal"
        );
    }

    #[test]
    fn extinction_is_reproducible() {
        let make = || {
            let mut e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
            let n = e.species_count();
            let mut hab = vec![1.0; n];
            hab[0] = 0.0;
            let extinct = apply_extinction(&mut e, &hab, 0.2);
            (e.species_count(), extinct)
        };
        let (count_a, ext_a) = make();
        let (count_b, ext_b) = make();
        assert_eq!(count_a, count_b);
        assert_eq!(ext_a, ext_b);
    }

    #[test]
    fn herbivores_compete_with_each_other() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        // If there are ≥2 herbivores/omnivores, there should be competition.
        if e.herbivores.len() >= 2 {
            assert!(
                !e.competition_links.is_empty(),
                "two herbivores should compete"
            );
        }
    }

    #[test]
    fn competition_overlap_is_bounded() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        for &(_, _, overlap) in &e.competition_links {
            assert!(
                (0.0..=1.0).contains(&overlap),
                "overlap {} out of range",
                overlap
            );
        }
    }

    #[test]
    fn niche_differentiation_reduces_overlap() {
        use crate::species::{BodyPlan, LocomotionType, SizeClass};
        use std::rc::Rc;
        // Two identical species: full overlap.
        let a = Species {
            name: Rc::from("A"),
            size_class: SizeClass::Medium,
            locomotion: vec![LocomotionType::Walker],
            trophic_level: TrophicLevel::Herbivore,
            ..Default::default()
        };
        let b = a.clone();
        let full = niche_overlap(&a, &b);

        // Different size + different locomotion: lower overlap.
        let c = Species {
            name: Rc::from("C"),
            size_class: SizeClass::Tiny,
            locomotion: vec![LocomotionType::Flyer],
            trophic_level: TrophicLevel::Herbivore,
            ..Default::default()
        };
        let reduced = niche_overlap(&a, &c);
        assert!(
            reduced < full,
            "differentiated overlap {} should be less than identical {}",
            reduced,
            full
        );
    }

    #[test]
    fn producers_dont_compete() {
        let e = generate_ecosystem_from_world(&input_for(LifeLevel::AnimalLike, 70.0));
        // Producer is index 0; it should not appear in competition links.
        for &(a, b, _) in &e.competition_links {
            let species: Vec<_> = e.all_species().map(|s| s.trophic_level).collect();
            assert_ne!(species[a], TrophicLevel::Autotroph);
            assert_ne!(species[b], TrophicLevel::Autotroph);
        }
    }
}
