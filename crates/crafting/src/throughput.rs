//! Throughput and logistics model for factory production lines.
//!
//! Estimates per-recipe throughput (kg/hour), models parallel instances,
//! and detects bottlenecks in multi-step production chains.

use crate::recipes::substance::Substance;
use crate::recipes::types::Recipe;
use serde::{Deserialize, Serialize};

/// Estimate base throughput for a recipe in kg per hour.
///
/// Heuristic: total output mass divided by duration. Recipes with very
/// short durations have high throughput; long processes are slower.
pub fn estimate_throughput_kg_per_hour(recipe: &Recipe) -> f32 {
    let output_mass: f32 = recipe.outputs.iter().map(|(_, qty)| qty).sum();
    if recipe.duration_hours <= 0.0 {
        return output_mass; // instantaneous
    }
    output_mass / recipe.duration_hours
}

/// A single production line running one recipe with N parallel instances.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductionLine {
    pub recipe_name: &'static str,
    pub recipe_id: u32,
    /// Base throughput per instance (kg/h).
    pub base_throughput_kg_h: f32,
    /// Number of parallel machines running this recipe.
    pub instances: u32,
    /// Energy throughput multiplier from `EnergyBudget` (0.0–1.0).
    pub energy_factor: f32,
}

impl ProductionLine {
    /// Create from a recipe with the given number of parallel instances.
    pub fn new(recipe: &'static Recipe, instances: u32) -> Self {
        Self {
            recipe_name: recipe.name,
            recipe_id: recipe.id,
            base_throughput_kg_h: estimate_throughput_kg_per_hour(recipe),
            instances,
            energy_factor: 1.0,
        }
    }

    /// Effective throughput considering parallel instances and energy factor.
    pub fn effective_throughput_kg_h(&self) -> f32 {
        self.base_throughput_kg_h * self.instances as f32 * self.energy_factor
    }
}

/// A multi-step production chain from raw input to final output.
#[derive(Clone, Debug, Default)]
pub struct ProductionChain {
    pub steps: Vec<ProductionLine>,
}

impl ProductionChain {
    /// Add a step to the chain.
    pub fn add_step(&mut self, line: ProductionLine) {
        self.steps.push(line);
    }

    /// Find the bottleneck: the step with the lowest effective throughput.
    /// Returns `(step_index, recipe_name, throughput)`.
    pub fn bottleneck(&self) -> Option<(usize, &'static str, f32)> {
        self.steps
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.effective_throughput_kg_h()
                    .partial_cmp(&b.effective_throughput_kg_h())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, line)| (i, line.recipe_name, line.effective_throughput_kg_h()))
    }

    /// Overall chain throughput = the bottleneck's throughput.
    pub fn chain_throughput_kg_h(&self) -> f32 {
        self.bottleneck().map(|(_, _, t)| t).unwrap_or(0.0)
    }

    /// Number of steps in the chain.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Suggest how many parallel instances of the bottleneck step are needed
    /// to match a target throughput.
    pub fn instances_needed_for_target(&self, target_kg_h: f32) -> Option<(usize, u32)> {
        let (idx, _, throughput) = self.bottleneck()?;
        if throughput <= 0.0 {
            return None;
        }
        let base = self.steps[idx].base_throughput_kg_h * self.steps[idx].energy_factor;
        if base <= 0.0 {
            return None;
        }
        let needed = (target_kg_h / base).ceil() as u32;
        Some((idx, needed))
    }
}

