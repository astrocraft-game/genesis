#![allow(dead_code)]
use super::types::*;
use super::substance::Substance as S;

const CRG_PROPYLENE_OXIDE: u32 = 400;
const CRG_ACETIC_ACID: u32 = 401;

pub static ORGANIC_RECIPES: &[Recipe] = &[
    // ===== HYDROCARBONS =====
    Recipe { id: 800, name: "Steam Cracking (Ethylene)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::CrudeOil, 1.5)], outputs: &[(S::Ethylene, 0.3), (S::Propylene, 0.15)],
        byproducts: &[(S::Butadiene, 0.05), (S::Benzene, 0.05), (S::HydrogenGas, 0.02)],
        min_temp_c: 850, pressure_atm: 2.0, catalyst: None, duration_hours: 0.01, cross_recipe_group: None },
    Recipe { id: 801, name: "Catalytic Reforming (BTX)", category: RecipeCategory::Refining,
        inputs: &[(S::CrudeOil, 1.5)], outputs: &[(S::Benzene, 0.3), (S::Toluene, 0.25), (S::Xylene, 0.2)],
        byproducts: &[(S::HydrogenGas, 0.05)],
        min_temp_c: 500, pressure_atm: 25.0, catalyst: Some(S::Platinum), duration_hours: 0.5, cross_recipe_group: None },
    Recipe { id: 802, name: "Acetylene from Carbide", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::CalciumCarbide, 1.0), (S::Water, 0.6)], outputs: &[(S::Acetylene, 0.4)],
        byproducts: &[(S::SlakedLite, 1.2)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 0.1, cross_recipe_group: None },

    // ===== ALCOHOLS =====
    Recipe { id: 810, name: "Methanol from Syngas", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::SynGas, 1.5)], outputs: &[(S::Methanol, 1.0)], byproducts: &[(S::Water, 0.3)],
        min_temp_c: 250, pressure_atm: 80.0, catalyst: Some(S::Copper), duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 811, name: "Ethanol from Ethylene Hydration", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Ethylene, 0.6), (S::Water, 0.4)], outputs: &[(S::Ethanol, 1.0)], byproducts: &[],
        min_temp_c: 300, pressure_atm: 70.0, catalyst: Some(S::PhosphoricAcid), duration_hours: 1.0, cross_recipe_group: None },
    Recipe { id: 812, name: "Isopropanol from Propylene", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Propylene, 0.7), (S::Water, 0.3)], outputs: &[(S::Isopropanol, 1.0)], byproducts: &[],
        min_temp_c: 250, pressure_atm: 25.0, catalyst: Some(S::SulfuricAcid), duration_hours: 1.0, cross_recipe_group: None },
    Recipe { id: 813, name: "Butanol ABE Fermentation", category: RecipeCategory::FoodBiological,
        inputs: &[(S::Glucose, 2.0), (S::Water, 5.0)], outputs: &[(S::Butanol, 0.6), (S::Acetone, 0.3), (S::Ethanol, 0.1)],
        byproducts: &[(S::CarbonDioxide, 1.0)],
        min_temp_c: 35, pressure_atm: 1.0, catalyst: None, duration_hours: 72.0, cross_recipe_group: None },
    Recipe { id: 814, name: "Ethylene Glycol from Ethylene Oxide", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::EthyleneOxide, 0.7), (S::Water, 0.3)], outputs: &[(S::EthyleneGlycol, 1.0)], byproducts: &[],
        min_temp_c: 200, pressure_atm: 15.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },

    // ===== ALDEHYDES AND KETONES =====
    Recipe { id: 820, name: "Formaldehyde (Methanol Oxidation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Methanol, 0.9), (S::Oxygen, 0.1)], outputs: &[(S::Formaldehyde, 1.0)],
        byproducts: &[(S::Water, 0.2)],
        min_temp_c: 600, pressure_atm: 1.0, catalyst: Some(S::Silver), duration_hours: 0.1, cross_recipe_group: None },
    Recipe { id: 821, name: "Acetaldehyde (Wacker Process)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Ethylene, 0.6), (S::Oxygen, 0.4)], outputs: &[(S::Acetaldehyde, 1.0)],
        byproducts: &[(S::Water, 0.2)],
        min_temp_c: 130, pressure_atm: 4.0, catalyst: Some(S::Copper), duration_hours: 1.0, cross_recipe_group: None },
    Recipe { id: 822, name: "Acetone (Cumene Process)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Benzene, 0.5), (S::Propylene, 0.5)], outputs: &[(S::Acetone, 0.4), (S::Phenol, 0.6)],
        byproducts: &[],
        min_temp_c: 90, pressure_atm: 1.0, catalyst: Some(S::SulfuricAcid), duration_hours: 3.0, cross_recipe_group: None },

    // ===== CARBOXYLIC ACIDS =====
    Recipe { id: 830, name: "Acetic Acid (Monsanto Process)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Methanol, 0.5), (S::SynGas, 0.5)], outputs: &[(S::AceticAcid, 1.0)], byproducts: &[],
        min_temp_c: 180, pressure_atm: 40.0, catalyst: Some(S::Platinum), duration_hours: 1.0, cross_recipe_group: None },
    Recipe { id: 831, name: "Citric Acid Fermentation", category: RecipeCategory::FoodBiological,
        inputs: &[(S::Sugar, 1.5), (S::Water, 3.0)], outputs: &[(S::CitricAcid, 1.0)], byproducts: &[],
        min_temp_c: 30, pressure_atm: 1.0, catalyst: None, duration_hours: 168.0, cross_recipe_group: None },
    Recipe { id: 832, name: "Adipic Acid (Cyclohexane Oxidation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Benzene, 0.8), (S::NitricAcid, 0.5)], outputs: &[(S::AdipicAcid, 1.0)],
        byproducts: &[(S::Water, 0.3)],
        min_temp_c: 160, pressure_atm: 10.0, catalyst: Some(S::Cobalt), duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 833, name: "Terephthalic Acid (PX Oxidation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Xylene, 0.8), (S::Oxygen, 0.3)], outputs: &[(S::TerephthalicAcid, 1.0)],
        byproducts: &[(S::Water, 0.2)],
        min_temp_c: 200, pressure_atm: 20.0, catalyst: Some(S::Cobalt), duration_hours: 4.0, cross_recipe_group: None },

    // ===== ESTERS =====
    Recipe { id: 840, name: "Ethyl Acetate (Fischer Esterification)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Ethanol, 0.5), (S::AceticAcid, 0.5)], outputs: &[(S::EthylAcetate, 0.9)],
        byproducts: &[(S::Water, 0.1)],
        min_temp_c: 80, pressure_atm: 1.0, catalyst: Some(S::SulfuricAcid), duration_hours: 4.0, cross_recipe_group: None },

    // ===== AMINES AND N-COMPOUNDS =====
    Recipe { id: 850, name: "Aniline (Bechamp Reduction)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Benzene, 0.6), (S::NitricAcid, 0.3), (S::Iron, 0.2)],
        outputs: &[(S::Aniline, 1.0)], byproducts: &[(S::Water, 0.3)],
        min_temp_c: 100, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: None },
    Recipe { id: 851, name: "Caprolactam (for Nylon-6)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Benzene, 0.8), (S::Ammonia, 0.2)], outputs: &[(S::Caprolactam, 1.0)],
        byproducts: &[(S::Ammonium, 0.5)],
        min_temp_c: 130, pressure_atm: 1.0, catalyst: Some(S::SulfuricAcid), duration_hours: 6.0, cross_recipe_group: None },
    Recipe { id: 852, name: "Acrylonitrile (Sohio Process)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Propylene, 0.4), (S::Ammonia, 0.15), (S::Oxygen, 0.45)],
        outputs: &[(S::Acrylonitrile, 1.0)], byproducts: &[(S::Water, 0.3)],
        min_temp_c: 450, pressure_atm: 2.0, catalyst: Some(S::Molybdenum), duration_hours: 0.1, cross_recipe_group: None },
    Recipe { id: 853, name: "Hydrogen Cyanide (Andrussow)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Methane, 0.2), (S::Ammonia, 0.2), (S::Oxygen, 0.6)],
        outputs: &[(S::HydrogenCyanide, 1.0)], byproducts: &[(S::Water, 0.5)],
        min_temp_c: 1100, pressure_atm: 1.0, catalyst: Some(S::Platinum), duration_hours: 0.01, cross_recipe_group: None },
    Recipe { id: 854, name: "Melamine from Urea", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Urea, 1.5)], outputs: &[(S::Melamine, 1.0)],
        byproducts: &[(S::Ammonia, 0.3), (S::CarbonDioxide, 0.2)],
        min_temp_c: 400, pressure_atm: 10.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },

    // ===== ETHERS AND EPOXIDES =====
    Recipe { id: 860, name: "Diethyl Ether (Ethanol Dehydration)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Ethanol, 1.0)], outputs: &[(S::DiethylEther, 0.7)], byproducts: &[(S::Water, 0.3)],
        min_temp_c: 140, pressure_atm: 1.0, catalyst: Some(S::SulfuricAcid), duration_hours: 2.0, cross_recipe_group: None },
    Recipe { id: 861, name: "Ethylene Oxide", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Ethylene, 0.6), (S::Oxygen, 0.4)], outputs: &[(S::EthyleneOxide, 1.0)], byproducts: &[],
        min_temp_c: 270, pressure_atm: 20.0, catalyst: Some(S::Silver), duration_hours: 0.1, cross_recipe_group: None },

    // ===== HALOGENATED =====
    Recipe { id: 870, name: "Vinyl Chloride Monomer", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Ethylene, 0.45), (S::ChlorineGas, 0.55)], outputs: &[(S::VinylChloride, 1.0)],
        byproducts: &[(S::HydrochloricAcid, 0.3)],
        min_temp_c: 500, pressure_atm: 1.0, catalyst: None, duration_hours: 0.1, cross_recipe_group: None },
    Recipe { id: 871, name: "Chloroform (Methane Chlorination)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Methane, 0.15), (S::ChlorineGas, 0.85)], outputs: &[(S::Chloroform, 1.0)],
        byproducts: &[(S::HydrochloricAcid, 0.3)],
        min_temp_c: 400, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },

    // ===== POLYMERS FROM MONOMERS =====
    Recipe { id: 880, name: "Polystyrene", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Styrene, 1.0)], outputs: &[(S::Polyethylene, 1.0)], byproducts: &[],
        min_temp_c: 120, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0, cross_recipe_group: None },
    Recipe { id: 881, name: "PET (Polyester)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::EthyleneGlycol, 0.35), (S::TerephthalicAcid, 0.65)],
        outputs: &[(S::PolyesterFiber, 1.0)], byproducts: &[(S::Water, 0.15)],
        min_temp_c: 280, pressure_atm: 1.0, catalyst: None, duration_hours: 4.0, cross_recipe_group: None },
    Recipe { id: 882, name: "Nylon-6 from Caprolactam", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Caprolactam, 1.0), (S::Water, 0.05)], outputs: &[(S::NylonResin, 1.0)], byproducts: &[],
        min_temp_c: 260, pressure_atm: 1.0, catalyst: None, duration_hours: 12.0, cross_recipe_group: None },

    // ===== ADDITIONAL ORGANIC RECIPES =====

    // 1. Formic acid from CO + NaOH (Koch carbonylation)
    Recipe { id: 883, name: "Formic Acid (CO + NaOH)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::SynGas, 0.45), (S::SodiumHydroxide, 0.55)], outputs: &[(S::FormicAcid, 0.7)],
        byproducts: &[(S::Water, 0.3)],
        min_temp_c: 130, pressure_atm: 8.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },

    // 2. Oxalic acid from sodium formate pyrolysis
    Recipe { id: 884, name: "Oxalic Acid (Sodium Formate Pyrolysis)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::SodiumFormate, 1.0)], outputs: &[(S::OxalicAcid, 0.65)],
        byproducts: &[(S::Hydrogen, 0.05), (S::SodiumHydroxide, 0.3)],
        min_temp_c: 400, pressure_atm: 1.0, catalyst: None, duration_hours: 1.5, cross_recipe_group: None },

    // 3. MEK from 2-butanol dehydrogenation
    Recipe { id: 885, name: "MEK (2-Butanol Dehydrogenation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::SecButanol, 1.0)], outputs: &[(S::MethylEthylKetone, 0.95)],
        byproducts: &[(S::HydrogenGas, 0.05)],
        min_temp_c: 300, pressure_atm: 1.0, catalyst: Some(S::Copper), duration_hours: 0.5, cross_recipe_group: None },

    // 4. Glycerol synthetic route via propylene (allyl chloride -> epichlorohydrin -> glycerol)
    Recipe { id: 886, name: "Glycerol Synthetic (via Propylene)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Propylene, 0.6), (S::ChlorineGas, 0.3), (S::SodiumHydroxide, 0.3)],
        outputs: &[(S::Glycerol, 0.7)],
        byproducts: &[(S::HydrochloricAcid, 0.2), (S::Salt, 0.3)],
        min_temp_c: 510, pressure_atm: 1.0, catalyst: None, duration_hours: 3.0, cross_recipe_group: None },

    // 5. MMA from acetone cyanohydrin (ACH process)
    Recipe { id: 887, name: "MMA (Acetone Cyanohydrin Process)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Acetone, 0.4), (S::HydrogenCyanide, 0.2), (S::Methanol, 0.2), (S::SulfuricAcid, 0.2)],
        outputs: &[(S::MethylMethacrylate, 0.7)],
        byproducts: &[(S::Ammonium, 0.2), (S::Water, 0.1)],
        min_temp_c: 80, pressure_atm: 1.0, catalyst: None, duration_hours: 2.0, cross_recipe_group: None },

    // 6. MTBE from methanol + isobutylene
    Recipe { id: 888, name: "MTBE (Methanol + Isobutylene)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Methanol, 0.35), (S::Butadiene, 0.65)], outputs: &[(S::MTBE, 1.0)], byproducts: &[],
        min_temp_c: 75, pressure_atm: 7.0, catalyst: Some(S::SulfuricAcid), duration_hours: 1.0, cross_recipe_group: None },

    // 7. Propylene oxide chlorohydrin process
    Recipe { id: 889, name: "Propylene Oxide (Chlorohydrin)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Propylene, 0.5), (S::ChlorineGas, 0.35), (S::SodiumHydroxide, 0.3)],
        outputs: &[(S::PropyleneOxide, 0.7)],
        byproducts: &[(S::Salt, 0.3), (S::Water, 0.15)],
        min_temp_c: 50, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: Some(CRG_PROPYLENE_OXIDE) },

    // 8. Propylene oxide HPPO process (H2O2 / TS-1 zeolite)
    Recipe { id: 890, name: "Propylene Oxide (HPPO Process)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Propylene, 0.55), (S::HydrogenPeroxide, 0.45)],
        outputs: &[(S::PropyleneOxide, 0.75)],
        byproducts: &[(S::Water, 0.25)],
        min_temp_c: 45, pressure_atm: 30.0, catalyst: Some(S::Titanium), duration_hours: 0.5, cross_recipe_group: Some(CRG_PROPYLENE_OXIDE) },

    // 9. CCl4 from methane + excess Cl2
    Recipe { id: 891, name: "Carbon Tetrachloride (Methane + Cl2)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Methane, 0.1), (S::ChlorineGas, 0.9)], outputs: &[(S::CarbonTetrachloride, 0.8)],
        byproducts: &[(S::HydrochloricAcid, 0.2)],
        min_temp_c: 560, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5, cross_recipe_group: None },

    // 10. Freon-12 (CFC-12) from CCl4 + HF (Swarts reaction)
    Recipe { id: 892, name: "Freon-12 (Swarts Reaction)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::CarbonTetrachloride, 0.7), (S::HydrochloricAcid, 0.3)],
        outputs: &[(S::Freon12, 0.8)],
        byproducts: &[(S::HydrochloricAcid, 0.2)],
        min_temp_c: 130, pressure_atm: 1.0, catalyst: Some(S::Chromium), duration_hours: 2.0, cross_recipe_group: None },

    // 11. TFE from CHClF2 pyrolysis (for Teflon)
    Recipe { id: 893, name: "Tetrafluoroethylene (CHClF2 Pyrolysis)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Freon12, 1.0)], outputs: &[(S::Tetrafluoroethylene, 0.8)],
        byproducts: &[(S::HydrochloricAcid, 0.2)],
        min_temp_c: 700, pressure_atm: 1.0, catalyst: None, duration_hours: 0.01, cross_recipe_group: None },

    // 12. PTFE (Teflon) polymerization from TFE
    Recipe { id: 894, name: "PTFE Polymerization", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Tetrafluoroethylene, 1.0)], outputs: &[(S::PTFE, 1.0)], byproducts: &[],
        min_temp_c: 70, pressure_atm: 25.0, catalyst: None, duration_hours: 8.0, cross_recipe_group: None },

    // 13. Polypropylene from propylene (Ziegler-Natta)
    Recipe { id: 895, name: "Polypropylene (Ziegler-Natta)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Propylene, 1.0)], outputs: &[(S::Polypropylene, 1.0)], byproducts: &[],
        min_temp_c: 70, pressure_atm: 30.0, catalyst: Some(S::Titanium), duration_hours: 4.0, cross_recipe_group: None },

    // 14. Nitrobenzene from benzene nitration
    Recipe { id: 896, name: "Nitrobenzene (Benzene Nitration)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Benzene, 0.55), (S::NitricAcid, 0.45)],
        outputs: &[(S::Nitrobenzene, 0.85)],
        byproducts: &[(S::Water, 0.15)],
        min_temp_c: 55, pressure_atm: 1.0, catalyst: Some(S::SulfuricAcid), duration_hours: 1.0, cross_recipe_group: None },

    // 15. Acetic acid (Cativa process - Ir catalyzed methanol carbonylation)
    Recipe { id: 897, name: "Acetic Acid (Cativa Process)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Methanol, 0.5), (S::SynGas, 0.5)], outputs: &[(S::AceticAcid, 1.0)], byproducts: &[],
        min_temp_c: 180, pressure_atm: 30.0, catalyst: Some(S::Platinum), duration_hours: 0.5, cross_recipe_group: Some(CRG_ACETIC_ACID) },

    // 16. Acetic acid from vinegar fermentation
    Recipe { id: 898, name: "Acetic Acid (Vinegar Fermentation)", category: RecipeCategory::FoodBiological,
        inputs: &[(S::Ethanol, 0.6), (S::Oxygen, 0.4)], outputs: &[(S::AceticAcid, 0.8)],
        byproducts: &[(S::Water, 0.2)],
        min_temp_c: 30, pressure_atm: 1.0, catalyst: None, duration_hours: 168.0, cross_recipe_group: Some(CRG_ACETIC_ACID) },

    // 17. Cyclohexanone from cyclohexane oxidation
    Recipe { id: 899, name: "Cyclohexanone (Cyclohexane Oxidation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Benzene, 0.7), (S::HydrogenGas, 0.1), (S::Oxygen, 0.2)],
        outputs: &[(S::Cyclohexanone, 0.85)],
        byproducts: &[(S::Water, 0.15)],
        min_temp_c: 155, pressure_atm: 12.0, catalyst: Some(S::Cobalt), duration_hours: 2.0, cross_recipe_group: None },

    // 18. Styrene from ethylbenzene dehydrogenation
    Recipe { id: 1100, name: "Styrene (Ethylbenzene Dehydrogenation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Ethylbenzene, 1.0)], outputs: &[(S::Styrene, 0.9)],
        byproducts: &[(S::HydrogenGas, 0.1)],
        min_temp_c: 620, pressure_atm: 1.0, catalyst: Some(S::Iron), duration_hours: 0.1, cross_recipe_group: None },

    // 19. Phenol from cumene oxidation (co-product split from cumene process)
    Recipe { id: 1101, name: "Phenol (Cumene Oxidation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Benzene, 0.55), (S::Propylene, 0.35), (S::Oxygen, 0.1)],
        outputs: &[(S::Phenol, 0.65)],
        byproducts: &[(S::Acetone, 0.35)],
        min_temp_c: 90, pressure_atm: 5.0, catalyst: Some(S::PhosphoricAcid), duration_hours: 4.0, cross_recipe_group: None },

    // 20. Ethylbenzene from benzene + ethylene
    Recipe { id: 1102, name: "Ethylbenzene (Benzene + Ethylene)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Benzene, 0.55), (S::Ethylene, 0.45)],
        outputs: &[(S::Ethylbenzene, 1.0)], byproducts: &[],
        min_temp_c: 450, pressure_atm: 2.0, catalyst: Some(S::Aluminum), duration_hours: 0.5, cross_recipe_group: None },

    // 21. Methyl chloride from methane + Cl2
    Recipe { id: 1103, name: "Methyl Chloride (Methane + Cl2)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Methane, 0.3), (S::ChlorineGas, 0.7)], outputs: &[(S::MethylChloride, 0.8)],
        byproducts: &[(S::HydrochloricAcid, 0.2)],
        min_temp_c: 400, pressure_atm: 1.0, catalyst: None, duration_hours: 0.3, cross_recipe_group: None },

    // 22. Dichloromethane from methane + Cl2
    Recipe { id: 1104, name: "Dichloromethane (Methane + Cl2)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Methane, 0.2), (S::ChlorineGas, 0.8)], outputs: &[(S::Dichloromethane, 0.8)],
        byproducts: &[(S::HydrochloricAcid, 0.2)],
        min_temp_c: 400, pressure_atm: 1.0, catalyst: None, duration_hours: 0.3, cross_recipe_group: None },

    // 23. Allyl chloride from propylene + Cl2
    Recipe { id: 1105, name: "Allyl Chloride (Propylene + Cl2)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Propylene, 0.55), (S::ChlorineGas, 0.45)], outputs: &[(S::AllylChloride, 0.8)],
        byproducts: &[(S::HydrochloricAcid, 0.2)],
        min_temp_c: 510, pressure_atm: 1.0, catalyst: None, duration_hours: 0.1, cross_recipe_group: None },

    // 24. Epichlorohydrin from allyl chloride
    Recipe { id: 1106, name: "Epichlorohydrin (from Allyl Chloride)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::AllylChloride, 0.6), (S::ChlorineGas, 0.2), (S::SodiumHydroxide, 0.2)],
        outputs: &[(S::Epichlorohydrin, 0.7)],
        byproducts: &[(S::Salt, 0.2), (S::Water, 0.1)],
        min_temp_c: 38, pressure_atm: 1.0, catalyst: None, duration_hours: 1.0, cross_recipe_group: None },

    // 25. Acrylic acid from propylene oxidation
    Recipe { id: 1107, name: "Acrylic Acid (Propylene Oxidation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Propylene, 0.6), (S::Oxygen, 0.4)], outputs: &[(S::AcrylicAcid, 0.85)],
        byproducts: &[(S::Water, 0.15)],
        min_temp_c: 330, pressure_atm: 2.0, catalyst: Some(S::Molybdenum), duration_hours: 0.1, cross_recipe_group: None },

    // 26. Maleic anhydride from butane oxidation
    Recipe { id: 1108, name: "Maleic Anhydride (Butane Oxidation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Butadiene, 0.5), (S::Oxygen, 0.5)], outputs: &[(S::MaleicAnhydride, 0.7)],
        byproducts: &[(S::Water, 0.2), (S::CarbonDioxide, 0.1)],
        min_temp_c: 400, pressure_atm: 2.0, catalyst: Some(S::Vanadium), duration_hours: 0.1, cross_recipe_group: None },

    // 27. Phthalic anhydride from o-xylene
    Recipe { id: 1109, name: "Phthalic Anhydride (o-Xylene Oxidation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Xylene, 0.7), (S::Oxygen, 0.3)], outputs: &[(S::PhthalicAnhydride, 0.85)],
        byproducts: &[(S::Water, 0.1), (S::CarbonDioxide, 0.05)],
        min_temp_c: 370, pressure_atm: 1.0, catalyst: Some(S::Vanadium), duration_hours: 0.1, cross_recipe_group: None },

    // 28. 2-Ethylhexanol from propylene (hydroformylation -> aldol -> hydrogenation)
    Recipe { id: 1110, name: "2-Ethylhexanol (Propylene Hydroformylation)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Propylene, 0.5), (S::SynGas, 0.3), (S::HydrogenGas, 0.2)],
        outputs: &[(S::Ethylhexanol, 0.9)],
        byproducts: &[(S::Water, 0.1)],
        min_temp_c: 130, pressure_atm: 20.0, catalyst: Some(S::Cobalt), duration_hours: 3.0, cross_recipe_group: None },

    // 29. Vinyl acetate from ethylene + acetic acid + O2
    Recipe { id: 1111, name: "Vinyl Acetate (Ethylene + Acetic Acid)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Ethylene, 0.35), (S::AceticAcid, 0.45), (S::Oxygen, 0.2)],
        outputs: &[(S::VinylAcetate, 0.85)],
        byproducts: &[(S::Water, 0.15)],
        min_temp_c: 175, pressure_atm: 8.0, catalyst: Some(S::Gold), duration_hours: 0.5, cross_recipe_group: None },

    // 30. Dimethyl terephthalate from p-xylene + methanol
    Recipe { id: 1112, name: "Dimethyl Terephthalate (p-Xylene + MeOH)", category: RecipeCategory::ChemicalSynthesis,
        inputs: &[(S::Xylene, 0.6), (S::Methanol, 0.25), (S::Oxygen, 0.15)],
        outputs: &[(S::DimethylTerephthalate, 0.85)],
        byproducts: &[(S::Water, 0.15)],
        min_temp_c: 250, pressure_atm: 5.0, catalyst: Some(S::Cobalt), duration_hours: 3.0, cross_recipe_group: None },
];
