#![allow(dead_code)]
use crate::recipes::types::*;
use crate::recipes::substance::Substance as S;

const CRG_YOGURT: u32 = 500;
const CRG_KEFIR: u32 = 501;
const CRG_VINEGAR: u32 = 502;
const CRG_SAUERKRAUT: u32 = 503;
const CRG_BUTTER: u32 = 504;

pub static FERMENTATION_RECIPES: &[Recipe] = &[
    // ===================================================================
    // FERMENTED BEVERAGES
    // ===================================================================

    // --- Mead ---
    // Organism: Saccharomyces cerevisiae (wine yeast)
    // Inputs: honey + water (must), yeast, nutrients
    // Outputs: mead (10-14% ABV), CO2
    // Temperature: 16-24C, Duration: 4-12 weeks primary + months secondary
    Recipe { id: 1300, name: "Mead (Traditional)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Honey, 1.5), (S::Water, 3.5), (S::Yeast, 0.005)],
        outputs: &[(S::Mead, 4.5)],
        byproducts: &[(S::CarbonDioxide, 0.3)],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 1008.0,
        cross_recipe_group: None },

    // --- Sake ---
    // Organism: Aspergillus oryzae (koji mold) + Saccharomyces cerevisiae
    // Parallel saccharification and fermentation (multiple parallel fermentation)
    // Temperature: 30-32C for koji, 10-15C for main fermentation
    // Duration: 2-3 days koji + 18-32 days moromi fermentation
    Recipe { id: 1301, name: "Sake Brewing", category: RecipeCategory::Fermentation,
        inputs: &[(S::RiceGrain, 5.0), (S::Water, 8.0), (S::KojiMold, 0.05), (S::Yeast, 0.005)],
        outputs: &[(S::Sake, 7.0)],
        byproducts: &[(S::CarbonDioxide, 0.5)],
        min_temp_c: 15, pressure_atm: 1.0, catalyst: None, duration_hours: 768.0,
        cross_recipe_group: None },

    // --- Cider ---
    // Organism: Saccharomyces cerevisiae or wild yeasts
    // Inputs: apple juice, yeast
    // Temperature: 10-18C, Duration: 2-4 weeks primary + months aging
    Recipe { id: 1302, name: "Hard Cider", category: RecipeCategory::Fermentation,
        inputs: &[(S::Sugar, 2.0), (S::Water, 8.0), (S::Yeast, 0.005)],
        outputs: &[(S::Cider, 9.5)],
        byproducts: &[(S::CarbonDioxide, 0.3)],
        min_temp_c: 15, pressure_atm: 1.0, catalyst: None, duration_hours: 672.0,
        cross_recipe_group: None },

    // --- Perry ---
    // Organism: Saccharomyces cerevisiae, wild yeasts
    // Inputs: perry pear juice, yeast
    // Temperature: 12-18C, Duration: 3-6 weeks
    Recipe { id: 1303, name: "Perry (Pear Cider)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Sugar, 2.0), (S::Water, 8.0), (S::Yeast, 0.005)],
        outputs: &[(S::Perry, 9.5)],
        byproducts: &[(S::CarbonDioxide, 0.3)],
        min_temp_c: 15, pressure_atm: 1.0, catalyst: None, duration_hours: 840.0,
        cross_recipe_group: None },

    // --- Pulque ---
    // Organism: Zymomonas mobilis, Leuconostoc mesenteroides, Saccharomyces cerevisiae
    // Inputs: aguamiel (agave sap)
    // Temperature: 25-30C, Duration: 36-48 hours (very fast)
    Recipe { id: 1304, name: "Pulque (Agave Fermentation)", category: RecipeCategory::Fermentation,
        inputs: &[(S::AgavePlant, 5.0), (S::Water, 3.0)],
        outputs: &[(S::Pulque, 7.5)],
        byproducts: &[(S::CarbonDioxide, 0.2)],
        min_temp_c: 27, pressure_atm: 1.0, catalyst: None, duration_hours: 36.0,
        cross_recipe_group: None },

    // --- Chicha ---
    // Organism: wild yeasts + LAB, traditionally saliva amylase starts saccharification
    // Inputs: corn (maize), water
    // Temperature: 20-30C, Duration: 2-7 days
    Recipe { id: 1305, name: "Chicha (Corn Beer)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Flour, 3.0), (S::Water, 7.0), (S::Yeast, 0.005)],
        outputs: &[(S::Chicha, 9.5)],
        byproducts: &[(S::CarbonDioxide, 0.3)],
        min_temp_c: 25, pressure_atm: 1.0, catalyst: None, duration_hours: 120.0,
        cross_recipe_group: None },

    // --- Kumiss ---
    // Organism: Lactobacillus delbrueckii subsp. bulgaricus, Kluyveromyces marxianus
    // Inputs: mare's milk
    // Temperature: 25-30C, Duration: 5-8 hours (rapid)
    Recipe { id: 1306, name: "Kumiss (Fermented Mare's Milk)", category: RecipeCategory::Fermentation,
        inputs: &[(S::MaresMilk, 5.0), (S::StarterCulture, 0.1)],
        outputs: &[(S::Kumiss, 5.0)],
        byproducts: &[(S::CarbonDioxide, 0.05)],
        min_temp_c: 27, pressure_atm: 1.0, catalyst: None, duration_hours: 6.0,
        cross_recipe_group: None },

    // --- Ayran ---
    // Organism: Lactobacillus bulgaricus, Streptococcus thermophilus
    // Inputs: yogurt, water, salt
    // Temperature: 4-10C (served cold), Duration: 0.5 hours (mixing)
    Recipe { id: 1307, name: "Ayran (Yogurt Drink)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Yogurt, 1.0), (S::Water, 1.0), (S::Salt, 0.02)],
        outputs: &[(S::Ayran, 2.0)],
        byproducts: &[],
        min_temp_c: 5, pressure_atm: 1.0, catalyst: None, duration_hours: 0.5,
        cross_recipe_group: None },

    // --- Kvass ---
    // Organism: Saccharomyces cerevisiae, Lactobacillus spp.
    // Inputs: rye bread, water, sugar, yeast
    // Temperature: 20-28C, Duration: 1-3 days
    Recipe { id: 1308, name: "Kvass (Bread Fermentation)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Bread, 0.5), (S::Water, 5.0), (S::Sugar, 0.2), (S::Yeast, 0.005)],
        outputs: &[(S::Kvass, 5.5)],
        byproducts: &[(S::CarbonDioxide, 0.1)],
        min_temp_c: 24, pressure_atm: 1.0, catalyst: None, duration_hours: 48.0,
        cross_recipe_group: None },

    // --- Ginger Beer ---
    // Organism: Ginger beer plant (Brevibacterium vermiforme + S. florentinus symbiosis)
    // Inputs: ginger, sugar, water, lemon
    // Temperature: 20-25C, Duration: 3-7 days
    Recipe { id: 1309, name: "Ginger Beer (Fermented)", category: RecipeCategory::Fermentation,
        inputs: &[(S::GingerRoot, 0.1), (S::Sugar, 0.3), (S::Water, 5.0), (S::Yeast, 0.005)],
        outputs: &[(S::GingerBeer, 5.3)],
        byproducts: &[(S::CarbonDioxide, 0.1)],
        min_temp_c: 22, pressure_atm: 1.0, catalyst: None, duration_hours: 120.0,
        cross_recipe_group: None },

    // --- Tepache ---
    // Organism: Lactobacillus pentosus, L. plantarum, Saccharomyces spp. (wild, from pineapple skin)
    // Inputs: pineapple peel, piloncillo/sugar, water, cinnamon
    // Temperature: 20-30C, Duration: 2-3 days
    Recipe { id: 1310, name: "Tepache (Pineapple Fermentation)", category: RecipeCategory::Fermentation,
        inputs: &[(S::PineappleFruit, 1.0), (S::Sugar, 0.2), (S::Water, 3.0)],
        outputs: &[(S::Tepache, 4.0)],
        byproducts: &[(S::CarbonDioxide, 0.1)],
        min_temp_c: 25, pressure_atm: 1.0, catalyst: None, duration_hours: 60.0,
        cross_recipe_group: None },

    // --- Water Kefir ---
    // Organism: polysaccharide grain SCOBY: Lactobacillus, Leuconostoc, Acetobacter, Saccharomyces
    // Inputs: water kefir grains, sugar, water
    // Temperature: 20-25C, Duration: 24-48 hours
    Recipe { id: 1311, name: "Water Kefir", category: RecipeCategory::Fermentation,
        inputs: &[(S::KefirGrains, 0.05), (S::Sugar, 0.1), (S::Water, 1.0)],
        outputs: &[(S::WaterKefir, 1.1)],
        byproducts: &[(S::CarbonDioxide, 0.03)],
        min_temp_c: 22, pressure_atm: 1.0, catalyst: None, duration_hours: 36.0,
        cross_recipe_group: None },

    // --- Milk Kefir ---
    // Organism: kefir grains SCOBY: 30-50 species incl. Lactobacillus kefiranofaciens,
    //   Kluyveromyces marxianus, Saccharomyces cerevisiae
    // Inputs: milk, kefir grains
    // Temperature: 20-25C, Duration: 18-24 hours
    Recipe { id: 1312, name: "Milk Kefir", category: RecipeCategory::Fermentation,
        inputs: &[(S::Milk, 1.0), (S::KefirGrains, 0.05)],
        outputs: &[(S::MilkKefir, 1.0)],
        byproducts: &[(S::CarbonDioxide, 0.02)],
        min_temp_c: 22, pressure_atm: 1.0, catalyst: None, duration_hours: 22.0,
        cross_recipe_group: Some(CRG_KEFIR) },

    // --- Kombucha ---
    // Organism: SCOBY (symbiotic culture of bacteria and yeast): Acetobacter, Gluconobacter,
    //   Saccharomyces cerevisiae, Zygosaccharomyces
    // Inputs: sweet tea, SCOBY
    // Temperature: 24-29C, Duration: 7-14 days
    Recipe { id: 1313, name: "Kombucha", category: RecipeCategory::Fermentation,
        inputs: &[(S::TeaLeaves, 0.01), (S::Sugar, 0.1), (S::Water, 1.0), (S::SCOBYCulture, 0.05)],
        outputs: &[(S::Kombucha, 1.1)],
        byproducts: &[(S::CarbonDioxide, 0.03)],
        min_temp_c: 26, pressure_atm: 1.0, catalyst: None, duration_hours: 240.0,
        cross_recipe_group: None },

    // --- Jun ---
    // Organism: Jun SCOBY (similar to kombucha but adapted to honey + green tea)
    // Inputs: green tea, honey, jun SCOBY
    // Temperature: 21-27C, Duration: 5-7 days (faster than kombucha)
    Recipe { id: 1314, name: "Jun (Honey Kombucha)", category: RecipeCategory::Fermentation,
        inputs: &[(S::TeaLeaves, 0.01), (S::Honey, 0.1), (S::Water, 1.0), (S::SCOBYCulture, 0.05)],
        outputs: &[(S::Jun, 1.1)],
        byproducts: &[(S::CarbonDioxide, 0.03)],
        min_temp_c: 24, pressure_atm: 1.0, catalyst: None, duration_hours: 144.0,
        cross_recipe_group: None },

    // ===================================================================
    // FERMENTED FOODS
    // ===================================================================

    // --- Tempeh ---
    // Organism: Rhizopus oligosporus (mold)
    // Inputs: soybeans (soaked, cooked, dehulled), tempeh starter
    // Temperature: 30-32C, Duration: 24-48 hours
    Recipe { id: 1320, name: "Tempeh", category: RecipeCategory::Fermentation,
        inputs: &[(S::SoybeanRaw, 1.0), (S::Water, 1.5), (S::StarterCulture, 0.005)],
        outputs: &[(S::Tempeh, 1.5)],
        byproducts: &[(S::Water, 0.8)],
        min_temp_c: 31, pressure_atm: 1.0, catalyst: None, duration_hours: 36.0,
        cross_recipe_group: None },

    // --- Miso ---
    // Organism: Aspergillus oryzae (koji), Lactobacillus, Pediococcus halophilus, Zygosaccharomyces rouxii
    // Inputs: soybeans, rice/barley koji, salt
    // Temperature: 25-30C initial, then ambient (seasonal) for months
    // Duration: 3-24 months depending on style (shiro miso 3mo, aka miso 1-3 years)
    Recipe { id: 1321, name: "Miso (White/Shiro, Short Aged)", category: RecipeCategory::Fermentation,
        inputs: &[(S::SoybeanRaw, 1.0), (S::RiceGrain, 1.0), (S::KojiMold, 0.05), (S::Salt, 0.3)],
        outputs: &[(S::Miso, 2.0)],
        byproducts: &[],
        min_temp_c: 25, pressure_atm: 1.0, catalyst: None, duration_hours: 2160.0,
        cross_recipe_group: None },

    Recipe { id: 1322, name: "Miso (Red/Aka, Long Aged)", category: RecipeCategory::Fermentation,
        inputs: &[(S::SoybeanRaw, 2.0), (S::RiceGrain, 0.5), (S::KojiMold, 0.05), (S::Salt, 0.5)],
        outputs: &[(S::Miso, 2.5)],
        byproducts: &[],
        min_temp_c: 20, pressure_atm: 1.0, catalyst: None, duration_hours: 17520.0,
        cross_recipe_group: None },

    // --- Natto ---
    // Organism: Bacillus subtilis var. natto
    // Inputs: soybeans (soaked, steamed), natto starter
    // Temperature: 40-45C, Duration: 18-24 hours
    Recipe { id: 1323, name: "Natto", category: RecipeCategory::Fermentation,
        inputs: &[(S::SoybeanRaw, 1.0), (S::Water, 1.0), (S::StarterCulture, 0.001)],
        outputs: &[(S::Natto, 1.5)],
        byproducts: &[],
        min_temp_c: 42, pressure_atm: 1.0, catalyst: None, duration_hours: 22.0,
        cross_recipe_group: None },

    // --- Kimchi ---
    // Organism: Leuconostoc mesenteroides (early), Lactobacillus plantarum (late),
    //   Lactobacillus brevis, Pediococcus cerevisiae
    // Inputs: napa cabbage, salt, gochugaru, garlic, ginger, fish sauce
    // Temperature: 20-25C for 1-3 days, then refrigerate (3-5C) for weeks
    // Duration: 3-14 days active, weeks to months aging
    Recipe { id: 1324, name: "Kimchi", category: RecipeCategory::Fermentation,
        inputs: &[(S::Water, 3.0), (S::Salt, 0.15), (S::GingerRoot, 0.05), (S::FishSauce, 0.05)],
        outputs: &[(S::Kimchi, 3.0)],
        byproducts: &[(S::CarbonDioxide, 0.05)],
        min_temp_c: 22, pressure_atm: 1.0, catalyst: None, duration_hours: 672.0,
        cross_recipe_group: Some(CRG_SAUERKRAUT) },

    // --- Sauerkraut ---
    // Organism: Leuconostoc mesenteroides (stage 1), Lactobacillus plantarum (stage 2),
    //   Lactobacillus brevis
    // Inputs: cabbage, salt (2-3% by weight)
    // Temperature: 16-21C (18C ideal), Duration: 2-4 weeks
    Recipe { id: 1325, name: "Sauerkraut", category: RecipeCategory::Fermentation,
        inputs: &[(S::Water, 5.0), (S::Salt, 0.1)],
        outputs: &[(S::Sauerkraut, 4.8)],
        byproducts: &[(S::CarbonDioxide, 0.05)],
        min_temp_c: 18, pressure_atm: 1.0, catalyst: None, duration_hours: 504.0,
        cross_recipe_group: Some(CRG_SAUERKRAUT) },

    // --- Curtido ---
    // Organism: Leuconostoc, Lactobacillus (similar to sauerkraut, Central American)
    // Inputs: cabbage, onion, carrot, oregano, vinegar, salt
    // Temperature: 20-25C, Duration: 1-3 days (shorter than sauerkraut)
    Recipe { id: 1326, name: "Curtido (Central American Sauerkraut)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Water, 3.0), (S::Salt, 0.1), (S::Vinegar, 0.2)],
        outputs: &[(S::Curtido, 3.0)],
        byproducts: &[(S::CarbonDioxide, 0.02)],
        min_temp_c: 22, pressure_atm: 1.0, catalyst: None, duration_hours: 48.0,
        cross_recipe_group: Some(CRG_SAUERKRAUT) },

    // --- Tsukemono (Japanese Pickles) ---
    // Organism: Lactobacillus plantarum, L. brevis (for nuka-zuke rice bran pickles)
    // Inputs: vegetables, salt, rice bran (nuka)
    // Temperature: 20-25C, Duration: hours to months depending on type
    Recipe { id: 1327, name: "Tsukemono (Rice Bran Pickles)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Water, 3.0), (S::Salt, 0.2), (S::RiceGrain, 0.5)],
        outputs: &[(S::Tsukemono, 3.0)],
        byproducts: &[],
        min_temp_c: 22, pressure_atm: 1.0, catalyst: None, duration_hours: 168.0,
        cross_recipe_group: None },

    // --- Dosa Batter ---
    // Organism: Leuconostoc mesenteroides, Lactobacillus fermentum, Streptococcus faecalis
    // Inputs: rice (3 parts), black gram/urad dal (1 part), water
    // Temperature: 25-35C, Duration: 6-12 hours
    Recipe { id: 1328, name: "Dosa Batter Fermentation", category: RecipeCategory::Fermentation,
        inputs: &[(S::RiceGrain, 0.75), (S::SoybeanRaw, 0.25), (S::Water, 1.0)],
        outputs: &[(S::DosaBatter, 2.0)],
        byproducts: &[(S::CarbonDioxide, 0.02)],
        min_temp_c: 30, pressure_atm: 1.0, catalyst: None, duration_hours: 10.0,
        cross_recipe_group: None },

    // --- Idli ---
    // Organism: Leuconostoc mesenteroides, Streptococcus thermophilus, Torulopsis
    // Inputs: rice, urad dal, water (same batter as dosa, steamed)
    // Temperature: 30C fermentation, then steam at 100C
    // Duration: 12-18 hours fermentation + 15 min steaming
    Recipe { id: 1329, name: "Idli (Steamed Fermented Rice Cake)", category: RecipeCategory::Fermentation,
        inputs: &[(S::RiceGrain, 0.75), (S::SoybeanRaw, 0.25), (S::Water, 1.0)],
        outputs: &[(S::Idli, 1.8)],
        byproducts: &[(S::Steam, 0.2)],
        min_temp_c: 30, pressure_atm: 1.0, catalyst: None, duration_hours: 15.0,
        cross_recipe_group: None },

    // --- Injera ---
    // Organism: Candida milleri, Saccharomyces cerevisiae, Lactobacillus, wild yeasts
    // Inputs: teff flour, water
    // Temperature: 25C, Duration: 48-72 hours (traditional), 12-24 hours (quick)
    Recipe { id: 1330, name: "Injera (Teff Sourdough Flatbread)", category: RecipeCategory::Fermentation,
        inputs: &[(S::TeffGrain, 1.0), (S::Water, 1.5)],
        outputs: &[(S::Injera, 2.0)],
        byproducts: &[(S::CarbonDioxide, 0.05)],
        min_temp_c: 25, pressure_atm: 1.0, catalyst: None, duration_hours: 64.0,
        cross_recipe_group: None },

    // --- Fermented Hot Sauce ---
    // Organism: Lactobacillus plantarum, L. brevis
    // Inputs: chili peppers, salt, garlic
    // Temperature: 20-25C, Duration: 5-14 days
    Recipe { id: 1331, name: "Fermented Hot Sauce", category: RecipeCategory::Fermentation,
        inputs: &[(S::Water, 2.0), (S::Salt, 0.1)],
        outputs: &[(S::FermentedHotSauce, 2.0)],
        byproducts: &[(S::CarbonDioxide, 0.02)],
        min_temp_c: 22, pressure_atm: 1.0, catalyst: None, duration_hours: 240.0,
        cross_recipe_group: None },

    // --- Fermented Garlic (Black Garlic) ---
    // Process: Maillard reaction under controlled heat and humidity (not truly fermentation
    //   but often classified as such)
    // Inputs: whole garlic bulbs
    // Temperature: 60-77C, Duration: 2-4 weeks
    Recipe { id: 1332, name: "Black Garlic (Fermented Garlic)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Water, 1.0)],
        outputs: &[(S::FermentedGarlic, 0.7)],
        byproducts: &[(S::Water, 0.2)],
        min_temp_c: 65, pressure_atm: 1.0, catalyst: None, duration_hours: 504.0,
        cross_recipe_group: None },

    // --- Surstromming ---
    // Organism: Haloanaerobium praevalens, Haloanaerobium spp. (halophilic anaerobes)
    // Inputs: Baltic herring, salt (10-15%)
    // Temperature: 15-18C, Duration: 6+ months
    Recipe { id: 1333, name: "Surstromming (Fermented Herring)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Water, 5.0), (S::Salt, 0.7)],
        outputs: &[(S::Surstromming, 3.0)],
        byproducts: &[(S::CarbonDioxide, 0.1)],
        min_temp_c: 17, pressure_atm: 1.0, catalyst: None, duration_hours: 4320.0,
        cross_recipe_group: None },

    // --- Hakarl ---
    // Organism: autolytic enzymes + halotolerant bacteria
    // Inputs: Greenland shark meat
    // Temperature: ambient (Iceland, ~5-10C), Duration: 6-12 weeks buried + 4-5 months drying
    Recipe { id: 1334, name: "Hakarl (Fermented Shark)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Water, 10.0), (S::Sand, 5.0)],
        outputs: &[(S::Hakarl, 3.0)],
        byproducts: &[(S::Water, 5.0), (S::Ammonia, 0.1)],
        min_temp_c: 8, pressure_atm: 1.0, catalyst: None, duration_hours: 5040.0,
        cross_recipe_group: None },

    // --- Shrimp Paste ---
    // Organism: halophilic bacteria (Halococcus, Halobacterium), Bacillus spp.
    // Inputs: small shrimp/krill, salt (15-25%)
    // Temperature: 30-35C (tropical ambient), Duration: 1-6 months
    Recipe { id: 1335, name: "Shrimp Paste (Belacan/Kapi)", category: RecipeCategory::Fermentation,
        inputs: &[(S::Water, 5.0), (S::Salt, 1.2)],
        outputs: &[(S::ShrimpPaste, 2.5)],
        byproducts: &[(S::Water, 2.0)],
        min_temp_c: 32, pressure_atm: 1.0, catalyst: None, duration_hours: 4320.0,
        cross_recipe_group: None },

    // ===================================================================
    // FERMENTED DAIRY
    // ===================================================================

    // --- Kefir (traditional, from grains) ---
    // Organism: kefir grains SCOBY (30-50 species): Lactobacillus kefiranofaciens,
    //   L. kefiri, Leuconostoc, Kluyveromyces marxianus, S. cerevisiae, Acetobacter
    // Temperature: 20-25C, Duration: 18-24 hours
    Recipe { id: 1340, name: "Kefir (Traditional Grain)", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 1.0), (S::KefirGrains, 0.05)],
        outputs: &[(S::Kefir, 1.0)],
        byproducts: &[(S::CarbonDioxide, 0.01)],
        min_temp_c: 22, pressure_atm: 1.0, catalyst: None, duration_hours: 22.0,
        cross_recipe_group: Some(CRG_KEFIR) },

    // --- Skyr ---
    // Organism: Streptococcus thermophilus, Lactobacillus bulgaricus, L. helveticus + rennet
    // Inputs: skim milk, skyr starter, rennet
    // Temperature: 38-40C, Duration: 4-6 hours fermentation + straining
    Recipe { id: 1341, name: "Skyr (Icelandic Strained Yogurt)", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 4.0), (S::StarterCulture, 0.05), (S::RennetEnzyme, 0.001)],
        outputs: &[(S::Skyr, 1.0)],
        byproducts: &[(S::Whey, 2.8)],
        min_temp_c: 39, pressure_atm: 1.0, catalyst: None, duration_hours: 5.0,
        cross_recipe_group: Some(CRG_YOGURT) },

    // --- Labneh ---
    // Organism: S. lactis, S. thermophilus, L. bulgaricus, lactose-fermenting yeasts
    // Inputs: yogurt, salt, cheesecloth
    // Temperature: 4-10C (strained cold), Duration: 12-24 hours draining
    Recipe { id: 1342, name: "Labneh (Strained Yogurt Cheese)", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Yogurt, 2.0), (S::Salt, 0.02)],
        outputs: &[(S::Labneh, 1.0)],
        byproducts: &[(S::Whey, 0.9)],
        min_temp_c: 6, pressure_atm: 1.0, catalyst: None, duration_hours: 18.0,
        cross_recipe_group: None },

    // --- Creme Fraiche ---
    // Organism: Lactococcus lactis, Leuconostoc cremoris (mesophilic)
    // Inputs: heavy cream, buttermilk/starter
    // Temperature: 22-28C, Duration: 12-24 hours
    Recipe { id: 1343, name: "Creme Fraiche", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Cream, 1.0), (S::StarterCulture, 0.02)],
        outputs: &[(S::CremeFraiche, 1.0)],
        byproducts: &[],
        min_temp_c: 24, pressure_atm: 1.0, catalyst: None, duration_hours: 18.0,
        cross_recipe_group: None },

    // --- Cultured Butter ---
    // Organism: Lactococcus lactis subsp. lactis biovar diacetylactis (produces diacetyl flavor)
    // Inputs: cream, mesophilic culture
    // Temperature: 24-28C fermentation, then churn at 12-15C
    // Duration: 12-24 hours fermentation + 0.5 hours churning
    Recipe { id: 1344, name: "Cultured Butter", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Cream, 2.0), (S::StarterCulture, 0.02)],
        outputs: &[(S::CulturedButter, 0.8)],
        byproducts: &[(S::Buttermilk, 1.0)],
        min_temp_c: 14, pressure_atm: 1.0, catalyst: None, duration_hours: 24.0,
        cross_recipe_group: Some(CRG_BUTTER) },

    // --- Buttermilk ---
    // Organism: Lactococcus lactis, Leuconostoc cremoris (mesophilic)
    // Inputs: milk or skim milk, mesophilic culture
    // Temperature: 20-25C, Duration: 12-24 hours (up to 72 in cold conditions)
    Recipe { id: 1345, name: "Cultured Buttermilk", category: RecipeCategory::DairyProcessing,
        inputs: &[(S::Milk, 1.0), (S::StarterCulture, 0.02)],
        outputs: &[(S::Buttermilk, 1.0)],
        byproducts: &[],
        min_temp_c: 22, pressure_atm: 1.0, catalyst: None, duration_hours: 18.0,
        cross_recipe_group: None },
];
