#![allow(dead_code)]
use crate::internal::*;

pub mod substance;
pub mod types;
pub mod extraction;
pub mod alloys;
pub mod chemistry;
pub mod construction;
pub mod fuel;
pub mod biological;
pub mod phase_change;
pub mod organic;
pub mod inorganic;
pub mod electrochemistry;
pub mod natural;

use types::Recipe;

/// Returns all recipes in the database.
pub fn all_recipes() -> Vec<&'static Recipe> {
    let mut all = Vec::new();
    all.extend(extraction::EXTRACTION_RECIPES.iter());
    all.extend(alloys::ALLOY_RECIPES.iter());
    all.extend(chemistry::CHEMISTRY_RECIPES.iter());
    all.extend(construction::CONSTRUCTION_RECIPES.iter());
    all.extend(fuel::FUEL_RECIPES.iter());
    all.extend(biological::BIOLOGICAL_RECIPES.iter());
    all.extend(phase_change::PHASE_CHANGE_RECIPES.iter());
    all.extend(organic::ORGANIC_RECIPES.iter());
    all.extend(inorganic::INORGANIC_RECIPES.iter());
    all.extend(electrochemistry::ELECTROCHEMISTRY_RECIPES.iter());
    all.extend(natural::NATURAL_RECIPES.iter());
    all
}

/// Returns all recipes that produce the given substance as a primary output.
pub fn recipes_for_output(target: substance::Substance) -> Vec<&'static Recipe> {
    all_recipes().into_iter().filter(|r| {
        r.outputs.iter().any(|(s, _)| *s == target)
    }).collect()
}

/// Returns all recipes in a cross-recipe group (multiple paths to same output).
pub fn cross_recipes(group_id: u32) -> Vec<&'static Recipe> {
    all_recipes().into_iter().filter(|r| {
        r.cross_recipe_group == Some(group_id)
    }).collect()
}

/// Returns all recipes that require the given substance as input.
pub fn recipes_using_input(input: substance::Substance) -> Vec<&'static Recipe> {
    all_recipes().into_iter().filter(|r| {
        r.inputs.iter().any(|(s, _)| *s == input)
    }).collect()
}

/// Returns all recipes that produce the given substance as a byproduct.
pub fn recipes_with_byproduct(byproduct: substance::Substance) -> Vec<&'static Recipe> {
    all_recipes().into_iter().filter(|r| {
        r.byproducts.iter().any(|(s, _)| *s == byproduct)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use substance::Substance as S;
    use types::RecipeCategory;

    #[test]
    fn total_recipe_count() {
        let all = all_recipes();
        assert!(all.len() >= 100, "Should have at least 100 recipes, got {}", all.len());
        println!("Total recipes: {}", all.len());
    }

    #[test]
    fn iron_has_multiple_paths() {
        let iron_recipes = recipes_for_output(S::PigIron);
        assert!(iron_recipes.len() >= 2, "Iron should have multiple extraction paths, got {}", iron_recipes.len());
    }

    #[test]
    fn copper_has_cross_recipes() {
        let copper_group = cross_recipes(2); // CRG_COPPER = 2
        assert!(copper_group.len() >= 3, "Copper should have 3+ cross-recipes, got {}", copper_group.len());
    }

    #[test]
    fn sulfuric_acid_has_cross_recipes() {
        let acid_recipes = recipes_for_output(S::SulfuricAcid);
        assert!(acid_recipes.len() >= 2, "H2SO4 should have 2+ paths, got {}", acid_recipes.len());
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
        assert!(slag_producers.len() >= 5, "Many smelting recipes should produce slag, got {}", slag_producers.len());
    }

    #[test]
    fn extraction_category_count() {
        let all = all_recipes();
        let extraction = all.iter().filter(|r| r.category == RecipeCategory::Extraction).count();
        assert!(extraction >= 20, "Should have 20+ extraction recipes, got {}", extraction);
    }

    #[test]
    fn alloy_category_count() {
        let all = all_recipes();
        let alloys = all.iter().filter(|r| r.category == RecipeCategory::Alloying).count();
        assert!(alloys >= 15, "Should have 15+ alloy recipes, got {}", alloys);
    }

    #[test]
    fn chemistry_category_count() {
        let all = all_recipes();
        let chem = all.iter().filter(|r| r.category == RecipeCategory::ChemicalSynthesis).count();
        assert!(chem >= 15, "Should have 15+ chemistry recipes, got {}", chem);
    }

    #[test]
    fn every_recipe_has_inputs_and_outputs() {
        for recipe in all_recipes() {
            assert!(!recipe.inputs.is_empty(), "Recipe '{}' has no inputs", recipe.name);
            assert!(!recipe.outputs.is_empty(), "Recipe '{}' has no outputs", recipe.name);
        }
    }
}
