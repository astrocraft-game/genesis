#![allow(dead_code)]
use super::types::*;
use super::substance::Substance as S;

/// Cross-recipe group IDs for metals with multiple extraction paths
const CRG_IRON: u32 = 1;
const CRG_COPPER: u32 = 2;
const CRG_GOLD: u32 = 3;
const CRG_SILVER: u32 = 4;
const CRG_ZINC: u32 = 5;
const CRG_ALUMINUM: u32 = 6;
const CRG_NICKEL: u32 = 7;
const CRG_LEAD: u32 = 8;
const CRG_LITHIUM: u32 = 9;
const CRG_MAGNESIUM: u32 = 10;
const CRG_HYDROGEN: u32 = 11;
const CRG_TITANIUM: u32 = 12;
const CRG_CHROMIUM: u32 = 13;

pub static EXTRACTION_RECIPES: &[Recipe] = &[
    // ===== IRON (6 paths) =====
    Recipe { id: 1, name: "Bloomery Iron Smelting", category: RecipeCategory::Extraction,
        inputs: &[(S::Hematite, 2.0), (S::Charcoal, 10.0)],
        outputs: &[(S::WroughtIron, 1.0)],
        byproducts: &[(S::Slag, 1.5)],
        min_temp_c: 1200, pressure_atm: 1.0, catalyst: None, duration_hours: 10.0,
        cross_recipe_group: Some(CRG_IRON) },
    Recipe { id: 2, name: "Blast Furnace (Hematite)", category: RecipeCategory::Extraction,
        inputs: &[(S::Hematite, 1.6), (S::Coke, 0.5), (S::Limestone, 0.3)],
        outputs: &[(S::PigIron, 1.0)],
        byproducts: &[(S::Slag, 0.4)],
        min_temp_c: 1500, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: Some(CRG_IRON) },
    Recipe { id: 3, name: "Blast Furnace (Magnetite)", category: RecipeCategory::Extraction,
        inputs: &[(S::Magnetite, 1.5), (S::Coke, 0.5), (S::Limestone, 0.3)],
        outputs: &[(S::PigIron, 1.0)],
        byproducts: &[(S::Slag, 0.4)],
        min_temp_c: 1500, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: Some(CRG_IRON) },
    Recipe { id: 4, name: "Direct Reduction (Hydrogen)", category: RecipeCategory::Extraction,
        inputs: &[(S::Hematite, 1.4), (S::HydrogenGas, 0.1)],
        outputs: &[(S::Iron, 1.0)],
        byproducts: &[(S::Water, 0.5)],
        min_temp_c: 900, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: Some(CRG_IRON) },
    Recipe { id: 5, name: "Pyrite Roasting → Iron", category: RecipeCategory::Extraction,
        inputs: &[(S::PyriteOre, 2.0), (S::Coke, 0.3)],
        outputs: &[(S::Iron, 0.9)],
        byproducts: &[(S::SulfuricAcid, 0.8), (S::Slag, 0.3)],
        min_temp_c: 700, pressure_atm: 1.0, catalyst: None, duration_hours: 8.0,
        cross_recipe_group: Some(CRG_IRON) },
    Recipe { id: 6, name: "Wrought Iron from Pig Iron", category: RecipeCategory::Refining,
        inputs: &[(S::PigIron, 1.1)],
        outputs: &[(S::WroughtIron, 1.0)],
        byproducts: &[(S::Slag, 0.1)],
        min_temp_c: 1100, pressure_atm: 1.0, catalyst: None, duration_hours: 3.0,
        cross_recipe_group: Some(CRG_IRON) },

    // ===== COPPER (5 paths) =====
    Recipe { id: 10, name: "Ancient Copper Smelting (Malachite)", category: RecipeCategory::Extraction,
        inputs: &[(S::CopperOxideOre, 2.0), (S::Charcoal, 3.0)],
        outputs: &[(S::Copper, 1.0)],
        byproducts: &[(S::Slag, 1.0)],
        min_temp_c: 1200, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: Some(CRG_COPPER) },
    Recipe { id: 11, name: "Flash Smelting (Chalcopyrite)", category: RecipeCategory::Extraction,
        inputs: &[(S::Chalcopyrite, 3.0), (S::Oxygen, 0.5), (S::SilicaSand, 0.3)],
        outputs: &[(S::CopperMatte, 1.5)],
        byproducts: &[(S::Slag, 1.0)],
        min_temp_c: 1300, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0,
        cross_recipe_group: Some(CRG_COPPER) },
    Recipe { id: 12, name: "Copper Converting", category: RecipeCategory::Refining,
        inputs: &[(S::CopperMatte, 1.5), (S::Oxygen, 0.3)],
        outputs: &[(S::BlisterCopper, 1.0)],
        byproducts: &[(S::Slag, 0.3)],
        min_temp_c: 1250, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0,
        cross_recipe_group: Some(CRG_COPPER) },
    Recipe { id: 13, name: "Copper Electrorefining", category: RecipeCategory::Refining,
        inputs: &[(S::BlisterCopper, 1.02), (S::SulfuricAcid, 0.05)],
        outputs: &[(S::Copper, 1.0)],
        byproducts: &[(S::Gold, 0.0001), (S::Silver, 0.001)],
        min_temp_c: 60, pressure_atm: 1.0, catalyst: None, duration_hours: 168.0,
        cross_recipe_group: Some(CRG_COPPER) },
    Recipe { id: 14, name: "Copper Heap Leaching", category: RecipeCategory::Extraction,
        inputs: &[(S::CopperOxideOre, 50.0), (S::SulfuricAcid, 2.0)],
        outputs: &[(S::Copper, 1.0)],
        byproducts: &[(S::Tailings, 48.0)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 720.0,
        cross_recipe_group: Some(CRG_COPPER) },

    // ===== GOLD (4 paths) =====
    Recipe { id: 20, name: "Gold Panning/Gravity", category: RecipeCategory::Extraction,
        inputs: &[(S::GoldOre, 1000.0), (S::Water, 500.0)],
        outputs: &[(S::Gold, 0.005)],
        byproducts: &[(S::Sand, 999.0)],
        min_temp_c: 10, pressure_atm: 1.0, catalyst: None, duration_hours: 8.0,
        cross_recipe_group: Some(CRG_GOLD) },
    Recipe { id: 21, name: "Gold Cyanidation", category: RecipeCategory::Extraction,
        inputs: &[(S::GoldOre, 100.0), (S::SodiumCyanide, 0.05), (S::Oxygen, 0.01)],
        outputs: &[(S::Gold, 0.003)],
        byproducts: &[(S::Tailings, 99.0)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 48.0,
        cross_recipe_group: Some(CRG_GOLD) },
    Recipe { id: 22, name: "Gold Mercury Amalgamation", category: RecipeCategory::Extraction,
        inputs: &[(S::GoldOre, 100.0), (S::Mercury, 5.0)],
        outputs: &[(S::Gold, 0.002)],
        byproducts: &[(S::Mercury, 4.9), (S::Tailings, 99.0)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: Some(CRG_GOLD) },
    Recipe { id: 23, name: "Gold Aqua Regia Refining", category: RecipeCategory::Refining,
        inputs: &[(S::Gold, 1.0), (S::HydrochloricAcid, 3.0), (S::NitricAcid, 1.0)],
        outputs: &[(S::Gold, 0.999)],
        byproducts: &[(S::Water, 2.0)],
        min_temp_c: 80, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: Some(CRG_GOLD) },

    // ===== SILVER (3 paths) =====
    Recipe { id: 30, name: "Silver Cupellation", category: RecipeCategory::Extraction,
        inputs: &[(S::SilverOre, 10.0), (S::Lead, 5.0)],
        outputs: &[(S::Silver, 0.5)],
        byproducts: &[(S::Slag, 14.0)],
        min_temp_c: 1000, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: Some(CRG_SILVER) },
    Recipe { id: 31, name: "Silver Cyanidation", category: RecipeCategory::Extraction,
        inputs: &[(S::SilverOre, 100.0), (S::SodiumCyanide, 0.1)],
        outputs: &[(S::Silver, 0.3)],
        byproducts: &[(S::Tailings, 99.0)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 48.0,
        cross_recipe_group: Some(CRG_SILVER) },
    Recipe { id: 32, name: "Silver from Copper Anode Slimes", category: RecipeCategory::Refining,
        inputs: &[(S::BlisterCopper, 100.0)],
        outputs: &[(S::Silver, 0.1), (S::Copper, 99.5)],
        byproducts: &[(S::Gold, 0.01)],
        min_temp_c: 60, pressure_atm: 1.0, catalyst: None, duration_hours: 168.0,
        cross_recipe_group: Some(CRG_SILVER) },

    // ===== TIN =====
    Recipe { id: 40, name: "Tin Smelting", category: RecipeCategory::Extraction,
        inputs: &[(S::Cassiterite, 1.3), (S::Carbon, 0.3)],
        outputs: &[(S::Tin, 1.0)],
        byproducts: &[(S::Slag, 0.3)],
        min_temp_c: 1200, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: None },

    // ===== LEAD (2 paths) =====
    Recipe { id: 50, name: "Lead Roast-Reduction", category: RecipeCategory::Extraction,
        inputs: &[(S::Galena, 1.2), (S::Carbon, 0.2), (S::Limestone, 0.1)],
        outputs: &[(S::Lead, 1.0)],
        byproducts: &[(S::Slag, 0.3)],
        min_temp_c: 1000, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: Some(CRG_LEAD) },
    Recipe { id: 51, name: "Lead Self-Reduction (Ore Hearth)", category: RecipeCategory::Extraction,
        inputs: &[(S::Galena, 1.5)],
        outputs: &[(S::Lead, 1.0)],
        byproducts: &[(S::Slag, 0.2)],
        min_temp_c: 900, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: Some(CRG_LEAD) },

    // ===== ZINC (3 paths) =====
    Recipe { id: 60, name: "Zinc Roast-Leach-Electrowin", category: RecipeCategory::Extraction,
        inputs: &[(S::Sphalerite, 1.5), (S::SulfuricAcid, 0.5)],
        outputs: &[(S::Zinc, 1.0)],
        byproducts: &[(S::SulfuricAcid, 0.3)],
        min_temp_c: 950, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0,
        cross_recipe_group: Some(CRG_ZINC) },
    Recipe { id: 61, name: "Zinc Retort Distillation", category: RecipeCategory::Extraction,
        inputs: &[(S::Sphalerite, 1.5), (S::Carbon, 0.5)],
        outputs: &[(S::Zinc, 1.0)],
        byproducts: &[(S::Slag, 0.5)],
        min_temp_c: 1100, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0,
        cross_recipe_group: Some(CRG_ZINC) },

    // ===== ALUMINUM (2 paths) =====
    Recipe { id: 70, name: "Bayer Process (Bauxite → Alumina)", category: RecipeCategory::Refining,
        inputs: &[(S::Bauxite, 2.5), (S::SodiumHydroxide, 0.3)],
        outputs: &[(S::Alumina, 1.0)],
        byproducts: &[(S::RedMud, 1.5)],
        min_temp_c: 250, pressure_atm: 6.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: Some(CRG_ALUMINUM) },
    Recipe { id: 71, name: "Hall-Héroult (Alumina → Aluminum)", category: RecipeCategory::Extraction,
        inputs: &[(S::Alumina, 1.9), (S::Carbon, 0.4), (S::Cryolite, 0.05)],
        outputs: &[(S::Aluminum, 1.0)],
        byproducts: &[(S::Slag, 0.1)],
        min_temp_c: 960, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0,
        cross_recipe_group: Some(CRG_ALUMINUM) },

    // ===== TITANIUM =====
    Recipe { id: 80, name: "Kroll Process (Titanium)", category: RecipeCategory::Extraction,
        inputs: &[(S::Rutile, 1.7), (S::Magnesium, 0.9), (S::ChlorineGas, 1.3)],
        outputs: &[(S::Titanium, 1.0)],
        byproducts: &[(S::TitaniumTetrachloride, 0.1)],
        min_temp_c: 850, pressure_atm: 1.0, catalyst: None, duration_hours: 48.0,
        cross_recipe_group: Some(CRG_TITANIUM) },

    // ===== CHROMIUM (2 paths) =====
    Recipe { id: 85, name: "Ferrochrome (Carbothermic)", category: RecipeCategory::Extraction,
        inputs: &[(S::Chromite, 2.5), (S::Coke, 1.0)],
        outputs: &[(S::Ferrochrome, 1.0)],
        byproducts: &[(S::Slag, 1.5)],
        min_temp_c: 1600, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: Some(CRG_CHROMIUM) },
    Recipe { id: 86, name: "Chromium Aluminothermic", category: RecipeCategory::Extraction,
        inputs: &[(S::Chromite, 1.5), (S::Aluminum, 0.5)],
        outputs: &[(S::Chromium, 1.0)],
        byproducts: &[(S::Alumina, 0.5)],
        min_temp_c: 2500, pressure_atm: 1.0, catalyst: None, duration_hours: 0.1,
        cross_recipe_group: Some(CRG_CHROMIUM) },

    // ===== NICKEL (2 paths) =====
    Recipe { id: 90, name: "Nickel Flash Smelting", category: RecipeCategory::Extraction,
        inputs: &[(S::Pentlandite, 3.0), (S::Oxygen, 0.5)],
        outputs: &[(S::Nickel, 1.0)],
        byproducts: &[(S::Slag, 1.5), (S::Copper, 0.3)],
        min_temp_c: 1350, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: Some(CRG_NICKEL) },
    Recipe { id: 91, name: "Nickel Mond Process (Carbonyl)", category: RecipeCategory::Refining,
        inputs: &[(S::Nickel, 1.01)],
        outputs: &[(S::Nickel, 1.0)],
        byproducts: &[],
        min_temp_c: 50, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0,
        cross_recipe_group: Some(CRG_NICKEL) },

    // ===== MANGANESE =====
    Recipe { id: 95, name: "Ferromanganese Smelting", category: RecipeCategory::Extraction,
        inputs: &[(S::Manganese, 1.5), (S::Carbon, 0.5), (S::Iron, 0.3)],
        outputs: &[(S::Ferromanganese, 1.0)],
        byproducts: &[(S::Slag, 0.8)],
        min_temp_c: 1400, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: None },

    // ===== TUNGSTEN =====
    Recipe { id: 100, name: "Tungsten Hydrogen Reduction", category: RecipeCategory::Extraction,
        inputs: &[(S::Wolframite, 1.3), (S::HydrogenGas, 0.1)],
        outputs: &[(S::Tungsten, 1.0)],
        byproducts: &[(S::Water, 0.3)],
        min_temp_c: 1000, pressure_atm: 1.0, catalyst: None, duration_hours: 8.0,
        cross_recipe_group: None },

    // ===== SILICON =====
    Recipe { id: 105, name: "Silicon Carbothermic Reduction", category: RecipeCategory::Extraction,
        inputs: &[(S::SilicaSand, 2.0), (S::Carbon, 0.8)],
        outputs: &[(S::Silicon, 1.0)],
        byproducts: &[(S::Slag, 0.5)],
        min_temp_c: 1900, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: None },

    // ===== MERCURY =====
    Recipe { id: 110, name: "Mercury Roasting", category: RecipeCategory::Extraction,
        inputs: &[(S::Cinnabar, 1.2)],
        outputs: &[(S::Mercury, 1.0)],
        byproducts: &[],
        min_temp_c: 400, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: None },

    // ===== URANIUM =====
    Recipe { id: 115, name: "Uranium Acid Leaching", category: RecipeCategory::Extraction,
        inputs: &[(S::Uraninite, 10.0), (S::SulfuricAcid, 2.0)],
        outputs: &[(S::Yellowcake, 1.0)],
        byproducts: &[(S::Tailings, 10.0)],
        min_temp_c: 60, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0,
        cross_recipe_group: None },
    Recipe { id: 116, name: "Nuclear Fuel Rod Fabrication", category: RecipeCategory::Manufacturing,
        inputs: &[(S::Yellowcake, 2.0)],
        outputs: &[(S::NuclearFuelRod, 1.0)],
        byproducts: &[],
        min_temp_c: 1700, pressure_atm: 1.0, catalyst: None, duration_hours: 48.0,
        cross_recipe_group: None },

    // ===== LITHIUM (2 paths) =====
    Recipe { id: 120, name: "Lithium from Spodumene", category: RecipeCategory::Extraction,
        inputs: &[(S::Spodumene, 8.0), (S::SulfuricAcid, 1.0)],
        outputs: &[(S::Lithium, 1.0)],
        byproducts: &[(S::Tailings, 7.0)],
        min_temp_c: 1100, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0,
        cross_recipe_group: Some(CRG_LITHIUM) },
    Recipe { id: 121, name: "Lithium from Brine (Solar)", category: RecipeCategory::Extraction,
        inputs: &[(S::Water, 1000.0), (S::SodaAsh, 0.5)],
        outputs: &[(S::Lithium, 1.0)],
        byproducts: &[(S::Salt, 50.0)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 8760.0,
        cross_recipe_group: Some(CRG_LITHIUM) },

    // ===== MAGNESIUM (2 paths) =====
    Recipe { id: 125, name: "Magnesium Pidgeon Process", category: RecipeCategory::Extraction,
        inputs: &[(S::Limestone, 2.5), (S::Silicon, 0.3)],
        outputs: &[(S::Magnesium, 1.0)],
        byproducts: &[(S::Slag, 1.5)],
        min_temp_c: 1200, pressure_atm: 0.001, catalyst: None, duration_hours: 8.0,
        cross_recipe_group: Some(CRG_MAGNESIUM) },
    Recipe { id: 126, name: "Magnesium Electrolytic (Dow)", category: RecipeCategory::Extraction,
        inputs: &[(S::Water, 100.0), (S::HydrochloricAcid, 2.0)],
        outputs: &[(S::Magnesium, 1.0)],
        byproducts: &[(S::ChlorineGas, 1.5)],
        min_temp_c: 720, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0,
        cross_recipe_group: Some(CRG_MAGNESIUM) },

    // ===== COBALT =====
    Recipe { id: 130, name: "Cobalt from Copper-Cobalt Ore", category: RecipeCategory::Extraction,
        inputs: &[(S::CobaltiteOre, 5.0), (S::SulfuricAcid, 1.0)],
        outputs: &[(S::Cobalt, 1.0)],
        byproducts: &[(S::Copper, 0.5), (S::Tailings, 3.5)],
        min_temp_c: 60, pressure_atm: 1.0, catalyst: None, duration_hours: 48.0,
        cross_recipe_group: None },

    // ===== MOLYBDENUM =====
    Recipe { id: 135, name: "Molybdenum Roasting + Reduction", category: RecipeCategory::Extraction,
        inputs: &[(S::MolybdeniteOre, 1.7), (S::HydrogenGas, 0.1)],
        outputs: &[(S::Molybdenum, 1.0)],
        byproducts: &[(S::SulfuricAcid, 0.5)],
        min_temp_c: 650, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0,
        cross_recipe_group: None },

    // ===== RARE EARTHS =====
    Recipe { id: 140, name: "Rare Earth Extraction (Monazite)", category: RecipeCategory::Extraction,
        inputs: &[(S::Monazite, 5.0), (S::SodiumHydroxide, 1.0)],
        outputs: &[(S::Calcium, 1.0)], // placeholder for REE mix
        byproducts: &[(S::Tailings, 3.5), (S::PhosphoricAcid, 0.5)],
        min_temp_c: 150, pressure_atm: 3.0, catalyst: None, duration_hours: 8.0,
        cross_recipe_group: None },

    // ===== PHOSPHORUS =====
    Recipe { id: 145, name: "Phosphorus Electric Furnace", category: RecipeCategory::Extraction,
        inputs: &[(S::PhosphateRock, 5.0), (S::SilicaSand, 1.0), (S::Coke, 1.0)],
        outputs: &[(S::Phosphorus, 1.0)],
        byproducts: &[(S::Slag, 4.0)],
        min_temp_c: 1500, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: None },

    // ===== SULFUR from pyrite =====
    Recipe { id: 150, name: "Sulfur from Pyrite Roasting", category: RecipeCategory::Extraction,
        inputs: &[(S::PyriteOre, 2.0)],
        outputs: &[(S::Sulfur, 0.5)],
        byproducts: &[(S::Hematite, 1.0)],
        min_temp_c: 700, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0,
        cross_recipe_group: None },
];
