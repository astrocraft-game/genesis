#![allow(dead_code)]
use super::substance::Substance as S;
use super::types::*;

const CRG_LEATHER: u32 = 301;
const CRG_PAPER: u32 = 302;
const CRG_GELATIN: u32 = 550;
const CRG_ANIMAL_MEAL: u32 = 551;

pub static BIOMATERIAL_RECIPES: &[Recipe] = &[
    // ===================================================================
    // LEATHER AND HIDE PROCESSING
    // ===================================================================

    // --- Chrome Tanning ---
    // Chemicals: chromium(III) sulfate (Cr2(SO4)3)
    // Steps: soak raw hide -> lime (remove hair) -> delime/bate -> pickle (acidify)
    //   -> chrome tan (pH 3.0-4.0, then basify to 3.8-4.2) -> neutralize -> dye/finish
    // Temperature: 25-35C tanning, Duration: 1-3 days total (much faster than veg)
    // Result: soft, supple, heat-resistant (withstands 100C+), blue-green "wet blue"
    Recipe {
        id: 1800,
        name: "Chrome Tanned Leather",
        category: RecipeCategory::LeatherTanning,
        inputs: &[
            (S::AnimalHide, 3.0),
            (S::ChromiumSulfate, 0.1),
            (S::SulfuricAcid, 0.05),
            (S::Salt, 0.2),
            (S::Water, 5.0),
        ],
        outputs: &[(S::LeatherProduct, 1.0)],
        byproducts: &[(S::Water, 5.0)],
        min_temp_c: 30,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 48.0,
        cross_recipe_group: Some(CRG_LEATHER),
    },
    // --- Vegetable Tanning ---
    // Chemicals: tannins from oak bark, chestnut, mimosa, quebracho, tara
    // Steps: soak hide -> lime -> delime -> tan in pits with increasing tannin concentration
    // Temperature: 20-25C, Duration: 30-60 days (pit tanning), 2-3 months traditional
    // Result: firm, stiff initially (softens with use), brown color, ages beautifully
    Recipe {
        id: 1801,
        name: "Vegetable Tanned Leather",
        category: RecipeCategory::LeatherTanning,
        inputs: &[(S::AnimalHide, 3.0), (S::Tannin, 0.8), (S::Water, 10.0)],
        outputs: &[(S::LeatherProduct, 1.0)],
        byproducts: &[(S::Water, 10.0)],
        min_temp_c: 22,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 1440.0,
        cross_recipe_group: Some(CRG_LEATHER),
    },
    // --- Brain Tanning ---
    // Chemicals: animal brain tissue (contains lecithin/phospholipids that lubricate fibers)
    // Steps: scrape hide -> soak in brain solution -> stretch repeatedly while drying -> smoke
    //   Every animal has enough brain to tan its own hide (traditional saying).
    // Temperature: ambient (20-30C), Duration: 3-5 days
    // Result: soft, supple, washable buckskin. Smoking provides water resistance.
    Recipe {
        id: 1802,
        name: "Brain Tanned Leather",
        category: RecipeCategory::LeatherTanning,
        inputs: &[(S::AnimalHide, 2.0), (S::BrainTissue, 0.3), (S::Water, 3.0)],
        outputs: &[(S::BrainTannedLeather, 1.0)],
        byproducts: &[(S::Water, 3.0)],
        min_temp_c: 25,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 96.0,
        cross_recipe_group: Some(CRG_LEATHER),
    },
    // --- Alum Tanning ---
    // Chemicals: potassium aluminum sulfate (alum) + salt + flour/egg yolk
    // Steps: soak prepared hide in alum + salt solution for 1-4 weeks
    // Temperature: 20-25C, Duration: 1-4 weeks
    // Result: white leather (tawing). Not fully waterproof. Used for bookbinding, gloves.
    Recipe {
        id: 1803,
        name: "Alum Tanned Leather (Tawing)",
        category: RecipeCategory::LeatherTanning,
        inputs: &[
            (S::AnimalHide, 2.0),
            (S::AlumCompound, 0.3),
            (S::Salt, 0.2),
            (S::Water, 5.0),
        ],
        outputs: &[(S::AlumTannedLeather, 1.0)],
        byproducts: &[(S::Water, 5.0)],
        min_temp_c: 22,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 504.0,
        cross_recipe_group: Some(CRG_LEATHER),
    },
    // --- Oil Tanning (Chamois) ---
    // Chemicals: fish oil or other marine oils (cod liver oil, whale oil historically)
    // Steps: prepare split hide (grain removed) -> saturate with oil -> oxidize (pound, heat)
    //   Oil polymerizes in the hide, forming waterproof, absorbent leather.
    // Temperature: 40-50C, Duration: several days of oiling + oxidizing
    // Result: extremely absorbent, washable. Used for cleaning, polishing.
    Recipe {
        id: 1804,
        name: "Chamois Leather (Oil Tanning)",
        category: RecipeCategory::LeatherTanning,
        inputs: &[(S::AnimalHide, 2.0), (S::OliveOil, 0.5), (S::Water, 3.0)],
        outputs: &[(S::ChamoisLeather, 1.0)],
        byproducts: &[(S::Water, 3.0)],
        min_temp_c: 45,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 168.0,
        cross_recipe_group: Some(CRG_LEATHER),
    },
    // --- Synthetic Leather (PU) ---
    // Steps: coat fabric base with polyurethane solution -> dry -> emboss grain pattern
    // Temperature: 80-120C drying/curing, Duration: 1-4 hours
    Recipe {
        id: 1805,
        name: "Synthetic Leather (PU)",
        category: RecipeCategory::LeatherTanning,
        inputs: &[
            (S::PolyesterFiber, 0.5),
            (S::Ethylene, 0.5),
            (S::Water, 1.0),
        ],
        outputs: &[(S::SyntheticLeather, 1.0)],
        byproducts: &[(S::Water, 0.5)],
        min_temp_c: 100,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: None,
    },
    // ===================================================================
    // PAPER AND PULP PROCESSES
    // ===================================================================

    // --- Kraft Pulp ---
    // Already exists as id 666 in biological.rs. These are additional pulping methods.
    // Chemicals: NaOH + Na2S (white liquor), cooking at 170C, 7-10 atm, 2-4 hours
    // Yield: 40-50% of wood. Strongest pulp. Black liquor byproduct burned for energy.
    // (Existing recipe covers this - adding sulfite and mechanical alternatives)

    // --- Sulfite Pulp ---
    // Chemicals: sulfurous acid (H2SO3) + calcium/magnesium/sodium bisulfite
    // Temperature: 130-160C, Duration: 6-12 hours cooking
    // Yield: 40-50%. Softer, more flexible than kraft. Good for fine papers.
    Recipe {
        id: 1810,
        name: "Sulfite Pulp",
        category: RecipeCategory::PaperPulping,
        inputs: &[(S::WoodChips, 2.5), (S::SulfuricAcid, 0.2), (S::Water, 5.0)],
        outputs: &[(S::SulfitePulp, 1.0)],
        byproducts: &[(S::Water, 4.0)],
        min_temp_c: 145,
        pressure_atm: 7.0,
        catalyst: None,
        duration_hours: 9.0,
        cross_recipe_group: Some(CRG_PAPER),
    },
    // --- Mechanical Pulp (Stone Groundwood) ---
    // Process: press logs against rotating grindstone. No chemicals.
    // Temperature: ambient (friction generates heat), Duration: continuous
    // Yield: 90-98% of wood. Weak, yellows easily (contains lignin). Newsprint.
    Recipe {
        id: 1811,
        name: "Mechanical Pulp (Groundwood)",
        category: RecipeCategory::PaperPulping,
        inputs: &[(S::WoodLogs, 1.1)],
        outputs: &[(S::MechanicalPulp, 1.0)],
        byproducts: &[],
        min_temp_c: 20,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 2.0,
        cross_recipe_group: Some(CRG_PAPER),
    },
    // --- Thermomechanical Pulp (TMP) ---
    // Process: steam wood chips at 110-130C under pressure, then refine between discs
    // Temperature: 110-130C, Duration: 1-2 hours
    // Yield: 92-95%. Stronger than groundwood, still contains lignin.
    Recipe {
        id: 1812,
        name: "Thermomechanical Pulp (TMP)",
        category: RecipeCategory::PaperPulping,
        inputs: &[(S::WoodChips, 1.1), (S::Steam, 0.5)],
        outputs: &[(S::ThermoMechanicalPulp, 1.0)],
        byproducts: &[(S::Water, 0.3)],
        min_temp_c: 120,
        pressure_atm: 3.0,
        catalyst: None,
        duration_hours: 1.5,
        cross_recipe_group: Some(CRG_PAPER),
    },
    // --- Recycled Paper ---
    // Steps: pulp waste paper in water -> screen/clean -> deink (flotation or washing)
    //   -> bleach (optional) -> form new sheets
    // Temperature: 40-50C (deinking), Duration: 2-4 hours
    Recipe {
        id: 1813,
        name: "Recycled Paper Pulping",
        category: RecipeCategory::PaperPulping,
        inputs: &[
            (S::PaperProduct, 1.2),
            (S::Water, 5.0),
            (S::SodiumHydroxide, 0.05),
        ],
        outputs: &[(S::RecycledPaper, 1.0)],
        byproducts: &[(S::Water, 4.5)],
        min_temp_c: 45,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: Some(CRG_PAPER),
    },
    // --- Rice Paper (East Asian) ---
    // Made from rice straw pulp or pith of Tetrapanax papyrifer plant.
    // Steps: soak rice straw -> cook with NaOH -> wash -> form sheets -> dry
    // Temperature: 100C cooking, Duration: 4-8 hours
    Recipe {
        id: 1814,
        name: "Rice Paper",
        category: RecipeCategory::PaperPulping,
        inputs: &[
            (S::StrawFiber, 2.0),
            (S::SodiumHydroxide, 0.1),
            (S::Water, 5.0),
        ],
        outputs: &[(S::RicePaper, 1.0)],
        byproducts: &[(S::Water, 4.5)],
        min_temp_c: 100,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 6.0,
        cross_recipe_group: None,
    },
    // --- Handmade Paper ---
    // Steps: soak plant fibers -> beat/pound -> form sheets on screen/mould & deckle
    //   -> press -> dry (air or sun)
    // Temperature: ambient, Duration: 1-3 days (including drying)
    Recipe {
        id: 1815,
        name: "Handmade Paper",
        category: RecipeCategory::PaperPulping,
        inputs: &[(S::CottonFiber, 0.5), (S::Water, 5.0)],
        outputs: &[(S::HandmadePaper, 0.5)],
        byproducts: &[(S::Water, 4.5)],
        min_temp_c: 20,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 48.0,
        cross_recipe_group: None,
    },
    // --- Papyrus ---
    // Steps: slice papyrus pith into thin strips -> soak in water -> layer crosswise
    //   -> pound/press -> dry (sun). Natural sugars act as adhesive.
    // Temperature: ambient, Duration: 2-5 days
    Recipe {
        id: 1816,
        name: "Papyrus",
        category: RecipeCategory::PaperPulping,
        inputs: &[(S::StrawFiber, 2.0), (S::Water, 3.0)],
        outputs: &[(S::Papyrus, 1.0)],
        byproducts: &[(S::Water, 3.0)],
        min_temp_c: 25,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 72.0,
        cross_recipe_group: None,
    },
    // --- Bark Cloth / Tapa ---
    // Steps: strip inner bark of fig/mulberry tree -> soak -> beat with wooden mallet
    //   -> stretch thin -> dry (sun). Used in Polynesia, Africa, SE Asia.
    // Temperature: ambient, Duration: 1-3 days
    Recipe {
        id: 1817,
        name: "Bark Cloth (Tapa)",
        category: RecipeCategory::PaperPulping,
        inputs: &[(S::WoodLogs, 3.0), (S::Water, 3.0)],
        outputs: &[(S::BarkCloth, 1.0)],
        byproducts: &[(S::Water, 3.0), (S::WoodChips, 1.5)],
        min_temp_c: 25,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 48.0,
        cross_recipe_group: None,
    },
    // ===================================================================
    // BIOLOGICAL MATERIALS - ANIMAL FEED MEALS
    // ===================================================================

    // --- Bone Meal ---
    // Steps: render bones (steam cook 133C, 3 bar, 20 min) -> dehydrate -> grind
    // Temperature: 133C+, Duration: 2-4 hours total
    // Used as: fertilizer (phosphorus, calcium), animal feed supplement
    Recipe {
        id: 1830,
        name: "Bone Meal Production",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::AnimalBones, 3.0), (S::Steam, 1.0)],
        outputs: &[(S::BoneMeal, 1.0)],
        byproducts: &[(S::Water, 1.5)],
        min_temp_c: 133,
        pressure_atm: 3.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: Some(CRG_ANIMAL_MEAL),
    },
    // --- Blood Meal ---
    // Steps: collect slaughter blood -> coagulate (steam or flash dry) -> dry -> grind
    //   Ring dryer or spray dryer at 200-300C (flash drying) or 100-120C (slower)
    // Temperature: 100-300C, Duration: 1-4 hours
    // Used as: fertilizer (high nitrogen 12-13%), animal feed protein
    Recipe {
        id: 1831,
        name: "Blood Meal Production",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::AnimalBlood, 5.0)],
        outputs: &[(S::BloodMeal, 1.0)],
        byproducts: &[(S::Water, 3.5)],
        min_temp_c: 120,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: Some(CRG_ANIMAL_MEAL),
    },
    // --- Fish Meal ---
    // Steps: cook whole fish/trimmings (85-95C) -> press (separate oil + water)
    //   -> dry (80-100C) -> grind. Oil is separated as fish oil byproduct.
    // Temperature: 90C cooking, 90C drying, Duration: 4-6 hours
    // Used as: animal feed (high protein 60-72%), aquaculture feed
    Recipe {
        id: 1832,
        name: "Fish Meal Production",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::FishOffal, 5.0), (S::Steam, 0.5)],
        outputs: &[(S::FishMeal, 1.0)],
        byproducts: &[(S::OliveOil, 0.3), (S::Water, 3.0)],
        min_temp_c: 90,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 5.0,
        cross_recipe_group: Some(CRG_ANIMAL_MEAL),
    },
    // --- Feather Meal ---
    // Steps: wash feathers -> hydrolyze under pressure (130-150C, 3-4 bar, 30-60 min)
    //   Keratin is very insoluble; high pressure steam breaks disulfide bonds.
    //   -> dry -> grind. Slow-release nitrogen fertilizer.
    // Temperature: 140C, Duration: 2-4 hours
    Recipe {
        id: 1833,
        name: "Feather Meal Production",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::PoultryFeathers, 5.0), (S::Steam, 1.0)],
        outputs: &[(S::FeatherMeal, 1.0)],
        byproducts: &[(S::Water, 3.5)],
        min_temp_c: 140,
        pressure_atm: 3.5,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: Some(CRG_ANIMAL_MEAL),
    },
    // ===================================================================
    // BIOLOGICAL MATERIALS - GELLING AGENTS AND EXTRACTS
    // ===================================================================

    // --- Gelatin ---
    // Source: collagen from animal bones, skin, connective tissue (pig, cow, fish)
    // Steps: wash raw material -> acid or alkaline pretreatment (Type A acid, Type B alkaline)
    //   -> extraction in hot water (50-90C, multiple passes at increasing temp)
    //   -> filter -> concentrate (vacuum evaporator) -> dry (spray or belt dryer)
    // Temperature: 50-90C extraction, Duration: 3-9 hours per extraction pass
    Recipe {
        id: 1840,
        name: "Gelatin Extraction",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[
            (S::AnimalBones, 3.0),
            (S::Water, 5.0),
            (S::HydrochloricAcid, 0.1),
        ],
        outputs: &[(S::Gelatin, 1.0)],
        byproducts: &[(S::Water, 5.0), (S::BoneMeal, 0.5)],
        min_temp_c: 65,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 12.0,
        cross_recipe_group: Some(CRG_GELATIN),
    },
    // --- Isinglass ---
    // Source: swim bladders of sturgeon and other fish
    // Steps: clean swim bladders -> dry -> cut into strips -> dissolve in warm water (40-60C)
    // Temperature: 40-60C, Duration: 1-4 hours dissolution
    // Used as: fining agent for wine and beer (clarifies by attracting suspended particles)
    Recipe {
        id: 1841,
        name: "Isinglass Production",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::SwimBladder, 2.0), (S::Water, 3.0)],
        outputs: &[(S::Isinglass, 1.0)],
        byproducts: &[(S::Water, 3.0)],
        min_temp_c: 50,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 24.0,
        cross_recipe_group: Some(CRG_GELATIN),
    },
    // --- Carrageenan ---
    // Source: red seaweed (Chondrus crispus, Eucheuma, Kappaphycus)
    // Steps: wash seaweed -> cook in hot alkali (KOH, 70-80C, 2h) -> filter -> precipitate
    //   -> dry -> mill. Two types: kappa (gels with K+), iota (gels with Ca2+), lambda (thickens).
    // Temperature: 70-80C, Duration: 4-6 hours total
    Recipe {
        id: 1842,
        name: "Carrageenan Extraction",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[
            (S::Seaweed, 5.0),
            (S::Water, 10.0),
            (S::SodiumHydroxide, 0.1),
        ],
        outputs: &[(S::Carrageenan, 1.0)],
        byproducts: &[(S::Water, 12.0)],
        min_temp_c: 75,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 5.0,
        cross_recipe_group: None,
    },
    // --- Agar-Agar ---
    // Source: red algae (Gelidium, Gracilaria)
    // Steps: wash seaweed -> boil in water (100C, 2-4h) -> filter -> cool (gels at 32-40C)
    //   -> freeze (-20C) to purify -> thaw -> dry. Gel sets at ~38C, melts at ~85C.
    // Temperature: 100C extraction, Duration: 4-8 hours
    Recipe {
        id: 1843,
        name: "Agar-Agar Extraction",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::Seaweed, 5.0), (S::Water, 10.0)],
        outputs: &[(S::AgarAgar, 1.0)],
        byproducts: &[(S::Water, 12.0)],
        min_temp_c: 100,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 6.0,
        cross_recipe_group: None,
    },
    // --- Pectin ---
    // Source: fruit peels (citrus, apple)
    // Steps: acid extraction (HCl or HNO3, pH 1.5-3.0, 60-100C, 1-3h)
    //   -> filter -> precipitate with alcohol (ethanol/isopropanol) -> dry
    // Temperature: 70-100C, Duration: 3-6 hours
    // Enzymes: pectinase can be used for enzymatic extraction (milder conditions)
    Recipe {
        id: 1844,
        name: "Pectin Extraction",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[
            (S::FruitPeel, 5.0),
            (S::Water, 5.0),
            (S::HydrochloricAcid, 0.1),
        ],
        outputs: &[(S::Pectin, 1.0)],
        byproducts: &[(S::Water, 7.0)],
        min_temp_c: 85,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 4.0,
        cross_recipe_group: None,
    },
    // ===================================================================
    // BIOLOGICAL MATERIALS - ANIMAL SECRETIONS AND WAXES
    // ===================================================================

    // --- Shellac ---
    // Source: lac resin secreted by Kerria lacca (lac insect) on host trees
    // Steps: harvest encrusted twigs (sticklac) -> wash -> grind (seedlac) -> melt/filter
    //   (hand or machine process at 75-80C) -> stretch into thin sheets -> break into flakes
    // Temperature: 75-80C processing (melts at 75-85C, decomposes above 280C)
    // Duration: 4-8 hours processing
    Recipe {
        id: 1850,
        name: "Shellac Processing",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::LacInsectResin, 3.0), (S::Water, 2.0)],
        outputs: &[(S::Shellac, 1.0)],
        byproducts: &[(S::Water, 2.0), (S::IndigoDye, 0.05)],
        min_temp_c: 78,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 6.0,
        cross_recipe_group: None,
    },
    // --- Beeswax ---
    // Source: secreted by worker bees (Apis mellifera) from abdominal glands
    // Steps: melt combs in hot water (64-65C, melting point 62-64C) -> separate wax from honey
    //   -> filter -> cool into blocks. Solar wax melters also used.
    // Temperature: 65C, Duration: 2-4 hours
    Recipe {
        id: 1851,
        name: "Beeswax Rendering",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::Honey, 0.5), (S::Water, 2.0)],
        outputs: &[(S::Beeswax, 1.0)],
        byproducts: &[(S::Honey, 0.3), (S::Water, 1.0)],
        min_temp_c: 65,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: None,
    },
    // --- Lanolin ---
    // Source: sebaceous glands of sheep (in raw wool, 10-25% by weight)
    // Steps: scour raw wool in hot water + detergent -> centrifuge wash water
    //   -> separate lanolin from water (it's a wax ester, not a fat)
    //   -> refine/deodorize. Pharmaceutical grade requires further purification.
    // Temperature: 55-65C (scouring), Duration: 2-4 hours
    Recipe {
        id: 1852,
        name: "Lanolin Extraction (Wool Scouring)",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::RawWool, 5.0), (S::Water, 10.0), (S::SoapProduct, 0.2)],
        outputs: &[(S::Lanolin, 1.0)],
        byproducts: &[(S::WoolFiber, 3.5), (S::Water, 10.0)],
        min_temp_c: 60,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 3.0,
        cross_recipe_group: None,
    },
    // ===================================================================
    // BIOLOGICAL MATERIALS - NATURAL ADHESIVES
    // ===================================================================

    // --- Hide Glue ---
    // Source: collagen from animal hides and connective tissue (similar to gelatin but lower purity)
    // Steps: clean hide scraps -> soak in lime (weeks) -> wash -> cook in water at ~70C
    //   -> draw off glue liquor (repeat at higher temps) -> concentrate -> dry
    //   Applied hot at 60C. Glutin breaks down above 65C long-term exposure.
    // Temperature: 70C extraction, 60C application. Duration: 4-12 hours total
    Recipe {
        id: 1860,
        name: "Hide Glue Production",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::AnimalHide, 2.0), (S::Water, 5.0), (S::SlakedLite, 0.1)],
        outputs: &[(S::HideGlue, 1.0)],
        byproducts: &[(S::Water, 4.5)],
        min_temp_c: 70,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 48.0,
        cross_recipe_group: None,
    },
    // --- Casein Glue ---
    // Source: casein protein precipitated from milk with acid
    // Steps: heat milk to 60C -> add acid (vinegar or HCl) -> casein curds precipitate
    //   -> wash curds -> dry -> grind to powder -> mix with alkali (lime/borax) to dissolve
    //   Water-resistant when cured. Used for woodworking, labeling bottles.
    // Temperature: 60C for casein extraction, ambient for glue preparation
    Recipe {
        id: 1861,
        name: "Casein Glue Production",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::Milk, 5.0), (S::Vinegar, 0.1), (S::SlakedLite, 0.05)],
        outputs: &[(S::CaseinGlue, 1.0)],
        byproducts: &[(S::Whey, 3.5)],
        min_temp_c: 60,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 4.0,
        cross_recipe_group: None,
    },
    // --- Starch Paste ---
    // Source: starch from wheat, rice, corn, potato
    // Steps: mix starch with cold water -> heat while stirring (60-90C)
    //   -> gelatinization occurs (starch granules swell and burst, forming paste)
    //   Simple, cheap, reversible adhesive. Used for bookbinding, wallpaper, paper mache.
    // Temperature: 70-90C, Duration: 0.5-1 hour
    Recipe {
        id: 1862,
        name: "Starch Paste Adhesive",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::Starch, 0.3), (S::Water, 1.0)],
        outputs: &[(S::StarchPaste, 1.2)],
        byproducts: &[],
        min_temp_c: 80,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 0.5,
        cross_recipe_group: None,
    },
    // --- Birch Bark Tar ---
    // Source: birch tree bark (Betula spp.)
    // Process: dry distillation (pyrolysis). Bark heated in absence of oxygen (anoxic conditions).
    //   Triterpenoid compounds (betulin, lupeol) decompose into adhesive tar.
    //   Neanderthals produced this 200,000+ years ago.
    // Methods: condensation (above fire), pit roll (birch bark roll in fire pit), raised structure
    // Temperature: 300-400C, Duration: 2-6 hours
    Recipe {
        id: 1863,
        name: "Birch Bark Tar (Dry Distillation)",
        category: RecipeCategory::BiologicalMaterial,
        inputs: &[(S::BirchBark, 3.0)],
        outputs: &[(S::BirchBarkTar, 1.0)],
        byproducts: &[(S::Charcoal, 0.5), (S::CarbonDioxide, 0.3)],
        min_temp_c: 350,
        pressure_atm: 1.0,
        catalyst: None,
        duration_hours: 4.0,
        cross_recipe_group: None,
    },
];
