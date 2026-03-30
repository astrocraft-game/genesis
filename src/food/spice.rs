#![allow(dead_code)]
use crate::recipes::types::*;
use crate::recipes::substance::Substance as S;

const CRG_PEPPER: u32 = 530;
const CRG_TEA: u32 = 531;
const CRG_SUGAR: u32 = 532;
const CRG_COFFEE: u32 = 533;

pub static SPICE_RECIPES: &[Recipe] = &[
    // ===================================================================
    // SPICE AND FLAVOR PROCESSING
    // ===================================================================

    // --- Vanilla Curing ---
    // Steps: blanching (60-65C, 3 min) -> sweating (40C, 24-72h) -> drying (35C, 4-6 weeks)
    //   -> conditioning (ambient, 2-3 months)
    // Enzymes: beta-glucosidase converts glucovanillin -> vanillin during sweating
    // Inputs: green vanilla pods. Outputs: cured vanilla beans (20-35% moisture)
    Recipe { id: 1600, name: "Vanilla Bean Curing", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::VanillaGreenPod, 5.0)],
        outputs: &[(S::VanillaBean, 1.0)],
        byproducts: &[(S::Water, 3.5)],
        min_temp_c: 40, pressure_atm: 1.0, catalyst: None, duration_hours: 3360.0,
        cross_recipe_group: None },

    // --- Cinnamon Bark Processing ---
    // Steps: harvest inner bark -> roll into quills -> dry in sun (30-40C) for 4-5 days
    // Temperature: 30-40C drying. Duration: 3-5 days drying + 2-3 days fermentation/sweating
    Recipe { id: 1601, name: "Cinnamon Bark Processing", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::CinnamonBark, 3.0)],
        outputs: &[(S::Cinnamon, 1.0)],
        byproducts: &[(S::Water, 1.5)],
        min_temp_c: 35, pressure_atm: 1.0, catalyst: None, duration_hours: 120.0,
        cross_recipe_group: None },

    // --- Black Pepper ---
    // Harvest green unripe drupes -> blanch in hot water briefly -> sun dry 3-5 days
    // Enzymes cause browning (polyphenol oxidase). Dried to 10-12% moisture.
    // Temperature: 50-65C drying, Duration: 3-5 days
    Recipe { id: 1602, name: "Black Pepper Processing", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::PepperDrupe, 3.0), (S::Water, 1.0)],
        outputs: &[(S::BlackPepper, 1.0)],
        byproducts: &[(S::Water, 2.5)],
        min_temp_c: 55, pressure_atm: 1.0, catalyst: None, duration_hours: 96.0,
        cross_recipe_group: Some(CRG_PEPPER) },

    // --- White Pepper ---
    // Harvest ripe red drupes -> soak in water 7-14 days (retting/fermentation removes skin)
    //   -> dry. Milder flavor, no skin.
    // Organism: bacterial fermentation removes outer pericarp
    // Temperature: 50-55C drying, Duration: 7-14 days soaking + drying
    Recipe { id: 1603, name: "White Pepper Processing", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::PepperDrupe, 3.0), (S::Water, 5.0)],
        outputs: &[(S::WhitePepper, 1.0)],
        byproducts: &[(S::Water, 6.0)],
        min_temp_c: 52, pressure_atm: 1.0, catalyst: None, duration_hours: 264.0,
        cross_recipe_group: Some(CRG_PEPPER) },

    // --- Green Pepper ---
    // Harvest unripe drupes -> quick dry or brine/freeze to preserve green color.
    // Drying: quick hot-air dry at 70-100C or freeze-dry
    // Duration: hours (fast drying) to preserve chlorophyll
    Recipe { id: 1604, name: "Green Pepper Processing (Dried)", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::PepperDrupe, 3.0)],
        outputs: &[(S::GreenPepper, 1.0)],
        byproducts: &[(S::Water, 1.5)],
        min_temp_c: 80, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: Some(CRG_PEPPER) },

    // --- Cacao/Chocolate Processing ---
    // Step 1: Fermentation. Organisms: Acetobacter, Lactobacillus, Saccharomyces cerevisiae
    //   Pulp ferments (acetic/lactic acid), kills embryo, develops flavor precursors.
    //   Temperature: 45-50C peak, Duration: 5-7 days (Forastero), 1-3 days (Criollo)
    Recipe { id: 1610, name: "Cacao Fermentation", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::CacaoFruit, 6.0)],
        outputs: &[(S::CacaoBean, 1.0)],
        byproducts: &[(S::Water, 3.0), (S::AceticAcid, 0.2), (S::Ethanol, 0.1)],
        min_temp_c: 45, pressure_atm: 1.0, catalyst: None, duration_hours: 144.0,
        cross_recipe_group: None },

    // Step 2: Roasting. Maillard reaction + Strecker degradation.
    //   Temperature: 120-160C (250-320F), Duration: 20-90 minutes
    //   Develops hundreds of flavor compounds from fermentation precursors.
    Recipe { id: 1611, name: "Cacao Roasting", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::CacaoBean, 1.0)],
        outputs: &[(S::RoastedCacao, 0.85)],
        byproducts: &[(S::Water, 0.1), (S::CarbonDioxide, 0.03)],
        min_temp_c: 140, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0,
        cross_recipe_group: None },

    // Step 3: Conching. Grinding + aeration + heat. Reduces particle size, develops texture.
    //   Temperature: 49C (milk chocolate) to 82C (dark chocolate)
    //   Duration: 4-72 hours
    Recipe { id: 1612, name: "Chocolate Conching", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::RoastedCacao, 0.7), (S::Sugar, 0.3), (S::Butter, 0.05)],
        outputs: &[(S::Chocolate, 1.0)],
        byproducts: &[(S::Water, 0.02)],
        min_temp_c: 65, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0,
        cross_recipe_group: None },

    // --- Coffee Processing ---
    // Step 1a: Wet processing. Depulp, ferment in water tanks 12-36 hours to remove mucilage.
    //   Organisms: wild Lactobacillus, Leuconostoc, Enterococcus
    //   Temperature: 15-25C, Duration: 12-36 hours
    Recipe { id: 1620, name: "Coffee Wet Processing", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::CoffeeCherries, 5.0), (S::Water, 5.0)],
        outputs: &[(S::GreenCoffeeBeans, 1.0)],
        byproducts: &[(S::Water, 7.0), (S::StrawFiber, 1.5)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 120.0,
        cross_recipe_group: Some(CRG_COFFEE) },

    // Step 1b: Dry processing (natural). Whole cherries sun-dried 3-4 weeks to 11-12% moisture.
    //   Temperature: 30-45C (sun/mechanical dryers), Duration: 2-4 weeks
    Recipe { id: 1621, name: "Coffee Dry Processing (Natural)", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::CoffeeCherries, 5.0)],
        outputs: &[(S::GreenCoffeeBeans, 1.0)],
        byproducts: &[(S::StrawFiber, 2.0), (S::Water, 1.5)],
        min_temp_c: 35, pressure_atm: 1.0, catalyst: None, duration_hours: 504.0,
        cross_recipe_group: Some(CRG_COFFEE) },

    // Step 2: Coffee roasting. Maillard reaction, caramelization, first crack, second crack.
    //   Temperature: 188-282C (370-540F), Duration: 12-30 minutes
    //   Light roast ~205C, Medium ~220C, Dark ~240C
    Recipe { id: 1622, name: "Coffee Roasting", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::GreenCoffeeBeans, 1.0)],
        outputs: &[(S::RoastedCoffee, 0.8)],
        byproducts: &[(S::Water, 0.1), (S::CarbonDioxide, 0.05)],
        min_temp_c: 220, pressure_atm: 1.0, catalyst: None, duration_hours: 0.4,
        cross_recipe_group: None },

    // --- Tea Processing ---
    // Green Tea: withering (optional) -> kill-green (pan-fire/steam 80-100C) -> roll -> dry
    //   No oxidation (<5%). Temperature: 80-100C kill-green, 90-100C drying
    Recipe { id: 1630, name: "Green Tea Processing", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::TeaLeaves, 4.0)],
        outputs: &[(S::GreenTea, 1.0)],
        byproducts: &[(S::Water, 2.5)],
        min_temp_c: 90, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: Some(CRG_TEA) },

    // Black Tea: wither (8-18h, 25-30C) -> roll (45-90 min) -> oxidize (3h, 25-30C, 90-95% RH)
    //   -> dry (70-100C). Oxidation: 80-95%. Polyphenol oxidase converts catechins to theaflavins.
    Recipe { id: 1631, name: "Black Tea Processing", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::TeaLeaves, 4.5)],
        outputs: &[(S::BlackTea, 1.0)],
        byproducts: &[(S::Water, 3.0)],
        min_temp_c: 28, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0,
        cross_recipe_group: Some(CRG_TEA) },

    // Oolong Tea: wither -> bruise/tumble -> partial oxidize (15-70%) -> kill-green -> roll -> dry
    //   Oxidation is stopped partway through. Duration: 2-3 days total.
    Recipe { id: 1632, name: "Oolong Tea Processing", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::TeaLeaves, 4.0)],
        outputs: &[(S::OolongTea, 1.0)],
        byproducts: &[(S::Water, 2.5)],
        min_temp_c: 28, pressure_atm: 1.0, catalyst: None, duration_hours: 48.0,
        cross_recipe_group: Some(CRG_TEA) },

    // --- Sugar Processing ---
    // Sugarcane: crush -> juice extraction -> clarify (lime) -> evaporate -> crystallize -> centrifuge
    //   Temperature: 65-70C clarification, 100-120C evaporation, 55-75C crystallization
    //   Duration: 8-12 hours total
    Recipe { id: 1640, name: "Sugar Refining (Cane)", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::SugarcaneJuice, 10.0), (S::SlakedLite, 0.1)],
        outputs: &[(S::RefinedSugar, 1.0)],
        byproducts: &[(S::Water, 7.0), (S::StrawFiber, 1.5)],
        min_temp_c: 110, pressure_atm: 1.0, catalyst: None, duration_hours: 10.0,
        cross_recipe_group: Some(CRG_SUGAR) },

    // Sugar beet: wash -> slice into cossettes -> diffuse in hot water (70-75C) -> purify (lime + CO2)
    //   -> evaporate -> crystallize. Duration: 8-14 hours.
    Recipe { id: 1641, name: "Sugar Refining (Beet)", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::SugarBeet, 8.0), (S::Water, 5.0), (S::SlakedLite, 0.1)],
        outputs: &[(S::RefinedSugar, 1.0)],
        byproducts: &[(S::Water, 8.0), (S::StrawFiber, 3.5)],
        min_temp_c: 75, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0,
        cross_recipe_group: Some(CRG_SUGAR) },

    // --- Maple Syrup ---
    // Sap collection (2-3% sugar) -> evaporation to 66-67 Brix
    //   40L sap -> 1L syrup. Temperature: 104C (7.4F above boiling). Duration: hours.
    Recipe { id: 1642, name: "Maple Syrup Production", category: RecipeCategory::SpiceProcessing,
        inputs: &[(S::MapleSap, 40.0)],
        outputs: &[(S::MapleSyrup, 1.0)],
        byproducts: &[(S::Steam, 38.0)],
        min_temp_c: 104, pressure_atm: 1.0, catalyst: None, duration_hours: 8.0,
        cross_recipe_group: None },
];
