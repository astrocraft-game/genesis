//! Energy model for factory operations.
//!
//! Each recipe consumes energy proportional to its temperature and pressure
//! requirements. Power sources supply energy; an `EnergyBudget` tracks the
//! balance. When demand exceeds supply, throughput is reduced proportionally.

use crate::recipes::types::Recipe;
use serde::{Deserialize, Serialize};

/// Classification of power source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PowerSourceKind {
    /// Geothermal vent — permanent, moderate output.
    Geothermal,
    /// Coal/oil/gas combustion — finite fuel, high output.
    FossilFuel,
    /// Photovoltaic — biome-dependent (clear sky = best), no fuel.
    Solar,
    /// Nuclear fission — high tech, very high output, needs fuel rods.
    Fission,
    /// Nuclear fusion — late-game, extreme output, needs deuterium/tritium.
    Fusion,
    /// Manual/animal power — very low, always available.
    Manual,
}

/// A single power source contributing to the factory's energy budget.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerSource {
    pub kind: PowerSourceKind,
    /// Maximum power output in kilowatts (kW).
    pub capacity_kw: f32,
    /// Current availability factor 0.0–1.0 (e.g., solar at night = 0).
    pub availability: f32,
}

impl PowerSource {
    /// Effective power output right now (kW).
    pub fn effective_kw(&self) -> f32 {
        self.capacity_kw * self.availability
    }
}

/// Factory energy budget: supply vs. demand.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnergyBudget {
    pub sources: Vec<PowerSource>,
    /// Total energy demanded by active recipes this tick (kJ).
    pub demand_kj: f32,
    /// Time span of one tick in seconds (for kW→kJ conversion).
    pub tick_seconds: f32,
}

impl EnergyBudget {
    /// Create a budget with a given tick duration.
    pub fn new(tick_seconds: f32) -> Self {
        Self {
            sources: Vec::new(),
            demand_kj: 0.0,
            tick_seconds,
        }
    }

    /// Add a power source.
    pub fn add_source(&mut self, source: PowerSource) {
        self.sources.push(source);
    }

    /// Total available energy this tick (kJ).
    pub fn supply_kj(&self) -> f32 {
        let total_kw: f32 = self.sources.iter().map(|s| s.effective_kw()).sum();
        total_kw * self.tick_seconds
    }

    /// Throughput multiplier: 1.0 if supply >= demand, proportionally
    /// less if in deficit. Never below 0.
    pub fn throughput_factor(&self) -> f32 {
        if self.demand_kj <= 0.0 {
            return 1.0;
        }
        let supply = self.supply_kj();
        (supply / self.demand_kj).clamp(0.0, 1.0)
    }

    /// Whether the factory is in energy deficit.
    pub fn in_deficit(&self) -> bool {
        self.supply_kj() < self.demand_kj
    }

    /// Surplus energy (kJ). Negative if in deficit.
    pub fn surplus_kj(&self) -> f32 {
        self.supply_kj() - self.demand_kj
    }

    /// Add demand for running one batch of a recipe.
    pub fn add_recipe_demand(&mut self, recipe: &Recipe) {
        self.demand_kj += estimate_recipe_energy_kj(recipe);
    }

    /// Reset demand to zero (start of new tick).
    pub fn reset_demand(&mut self) {
        self.demand_kj = 0.0;
    }
}

