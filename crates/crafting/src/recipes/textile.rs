#![allow(dead_code)]
use super::substance::Substance as S;
use super::types::*;

const CRG_COTTON: u32 = 540;
const CRG_LINEN: u32 = 541;
const CRG_PAPER: u32 = 302;

pub static TEXTILE_RECIPES: &[Recipe] = &[
    // ===================================================================
    // NATURAL TEXTILE FIBERS
    // ===================================================================

    // --- Cotton (ginning + carding + spinning) ---
    // Steps: harvest bolls -> gin (separate seed from lint) -> card (align fibers)
    //   -> draw (thin slivers) -> spin (twist into yarn) -> weave
    // Temperature: ambient, Duration: days total processing
    Recipe {
        id: 1700,
        name: "Cotton Ginning and Spinning",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::RawCottonBoll, 3.0)],
        outputs: &[(S::CottonFiber, 1.0)],
        byproducts: &[(S::OilSeed, 1.5), (S::StrawFiber, 0.3)],
        min_temp_c: 20,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 8.0,
        cross_recipe_group: Some(CRG_COTTON),
    },
    // --- Wool Processing ---
    // Steps: shear -> scour (wash, 55-65C with soap/detergent) -> card -> comb -> spin
    //   Scouring removes lanolin (10-25% of raw fleece weight) and dirt.
    // Temperature: 55-65C scouring, Duration: hours per batch
    Recipe {
        id: 1701,
        name: "Wool Scouring and Spinning",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::RawWool, 2.5), (S::Water, 5.0), (S::SoapProduct, 0.1)],
        outputs: &[(S::WoolFiber, 1.0)],
        byproducts: &[(S::Lanolin, 0.3), (S::Water, 5.0)],
        min_temp_c: 60,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 12.0,
        cross_recipe_group: None,
    },
    // --- Silk Production ---
    // Steps: silkworm rearing -> stifling cocoons (steam/heat) -> cooking/degumming (95-100C)
    //   -> reeling (combining 2-20 filaments) -> throwing (twisting yarn)
    //   Degumming: hot water removes sericin protein coating from fibroin core.
    // Temperature: 95-100C cooking, Duration: 10-15 min cooking + hours reeling
    // Yield: ~6000 cocoons per kg of raw silk
    Recipe {
        id: 1702,
        name: "Silk Reeling and Degumming",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::SilkCocoon, 6.0), (S::Water, 10.0)],
        outputs: &[(S::SilkFiber, 1.0)],
        byproducts: &[(S::Water, 12.0)],
        min_temp_c: 97,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 12.0,
        cross_recipe_group: None,
    },
    // --- Linen (Flax) Processing ---
    // Steps: harvest flax stalks -> ret (water or dew retting to decompose pectin binding)
    //   -> break (crush woody core) -> scutch (remove shives) -> hackle (comb) -> spin
    //   Water retting: 8-14 days in ponds (bacteria: Clostridium, Bacillus decompose pectin)
    //   Dew retting: 2-5 weeks on fields (fungi: Cladosporium, Epicoccum)
    // Temperature: 25-35C (water retting), Duration: 8-14 days
    Recipe {
        id: 1703,
        name: "Linen Flax Water Retting",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::FlaxStalk, 4.0), (S::Water, 10.0)],
        outputs: &[(S::LinenFiber, 1.0)],
        byproducts: &[(S::Water, 10.0), (S::StrawFiber, 2.5)],
        min_temp_c: 30,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 264.0,
        cross_recipe_group: Some(CRG_LINEN),
    },
    Recipe {
        id: 1704,
        name: "Linen Flax Dew Retting",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::FlaxStalk, 4.0)],
        outputs: &[(S::LinenFiber, 1.0)],
        byproducts: &[(S::StrawFiber, 2.5)],
        min_temp_c: 20,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 600.0,
        cross_recipe_group: Some(CRG_LINEN),
    },
    // --- Hemp Fiber ---
    // Steps: same as flax. Water retting 8-14 days or dew retting 2-5 weeks.
    //   Organisms: Clostridium, Bacillus (water retting)
    // Temperature: 25-35C, Duration: 8-14 days water ret
    Recipe {
        id: 1705,
        name: "Hemp Fiber Water Retting",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::HempStalk, 4.0), (S::Water, 10.0)],
        outputs: &[(S::HempFiber, 1.0)],
        byproducts: &[(S::Water, 10.0), (S::StrawFiber, 2.5)],
        min_temp_c: 30,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 240.0,
        cross_recipe_group: None,
    },
    // --- Jute Fiber ---
    // Steps: harvest -> ret in water 10-30 days -> strip -> wash -> dry
    //   Organisms: Bacillus, Clostridium (pectinolytic bacteria)
    // Temperature: 25-35C, Duration: 10-30 days retting
    Recipe {
        id: 1706,
        name: "Jute Fiber Retting",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::JuteStalk, 5.0), (S::Water, 10.0)],
        outputs: &[(S::JuteFiber, 1.0)],
        byproducts: &[(S::Water, 10.0), (S::StrawFiber, 3.5)],
        min_temp_c: 30,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 480.0,
        cross_recipe_group: None,
    },
    // --- Sisal Fiber ---
    // Steps: harvest leaves -> decorticate (mechanical scraping to remove pulp)
    //   -> wash -> dry (sun or machine at 40-60C)
    // Temperature: ambient (30-40C tropical), Duration: 1-2 days
    Recipe {
        id: 1707,
        name: "Sisal Fiber Decortication",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::StrawFiber, 5.0), (S::Water, 3.0)],
        outputs: &[(S::SisalFiber, 1.0)],
        byproducts: &[(S::Water, 5.0), (S::StrawFiber, 1.5)],
        min_temp_c: 35,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 36.0,
        cross_recipe_group: None,
    },
    // --- Coir Fiber (coconut) ---
    // Steps: soak coconut husks in water 6-12 months (retting) -> beat -> separate -> dry
    //   Organisms: anaerobic bacteria decompose pectin
    // Temperature: ambient (tropical 25-35C), Duration: 6-12 months retting
    Recipe {
        id: 1708,
        name: "Coir Fiber Retting",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::StrawFiber, 5.0), (S::Water, 10.0)],
        outputs: &[(S::CoirFiber, 1.0)],
        byproducts: &[(S::Water, 10.0), (S::StrawFiber, 3.5)],
        min_temp_c: 28,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 4320.0,
        cross_recipe_group: None,
    },
    // --- Ramie Fiber ---
    // Steps: strip bark from stems -> chemical degumming (NaOH 2-5%, 90-100C, 2-4h)
    //   to remove gums, pectin, hemicelluloses -> wash -> dry -> comb
    // Temperature: 95C degumming, Duration: 3-4 hours processing
    Recipe {
        id: 1709,
        name: "Ramie Fiber Degumming",
        category: RecipeCategory::TextileProcessing,
        inputs: &[
            (S::StrawFiber, 4.0),
            (S::SodiumHydroxide, 0.15),
            (S::Water, 5.0),
        ],
        outputs: &[(S::RamieFiber, 1.0)],
        byproducts: &[(S::Water, 6.0)],
        min_temp_c: 95,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 4.0,
        cross_recipe_group: None,
    },
    // --- Bamboo Fiber ---
    // Chemical process (viscose-like): crush bamboo -> soak in NaOH (20C, 1-3h)
    //   -> bleach -> dissolve in CS2 -> extrude. Essentially bamboo rayon.
    // Mechanical process: crush -> enzyme ret -> comb out fibers (more sustainable)
    Recipe {
        id: 1710,
        name: "Bamboo Fiber (Chemical/Viscose)",
        category: RecipeCategory::TextileProcessing,
        inputs: &[
            (S::Cellulose, 2.0),
            (S::SodiumHydroxide, 0.5),
            (S::Water, 5.0),
        ],
        outputs: &[(S::BambooFiber, 1.0)],
        byproducts: &[(S::Water, 5.0)],
        min_temp_c: 20,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 8.0,
        cross_recipe_group: None,
    },
    // --- Kapok Fiber ---
    // Steps: harvest seed pods -> open -> extract silky fibers (hand or machine)
    //   No retting needed - fibers are naturally loose in the pod.
    // Temperature: ambient, Duration: 2-4 hours per batch
    Recipe {
        id: 1711,
        name: "Kapok Fiber Extraction",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::StrawFiber, 3.0)],
        outputs: &[(S::KapokFiber, 1.0)],
        byproducts: &[(S::OilSeed, 1.0), (S::StrawFiber, 0.5)],
        min_temp_c: 25,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: None,
    },
    // ===================================================================
    // SYNTHETIC / REGENERATED TEXTILE FIBERS
    // ===================================================================

    // --- Rayon / Viscose ---
    // Steps: dissolve wood pulp in NaOH -> age -> treat with CS2 (xanthation)
    //   -> dissolve in dilute NaOH -> extrude through spinneret into acid bath
    // Temperature: 20-30C (aging), 30-40C (dissolution), 40-50C (spinning bath)
    // Duration: 2-3 days total
    Recipe {
        id: 1720,
        name: "Rayon / Viscose Fiber",
        category: RecipeCategory::TextileProcessing,
        inputs: &[
            (S::WoodPulpRaw, 1.5),
            (S::SodiumHydroxide, 0.5),
            (S::SulfuricAcid, 0.3),
            (S::Water, 5.0),
        ],
        outputs: &[(S::RayonFiber, 1.0)],
        byproducts: &[(S::Water, 4.5), (S::SulfurDioxide, 0.1)],
        min_temp_c: 35,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 48.0,
        cross_recipe_group: None,
    },
    // --- Modal ---
    // Same viscose process but with beechwood pulp + modified spinning conditions
    //   for higher wet strength and modulus. Spun with higher stretch ratio.
    Recipe {
        id: 1721,
        name: "Modal Fiber",
        category: RecipeCategory::TextileProcessing,
        inputs: &[
            (S::WoodPulpRaw, 1.5),
            (S::SodiumHydroxide, 0.5),
            (S::SulfuricAcid, 0.3),
            (S::Water, 5.0),
        ],
        outputs: &[(S::ModalFiber, 1.0)],
        byproducts: &[(S::Water, 4.5), (S::SulfurDioxide, 0.1)],
        min_temp_c: 35,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 48.0,
        cross_recipe_group: None,
    },
    // --- Lyocell / Tencel ---
    // Steps: dissolve wood pulp in NMMO (N-methylmorpholine N-oxide) solvent
    //   -> extrude through spinneret into water -> coagulate -> wash -> dry
    //   Closed-loop: 99.8% solvent recovery. No CS2 (cleaner than viscose).
    // Temperature: 80-120C (dissolution), Duration: 6-12 hours
    Recipe {
        id: 1722,
        name: "Lyocell / Tencel Fiber",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::WoodPulpRaw, 1.5), (S::Water, 5.0)],
        outputs: &[(S::LyocellFiber, 1.0)],
        byproducts: &[(S::Water, 4.5)],
        min_temp_c: 100,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 10.0,
        cross_recipe_group: None,
    },
    // --- Spandex / Elastane ---
    // Polyurethane-based synthetic. Steps: react diisocyanate + polyol diol -> prepolymer
    //   -> chain extend with diamine -> dissolve in solvent (DMAc) -> dry-spin
    // Temperature: 70-90C reaction, 200-230C spinning, Duration: hours
    Recipe {
        id: 1723,
        name: "Spandex / Elastane Fiber",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::Ethylene, 0.6), (S::Butadiene, 0.4)],
        outputs: &[(S::SpandexFiber, 1.0)],
        byproducts: &[],
        min_temp_c: 200,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 4.0,
        cross_recipe_group: None,
    },
    // --- Polypropylene Fiber ---
    // Melt-spin polypropylene pellets through spinneret, then draw/stretch.
    // Temperature: 220-280C (melt spinning), Duration: 1-2 hours
    Recipe {
        id: 1724,
        name: "Polypropylene Fiber Spinning",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::Polypropylene, 1.0)],
        outputs: &[(S::PolypropyleneFiber, 0.98)],
        byproducts: &[],
        min_temp_c: 250,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 1.5,
        cross_recipe_group: None,
    },
    // --- Acrylic Fiber ---
    // Polymer: polyacrylonitrile (PAN). Wet or dry spinning from acrylonitrile monomer.
    //   Polymerize acrylonitrile -> dissolve in solvent (DMF/DMAc) -> spin -> wash -> stretch -> dry
    // Temperature: 60-70C polymerization, 90-100C drawing, Duration: 4-8 hours
    Recipe {
        id: 1725,
        name: "Acrylic Fiber Spinning",
        category: RecipeCategory::TextileProcessing,
        inputs: &[(S::Acrylonitrile, 1.0), (S::Water, 2.0)],
        outputs: &[(S::AcrylicFiber, 0.95)],
        byproducts: &[(S::Water, 2.0)],
        min_temp_c: 65,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 6.0,
        cross_recipe_group: None,
    },
];
