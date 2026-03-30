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

    // ===== MORE CONSTRUCTION =====
    Recipe { id: 467, name: "Roman Concrete", category: RecipeCategory::Construction,
        inputs: &[(S::Quicklite, 0.25), (S::Sand, 0.5), (S::Water, 0.25)], // volcanic ash as Sand placeholder
        outputs: &[(S::Concrete, 1.0)],
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 2160.0, cross_recipe_group: None },
    Recipe { id: 468, name: "Slag Cement", category: RecipeCategory::Construction,
        inputs: &[(S::Slag, 0.5), (S::PortlandCite, 0.5)],
        outputs: &[(S::PortlandCite, 1.0)],
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: Some(CRG_CEMENT) },
    Recipe { id: 469, name: "High-Strength Concrete", category: RecipeCategory::Construction,
        inputs: &[(S::PortlandCite, 0.18), (S::Sand, 0.25), (S::Gravel, 0.42), (S::Water, 0.06), (S::SilicaSand, 0.02)],
        outputs: &[(S::Concrete, 1.0)],
        byproducts: &[],
        min_temp_c: 10, pressure_atm: 1.0, catalyst: None, duration_hours: 672.0, cross_recipe_group: None },
    Recipe { id: 470, name: "Lead Crystal Glass", category: RecipeCategory::Construction,
        inputs: &[(S::SilicaSand, 0.55), (S::Lead, 0.24), (S::SodaAsh, 0.13)],
        outputs: &[(S::Glass, 1.0)],
        byproducts: &[],
        min_temp_c: 1500, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0, cross_recipe_group: Some(CRG_GLASS) },
    Recipe { id: 471, name: "Lime Plaster (Traditional)", category: RecipeCategory::Construction,
        inputs: &[(S::SlakedLite, 0.25), (S::Sand, 0.70), (S::Water, 0.1)],
        outputs: &[(S::Stucco, 1.0)],
        byproducts: &[],
        min_temp_c: 5, pressure_atm: 1.0, catalyst: None, duration_hours: 720.0, cross_recipe_group: None },
    Recipe { id: 472, name: "Earthenware (Low-fire Pottery)", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.9), (S::Water, 0.15)],
        outputs: &[(S::Brick, 1.0)], // earthenware placeholder
        byproducts: &[],
        min_temp_c: 1000, pressure_atm: 1.0, catalyst: None, duration_hours: 8.0, cross_recipe_group: None },
    Recipe { id: 473, name: "Stoneware (High-fire Pottery)", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.85), (S::FeldsparOre, 0.1), (S::Water, 0.1)],
        outputs: &[(S::Porcelain, 1.0)],
        byproducts: &[],
        min_temp_c: 1250, pressure_atm: 1.0, catalyst: None, duration_hours: 10.0, cross_recipe_group: None },
    Recipe { id: 474, name: "Cellulose Insulation", category: RecipeCategory::Construction,
        inputs: &[(S::PaperProduct, 0.8), (S::SulfuricAcid, 0.01)], // boric acid placeholder
        outputs: &[(S::RockWool, 1.0)], // cellulose placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },
    Recipe { id: 475, name: "Pressure Treated Lumber", category: RecipeCategory::Construction,
        inputs: &[(S::WoodLogs, 1.0), (S::Copper, 0.005), (S::Water, 0.2)],
        outputs: &[(S::WoodLogs, 1.0)],
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 14.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: None },
    Recipe { id: 476, name: "OSB (Oriented Strand Board)", category: RecipeCategory::Construction,
        inputs: &[(S::WoodChips, 0.9), (S::Formaldehyde, 0.05)],
        outputs: &[(S::Plywood, 1.0)],
        byproducts: &[],
        min_temp_c: 200, pressure_atm: 3.0, catalyst: None, duration_hours: 0.3, cross_recipe_group: None },
    Recipe { id: 477, name: "Particleboard", category: RecipeCategory::Construction,
        inputs: &[(S::WoodChips, 0.88), (S::Formaldehyde, 0.08), (S::Water, 0.04)],
        outputs: &[(S::MDF, 1.0)],
        byproducts: &[],
        min_temp_c: 180, pressure_atm: 2.5, catalyst: None, duration_hours: 0.3, cross_recipe_group: None },

    // ===== EARTHEN CONSTRUCTION =====
    Recipe { id: 2200, name: "Rammed Earth", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.30), (S::Sand, 0.35), (S::Gravel, 0.25), (S::Water, 0.10)],
        outputs: &[(S::Concrete, 1.0)], // rammed earth placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 672.0, cross_recipe_group: None },
    Recipe { id: 2201, name: "Cob", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.35), (S::Sand, 0.30), (S::StrawFiber, 0.15), (S::Water, 0.20)],
        outputs: &[(S::AdobeBrick, 1.0)], // cob placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 720.0, cross_recipe_group: None },
    Recipe { id: 2202, name: "Wattle and Daub", category: RecipeCategory::Construction,
        inputs: &[(S::WoodLogs, 0.40), (S::Clay, 0.40), (S::StrawFiber, 0.20)],
        outputs: &[(S::AdobeBrick, 1.0)], // wattle-daub placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 168.0, cross_recipe_group: None },
    Recipe { id: 2203, name: "Compressed Earth Block (CEB)", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.40), (S::Sand, 0.50), (S::PortlandCite, 0.10)],
        outputs: &[(S::Brick, 1.0)], // CEB placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 672.0, cross_recipe_group: None },

    // ===== LIME FINISHES =====
    Recipe { id: 2204, name: "Lime Wash (Whitewash)", category: RecipeCategory::Construction,
        inputs: &[(S::SlakedLite, 0.60), (S::Water, 0.40)],
        outputs: &[(S::Stucco, 1.0)], // limewash placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0, cross_recipe_group: None },
    Recipe { id: 2205, name: "Venetian Plaster", category: RecipeCategory::Construction,
        inputs: &[(S::SlakedLite, 0.40), (S::Marble, 0.45), (S::Water, 0.15)],
        outputs: &[(S::Stucco, 1.0)], // venetian plaster placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 72.0, cross_recipe_group: None },
    Recipe { id: 2206, name: "Terrazzo", category: RecipeCategory::Construction,
        inputs: &[(S::Marble, 0.70), (S::PortlandCite, 0.30)],
        outputs: &[(S::Concrete, 1.0)], // terrazzo placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 168.0, cross_recipe_group: None },

    // ===== CERAMICS & TILE =====
    Recipe { id: 2207, name: "Ceramic Tile", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.70), (S::FeldsparOre, 0.30)],
        outputs: &[(S::Porcelain, 1.0)], // ceramic tile placeholder
        byproducts: &[],
        min_temp_c: 1100, pressure_atm: 1.0, catalyst: None, duration_hours: 10.0, cross_recipe_group: None },
    Recipe { id: 2208, name: "Bone China", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 0.25), (S::Calcium, 0.50), (S::FeldsparOre, 0.25)],
        outputs: &[(S::Porcelain, 1.0)], // bone china placeholder
        byproducts: &[],
        min_temp_c: 1260, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0, cross_recipe_group: None },

    // ===== GLASS VARIANTS =====
    Recipe { id: 2209, name: "Tempered Glass (Safety)", category: RecipeCategory::Construction,
        inputs: &[(S::Glass, 1.0)],
        outputs: &[(S::Glass, 1.0)],
        byproducts: &[],
        min_temp_c: 650, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },
    Recipe { id: 2210, name: "Laminated Glass", category: RecipeCategory::Construction,
        inputs: &[(S::Glass, 0.90), (S::Polyethylene, 0.10)],
        outputs: &[(S::Glass, 1.0)], // laminated glass placeholder
        byproducts: &[],
        min_temp_c: 140, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 2211, name: "Float Glass", category: RecipeCategory::Construction,
        inputs: &[(S::SilicaSand, 0.72), (S::SodaAsh, 0.15), (S::Limestone, 0.10)],
        outputs: &[(S::Glass, 1.0)],
        byproducts: &[],
        min_temp_c: 1550, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0, cross_recipe_group: Some(CRG_GLASS) },
    Recipe { id: 2212, name: "Blown Glass", category: RecipeCategory::Construction,
        inputs: &[(S::Glass, 1.05)],
        outputs: &[(S::Glass, 1.0)],
        byproducts: &[],
        min_temp_c: 1100, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },
    Recipe { id: 2213, name: "Stained Glass", category: RecipeCategory::Construction,
        inputs: &[(S::Glass, 0.90), (S::Cobalt, 0.05), (S::Copper, 0.05)],
        outputs: &[(S::Glass, 1.0)], // stained glass placeholder
        byproducts: &[],
        min_temp_c: 1200, pressure_atm: 1.0, catalyst: None, duration_hours: 8.0, cross_recipe_group: None },

    // ===== SPECIALTY BRICKS =====
    Recipe { id: 2214, name: "Sand-Lime Brick", category: RecipeCategory::Construction,
        inputs: &[(S::Sand, 0.60), (S::SlakedLite, 0.30), (S::Water, 0.10)],
        outputs: &[(S::Brick, 1.0)],
        byproducts: &[],
        min_temp_c: 180, pressure_atm: 1.2, catalyst: None, duration_hours: 8.0, cross_recipe_group: Some(CRG_BRICK) },
    Recipe { id: 2215, name: "Fly Ash Brick", category: RecipeCategory::Construction,
        inputs: &[(S::FlyAsh, 0.50), (S::PortlandCite, 0.20), (S::Sand, 0.30)],
        outputs: &[(S::Brick, 1.0)],
        byproducts: &[],
        min_temp_c: 60, pressure_atm: 1.0, catalyst: None, duration_hours: 672.0, cross_recipe_group: Some(CRG_BRICK) },

    // ===== BIO-CONSTRUCTION =====
    Recipe { id: 2216, name: "Hempcrete", category: RecipeCategory::Construction,
        inputs: &[(S::SlakedLite, 0.35), (S::StrawFiber, 0.50), (S::Water, 0.15)],
        outputs: &[(S::Concrete, 1.0)], // hempcrete placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 2160.0, cross_recipe_group: None },

    // ===== ENGINEERED TIMBER =====
    Recipe { id: 2217, name: "CLT (Cross-Laminated Timber)", category: RecipeCategory::Construction,
        inputs: &[(S::WoodLogs, 0.95), (S::Formaldehyde, 0.05)],
        outputs: &[(S::Plywood, 1.0)], // CLT placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: None },
    Recipe { id: 2218, name: "Glulam (Glued Laminated Timber)", category: RecipeCategory::Construction,
        inputs: &[(S::WoodLogs, 0.95), (S::Formaldehyde, 0.05)],
        outputs: &[(S::Plywood, 1.0)], // glulam placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: None },
    Recipe { id: 2219, name: "LVL (Laminated Veneer Lumber)", category: RecipeCategory::Construction,
        inputs: &[(S::WoodLogs, 0.93), (S::Formaldehyde, 0.07)],
        outputs: &[(S::Plywood, 1.0)], // LVL placeholder
        byproducts: &[],
        min_temp_c: 150, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },

    // ===== INSULATION MATERIALS =====
    Recipe { id: 2220, name: "Cork Insulation", category: RecipeCategory::Construction,
        inputs: &[(S::WoodLogs, 1.5)],
        outputs: &[(S::RockWool, 1.0)], // cork insulation placeholder
        byproducts: &[],
        min_temp_c: 350, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },
    Recipe { id: 2221, name: "Vermiculite Expansion", category: RecipeCategory::Construction,
        inputs: &[(S::Sand, 1.0)],
        outputs: &[(S::RockWool, 1.0)], // vermiculite placeholder
        byproducts: &[],
        min_temp_c: 900, pressure_atm: 1.0, catalyst: None, duration_hours: 0.1, cross_recipe_group: None },
    Recipe { id: 2222, name: "Perlite Expansion", category: RecipeCategory::Construction,
        inputs: &[(S::Sand, 1.0)],
        outputs: &[(S::RockWool, 1.0)], // perlite placeholder
        byproducts: &[],
        min_temp_c: 1000, pressure_atm: 1.0, catalyst: None, duration_hours: 0.1, cross_recipe_group: None },
    Recipe { id: 2223, name: "EPS Foam (Expanded Polystyrene)", category: RecipeCategory::Construction,
        inputs: &[(S::Polyethylene, 0.90), (S::Steam, 0.10)],
        outputs: &[(S::Polyethylene, 1.0)], // EPS placeholder
        byproducts: &[],
        min_temp_c: 100, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },
    Recipe { id: 2224, name: "XPS Foam (Extruded Polystyrene)", category: RecipeCategory::Construction,
        inputs: &[(S::Polyethylene, 1.0)],
        outputs: &[(S::Polyethylene, 1.0)], // XPS placeholder
        byproducts: &[],
        min_temp_c: 220, pressure_atm: 1.0, catalyst: None, duration_hours: 0.3, cross_recipe_group: None },
    Recipe { id: 2225, name: "PUR Foam Insulation", category: RecipeCategory::Construction,
        inputs: &[(S::CrudeOil, 0.70), (S::Ammonia, 0.30)],
        outputs: &[(S::RockWool, 1.0)], // PUR foam placeholder
        byproducts: &[],
        min_temp_c: 60, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },
    Recipe { id: 2226, name: "Foam Glass Insulation", category: RecipeCategory::Construction,
        inputs: &[(S::Glass, 0.90), (S::CarbonBlack, 0.10)],
        outputs: &[(S::Fiberglass, 1.0)], // foam glass placeholder
        byproducts: &[],
        min_temp_c: 850, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: None },

    // ===== ROOFING MATERIALS =====
    Recipe { id: 2227, name: "Thatch Roofing", category: RecipeCategory::Construction,
        inputs: &[(S::StrawFiber, 1.0)],
        outputs: &[(S::StrawFiber, 1.0)], // thatch placeholder
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 8.0, cross_recipe_group: None },
    Recipe { id: 2228, name: "Slate Roofing", category: RecipeCategory::Construction,
        inputs: &[(S::Slate, 1.1)],
        outputs: &[(S::Slate, 1.0)], // cut slate placeholder
        byproducts: &[(S::Gravel, 0.1)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 2229, name: "Clay Roof Tiles", category: RecipeCategory::Construction,
        inputs: &[(S::Clay, 1.0)],
        outputs: &[(S::Brick, 1.0)], // roof tile placeholder
        byproducts: &[],
        min_temp_c: 1050, pressure_atm: 1.0, catalyst: None, duration_hours: 16.0, cross_recipe_group: None },
];
