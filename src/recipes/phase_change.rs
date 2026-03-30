#![allow(dead_code)]
use super::types::*;
use super::substance::Substance as S;

pub static PHASE_CHANGE_RECIPES: &[Recipe] = &[
    // ===== WATER =====
    Recipe { id: 700, name: "Ice → Water (Melting)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Water, 1.0)], outputs: &[(S::Water, 1.0)], byproducts: &[],
        min_temp_c: 0, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },
    Recipe { id: 701, name: "Water → Steam (Boiling)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Water, 1.0)], outputs: &[(S::Steam, 1.0)], byproducts: &[],
        min_temp_c: 100, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },
    Recipe { id: 702, name: "Steam → Water (Condensation)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Steam, 1.0)], outputs: &[(S::Water, 1.0)], byproducts: &[],
        min_temp_c: -100, pressure_atm: 1.0, catalyst: None, duration_hours: 0.1, cross_recipe_group: None },
    Recipe { id: 703, name: "Freeze-Drying (Sublimation)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Water, 1.2)], outputs: &[(S::Water, 1.0)], byproducts: &[(S::Water, 0.2)],
        min_temp_c: -40, pressure_atm: 0.01, catalyst: None, duration_hours: 24.0, cross_recipe_group: None },

    // ===== METALS =====
    Recipe { id: 710, name: "Iron Sintering (Powder → Pellets)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Hematite, 1.1)], outputs: &[(S::Hematite, 1.0)], byproducts: &[],
        min_temp_c: 1300, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 711, name: "Steel Quenching (Hardening)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::HighCarbonSteel, 1.0), (S::Water, 5.0)],
        outputs: &[(S::HighCarbonSteel, 1.0)], byproducts: &[(S::Steam, 4.0)],
        min_temp_c: 800, pressure_atm: 1.0, catalyst: None, duration_hours: 0.01, cross_recipe_group: None },
    Recipe { id: 712, name: "Steel Tempering", category: RecipeCategory::PhaseChange,
        inputs: &[(S::HighCarbonSteel, 1.0)],
        outputs: &[(S::MediumCarbonSteel, 1.0)], byproducts: &[],
        min_temp_c: 300, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },
    Recipe { id: 713, name: "Steel Annealing (Softening)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::HighCarbonSteel, 1.0)],
        outputs: &[(S::LowCarbonSteel, 1.0)], byproducts: &[],
        min_temp_c: 700, pressure_atm: 1.0, catalyst: None, duration_hours: 8.0, cross_recipe_group: None },
    Recipe { id: 714, name: "Copper Annealing", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Copper, 1.0)], outputs: &[(S::Copper, 1.0)], byproducts: &[],
        min_temp_c: 400, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },

    // ===== GLASS =====
    Recipe { id: 720, name: "Glass Annealing (Stress Relief)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Glass, 1.0)], outputs: &[(S::Glass, 1.0)], byproducts: &[],
        min_temp_c: 550, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0, cross_recipe_group: None },
    Recipe { id: 721, name: "Glass Tempering (Safety Glass)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Glass, 1.0)], outputs: &[(S::Glass, 1.0)], byproducts: &[],
        min_temp_c: 620, pressure_atm: 1.0, catalyst: None, duration_hours: 0.1, cross_recipe_group: None },

    // ===== CO2 =====
    Recipe { id: 725, name: "Dry Ice Sublimation", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Carbon, 1.0)], outputs: &[(S::Air, 1.0)], byproducts: &[],
        min_temp_c: -78, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },

    // ===== CRYSTALLIZATION =====
    Recipe { id: 730, name: "Salt Crystallization (from brine)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Water, 5.0)], outputs: &[(S::Salt, 1.0)], byproducts: &[(S::Water, 4.0)],
        min_temp_c: 100, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0, cross_recipe_group: None },
    Recipe { id: 731, name: "Sugar Crystallization", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Water, 3.0)], outputs: &[(S::Sugar, 1.0)], byproducts: &[(S::Water, 2.0)],
        min_temp_c: 70, pressure_atm: 0.3, catalyst: None, duration_hours: 8.0, cross_recipe_group: None },

    // ===== CASTING =====
    Recipe { id: 735, name: "Iron Casting (Melting → Mold)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::CastIron, 1.02)], outputs: &[(S::CastIron, 1.0)], byproducts: &[(S::Slag, 0.02)],
        min_temp_c: 1200, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: None },
    Recipe { id: 736, name: "Bronze Casting", category: RecipeCategory::PhaseChange,
        inputs: &[(S::TinBronze, 1.01)], outputs: &[(S::TinBronze, 1.0)], byproducts: &[],
        min_temp_c: 1000, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 737, name: "Aluminum Casting", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Aluminum, 1.02)], outputs: &[(S::Aluminum, 1.0)], byproducts: &[(S::Slag, 0.01)],
        min_temp_c: 660, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 738, name: "Gold Casting", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Gold, 1.0)], outputs: &[(S::Gold, 1.0)], byproducts: &[],
        min_temp_c: 1064, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },

    // ===== DISTILLATION =====
    Recipe { id: 740, name: "Mercury Distillation (Purification)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Mercury, 1.01)], outputs: &[(S::Mercury, 1.0)], byproducts: &[],
        min_temp_c: 357, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 741, name: "Zinc Distillation (Purification)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Zinc, 1.05)], outputs: &[(S::Zinc, 1.0)], byproducts: &[(S::Lead, 0.02)],
        min_temp_c: 907, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: None },
];
