#![allow(dead_code)]
use super::types::*;
use super::substance::Substance as S;

const CRG_HARD_CHEESE: u32 = 510;
const CRG_SOFT_CHEESE: u32 = 511;
const CRG_FRESH_CHEESE: u32 = 512;
const CRG_BLUE_CHEESE: u32 = 513;

pub static CHEESE_RECIPES: &[Recipe] = &[
    // ===================================================================
    // CHEESE VARIETIES
    // Each cheese uses specific cultures, temperatures, and aging.
    // Milk input is generic; real recipes use cow, sheep, goat, or buffalo milk.
    // ===================================================================

    // --- Cheddar ---
    // Culture: Mesophilic (Lactococcus lactis subsp. lactis & cremoris)
    // Rennet: yes. Cheddaring process (cutting, stacking curd slabs).
    // Cook temp: 38-39C, Aging: 3 months (mild) to 5+ years (extra sharp), 10-13C
    Recipe { id: 1400, name: "Cheddar Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 10.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.001), (S::Salt, 0.05)],
        outputs: &[(S::CheddarCheese, 1.0)],
        byproducts: &[(S::Whey, 8.5)],
        min_temp_c: 38, pressure_atm: 1.0, catalyst: None, duration_hours: 4320.0,
        cross_recipe_group: Some(CRG_HARD_CHEESE) },

    // --- Gouda ---
    // Culture: Mesophilic (Lactococcus lactis) + sometimes Leuconostoc for eyes
    // Rennet: yes. Washed curd process (replace whey with warm water to reduce acidity).
    // Cook temp: 35-38C, Aging: 4 weeks (young) to 18+ months (aged), 10-14C
    Recipe { id: 1401, name: "Gouda Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 10.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.001), (S::Salt, 0.04)],
        outputs: &[(S::GoudaCheese, 1.0)],
        byproducts: &[(S::Whey, 8.5)],
        min_temp_c: 36, pressure_atm: 1.0, catalyst: None, duration_hours: 2880.0,
        cross_recipe_group: Some(CRG_HARD_CHEESE) },

    // --- Brie ---
    // Culture: Mesophilic (Lactococcus) + Penicillium camemberti (white mold rind)
    // Rennet: yes. Mold-ripened, ripens from outside in.
    // Cook temp: 30-37C, Aging: 4-8 weeks at 7-10C
    Recipe { id: 1402, name: "Brie Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 10.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.001), (S::Salt, 0.03)],
        outputs: &[(S::BrieCheese, 1.2)],
        byproducts: &[(S::Whey, 8.3)],
        min_temp_c: 32, pressure_atm: 1.0, catalyst: None, duration_hours: 1344.0,
        cross_recipe_group: Some(CRG_SOFT_CHEESE) },

    // --- Camembert ---
    // Culture: Mesophilic + Penicillium camemberti (thicker rind than Brie)
    // Rennet: yes. Ladled curd, not cut.
    // Cook temp: 30-33C, Aging: 3-5 weeks at 10-12C
    Recipe { id: 1403, name: "Camembert Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 10.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.001), (S::Salt, 0.03)],
        outputs: &[(S::CamembertCheese, 1.2)],
        byproducts: &[(S::Whey, 8.3)],
        min_temp_c: 31, pressure_atm: 1.0, catalyst: None, duration_hours: 840.0,
        cross_recipe_group: Some(CRG_SOFT_CHEESE) },

    // --- Parmesan (Parmigiano-Reggiano) ---
    // Culture: Thermophilic (Lactobacillus helveticus, S. thermophilus)
    // Rennet: yes. Part-skim milk, large wheels, long aging.
    // Cook temp: 55C, Aging: 12-36 months at 15-18C
    Recipe { id: 1404, name: "Parmesan Cheese (Parmigiano-Reggiano)", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 16.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.002), (S::Salt, 0.1)],
        outputs: &[(S::ParmesanCheese, 1.0)],
        byproducts: &[(S::Whey, 14.5)],
        min_temp_c: 55, pressure_atm: 1.0, catalyst: None, duration_hours: 17520.0,
        cross_recipe_group: Some(CRG_HARD_CHEESE) },

    // --- Mozzarella ---
    // Culture: Thermophilic (S. thermophilus)
    // Rennet: yes. Pasta filata (stretched curd) process in hot water (80-85C).
    // Cook temp: 35-40C, Fresh (no aging) or low-moisture (aged weeks)
    Recipe { id: 1405, name: "Mozzarella Cheese (Fresh)", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 8.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.001), (S::Salt, 0.03)],
        outputs: &[(S::MozzarellaCheese, 1.0)],
        byproducts: &[(S::Whey, 6.5)],
        min_temp_c: 38, pressure_atm: 1.0, catalyst: None, duration_hours: 5.0,
        cross_recipe_group: Some(CRG_FRESH_CHEESE) },

    // --- Ricotta ---
    // Culture: none (acid-set from whey). Uses heat + acid (citric or vinegar).
    // No rennet. Made from whey (byproduct of other cheeses).
    // Temperature: 85-90C, Duration: 1-2 hours total
    Recipe { id: 1406, name: "Ricotta Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Whey, 10.0), (S::Vinegar, 0.05)],
        outputs: &[(S::RicottaCheese, 0.5)],
        byproducts: &[(S::Whey, 9.3)],
        min_temp_c: 88, pressure_atm: 1.0, catalyst: None, duration_hours: 1.5,
        cross_recipe_group: Some(CRG_FRESH_CHEESE) },

    // --- Feta ---
    // Culture: Mesophilic (Lactococcus lactis) or mixed meso/thermo
    // Rennet: yes. Brined cheese (traditionally sheep/goat milk).
    // Cook temp: 32-35C, Brine aging: 2+ months in 7% salt brine
    Recipe { id: 1407, name: "Feta Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 8.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.001), (S::Salt, 0.3)],
        outputs: &[(S::FetaCheese, 1.0)],
        byproducts: &[(S::Whey, 6.5)],
        min_temp_c: 33, pressure_atm: 1.0, catalyst: None, duration_hours: 1440.0,
        cross_recipe_group: None },

    // --- Swiss / Emmental ---
    // Culture: Thermophilic (S. thermophilus, L. helveticus) + Propionibacterium freudenreichii (holes)
    // Rennet: yes. The propionibacteria produce CO2 during warm aging, creating the eyes.
    // Cook temp: 50-54C, Aging: 3-12 months, warm room (20-24C) for 2-6 weeks then cool
    Recipe { id: 1408, name: "Swiss / Emmental Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 12.0), (S::StarterCulture, 0.015), (S::RennetEnzyme, 0.002), (S::Salt, 0.04)],
        outputs: &[(S::SwissCheese, 1.0)],
        byproducts: &[(S::Whey, 10.5), (S::CarbonDioxide, 0.01)],
        min_temp_c: 52, pressure_atm: 1.0, catalyst: None, duration_hours: 4320.0,
        cross_recipe_group: Some(CRG_HARD_CHEESE) },

    // --- Blue Cheese (generic) ---
    // Culture: Mesophilic + Penicillium roqueforti (blue-green veins)
    // Rennet: yes. Needled to allow air for mold growth.
    // Cook temp: 30-32C, Aging: 2-6 months at 10-12C, 90-95% humidity
    Recipe { id: 1409, name: "Blue Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 10.0), (S::StarterCulture, 0.015), (S::RennetEnzyme, 0.001), (S::Salt, 0.08)],
        outputs: &[(S::BlueCheese, 1.2)],
        byproducts: &[(S::Whey, 8.3)],
        min_temp_c: 31, pressure_atm: 1.0, catalyst: None, duration_hours: 4320.0,
        cross_recipe_group: Some(CRG_BLUE_CHEESE) },

    // --- Gruyere ---
    // Culture: Thermophilic (S. thermophilus, L. helveticus) + Propionibacterium
    // Rennet: yes. Washed rind, pressed hard cheese.
    // Cook temp: 54-57C, Aging: 5-18 months at 13-17C
    Recipe { id: 1410, name: "Gruyere Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 12.0), (S::StarterCulture, 0.015), (S::RennetEnzyme, 0.002), (S::Salt, 0.05)],
        outputs: &[(S::GruyereCheese, 1.0)],
        byproducts: &[(S::Whey, 10.5)],
        min_temp_c: 55, pressure_atm: 1.0, catalyst: None, duration_hours: 8760.0,
        cross_recipe_group: Some(CRG_HARD_CHEESE) },

    // --- Manchego ---
    // Culture: Mesophilic (Lactococcus), sheep milk only
    // Rennet: yes. Pressed, rubbed with olive oil.
    // Cook temp: 30-37C, Aging: 2 months (fresco) to 2 years (viejo)
    Recipe { id: 1411, name: "Manchego Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 10.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.002), (S::Salt, 0.05)],
        outputs: &[(S::ManchegoCheese, 1.0)],
        byproducts: &[(S::Whey, 8.5)],
        min_temp_c: 34, pressure_atm: 1.0, catalyst: None, duration_hours: 4320.0,
        cross_recipe_group: Some(CRG_HARD_CHEESE) },

    // --- Roquefort ---
    // Culture: Mesophilic + Penicillium roqueforti (sheep milk, cave-aged)
    // Rennet: yes. Needled, aged in Combalou caves.
    // Cook temp: 28-32C, Aging: 3-5 months at 7-8C in caves, 95% humidity
    Recipe { id: 1412, name: "Roquefort Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 10.0), (S::StarterCulture, 0.015), (S::RennetEnzyme, 0.001), (S::Salt, 0.08)],
        outputs: &[(S::RoquefortCheese, 1.2)],
        byproducts: &[(S::Whey, 8.3)],
        min_temp_c: 30, pressure_atm: 1.0, catalyst: None, duration_hours: 3600.0,
        cross_recipe_group: Some(CRG_BLUE_CHEESE) },

    // --- Stilton ---
    // Culture: Mesophilic (Lactococcus lactis) + Penicillium roqueforti
    // Rennet: yes. English blue, cylindrical, needled.
    // Cook temp: 30-33C, Aging: 9-12 weeks at 10-12C
    Recipe { id: 1413, name: "Stilton Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 10.0), (S::StarterCulture, 0.015), (S::RennetEnzyme, 0.001), (S::Salt, 0.06)],
        outputs: &[(S::StiltonCheese, 1.2)],
        byproducts: &[(S::Whey, 8.3)],
        min_temp_c: 31, pressure_atm: 1.0, catalyst: None, duration_hours: 2016.0,
        cross_recipe_group: Some(CRG_BLUE_CHEESE) },

    // --- Halloumi ---
    // Culture: none or minimal mesophilic. Heat-set cheese.
    // Rennet: yes. Cooked in its own whey (makes it squeaky and heat-resistant).
    // Cook temp: 38-40C curd, then 90C whey cook, no aging (fresh or brined)
    Recipe { id: 1414, name: "Halloumi Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 8.0), (S::RennetEnzyme, 0.002), (S::Salt, 0.04)],
        outputs: &[(S::HalloumiCheese, 1.2)],
        byproducts: &[(S::Whey, 6.5)],
        min_temp_c: 40, pressure_atm: 1.0, catalyst: None, duration_hours: 3.0,
        cross_recipe_group: Some(CRG_FRESH_CHEESE) },

    // --- Paneer ---
    // Culture: none. Acid-set (lemon juice or vinegar).
    // No rennet, no aging. Indian fresh cheese.
    // Temperature: heat milk to 85-90C, add acid, press
    Recipe { id: 1415, name: "Paneer", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 4.0), (S::Vinegar, 0.05)],
        outputs: &[(S::Paneer, 0.5)],
        byproducts: &[(S::Whey, 3.3)],
        min_temp_c: 88, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0,
        cross_recipe_group: Some(CRG_FRESH_CHEESE) },

    // --- Cottage Cheese ---
    // Culture: Mesophilic (Lactococcus lactis)
    // Rennet: small amount. Cut large curds, washed with cold water.
    // Cook temp: 32-33C, no aging (fresh)
    Recipe { id: 1416, name: "Cottage Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 4.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.0005), (S::Salt, 0.02)],
        outputs: &[(S::CottageCheese, 0.8)],
        byproducts: &[(S::Whey, 3.0)],
        min_temp_c: 32, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: Some(CRG_FRESH_CHEESE) },

    // --- Cream Cheese ---
    // Culture: Mesophilic (Lactococcus lactis, Leuconostoc)
    // Rennet: tiny amount. Cream + milk, lightly cultured.
    // Cook temp: 30-33C, Duration: 12-18 hours fermentation + draining
    Recipe { id: 1417, name: "Cream Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 2.0), (S::Cream, 2.0), (S::StarterCulture, 0.01), (S::Salt, 0.02)],
        outputs: &[(S::CreamCheese, 1.5)],
        byproducts: &[(S::Whey, 2.3)],
        min_temp_c: 31, pressure_atm: 1.0, catalyst: None, duration_hours: 16.0,
        cross_recipe_group: Some(CRG_FRESH_CHEESE) },

    // --- Mascarpone ---
    // Culture: none. Acid-set from cream (citric acid or tartaric acid).
    // No rennet. Italian fresh cream cheese.
    // Temperature: heat cream to 85-90C, add acid, cool and drain
    Recipe { id: 1418, name: "Mascarpone Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Cream, 2.0), (S::CitricAcid, 0.01)],
        outputs: &[(S::MascarponeCheese, 1.5)],
        byproducts: &[(S::Whey, 0.4)],
        min_temp_c: 88, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0,
        cross_recipe_group: Some(CRG_FRESH_CHEESE) },

    // --- Provolone ---
    // Culture: Thermophilic (S. thermophilus, L. bulgaricus)
    // Rennet: yes. Pasta filata (stretched curd like mozzarella), then aged.
    // Cook temp: 46-49C, Aging: 2-12 months, sometimes smoked
    Recipe { id: 1419, name: "Provolone Cheese", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 10.0), (S::StarterCulture, 0.01), (S::RennetEnzyme, 0.002), (S::Salt, 0.05)],
        outputs: &[(S::ProvoloneCheese, 1.0)],
        byproducts: &[(S::Whey, 8.5)],
        min_temp_c: 47, pressure_atm: 1.0, catalyst: None, duration_hours: 4320.0,
        cross_recipe_group: Some(CRG_HARD_CHEESE) },
];
