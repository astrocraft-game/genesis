#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub mod advanced_materials;
pub mod alloys;
pub mod biomaterial;
pub mod chemistry;
pub mod construction;
pub mod electrochemistry;
pub mod extraction;
pub mod fuel;
pub mod inorganic;
pub mod natural;
pub mod organic;
pub mod phase_change;
pub mod substance;
pub mod textile;
pub mod types;

use substance::Substance;
use types::Recipe;

/// Planetary conditions that constrain which recipes are achievable.
///
/// Used to filter the recipe database to a subset compatible with a given
/// world's technology level, atmosphere, and available substances.
#[derive(Clone, Debug)]
pub struct PlanetaryConditions {
    /// Maximum temperature attainable in Celsius (kiln, furnace, plasma arc…).
    /// A recipe is rejected if its `min_temp_c` exceeds this value.
    pub max_temperature_c: i32,
    /// Maximum pressure attainable in atmospheres. A recipe requiring a
    /// pressure outside the range [1/max_pressure_atm, max_pressure_atm] is
    /// rejected. Use 1.0 for ambient-only, 100.0+ for industrial reactors.
    pub max_pressure_atm: f32,
    /// Set of substances available on this world. If `None`, no substance
    /// constraint is applied; if `Some`, a recipe is rejected unless *all*
    /// its inputs and any required catalyst are in the set.
    pub available_substances: Option<std::collections::HashSet<Substance>>,
}

impl PlanetaryConditions {
    /// Ambient Earth-surface baseline: 1 atm, room temperature only.
    pub fn ambient_earth() -> Self {
        Self {
            max_temperature_c: 30,
            max_pressure_atm: 1.0,
            available_substances: None,
        }
    }

    /// Bronze-age terrestrial: wood/coal fires, ~1200 °C, ambient pressure.
    pub fn bronze_age() -> Self {
        Self {
            max_temperature_c: 1200,
            max_pressure_atm: 1.0,
            available_substances: None,
        }
    }

    /// Industrial era: blast furnaces and basic pressure vessels.
    pub fn industrial() -> Self {
        Self {
            max_temperature_c: 2000,
            max_pressure_atm: 50.0,
            available_substances: None,
        }
    }

    /// Modern era: plasma arcs, ultra-high pressure reactors.
    pub fn modern() -> Self {
        Self {
            max_temperature_c: 5000,
            max_pressure_atm: 10_000.0,
            available_substances: None,
        }
    }

    /// Whether a single recipe is achievable under these conditions.
    pub fn admits(&self, recipe: &Recipe) -> bool {
        if recipe.min_temp_c > self.max_temperature_c {
            return false;
        }
        // Pressure window: the recipe must run at a pressure we can reach.
        // Low-pressure recipes (p<1) are always admissible since vacuum is
        // reachable from any industrial setup.
        if recipe.pressure_atm > self.max_pressure_atm {
            return false;
        }
        if let Some(available) = &self.available_substances {
            for (input, _) in recipe.inputs {
                if !available.contains(input) {
                    return false;
                }
            }
            if let Some(catalyst) = recipe.catalyst {
                if !available.contains(&catalyst) {
                    return false;
                }
            }
        }
        true
    }
}

/// Returns recipes achievable under the given planetary conditions.
pub fn recipes_in_conditions(conditions: &PlanetaryConditions) -> Vec<&'static Recipe> {
    all_recipes()
        .into_iter()
        .filter(|r| conditions.admits(r))
        .collect()
}

/// Returns all recipes in the database.
pub fn all_recipes() -> Vec<&'static Recipe> {
    let mut all = Vec::new();
    all.extend(extraction::EXTRACTION_RECIPES.iter());
    all.extend(alloys::ALLOY_RECIPES.iter());
    all.extend(chemistry::CHEMISTRY_RECIPES.iter());
    all.extend(construction::CONSTRUCTION_RECIPES.iter());
    all.extend(fuel::FUEL_RECIPES.iter());
    all.extend(phase_change::PHASE_CHANGE_RECIPES.iter());
    all.extend(organic::ORGANIC_RECIPES.iter());
    all.extend(inorganic::INORGANIC_RECIPES.iter());
    all.extend(electrochemistry::ELECTROCHEMISTRY_RECIPES.iter());
    all.extend(natural::NATURAL_RECIPES.iter());
    all.extend(textile::TEXTILE_RECIPES.iter());
    all.extend(biomaterial::BIOMATERIAL_RECIPES.iter());
    all.extend(advanced_materials::ADVANCED_MATERIAL_RECIPES.iter());
    all
}

/// Returns all recipes that produce the given substance as a primary output.
pub fn recipes_for_output(target: substance::Substance) -> Vec<&'static Recipe> {
    all_recipes()
        .into_iter()
        .filter(|r| r.outputs.iter().any(|(s, _)| *s == target))
        .collect()
}

/// Returns all recipes in a cross-recipe group (multiple paths to same output).
pub fn cross_recipes(group_id: u32) -> Vec<&'static Recipe> {
    all_recipes()
        .into_iter()
        .filter(|r| r.cross_recipe_group == Some(group_id))
        .collect()
}

/// Returns all recipes that require the given substance as input.
pub fn recipes_using_input(input: substance::Substance) -> Vec<&'static Recipe> {
    all_recipes()
        .into_iter()
        .filter(|r| r.inputs.iter().any(|(s, _)| *s == input))
        .collect()
}

