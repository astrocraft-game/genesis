//! Byproduct and waste tracking for factory operations.
//!
//! Accumulates waste substances produced as recipe byproducts. Unmanaged
//! waste converts to pollution pressure. Waste processing recipes can
//! reduce the stockpile.

use crate::recipes::substance::Substance;
use crate::recipes::types::Recipe;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Waste substances that contribute to environmental pollution when
/// unmanaged. Other byproducts (e.g., useful co-products) are not waste.
pub fn is_waste(s: Substance) -> bool {
    matches!(
        s,
        Substance::Slag
            | Substance::Tailings
            | Substance::RedMud
            | Substance::FlyAsh
            | Substance::CarbonDioxide
            | Substance::Wastewater
            | Substance::ToxicSludge
    )
}

/// Per-factory waste stockpile.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WasteTracker {
    /// Accumulated waste in kg per substance.
    pub stockpile: HashMap<Substance, f64>,
}

impl WasteTracker {
    /// Record waste produced by running one batch of a recipe.
    /// Only byproducts classified as waste are tracked.
    pub fn record_recipe(&mut self, recipe: &Recipe) {
        for &(substance, qty_kg) in recipe.byproducts {
            if is_waste(substance) {
                *self.stockpile.entry(substance).or_insert(0.0) += qty_kg as f64;
            }
        }
    }

    /// Manually add waste (e.g., from external source).
    pub fn add(&mut self, substance: Substance, kg: f64) {
        if is_waste(substance) {
            *self.stockpile.entry(substance).or_insert(0.0) += kg;
        }
    }

    /// Remove waste (e.g., processed by a waste-treatment recipe).
    /// Returns the amount actually removed (may be less if stockpile is low).
    pub fn remove(&mut self, substance: Substance, kg: f64) -> f64 {
        let entry = self.stockpile.entry(substance).or_insert(0.0);
        let actual = kg.min(*entry);
        *entry -= actual;
        if *entry <= 0.0 {
            self.stockpile.remove(&substance);
        }
        actual
    }

    /// Total waste mass in kg across all substances.
    pub fn total_kg(&self) -> f64 {
        self.stockpile.values().sum()
    }

    /// Number of distinct waste substances in the stockpile.
    pub fn substance_count(&self) -> usize {
        self.stockpile.len()
    }

    /// Convert unmanaged waste to a pollution pressure value (0.0–1.0+).
    ///
    /// Heuristic: 1000 kg of total waste ≈ 0.1 pollution pressure.
    /// Toxic sludge counts 3× its weight.
    pub fn pollution_pressure(&self) -> f32 {
        let mut weighted = 0.0f64;
        for (&substance, &kg) in &self.stockpile {
            let multiplier = if substance == Substance::ToxicSludge {
                3.0
            } else {
                1.0
            };
            weighted += kg * multiplier;
        }
        (weighted / 10_000.0) as f32
    }

    /// Whether the stockpile is empty.
    pub fn is_empty(&self) -> bool {
        self.stockpile.is_empty()
    }
}

/// Predefined waste-processing recipes that reduce stockpile.
/// These are logical operations (not in the main recipe DB) that the
/// game engine can offer to the player.
#[derive(Clone, Debug)]
pub struct WasteProcessingRecipe {
    pub name: &'static str,
    /// Waste substance consumed.
    pub input: Substance,
    /// kg consumed per batch.
    pub input_kg: f64,
    /// Useful output (if any).
    pub output: Option<(Substance, f32)>,
    /// Energy cost in kJ.
    pub energy_kj: f32,
}

