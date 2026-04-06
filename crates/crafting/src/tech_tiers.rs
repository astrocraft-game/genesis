//! Recipe tech tiers — classify every recipe by the factory stage needed.
//!
//! Tiers are derived from the recipe's minimum temperature and pressure
//! requirements, matching the `HistoricalEra::factory_stage()` progression.
//! The player's factory has a current max tier that gates which recipes
//! are available.

use crate::recipes::types::Recipe;
use serde::{Deserialize, Serialize};

/// Factory technology tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TechTier {
    /// Hand tools, campfire. ≤400 °C, 1 atm.
    Manual = 0,
    /// Clay kiln. ≤1200 °C, 1 atm.
    Kiln = 1,
    /// Charcoal/bellows furnace. ≤1600 °C, ≤2 atm.
    Furnace = 2,
    /// Coke-fired blast furnace. ≤2000 °C, ≤20 atm.
    BlastFurnace = 3,
    /// Electric arc furnace. ≤2500 °C, ≤200 atm.
    ElectricArc = 4,
    /// Chemical reactor (high pressure). ≤3500 °C, ≤1000 atm.
    ChemicalReactor = 5,
    /// Plasma torch / induction. ≤5000 °C, ≤10000 atm.
    Plasma = 6,
    /// Nuclear-powered processes.
    Nuclear = 7,
    /// Exotic / sci-fi processes (antimatter, zero-point, etc.).
    Exotic = 8,
}

impl TechTier {
    /// All tiers in order.
    pub const ALL: [TechTier; 9] = [
        TechTier::Manual,
        TechTier::Kiln,
        TechTier::Furnace,
        TechTier::BlastFurnace,
        TechTier::ElectricArc,
        TechTier::ChemicalReactor,
        TechTier::Plasma,
        TechTier::Nuclear,
        TechTier::Exotic,
    ];

    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            TechTier::Manual => "Manual",
            TechTier::Kiln => "Kiln",
            TechTier::Furnace => "Furnace",
            TechTier::BlastFurnace => "Blast Furnace",
            TechTier::ElectricArc => "Electric Arc",
            TechTier::ChemicalReactor => "Chemical Reactor",
            TechTier::Plasma => "Plasma",
            TechTier::Nuclear => "Nuclear",
            TechTier::Exotic => "Exotic",
        }
    }

    /// Numeric tier (0–8).
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Derive the tech tier of a recipe from its temperature and pressure.
pub fn recipe_tier(recipe: &Recipe) -> TechTier {
    let t = recipe.min_temp_c;
    let p = recipe.pressure_atm;

    if t > 5000 || p > 10_000.0 {
        TechTier::Exotic
    } else if t > 3500 || p > 1000.0 {
        TechTier::Plasma
    } else if t > 2500 || p > 200.0 {
        TechTier::ChemicalReactor
    } else if t > 2000 || p > 20.0 {
        TechTier::ElectricArc
    } else if t > 1600 || p > 2.0 {
        TechTier::BlastFurnace
    } else if t > 1200 {
        TechTier::Furnace
    } else if t > 400 {
        TechTier::Kiln
    } else {
        TechTier::Manual
    }
}

/// Filter recipes to those at or below the given max tier.
pub fn recipes_at_tier(max_tier: TechTier) -> Vec<&'static Recipe> {
    crate::recipes::all_recipes()
        .into_iter()
        .filter(|r| recipe_tier(r) <= max_tier)
        .collect()
}

/// Count recipes per tier across the entire database.
pub fn tier_distribution() -> [(TechTier, usize); 9] {
    let mut counts = [0usize; 9];
    for recipe in crate::recipes::all_recipes() {
        let tier = recipe_tier(recipe);
        counts[tier.as_u8() as usize] += 1;
    }
    let mut result = [(TechTier::Manual, 0); 9];
    for (i, tier) in TechTier::ALL.iter().enumerate() {
        result[i] = (*tier, counts[i]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_recipe_has_a_tier() {
        for recipe in crate::recipes::all_recipes() {
            let tier = recipe_tier(recipe);
            assert!(tier.as_u8() <= 8, "recipe {} has invalid tier", recipe.name);
        }
    }

    #[test]
    fn tier_increases_with_temperature() {
        // A recipe at 25°C should be lower tier than one at 2000°C.
        let cold = Recipe {
            id: 1,
            name: "Cold",
            category: crate::recipes::types::RecipeCategory::Extraction,
            inputs: &[],
            outputs: &[],
            byproducts: &[],
            min_temp_c: 25,
            pressure_atm: 1.0,
            catalyst: None,
            duration_hours: 1.0,
            cross_recipe_group: None,
        };
        let hot = Recipe {
            id: 2,
            name: "Hot",
            category: crate::recipes::types::RecipeCategory::Extraction,
            inputs: &[],
            outputs: &[],
            byproducts: &[],
            min_temp_c: 2000,
            pressure_atm: 1.0,
            catalyst: None,
            duration_hours: 1.0,
            cross_recipe_group: None,
        };
        assert!(recipe_tier(&hot) > recipe_tier(&cold));
    }

    #[test]
    fn manual_tier_is_lowest() {
        assert_eq!(TechTier::Manual.as_u8(), 0);
        assert!(TechTier::Manual < TechTier::Kiln);
        assert!(TechTier::Kiln < TechTier::Exotic);
    }

    #[test]
    fn higher_tier_unlocks_more_recipes() {
        let manual = recipes_at_tier(TechTier::Manual).len();
        let furnace = recipes_at_tier(TechTier::Furnace).len();
        let exotic = recipes_at_tier(TechTier::Exotic).len();
        assert!(
            furnace >= manual,
            "furnace {} should >= manual {}",
            furnace,
            manual
        );
        assert!(
            exotic >= furnace,
            "exotic {} should >= furnace {}",
            exotic,
            furnace
        );
    }

    #[test]
    fn tier_distribution_covers_all_recipes() {
        let dist = tier_distribution();
        let total: usize = dist.iter().map(|(_, c)| c).sum();
        let all = crate::recipes::all_recipes().len();
        assert_eq!(total, all);
    }

    #[test]
    fn manual_tier_has_some_recipes() {
        let manual = recipes_at_tier(TechTier::Manual);
        assert!(
            !manual.is_empty(),
            "manual tier should have at least some recipes"
        );
    }

    #[test]
    fn exotic_tier_includes_all() {
        let exotic = recipes_at_tier(TechTier::Exotic).len();
        let all = crate::recipes::all_recipes().len();
        assert_eq!(exotic, all);
    }

    #[test]
    fn pressure_influences_tier() {
        let low_p = Recipe {
            id: 1,
            name: "LowP",
            category: crate::recipes::types::RecipeCategory::Extraction,
            inputs: &[],
            outputs: &[],
            byproducts: &[],
            min_temp_c: 500,
            pressure_atm: 1.0,
            catalyst: None,
            duration_hours: 1.0,
            cross_recipe_group: None,
        };
        let high_p = Recipe {
            id: 2,
            name: "HighP",
            category: crate::recipes::types::RecipeCategory::Extraction,
            inputs: &[],
            outputs: &[],
            byproducts: &[],
            min_temp_c: 500,
            pressure_atm: 500.0,
            catalyst: None,
            duration_hours: 1.0,
            cross_recipe_group: None,
        };
        assert!(recipe_tier(&high_p) > recipe_tier(&low_p));
    }
}
