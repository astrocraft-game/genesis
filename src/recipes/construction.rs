#![allow(dead_code)]
use super::types::*;
use super::substance::Substance as S;

const CRG_CEMENT: u32 = 200;
const CRG_BRICK: u32 = 201;
const CRG_MORTAR: u32 = 202;
const CRG_GLASS: u32 = 203;

pub static CONSTRUCTION_RECIPES: &[Recipe] = &[
    // ===== LIME =====
    Recipe { id: 400, name: "Quicklime (Lime Burning)", category: RecipeCategory::Construction,
        inputs: &[(S::Limestone, 1.8)],
        outputs: &[(S::Quicklite, 1.0)],
        byproducts: &[],
        min_temp_c: 950, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0, cross_recipe_group: None },
    Recipe { id: 401, name: "Slaked Lime", category: RecipeCategory::Construction,
        inputs: &[(S::Quicklite, 0.76), (S::Water, 0.24)],
        outputs: &[(S::SlakedLite, 1.0)],
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },

    // ===== CEMENT (2 paths) =====
    Recipe { id: 405, name: "Portland Cement", category: RecipeCategory::Construction,
        inputs: &[(S::Limestone, 1.2), (S::Clay, 0.3), (S::ite, 0.05)],
        outputs: &[(S::Clinker, 1.0)],
        byproducts: &[(S::FlyAsh, 0.1)],
        min_temp_c: 1450, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: Some(CRG_CEMENT) },
    Recipe { id: 406, name: "Cement from Clinker", category: RecipeCategory::Construction,
        inputs: &[(S::Clinker, 0.95), (S::ite, 0.05)],
        outputs: &[(S::PortlandCite, 1.0)],
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: Some(CRG_CEMENT) },

    // ===== CONCRETE =====
    Recipe { id: 410, name: "Standard Concrete", category: RecipeCategory::Construction,
        inputs: &[(S::PortlandCite, 0.14), (S::Sand, 0.28), (S::Gravel, 0.44), (S::Water, 0.08)],
        outputs: &[(S::Concrete, 1.0)],
        byproducts: &[],
        min_temp_c: 10, pressure_atm: 1.0, catalyst: None, duration_hours: 672.0, cross_recipe_group: None },
    Recipe { id: 411, name: "Reinforced Concrete", category: RecipeCategory::Construction,
        inputs: &[(S::Concrete, 0.97), (S::LowCarbonSteel, 0.03)],
        outputs: &[(S::ReinforcedConcrete, 1.0)],
        byproducts: &[],
        min_temp_c: 10, pressure_atm: 1.0, catalyst: None, duration_hours: 672.0, cross_recipe_group: None },

    // ===== BRICK (3 paths) =====
    Recipe { id: 420, name: "Adobe Brick", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.60), (S::Sand, 0.25), (S::StrawFiber, 0.10), (S::Water, 0.10)],
        outputs: &[(S::AdobeBrick, 1.0)],
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 672.0, cross_recipe_group: Some(CRG_BRICK) },
    Recipe { id: 421, name: "Fired Clay Brick", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.85), (S::Water, 0.15)],
        outputs: &[(S::Brick, 1.0)],
        byproducts: &[],
        min_temp_c: 1000, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0, cross_recipe_group: Some(CRG_BRICK) },
    Recipe { id: 422, name: "Fire Brick (Refractory)", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.5), (S::Alumina, 0.4), (S::SilicaSand, 0.1)],
        outputs: &[(S::FireBrick, 1.0)],
        byproducts: &[],
        min_temp_c: 1600, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0, cross_recipe_group: Some(CRG_BRICK) },

    // ===== MORTAR (2 paths) =====
    Recipe { id: 425, name: "Lime Mortar", category: RecipeCategory::Construction,
        inputs: &[(S::SlakedLite, 0.25), (S::Sand, 0.75)],
        outputs: &[(S::LimeMortar, 1.0)],
        byproducts: &[],
        min_temp_c: 5, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: Some(CRG_MORTAR) },
    Recipe { id: 426, name: "Cement Mortar", category: RecipeCategory::Construction,
        inputs: &[(S::PortlandCite, 0.2), (S::Sand, 0.7), (S::Water, 0.1)],
        outputs: &[(S::CementMortar, 1.0)],
        byproducts: &[],
        min_temp_c: 5, pressure_atm: 1.0, catalyst: None, duration_hours: 48.0, cross_recipe_group: Some(CRG_MORTAR) },

    // ===== PLASTER =====
    Recipe { id: 430, name: "Plaster of Paris", category: RecipeCategory::Construction,
        inputs: &[(S::ite, 1.2)],
        outputs: &[(S::PlasterOfParis, 1.0)],
        byproducts: &[(S::Water, 0.2)],
        min_temp_c: 160, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 431, name: "Stucco", category: RecipeCategory::Construction,
        inputs: &[(S::PortlandCite, 0.15), (S::SlakedLite, 0.05), (S::Sand, 0.7), (S::Water, 0.1)],
        outputs: &[(S::Stucco, 1.0)],
        byproducts: &[],
        min_temp_c: 5, pressure_atm: 1.0, catalyst: None, duration_hours: 168.0, cross_recipe_group: None },

    // ===== GLASS (2 paths) =====
    Recipe { id: 440, name: "Soda-Lime Glass", category: RecipeCategory::Construction,
        inputs: &[(S::SilicaSand, 0.72), (S::SodaAsh, 0.15), (S::Limestone, 0.10)],
        outputs: &[(S::Glass, 1.0)],
        byproducts: &[],
        min_temp_c: 1550, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0, cross_recipe_group: Some(CRG_GLASS) },
    Recipe { id: 441, name: "Borosilicate Glass (Pyrex)", category: RecipeCategory::Construction,
        inputs: &[(S::SilicaSand, 0.80), (S::SodaAsh, 0.04), (S::Alumina, 0.02)],
        outputs: &[(S::BorosilicateGlass, 1.0)],
        byproducts: &[],
        min_temp_c: 1650, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0, cross_recipe_group: Some(CRG_GLASS) },

    // ===== CERAMICS =====
    Recipe { id: 450, name: "Porcelain", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.50), (S::FeldsparOre, 0.25), (S::SilicaSand, 0.25)],
        outputs: &[(S::Porcelain, 1.0)],
        byproducts: &[],
        min_temp_c: 1300, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0, cross_recipe_group: None },

    // ===== ASPHALT =====
    Recipe { id: 455, name: "Asphalt (Hot Mix)", category: RecipeCategory::Construction,
        inputs: &[(S::Gravel, 0.60), (S::Sand, 0.35), (S::Bitumen, 0.05)],
        outputs: &[(S::Asphalt, 1.0)],
        byproducts: &[],
        min_temp_c: 160, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },

    // ===== ENGINEERED WOOD =====
    Recipe { id: 460, name: "Plywood", category: RecipeCategory::Construction,
        inputs: &[(S::WoodLogs, 1.3), (S::Formaldehyde, 0.05)],
        outputs: &[(S::Plywood, 1.0)],
        byproducts: &[(S::WoodChips, 0.3)],
        min_temp_c: 140, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },
    Recipe { id: 461, name: "MDF (Medium Density Fiberboard)", category: RecipeCategory::Construction,
        inputs: &[(S::WoodChips, 0.85), (S::Formaldehyde, 0.09), (S::Water, 0.06)],
        outputs: &[(S::MDF, 1.0)],
        byproducts: &[],
        min_temp_c: 190, pressure_atm: 3.0, catalyst: None, duration_hours: 0.3, cross_recipe_group: None },

    // ===== INSULATION =====
    Recipe { id: 465, name: "Fiberglass Insulation", category: RecipeCategory::Construction,
        inputs: &[(S::SilicaSand, 0.5), (S::Glass, 0.45)],
        outputs: &[(S::Fiberglass, 1.0)],
        byproducts: &[],
        min_temp_c: 1450, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 466, name: "Rock Wool Insulation", category: RecipeCategory::Construction,
        inputs: &[(S::Limestone, 0.5), (S::Slag, 0.5)],
        outputs: &[(S::RockWool, 1.0)],
        byproducts: &[],
        min_temp_c: 1500, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
];
