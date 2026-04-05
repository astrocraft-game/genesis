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

    Ecosystem {
        producer,
        herbivores,
        predators,
        detritivores,
    }
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
}