/// Estimate energy cost (kJ) for one batch of a recipe.
///
/// Heuristic: energy scales with temperature requirement and pressure.
/// - Base: 10 kJ per recipe (handling, transport).
/// - Temperature: +1 kJ per °C above ambient (25 °C).
/// - Pressure: ×2 per order of magnitude above 1 atm.
/// - Duration: ×hours (longer processes consume more total energy).
pub fn estimate_recipe_energy_kj(recipe: &Recipe) -> f32 {
    let base = 10.0f32;
    let temp_cost = (recipe.min_temp_c - 25).max(0) as f32;
    let pressure_factor = if recipe.pressure_atm > 1.0 {
        1.0 + recipe.pressure_atm.log10()
    } else {
        1.0
    };
    let duration_factor = recipe.duration_hours.max(0.1);
    (base + temp_cost) * pressure_factor * duration_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_source(kind: PowerSourceKind, kw: f32) -> PowerSource {
        PowerSource {
            kind,
            capacity_kw: kw,
            availability: 1.0,
        }
    }

    #[test]
    fn empty_budget_has_no_deficit() {
        let b = EnergyBudget::new(1.0);
        assert!(!b.in_deficit());
        assert_eq!(b.throughput_factor(), 1.0);
    }

    #[test]
    fn supply_scales_with_sources() {
        let mut b = EnergyBudget::new(1.0); // 1 second tick
        b.add_source(simple_source(PowerSourceKind::Geothermal, 100.0));
        assert!((b.supply_kj() - 100.0).abs() < 0.01);
        b.add_source(simple_source(PowerSourceKind::Solar, 50.0));
        assert!((b.supply_kj() - 150.0).abs() < 0.01);
    }

    #[test]
    fn demand_exceeds_supply_causes_deficit() {
        let mut b = EnergyBudget::new(1.0);
        b.add_source(simple_source(PowerSourceKind::Manual, 10.0));
        b.demand_kj = 100.0;
        assert!(b.in_deficit());
        assert!(b.throughput_factor() < 1.0);
        assert!((b.throughput_factor() - 0.1).abs() < 0.01);
    }

    #[test]
    fn throughput_capped_at_one() {
        let mut b = EnergyBudget::new(1.0);
        b.add_source(simple_source(PowerSourceKind::Fission, 10000.0));
        b.demand_kj = 50.0;
        assert_eq!(b.throughput_factor(), 1.0);
    }

    #[test]
    fn availability_reduces_effective_power() {
        let mut b = EnergyBudget::new(1.0);
        b.add_source(PowerSource {
            kind: PowerSourceKind::Solar,
            capacity_kw: 100.0,
            availability: 0.5, // cloudy / half-day
        });
        assert!((b.supply_kj() - 50.0).abs() < 0.01);
    }

    #[test]
    fn recipe_energy_scales_with_temperature() {
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
            min_temp_c: 1500,
            pressure_atm: 1.0,
            catalyst: None,
            duration_hours: 1.0,
            cross_recipe_group: None,
        };
        assert!(estimate_recipe_energy_kj(&hot) > estimate_recipe_energy_kj(&cold));
    }

    #[test]
    fn recipe_energy_scales_with_pressure() {
        let atm = Recipe {
            id: 1,
            name: "Ambient",
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
            pressure_atm: 100.0,
            catalyst: None,
            duration_hours: 1.0,
            cross_recipe_group: None,
        };
        assert!(estimate_recipe_energy_kj(&high_p) > estimate_recipe_energy_kj(&atm));
    }

    #[test]
    fn add_recipe_demand_accumulates() {
        let mut b = EnergyBudget::new(1.0);
        b.add_source(simple_source(PowerSourceKind::FossilFuel, 10000.0));
        let r = Recipe {
            id: 1,
            name: "Test",
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
        b.add_recipe_demand(&r);
        assert!(b.demand_kj > 0.0);
        let first = b.demand_kj;
        b.add_recipe_demand(&r);
        assert!((b.demand_kj - first * 2.0).abs() < 0.01);
    }

    #[test]
    fn reset_demand_clears() {
        let mut b = EnergyBudget::new(1.0);
        b.demand_kj = 500.0;
        b.reset_demand();
        assert_eq!(b.demand_kj, 0.0);
    }

    #[test]
    fn surplus_positive_when_oversupplied() {
        let mut b = EnergyBudget::new(1.0);
        b.add_source(simple_source(PowerSourceKind::Fission, 1000.0));
        b.demand_kj = 200.0;
        assert!(b.surplus_kj() > 0.0);
        assert!((b.surplus_kj() - 800.0).abs() < 0.01);
    }
}
