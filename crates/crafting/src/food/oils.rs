#![allow(dead_code)]
use crate::recipes::substance::Substance as S;
use crate::recipes::types::*;

const CRG_COCONUT_OIL: u32 = 520;
const CRG_SUNFLOWER_OIL: u32 = 521;
const CRG_SOYBEAN_OIL: u32 = 522;
const CRG_RAPESEED_OIL: u32 = 523;

pub static OIL_RECIPES: &[Recipe] = &[
    // ===================================================================
    // OIL EXTRACTION PROCESSES
    // Three main methods: cold press (<50C), expeller press (60-99C), solvent extraction (hexane)
    // ===================================================================

    // --- Coconut Oil ---
    // Cold press: meat dried to copra, pressed at <50C
    // Expeller: higher temp, higher yield
    // Solvent: hexane extraction from copra
    Recipe {
        id: 1500,
        name: "Coconut Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 5.0)],
        outputs: &[(S::CoconutOil, 1.5)],
        byproducts: &[(S::StrawFiber, 3.0)],
        min_temp_c: 27,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.0,
        cross_recipe_group: Some(CRG_COCONUT_OIL),
    },
    Recipe {
        id: 1501,
        name: "Coconut Oil (Expeller Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 5.0)],
        outputs: &[(S::CoconutOil, 1.8)],
        byproducts: &[(S::StrawFiber, 2.7)],
        min_temp_c: 80,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 1.0,
        cross_recipe_group: Some(CRG_COCONUT_OIL),
    },
    // --- Palm Oil ---
    // Sterilization of fruit bunches at 120-140C, pressing, clarification
    // Temperature: 80-90C press, Duration: 1-2 hours per batch
    Recipe {
        id: 1502,
        name: "Palm Oil (Press Extraction)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 5.0), (S::Water, 1.0)],
        outputs: &[(S::PalmOil, 1.0)],
        byproducts: &[(S::StrawFiber, 3.5), (S::Water, 1.0)],
        min_temp_c: 85,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.0,
        cross_recipe_group: None,
    },
    // --- Sunflower Oil ---
    // Cold press: hulled seeds pressed at <50C, 25-30% yield
    // Solvent: hexane extraction, 40-45% yield
    Recipe {
        id: 1503,
        name: "Sunflower Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 4.0)],
        outputs: &[(S::SunflowerOil, 1.0)],
        byproducts: &[(S::StrawFiber, 2.5)],
        min_temp_c: 27,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.0,
        cross_recipe_group: Some(CRG_SUNFLOWER_OIL),
    },
    Recipe {
        id: 1504,
        name: "Sunflower Oil (Solvent Extraction)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 3.0), (S::Benzene, 0.5)],
        outputs: &[(S::SunflowerOil, 1.2)],
        byproducts: &[(S::StrawFiber, 1.5)],
        min_temp_c: 65,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: Some(CRG_SUNFLOWER_OIL),
    },
    // --- Rapeseed / Canola Oil ---
    // Cold press: seeds pressed at <50C
    // Solvent: hexane extraction at 55-70C
    Recipe {
        id: 1505,
        name: "Rapeseed Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 3.5)],
        outputs: &[(S::RapeseedOil, 1.0)],
        byproducts: &[(S::StrawFiber, 2.0)],
        min_temp_c: 40,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.0,
        cross_recipe_group: Some(CRG_RAPESEED_OIL),
    },
    Recipe {
        id: 1506,
        name: "Rapeseed Oil (Solvent Extraction)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 3.0), (S::Benzene, 0.5)],
        outputs: &[(S::RapeseedOil, 1.3)],
        byproducts: &[(S::StrawFiber, 1.5)],
        min_temp_c: 65,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: Some(CRG_RAPESEED_OIL),
    },
    // --- Soybean Oil ---
    // Almost always solvent extracted (soybeans are only 18-20% oil)
    Recipe {
        id: 1507,
        name: "Soybean Oil (Solvent Extraction)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::SoybeanRaw, 5.0), (S::Benzene, 0.5)],
        outputs: &[(S::SoybeanOil, 1.0)],
        byproducts: &[(S::StrawFiber, 3.5)],
        min_temp_c: 60,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: Some(CRG_SOYBEAN_OIL),
    },
    Recipe {
        id: 1508,
        name: "Soybean Oil (Expeller Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::SoybeanRaw, 6.0)],
        outputs: &[(S::SoybeanOil, 1.0)],
        byproducts: &[(S::StrawFiber, 4.5)],
        min_temp_c: 95,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 1.5,
        cross_recipe_group: Some(CRG_SOYBEAN_OIL),
    },
    // --- Peanut Oil ---
    // Cold press or expeller press. Peanuts are ~45-50% oil.
    Recipe {
        id: 1509,
        name: "Peanut Oil (Expeller Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 2.5)],
        outputs: &[(S::PeanutOil, 1.0)],
        byproducts: &[(S::StrawFiber, 1.2)],
        min_temp_c: 80,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 1.5,
        cross_recipe_group: None,
    },
    // --- Sesame Oil ---
    // Traditionally stone-ground and pressed. Seeds are ~50% oil.
    // Cold press: <50C, Duration: 2-3 hours
    Recipe {
        id: 1510,
        name: "Sesame Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 2.5)],
        outputs: &[(S::SesameOil, 1.0)],
        byproducts: &[(S::StrawFiber, 1.2)],
        min_temp_c: 40,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.5,
        cross_recipe_group: None,
    },
    // --- Flaxseed / Linseed Oil ---
    // Cold press: <45C to preserve omega-3s. Seeds are ~40% oil.
    Recipe {
        id: 1511,
        name: "Flaxseed Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 3.0)],
        outputs: &[(S::FlaxseedOil, 1.0)],
        byproducts: &[(S::StrawFiber, 1.7)],
        min_temp_c: 35,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.0,
        cross_recipe_group: None,
    },
    // --- Walnut Oil ---
    // Cold press: <45C. Walnuts are ~60-65% oil.
    Recipe {
        id: 1512,
        name: "Walnut Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 2.0)],
        outputs: &[(S::WalnutOil, 1.0)],
        byproducts: &[(S::StrawFiber, 0.7)],
        min_temp_c: 40,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.0,
        cross_recipe_group: None,
    },
    // --- Avocado Oil ---
    // Cold press from dried avocado pulp. ~25% oil content in pulp.
    // Temperature: <50C, Duration: 2-3 hours
    Recipe {
        id: 1513,
        name: "Avocado Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 5.0)],
        outputs: &[(S::AvocadoOil, 1.0)],
        byproducts: &[(S::StrawFiber, 3.5)],
        min_temp_c: 45,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.5,
        cross_recipe_group: None,
    },
    // --- Corn Oil ---
    // Extracted from corn germ (byproduct of starch/ethanol production). Solvent or expeller.
    Recipe {
        id: 1514,
        name: "Corn Oil (Solvent Extraction)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::Flour, 5.0), (S::Benzene, 0.5)],
        outputs: &[(S::CornOil, 1.0)],
        byproducts: &[(S::StrawFiber, 3.5)],
        min_temp_c: 60,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: None,
    },
    // --- Cottonseed Oil ---
    // Solvent or expeller press. Seeds are ~18-25% oil. Must be refined to remove gossypol toxin.
    Recipe {
        id: 1515,
        name: "Cottonseed Oil (Solvent Extraction)",
        category: RecipeCategory::OilExtraction,
        inputs: &[
            (S::OilSeed, 5.0),
            (S::Benzene, 0.5),
            (S::SodiumHydroxide, 0.1),
        ],
        outputs: &[(S::CottonseedOil, 1.0)],
        byproducts: &[(S::StrawFiber, 3.5)],
        min_temp_c: 60,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 4.0,
        cross_recipe_group: None,
    },
    // --- Castor Oil ---
    // Cold press from castor beans. Seeds are ~50% oil. Contains ricinoleic acid.
    Recipe {
        id: 1516,
        name: "Castor Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 2.5)],
        outputs: &[(S::CastorOil, 1.0)],
        byproducts: &[(S::StrawFiber, 1.2)],
        min_temp_c: 40,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.0,
        cross_recipe_group: None,
    },
    // --- Jojoba Oil ---
    // Actually a liquid wax ester. Cold press from jojoba seeds (~50% wax).
    Recipe {
        id: 1517,
        name: "Jojoba Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 2.5)],
        outputs: &[(S::JojobaOil, 1.0)],
        byproducts: &[(S::StrawFiber, 1.2)],
        min_temp_c: 40,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.0,
        cross_recipe_group: None,
    },
    // --- Argan Oil ---
    // Traditional: hand-cracked nuts, stone-ground, cold pressed. Very labor-intensive.
    // Seeds are ~50% oil. Temperature: <50C. Duration: 8-10 hours manual
    Recipe {
        id: 1518,
        name: "Argan Oil (Cold Press)",
        category: RecipeCategory::OilExtraction,
        inputs: &[(S::OilSeed, 3.0)],
        outputs: &[(S::ArganOil, 1.0)],
        byproducts: &[(S::StrawFiber, 1.5)],
        min_temp_c: 40,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 8.0,
        cross_recipe_group: None,
    },
];