/// Build a production chain from a sequence of recipes (as returned by
/// `CraftingGraph::production_chain`). Each step gets 1 instance by default.
pub fn chain_from_recipes(recipes: &[(&'static Recipe, Substance)]) -> ProductionChain {
    let mut chain = ProductionChain::default();
    for (recipe, _output) in recipes {
        chain.add_step(ProductionLine::new(recipe, 1));
    }
    chain
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipes::types::RecipeCategory;

    fn make_recipe(
        id: u32,
        name: &'static str,
        output_kg: f32,
        duration_h: f32,
    ) -> &'static Recipe {
        // Leak a Box to get 'static lifetime for testing.
        Box::leak(Box::new(Recipe {
            id,
            name,
            category: RecipeCategory::Extraction,
            inputs: &[],
            outputs: Box::leak(vec![(Substance::Iron, output_kg)].into_boxed_slice()),
            byproducts: &[],
            min_temp_c: 25,
            pressure_atm: 1.0,
            catalyst: None,
            duration_hours: duration_h,
            cross_recipe_group: None,
        }))
    }

    #[test]
    fn throughput_inversely_proportional_to_duration() {
        let fast = make_recipe(1, "Fast", 10.0, 1.0);
        let slow = make_recipe(2, "Slow", 10.0, 10.0);
        assert!(estimate_throughput_kg_per_hour(fast) > estimate_throughput_kg_per_hour(slow));
    }

    #[test]
    fn parallel_instances_scale_throughput() {
        let r = make_recipe(1, "Test", 10.0, 1.0);
        let single = ProductionLine::new(r, 1);
        let triple = ProductionLine::new(r, 3);
        assert!(
            (triple.effective_throughput_kg_h() - single.effective_throughput_kg_h() * 3.0).abs()
                < 0.01
        );
    }

    #[test]
    fn energy_factor_reduces_throughput() {
        let r = make_recipe(1, "Test", 10.0, 1.0);
        let mut line = ProductionLine::new(r, 1);
        let full = line.effective_throughput_kg_h();
        line.energy_factor = 0.5;
        let half = line.effective_throughput_kg_h();
        assert!((half - full * 0.5).abs() < 0.01);
    }

    #[test]
    fn bottleneck_is_slowest_step() {
        let fast = make_recipe(1, "Fast", 100.0, 1.0); // 100 kg/h
        let slow = make_recipe(2, "Slow", 5.0, 2.0); // 2.5 kg/h
        let medium = make_recipe(3, "Medium", 20.0, 1.0); // 20 kg/h

        let mut chain = ProductionChain::default();
        chain.add_step(ProductionLine::new(fast, 1));
        chain.add_step(ProductionLine::new(slow, 1));
        chain.add_step(ProductionLine::new(medium, 1));

        let (idx, name, throughput) = chain.bottleneck().unwrap();
        assert_eq!(idx, 1);
        assert_eq!(name, "Slow");
        assert!((throughput - 2.5).abs() < 0.01);
        assert!((chain.chain_throughput_kg_h() - 2.5).abs() < 0.01);
    }

    #[test]
    fn chain_throughput_equals_bottleneck() {
        let a = make_recipe(1, "A", 50.0, 1.0);
        let b = make_recipe(2, "B", 10.0, 1.0);
        let mut chain = ProductionChain::default();
        chain.add_step(ProductionLine::new(a, 1));
        chain.add_step(ProductionLine::new(b, 1));
        assert_eq!(chain.chain_throughput_kg_h(), 10.0);
    }

    #[test]
    fn instances_needed_for_target() {
        let slow = make_recipe(1, "Slow", 5.0, 1.0); // 5 kg/h per instance
        let mut chain = ProductionChain::default();
        chain.add_step(ProductionLine::new(slow, 1));
        let (idx, needed) = chain.instances_needed_for_target(20.0).unwrap();
        assert_eq!(idx, 0);
        assert_eq!(needed, 4); // ceil(20/5)
    }

    #[test]
    fn empty_chain_has_zero_throughput() {
        let chain = ProductionChain::default();
        assert_eq!(chain.chain_throughput_kg_h(), 0.0);
        assert!(chain.bottleneck().is_none());
    }

    #[test]
    fn parallel_bottleneck_removes_constraint() {
        let fast = make_recipe(1, "Fast", 100.0, 1.0); // 100 kg/h
        let slow = make_recipe(2, "Slow", 10.0, 1.0); // 10 kg/h

        let mut chain = ProductionChain::default();
        chain.add_step(ProductionLine::new(fast, 1));
        chain.add_step(ProductionLine::new(slow, 10)); // 10×10 = 100 kg/h

        // Both steps now at 100 kg/h — bottleneck is whichever comes first.
        assert!((chain.chain_throughput_kg_h() - 100.0).abs() < 0.01);
    }
}