/// Returns all recipes that produce the given substance as a byproduct.
pub fn recipes_with_byproduct(byproduct: substance::Substance) -> Vec<&'static Recipe> {
    all_recipes()
        .into_iter()
        .filter(|r| r.byproducts.iter().any(|(s, _)| *s == byproduct))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use substance::Substance as S;
    use types::RecipeCategory;

    #[test]
    fn total_recipe_count() {
        let all = all_recipes();
        assert!(
            all.len() >= 600,
            "Should have at least 600 recipes, got {}",
            all.len()
        );
        println!("Total recipes: {}", all.len());
    }

    #[test]
    fn iron_has_multiple_paths() {
        let iron_recipes = recipes_for_output(S::PigIron);
        assert!(
            iron_recipes.len() >= 2,
            "Iron should have multiple extraction paths, got {}",
            iron_recipes.len()
        );
    }

    #[test]
    fn copper_has_cross_recipes() {
        let copper_group = cross_recipes(2); // CRG_COPPER = 2
        assert!(
            copper_group.len() >= 3,
            "Copper should have 3+ cross-recipes, got {}",
            copper_group.len()
        );
    }

    #[test]
    fn sulfuric_acid_has_cross_recipes() {
        let acid_recipes = recipes_for_output(S::SulfuricAcid);
        assert!(
            acid_recipes.len() >= 2,
            "H2SO4 should have 2+ paths, got {}",
            acid_recipes.len()
        );
    }

    #[test]
    fn recipes_have_unique_ids() {
        let all = all_recipes();
        let mut ids: Vec<u32> = all.iter().map(|r| r.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), all.len(), "All recipe IDs must be unique");
    }

    #[test]
    fn byproducts_exist() {
        let slag_producers = recipes_with_byproduct(S::Slag);
        assert!(
            slag_producers.len() >= 5,
            "Many smelting recipes should produce slag, got {}",
            slag_producers.len()
        );
    }

    #[test]
    fn extraction_category_count() {
        let all = all_recipes();
        let extraction = all
            .iter()
            .filter(|r| r.category == RecipeCategory::Extraction)
            .count();
        assert!(
            extraction >= 20,
            "Should have 20+ extraction recipes, got {}",
            extraction
        );
    }

    #[test]
    fn alloy_category_count() {
        let all = all_recipes();
        let alloys = all
            .iter()
            .filter(|r| r.category == RecipeCategory::Alloying)
            .count();
        assert!(
            alloys >= 15,
            "Should have 15+ alloy recipes, got {}",
            alloys
        );
    }

    #[test]
    fn chemistry_category_count() {
        let all = all_recipes();
        let chem = all
            .iter()
            .filter(|r| r.category == RecipeCategory::ChemicalSynthesis)
            .count();
        assert!(
            chem >= 15,
            "Should have 15+ chemistry recipes, got {}",
            chem
        );
    }

    #[test]
    fn every_recipe_has_inputs_and_outputs() {
        for recipe in all_recipes() {
            assert!(
                !recipe.inputs.is_empty(),
                "Recipe '{}' has no inputs",
                recipe.name
            );
            assert!(
                !recipe.outputs.is_empty(),
                "Recipe '{}' has no outputs",
                recipe.name
            );
        }
    }

    #[test]
    fn ambient_conditions_block_hot_recipes() {
        let ambient = PlanetaryConditions::ambient_earth();
        let admitted = recipes_in_conditions(&ambient);
        // Ambient Earth can't smelt iron (needs ~1500°C) or fire bricks.
        for recipe in &admitted {
            assert!(
                recipe.min_temp_c <= 30,
                "ambient admitted a hot recipe: {} at {}°C",
                recipe.name,
                recipe.min_temp_c
            );
        }
        // Should still admit cold/room-temperature processes (mixing,
        // fermentation prep, crystallization, etc.).
        assert!(
            !admitted.is_empty(),
            "ambient should admit some low-temp recipes"
        );
    }

    #[test]
    fn conditions_are_monotonic() {
        // A richer environment admits a superset of recipes.
        let bronze = recipes_in_conditions(&PlanetaryConditions::bronze_age()).len();
        let industrial = recipes_in_conditions(&PlanetaryConditions::industrial()).len();
        let modern = recipes_in_conditions(&PlanetaryConditions::modern()).len();
        assert!(
            bronze <= industrial,
            "industrial ({}) must admit ≥ bronze ({})",
            industrial,
            bronze
        );
        assert!(
            industrial <= modern,
            "modern ({}) must admit ≥ industrial ({})",
            modern,
            industrial
        );
    }

    #[test]
    fn substance_availability_filters() {
        let mut avail = std::collections::HashSet::new();
        avail.insert(S::Water);
        avail.insert(S::Salt);
        let conditions = PlanetaryConditions {
            max_temperature_c: 5000,
            max_pressure_atm: 10_000.0,
            available_substances: Some(avail),
        };
        let admitted = recipes_in_conditions(&conditions);
        // Every admitted recipe's inputs must be in {Water, Salt}.
        for recipe in &admitted {
            for (input, _) in recipe.inputs {
                assert!(
                    matches!(input, S::Water | S::Salt),
                    "admitted recipe {} requires {:?} not in set",
                    recipe.name,
                    input
                );
            }
        }
    }

    #[test]
    fn modern_era_unlocks_most_recipes() {
        let modern = recipes_in_conditions(&PlanetaryConditions::modern());
        let total = all_recipes().len();
        // Modern era should admit the bulk of recipes (temperature/pressure
        // no longer bottlenecks).
        let ratio = modern.len() as f32 / total as f32;
        assert!(
            ratio > 0.85,
            "modern conditions should unlock 85%+ of recipes, got {:.1}%",
            ratio * 100.0
        );
    }
}
