#![allow(dead_code)]
use super::types::*;
use super::substance::Substance as S;

pub static ELECTROCHEMISTRY_RECIPES: &[Recipe] = &[
    // ===== BATTERIES =====
    Recipe { id: 950, name: "Lead-Acid Battery (Discharge)", category: RecipeCategory::Manufacturing,
        inputs: &[(S::Lead, 0.5), (S::SulfuricAcid, 0.5)], outputs: &[(S::LeadAcidBattery, 1.0)],
        byproducts: &[(S::Water, 0.2)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: None },
    Recipe { id: 951, name: "Li-ion Battery (Assembly)", category: RecipeCategory::Manufacturing,
        inputs: &[(S::Lithium, 0.1), (S::Cobalt, 0.2), (S::Carbon, 0.1), (S::Copper, 0.1)],
        outputs: &[(S::LithiumIonBattery, 1.0)], byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 8.0, cross_recipe_group: None },
    Recipe { id: 952, name: "Zinc-Carbon Battery", category: RecipeCategory::Manufacturing,
        inputs: &[(S::Zinc, 0.3), (S::Manganese, 0.4), (S::Carbon, 0.1), (S::Salt, 0.2)],
        outputs: &[(S::ZincCarbonBattery, 1.0)], byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },

    // ===== ELECTROPLATING =====
    Recipe { id: 960, name: "Chrome Plating", category: RecipeCategory::Manufacturing,
        inputs: &[(S::Chromium, 0.01), (S::SulfuricAcid, 0.1), (S::Water, 1.0)],
        outputs: &[(S::ChromePlating, 0.01)], byproducts: &[(S::HydrogenGas, 0.005)],
        min_temp_c: 50, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 961, name: "Gold Plating", category: RecipeCategory::Manufacturing,
        inputs: &[(S::Gold, 0.005), (S::SodiumCyanide, 0.01), (S::Water, 1.0)],
        outputs: &[(S::GoldPlating, 0.005)], byproducts: &[],
        min_temp_c: 55, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },
    Recipe { id: 962, name: "Nickel Plating (Watts Bath)", category: RecipeCategory::Manufacturing,
        inputs: &[(S::Nickel, 0.02), (S::SulfuricAcid, 0.05), (S::Water, 1.0)],
        outputs: &[(S::NickelPlating, 0.02)], byproducts: &[],
        min_temp_c: 55, pressure_atm: 1.0, catalyst: None, duration_hours: 3.0, cross_recipe_group: None },
    Recipe { id: 963, name: "Zinc Galvanizing", category: RecipeCategory::Manufacturing,
        inputs: &[(S::Zinc, 0.03), (S::HydrochloricAcid, 0.05), (S::Water, 1.0)],
        outputs: &[(S::ZincPlating, 0.03)], byproducts: &[],
        min_temp_c: 30, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 964, name: "Anodizing Aluminum", category: RecipeCategory::Manufacturing,
        inputs: &[(S::Aluminum, 1.0), (S::SulfuricAcid, 0.3), (S::Water, 2.0)],
        outputs: &[(S::Alumina, 0.05)], byproducts: &[(S::HydrogenGas, 0.01)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },

    // ===== NUCLEAR FISSION =====
    Recipe { id: 970, name: "U-235 Fission", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Uranium, 0.001)], outputs: &[(S::Steam, 100.0)],
        byproducts: &[(S::Slag, 0.001)], // fission products
        min_temp_c: 300, pressure_atm: 150.0, catalyst: None, duration_hours: 8760.0, cross_recipe_group: None },
    Recipe { id: 971, name: "Pu-239 Breeding (from U-238)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Uranium, 1.0)], outputs: &[(S::Uranium, 0.6)], // Pu placeholder
        byproducts: &[],
        min_temp_c: 500, pressure_atm: 1.0, catalyst: None, duration_hours: 8760.0, cross_recipe_group: None },

    // ===== NUCLEAR FUSION =====
    Recipe { id: 975, name: "D-T Fusion", category: RecipeCategory::PhaseChange,
        inputs: &[(S::HydrogenGas, 0.005)], // D+T
        outputs: &[(S::Steam, 500.0)], // enormous energy
        byproducts: &[],
        min_temp_c: i32::MAX, pressure_atm: 1.0, catalyst: None, duration_hours: 0.001, cross_recipe_group: None },
    Recipe { id: 976, name: "Stellar pp Chain (H → He)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::HydrogenGas, 4.0)], outputs: &[(S::Steam, 1000.0)], // placeholder for He+energy
        byproducts: &[],
        min_temp_c: i32::MAX, pressure_atm: 1000000.0, catalyst: None, duration_hours: 8760000000.0, cross_recipe_group: None },
    Recipe { id: 977, name: "Triple-Alpha (He → C)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Steam, 3.0)], // 3 He-4 placeholder
        outputs: &[(S::Carbon, 1.0)], byproducts: &[],
        min_temp_c: i32::MAX, pressure_atm: 100000.0, catalyst: None, duration_hours: 1000000.0, cross_recipe_group: None },

    // ===== NUCLEOSYNTHESIS =====
    Recipe { id: 980, name: "Silicon Burning → Iron Peak", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Silicon, 1.0)], outputs: &[(S::Iron, 1.0)], byproducts: &[],
        min_temp_c: i32::MAX, pressure_atm: 10000000.0, catalyst: None, duration_hours: 24.0, cross_recipe_group: None },
    Recipe { id: 981, name: "r-Process (Neutron Star Merger → Gold)", category: RecipeCategory::PhaseChange,
        inputs: &[(S::Iron, 1.0)], outputs: &[(S::Gold, 0.001), (S::Platinum, 0.001), (S::Uranium, 0.0001)],
        byproducts: &[(S::Iron, 0.99)],
        min_temp_c: i32::MAX, pressure_atm: 100000.0, catalyst: None, duration_hours: 0.001, cross_recipe_group: None },
];