/// Built-in waste processing options.
pub fn waste_processing_recipes() -> Vec<WasteProcessingRecipe> {
    vec![
        WasteProcessingRecipe {
            name: "Slag → Aggregate",
            input: Substance::Slag,
            input_kg: 100.0,
            output: Some((Substance::Concrete, 80.0)),
            energy_kj: 50.0,
        },
        WasteProcessingRecipe {
            name: "CO₂ Carbon Capture",
            input: Substance::CarbonDioxide,
            input_kg: 50.0,
            output: Some((Substance::Carbon, 10.0)),
            energy_kj: 200.0,
        },
        WasteProcessingRecipe {
            name: "Wastewater Treatment",
            input: Substance::Wastewater,
            input_kg: 1000.0,
            output: Some((Substance::Water, 950.0)),
            energy_kj: 30.0,
        },
        WasteProcessingRecipe {
            name: "Tailings Neutralisation",
            input: Substance::Tailings,
            input_kg: 200.0,
            output: None,
            energy_kj: 80.0,
        },
        WasteProcessingRecipe {
            name: "Toxic Sludge Incineration",
            input: Substance::ToxicSludge,
            input_kg: 50.0,
            output: None,
            energy_kj: 500.0,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_tracker_is_empty() {
        let wt = WasteTracker::default();
        assert!(wt.is_empty());
        assert_eq!(wt.total_kg(), 0.0);
        assert_eq!(wt.pollution_pressure(), 0.0);
    }

    #[test]
    fn add_accumulates_waste() {
        let mut wt = WasteTracker::default();
        wt.add(Substance::Slag, 100.0);
        wt.add(Substance::Slag, 50.0);
        assert_eq!(wt.stockpile[&Substance::Slag], 150.0);
        assert_eq!(wt.total_kg(), 150.0);
    }

    #[test]
    fn non_waste_substance_rejected() {
        let mut wt = WasteTracker::default();
        wt.add(Substance::Iron, 100.0); // not waste
        assert!(wt.is_empty());
    }

    #[test]
    fn remove_depletes_stockpile() {
        let mut wt = WasteTracker::default();
        wt.add(Substance::Tailings, 200.0);
        let removed = wt.remove(Substance::Tailings, 80.0);
        assert!((removed - 80.0).abs() < 0.01);
        assert!((wt.stockpile[&Substance::Tailings] - 120.0).abs() < 0.01);
    }

    #[test]
    fn remove_clamps_to_available() {
        let mut wt = WasteTracker::default();
        wt.add(Substance::Slag, 30.0);
        let removed = wt.remove(Substance::Slag, 100.0);
        assert!((removed - 30.0).abs() < 0.01);
        assert!(wt.is_empty()); // fully consumed, entry removed
    }

    #[test]
    fn pollution_pressure_scales_with_waste() {
        let mut wt = WasteTracker::default();
        wt.add(Substance::Slag, 5000.0);
        let p1 = wt.pollution_pressure();
        wt.add(Substance::CarbonDioxide, 5000.0);
        let p2 = wt.pollution_pressure();
        assert!(p2 > p1);
    }

    #[test]
    fn toxic_sludge_has_higher_pollution_weight() {
        let mut regular = WasteTracker::default();
        regular.add(Substance::Slag, 1000.0);
        let mut toxic = WasteTracker::default();
        toxic.add(Substance::ToxicSludge, 1000.0);
        assert!(
            toxic.pollution_pressure() > regular.pollution_pressure(),
            "toxic {} should exceed regular {}",
            toxic.pollution_pressure(),
            regular.pollution_pressure()
        );
    }

    #[test]
    fn record_recipe_captures_waste_byproducts() {
        // Simulate a recipe with Slag byproduct.
        let recipe = Recipe {
            id: 999,
            name: "TestSmelt",
            category: crate::recipes::types::RecipeCategory::Extraction,
            inputs: &[],
            outputs: &[],
            byproducts: &[(Substance::Slag, 10.0), (Substance::Iron, 5.0)],
            min_temp_c: 1500,
            pressure_atm: 1.0,
            catalyst: None,
            duration_hours: 2.0,
            cross_recipe_group: None,
        };
        let mut wt = WasteTracker::default();
        wt.record_recipe(&recipe);
        assert_eq!(wt.stockpile[&Substance::Slag], 10.0);
        // Iron is not waste — should not be tracked.
        assert!(!wt.stockpile.contains_key(&Substance::Iron));
    }

    #[test]
    fn waste_processing_recipes_exist() {
        let recipes = waste_processing_recipes();
        assert!(recipes.len() >= 4);
        for r in &recipes {
            assert!(is_waste(r.input));
            assert!(r.energy_kj > 0.0);
        }
    }

    #[test]
    fn processing_reduces_waste() {
        let mut wt = WasteTracker::default();
        wt.add(Substance::Wastewater, 5000.0);
        let before = wt.pollution_pressure();
        // Simulate processing 2 batches.
        let recipe = waste_processing_recipes()
            .into_iter()
            .find(|r| r.input == Substance::Wastewater)
            .unwrap();
        wt.remove(recipe.input, recipe.input_kg);
        wt.remove(recipe.input, recipe.input_kg);
        let after = wt.pollution_pressure();
        assert!(
            after < before,
            "processing should reduce pollution pressure"
        );
    }
}
