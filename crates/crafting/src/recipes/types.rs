#![allow(dead_code)]
use serde::{Serialize, Deserialize};
use super::substance::Substance;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum RecipeCategory {
    Extraction,
    Alloying,
    ChemicalSynthesis,
    Refining,
    Construction,
    FuelProcessing,
    FoodBiological,
    Manufacturing,
    PhaseChange,
    Recycling,
    Fermentation,
    DairyProcessing,
    TextileProcessing,
    LeatherTanning,
    PaperPulping,
    OilExtraction,
    SpiceProcessing,
    BiologicalMaterial,
}

#[derive(Clone, Debug)]
pub struct RecipeConditions {
    /// Minimum temperature in Celsius
    pub min_temperature_c: i32,
    /// Maximum temperature in Celsius
    pub max_temperature_c: i32,
    /// Required pressure in atmospheres (1.0 = ambient)
    pub pressure_atm: f32,
    /// Catalyst needed (not consumed)
    pub catalyst: Option<Substance>,
    /// Process duration in hours
    pub duration_hours: f32,
}

impl Default for RecipeConditions {
    fn default() -> Self {
        Self {
            min_temperature_c: 20,
            max_temperature_c: 20,
            pressure_atm: 1.0,
            catalyst: None,
            duration_hours: 1.0,
        }
    }
}

/// An input or output amount: substance + quantity in kg
#[derive(Clone, Debug)]
pub struct RecipeComponent {
    pub substance: Substance,
    /// Quantity in kg per batch
    pub quantity_kg: f32,
}

impl RecipeComponent {
    pub const fn new(substance: Substance, quantity_kg: f32) -> Self {
        Self { substance, quantity_kg }
    }
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub id: u32,
    pub name: &'static str,
    pub category: RecipeCategory,
    pub inputs: &'static [(Substance, f32)],
    pub outputs: &'static [(Substance, f32)],
    pub byproducts: &'static [(Substance, f32)],
    pub min_temp_c: i32,
    pub pressure_atm: f32,
    pub catalyst: Option<Substance>,
    pub duration_hours: f32,
    /// Cross-recipe group: recipes with the same group ID produce the same primary output via different paths
    pub cross_recipe_group: Option<u32>,
}
