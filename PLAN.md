# Implementation Plan - Recipe/Crafting System

New `src/recipes/` module with 750+ real recipes organized by category. Each recipe has inputs, outputs, byproducts, conditions (temperature, pressure, catalyst). Cross-recipes provide multiple paths to the same output.

---

## Architecture

```rust
pub struct Recipe {
    pub id: u32,
    pub name: &'static str,
    pub category: RecipeCategory,
    pub inputs: Vec<(Substance, f32)>,       // (material, quantity in kg)
    pub outputs: Vec<(Substance, f32)>,      // primary outputs
    pub byproducts: Vec<(Substance, f32)>,   // waste/secondary outputs
    pub conditions: RecipeConditions,
}

pub struct RecipeConditions {
    pub min_temperature_c: i32,        // minimum temperature needed
    pub max_temperature_c: i32,        // max operating temp
    pub pressure_atm: f32,             // required pressure (1.0 = ambient)
    pub catalyst: Option<Substance>,   // catalyst needed (not consumed)
    pub duration_hours: f32,           // how long the process takes
}

pub enum RecipeCategory {
    Extraction,        // ore → element
    Alloying,          // metal + metal → alloy
    ChemicalSynthesis, // compounds → new compounds
    Refining,          // crude → pure
    Construction,      // raw materials → building materials
    FuelProcessing,    // energy carriers
    FoodBiological,    // fermentation, cooking, textiles
    Manufacturing,     // advanced products
    PhaseChange,       // melting, boiling, condensing
    Recycling,         // product → partial inputs recovery
}

pub enum Substance {
    // ~200 substances covering elements, ores, compounds, alloys, products
    // Organized by: Elements, Ores, Metals, Alloys, Chemicals, Fuels,
    // Construction, Food, Textiles, Advanced
}
```

---

## 1. Substance Enum (~200 entries)

**Elements (30):**
Iron, Copper, Gold, Silver, Platinum, Tin, Lead, Zinc, Nickel, Cobalt, Chromium, Manganese, Tungsten, Molybdenum, Vanadium, Titanium, Aluminum, Silicon, Carbon, Sulfur, Phosphorus, Nitrogen, Oxygen, Hydrogen, Uranium, Lithium, Magnesium, Sodium, Calcium, Mercury

**Ores (30):**
Hematite, Magnetite, Chalcopyrite, Galena, Sphalerite, Cassiterite, Bauxite, Chromite, Pentlandite, Rutile, Ilmenite, Wolframite, Scheelite, Molybdenite, Cinnabar, Monazite, Uraninite, Spodumene, PyritOre, CopperOxideOre, GoldOre, SilverOre, Cobaltite, Stibnite, Limestone, SilicaSand,ite, Feldspar, Gypsum, PhosphateRock

**Alloys (25):**
LowCarbonSteel, MediumCarbonSteel, HighCarbonSteel, StainlessSteel304, StainlessSteel316, CastIron, ToolSteel, TinBronze, PhosphorBronze, AluminumBronze, Brass, CuproNickel, BerylliumCopper, Duralumin, Inconel, Monel, Nichrome, TiAlloy6Al4V, SterlingSilver, Electrum, Solder, Pewter, Ferromanganese, Ferrochrome, TungstenCarbide

**Chemicals (30):**
SulfuricAcid, NitricAcid, HydrochloricAcid, Ammonia, SodiumHydroxide, ChlorineGas, SodaAsh, Ethanol, Methanol, Acetone, Benzene, Toluene, Polyethylene, Nylon, PVC, Glycerol, Soap, Bleach, Gunpowder, TNT, Dynamite, Nitroglycerin, CalciumCarbide, Acetylene, PhosphoricAcid, AmmoniumNitrate, Urea, SyntheticRubber, Formaldehyde, HydrogenPeroxide

**Fuels (15):**
Charcoal, Coke, Gasoline, Diesel, Kerosene, FuelOil, NaturalGas, HydrogenGas, Biodiesel, Biogas, Bitumen, RocketFuelLOXRP1, RocketFuelLOXLH2, NuclearFuelRod, SolidRocketPropellant

**Construction Materials (20):**
PortlandCement, Concrete, Brick, FireBrick, AdobeBrick, LimeMortar, CementMortar, Plaster, Quicklime, SlakedLime, Glass, BorosilicateGlass, Asphalt, Plywood, MDF, Fiberglass, RockWool, ReinforcedConcrete, Stucco, Porcelain

**Food/Bio Products (20):**
Flour, Bread, Beer, Wine, Cheese, Butter, Yogurt, Sugar, Salt, Vinegar, OliveOil, SoyaSauce, FishSauce, DriedMeat, PickledVegetables, Spirits, Honey, Chocolate, Coffee, Tea

**Textiles/Organic (15):**
CottonFiber, WoolFiber, SilkFiber, LinenFiber, Leather, NylonFiber, PolyesterFiber, Paper, ActivatedCarbon, NaturalRubber, VulcanizedRubber, IndigoDye, Tannin, Lye, Alum

**Intermediate Products (15):**
PigIron, WroughtIron, BlisterSteel, CopperMatte, BlisterCopper, Alumina, Clinker, SynGas, CrudeOil, Slag, Tailings, RedMud, FlyAsh, Yellowcake, TitaniumTetrachloride

---

## 2. Extraction Recipes (~150)

### Iron (6 paths)
- Bloomery: Hematite + Charcoal → WroughtIron + Slag (1200°C)
- Blast Furnace: Hematite + Coke + Limestone → PigIron + Slag (1500°C)
- Blast Furnace from Magnetite: Magnetite + Coke + Limestone → PigIron + Slag (1500°C)
- Direct Reduction: Hematite + HydrogenGas → Iron + Water (900°C)
- Siderite calcination: Siderite → IronOxide + CO2 (500°C) then standard reduction
- Pyrite roasting: PyritOre → Hematite + SulfurDioxide (700°C) then standard smelting

### Copper (5 paths)
- Ancient smelting: CopperOxideOre + Charcoal → Copper + Slag (1200°C)
- Flash smelting: Chalcopyrite + Oxygen → CopperMatte + Slag + SO2 (1300°C)
- Converting: CopperMatte + Oxygen → BlisterCopper + SO2 (1250°C)
- Electrorefining: BlisterCopper → Copper (99.99%) + AnodeSlimes (60°C, electricity)
- Heap leaching: CopperOxideOre + SulfuricAcid → CopperSulfate → Copper (ambient)

### Gold (4 paths)
- Gravity panning: GoldOre → Gold (ambient, water)
- Mercury amalgamation: GoldOre + Mercury → GoldAmalgam → Gold + Mercury (350°C)
- Cyanidation: GoldOre + SodiumCyanide + Oxygen → GoldCyanide → Gold (ambient)
- Aqua regia: Gold + HCl + NitricAcid → GoldChloride → Gold (80°C)

### Silver (3 paths)
- Cupellation: SilverOre + Lead → LeadSilverAlloy → Silver (1000°C)
- Cyanidation: SilverOre + SodiumCyanide → SilverCyanide → Silver (ambient)
- Electrolytic: CrudeGold → Silver (from anode slimes, 35°C)

### Aluminum (2 paths)
- Bayer+Hall-Héroult: Bauxite + NaOH → Alumina + RedMud (250°C); Alumina + Cryolite → Aluminum (960°C, electricity)
- Direct: Bauxite → Alumina → Aluminum (two-step)

### Tin: Cassiterite + Carbon → Tin + CO (1200°C)
### Lead: Galena + Oxygen → LeadOxide + SO2 (800°C); LeadOxide + Carbon → Lead (900°C)
### Zinc (3 paths): Roast-Leach-Electrowin, Imperial Smelting, Retort distillation
### Titanium: Rutile + Chlorine + Carbon → TiCl4 (900°C); TiCl4 + Magnesium → Titanium (850°C)
### Chromium: Chromite + Carbon → Ferrochrome (1600°C); Chromite + Aluminum → Chromium (aluminothermic, 2500°C)
### Nickel (3 paths): Flash smelting, HPAL, Mond carbonyl
### Cobalt: From copper-cobalt ore via acid leaching
### Manganese: Pyrolusite + Carbon → Ferromanganese (1400°C)
### Tungsten: Wolframite → APT → WO3 + H2 → Tungsten (1000°C)
### Molybdenum: Molybdenite roasting → MoO3 + H2 → Molybdenum (1000°C)
### Uranium: Uraninite + H2SO4 → Yellowcake → UF6 → enriched UO2 (multi-step)
### Lithium (2 paths): Spodumene acid roast, Brine solar evaporation
### Magnesium (2 paths): Pidgeon silicothermic, Dow electrolytic
### Silicon: SilicaSand + Carbon → Silicon (1900°C)
### Mercury: Cinnabar + Oxygen → Mercury + SO2 (400°C)
### Rare Earths: Monazite + NaOH → REE hydroxides (150°C)
### Platinum Group: From nickel-copper anode slimes, sequential precipitation

**Plus dozens more for: Antimony, Bismuth, Cadmium, Indium, Gallium, Germanium, Beryllium, Zirconium, Niobium, Tantalum, Rhenium, Selenium, Tellurium**

---

## 3. Alloy Recipes (~80)

Each alloy is a recipe: inputs are metals at specific ratios.

**Steels (20):** LowCarbon, MediumCarbon, HighCarbon, SS304, SS316, SS430, CastIron, DuctileIron, Hadfield, Chromoly, SiliconSteel, WeatheringSteel, ToolSteel(M2,D2,H13,O1,W1,S7), Damascus/Crucible

**Copper alloys (10):** TinBronze, PhosphorBronze, AlBronze, SiBronze, Brass, NavalBrass, CartridgeBrass, CuproNickel, BeCu, Gunmetal

**Aluminum alloys (5):** Duralumin, Magnalium, AlSiCasting, 7075, AlLi

**Nickel alloys (5):** Inconel625, Inconel718, Monel400, Nichrome, Hastelloy

**Titanium alloys (2):** Ti6Al4V, Nitinol

**Precious (5):** SterlingSilver, 18KGold, WhiteGold, RoseGold, Electrum

**Other (8):** Solder, LeadFreeSolder, Pewter, Stellite, TungstenCarbide, WoodsMetal, TypeMetal, BabbittMetal

---

## 4. Chemical Synthesis (~100)

- Haber: N2 + 3H2 → 2NH3 (450°C, 200atm, Fe catalyst)
- Contact: SO2 + O2 → SO3 → H2SO4 (450°C, V2O5 catalyst)
- Ostwald: NH3 + O2 → NO → NO2 → HNO3 (900°C, Pt-Rh catalyst)
- Solvay: NaCl + CaCO3 → Na2CO3 + CaCl2 (multi-step)
- Chloralkali: NaCl + H2O → NaOH + Cl2 + H2 (electricity)
- Fischer-Tropsch: CO + H2 → hydrocarbons (200-350°C, Fe/Co catalyst)
- Polyethylene: ethylene → PE (150-300°C, 1000-3000atm / or Ziegler-Natta)
- Nylon: adipic acid + hexamethylenediamine → nylon66 (280°C)
- PVC: vinyl chloride → PVC (50°C)
- Vulcanization: rubber + sulfur → vulcanized rubber (150°C)
- Soap: fat + NaOH → soap + glycerol (40°C)
- Bleach: Cl2 + NaOH → NaOCl (ambient)
- Gunpowder: KNO3 + charcoal + sulfur → gunpowder (ambient mixing)
- TNT: toluene + HNO3/H2SO4 → TNT (30-100°C)
- Dynamite: nitroglycerin + diatomaceous earth → dynamite
- Nitroglycerin: glycerol + HNO3 + H2SO4 → nitroglycerin (<10°C)
- ANFO: ammonium nitrate + fuel oil → ANFO (ambient)
- Urea: NH3 + CO2 → urea (180°C, 200atm)
- Ammonium nitrate: NH3 + HNO3 → NH4NO3
- Calcium carbide: CaO + carbon → CaC2 (2000°C, arc furnace)
- Acetylene: CaC2 + H2O → C2H2 + Ca(OH)2 (ambient)
- Aspirin: salicylic acid + acetic anhydride → aspirin (85°C)
- Penicillin: fermentation of Penicillium (26°C, 5-7 days)
- Synthetic indigo: aniline synthesis chain
- Phosphorus: phosphate rock + SiO2 + C → P4 (1500°C)

**Plus ~75 more from the research data**

---

## 5. Construction Material Recipes (~60)

- Portland cement: limestone + clay → clinker (1450°C) + gypsum → cement
- Concrete: cement + sand + gravel + water (ambient)
- Reinforced concrete: concrete + steel rebar
- Roman concrete: quicklime + volcanic ash + seawater
- Fired brick: clay → brick (1000°C)
- Adobe: clay + sand + straw + water (sun-dried)
- Fire brick: high-alumina clay (1600°C)
- Quicklime: limestone → CaO + CO2 (950°C)
- Slaked lime: CaO + H2O → Ca(OH)2 (exothermic)
- Lime mortar: slaked lime + sand
- Cement mortar: cement + sand + water
- Gypsum plaster: gypsum → plaster of Paris (160°C)
- Glass: SiO2 + Na2CO3 + CaCO3 → glass (1500°C)
- Borosilicate glass: SiO2 + B2O3 + Na2O (1650°C)
- Porcelain: kaolin + feldspar + quartz (1300°C)
- Stoneware: stoneware clay (1250°C)
- Earthenware: common clay (1000°C)
- Asphalt: aggregate + bitumen (160°C)
- Plywood: wood veneer + phenol resin (150°C press)
- MDF: wood fiber + UF resin (190°C press)
- Fiberglass: glass melt → spun fibers (1450°C)
- Rock wool: basalt melt → spun fibers (1500°C)

**Plus ~38 more variants**

---

## 6. Fuel/Energy Recipes (~40)

- Charcoal: wood → charcoal + wood gas (450°C, no air)
- Coke: bituminous coal → coke + coal tar + ammonia (1100°C, no air)
- Petroleum distillation: crude oil → gasoline + diesel + kerosene + fuel oil + bitumen (350°C)
- Catalytic cracking: heavy oil → gasoline + light gases (500°C, zeolite catalyst)
- Biodiesel: vegetable oil + methanol + NaOH → biodiesel + glycerol (55°C)
- Ethanol: sugar + yeast → ethanol + CO2 (30°C, 48h)
- Hydrogen electrolysis: H2O → H2 + O2 (80°C, electricity)
- Steam reforming: CH4 + H2O → CO + 3H2 (850°C, Ni catalyst)
- Water-gas shift: CO + H2O → CO2 + H2 (350°C, Fe catalyst)
- Biogas: organic waste → CH4 + CO2 (37°C, 30 days)
- Producer gas: coal + limited air → CO + H2 + N2 (1000°C)
- Nuclear fuel: UO2 pellets → fuel rods + zirconium cladding (1700°C sinter)
- Solid propellant: NH4ClO4 + Al powder + binder → APCP (60°C cure)

**Plus ~27 more**

---

## 7. Food/Biological Recipes (~80)

- Bread: flour + water + yeast + salt → bread (240°C bake)
- Sourdough: flour + water + starter → sourdough (245°C)
- Beer: barley + water + hops + yeast → beer + CO2 (20°C, 14 days)
- Wine: grapes + yeast → wine + CO2 (25°C, 14 days)
- Spirits: fermented wash → distilled spirit (78°C)
- Cheese: milk + rennet + culture → cheese + whey (32°C, months aging)
- Butter: cream → butter + buttermilk (12°C churning)
- Yogurt: milk + culture → yogurt (44°C, 6h)
- Vinegar: ethanol + Acetobacter → acetic acid (28°C, weeks)
- Sugar: sugarcane → juice → raw sugar (80°C evaporation)
- Salt: seawater → salt (solar evaporation, weeks)
- Olive oil: olives → oil + pomace (cold press, <27°C)
- Soap: fat + lye → soap + glycerol (40°C)
- Leather (vegetable): rawhide + tannin → leather (ambient, 45 days)
- Leather (chrome): rawhide + Cr2(SO4)3 → leather (30°C, 2 days)
- Cotton fabric: cotton bolls → ginned fiber → spun yarn → woven fabric
- Wool fabric: fleece → scoured → carded → spun → woven
- Silk: cocoons → reeled filament → thrown yarn → woven (100°C stifling)
- Linen: flax → retted → broken → hackled → spun → woven
- Paper (rag): cotton rags → beaten pulp → sheet (ambient)
- Paper (wood): wood chips + NaOH + Na2S → cellulose pulp (170°C)
- Charcoal activation: charcoal + steam → activated carbon (1000°C)
- Composting: organic waste → humus (55°C, 8 weeks)
- Indigo: indigo plant → fermented → oxidized → pigment
- Cochineal: insects → dried → ground → carmine dye
- Fish sauce: fish + salt → fermented sauce (35°C, 3 months)
- Soy sauce: soybeans + wheat + koji mold → fermented → pressed (ambient, 12 months)

**Plus ~53 more**

---

## 8. Phase Change Recipes (~30)

Every substance with defined melting/boiling points becomes a recipe:
- Ice → Water (0°C)
- Water → Steam (100°C)
- Iron ore sinter: powder → sintered pellets (1300°C)
- Glass annealing: molten glass → solid glass (slow cool through 550°C)
- Steel quenching: hot steel → hardened steel (rapid cool in water/oil)
- Steel tempering: hardened steel → tempered steel (200-600°C, controlled)
- Freeze-drying: frozen material + vacuum → dried material (-40°C, low pressure)
- Sublimation: dry ice → CO2 gas (-78°C at 1atm)
- Condensation: steam → water (cooling below 100°C)
- Fractional crystallization: mixed salt solution → pure crystals (controlled cooling)

**Plus ~20 more**

---

## 9. Cross-Recipes (alternate paths - marked in recipe data)

Every output that has multiple recipes gets a `cross_recipe_group: Option<u32>` linking them:

- Iron: 6 different extraction paths
- Copper: 5 different paths
- Gold: 4 different paths
- Steel: BOF, EAF, cementation, crucible
- Hydrogen: electrolysis, steam reforming, water-gas shift
- Sulfuric acid: contact process from sulfur OR from pyrite roasting
- Glass: soda-lime, borosilicate, lead crystal
- Paper: rag, wood pulp (kraft), wood pulp (sulfite)
- Ethanol: grain fermentation, sugar fermentation, wood hydrolysis
- Soap: cold process, hot process, industrial
- Cement: Portland, pozzolanic, slag, Roman

**~100+ cross-recipe pairs**

---

## 10. Byproducts on Every Recipe

Every recipe lists byproducts:
- Blast furnace iron: **Slag** (calcium silicate)
- Copper smelting: **SO2 gas** (captured for sulfuric acid)
- Aluminum Bayer: **Red mud** (iron oxide waste)
- Steel BOF: **Slag** + CO2
- Petroleum refining: **each fraction** is a byproduct of the others
- Cement: **CO2** (enormous - 8% of world emissions)
- Charcoal: **wood vinegar** + tar + gases
- Coke: **coal tar** + ammonia + benzene + coal gas
- Beer: **spent grain** (animal feed) + CO2
- Cheese: **whey** (used for ricotta, protein)
- Copper electrorefining: **anode slimes** (contain gold, silver, platinum)
- Paper kraft: **black liquor** (burned for energy recovery)
- Sugar: **molasses** + **bagasse** (fuel)

---

## Implementation Order

- [x] 1. Substance enum (200 variants) - DONE
- [x] 2. Recipe struct + RecipeCategory enum - DONE
- [x] 3. extraction.rs (45 recipes: iron 6 paths, copper 5, gold 4, silver 3, tin, lead, zinc, aluminum, titanium, chromium, nickel, manganese, tungsten, silicon, mercury, uranium, lithium, magnesium, cobalt, molybdenum, rare earths, phosphorus, sulfur) - DONE
- [x] 4. alloys.rs (24 recipes: 7 steels, 5 copper alloys, duralumin, 3 Ni-alloys, Ti-6Al-4V, sterling silver, electrum, solder, pewter, ferrochrome, ferromanganese, tungsten carbide) - DONE
- [x] 5. chemistry.rs (30 recipes: Haber, Contact, Ostwald, Solvay, chloralkali, Fischer-Tropsch, steam reforming, electrolysis, polyethylene, nylon, PVC, synthetic rubber, gunpowder, nitroglycerin, dynamite, TNT, ammonium nitrate, urea, calcium carbide, acetylene, bleach, phosphoric/hydrochloric acid, H2O2, formaldehyde, soap 2 paths, vulcanization) - DONE
- [x] 6. construction.rs (21 recipes: quicklime, slaked lime, 2 cement, 2 concrete, 3 brick, 2 mortar, 2 plaster, 2 glass, porcelain, asphalt, plywood, MDF, 2 insulation) - DONE
- [x] 7. fuel.rs (9 recipes: charcoal, coke, petroleum distillation, biodiesel, ethanol, biogas, activated carbon, LOX, LH2) - DONE
- [x] 8. biological.rs (30 recipes: 2 bread, beer, wine, spirits, 3 dairy, sugar, 2 salt, vinegar, 2 preservation, fish/soy sauce, olive oil, 5 textile, 2 leather, 2 paper, indigo, lye, alum, composting) - DONE
- [x] 9. phase_change.rs (20 recipes: water states, metal heat treatment, glass tempering, casting, crystallization, distillation) - DONE
- [x] 10. Cross-recipe index and lookup functions - DONE (13 cross-recipe groups)
- [x] 11. Tests (10 tests: total count, cross-recipes, byproducts, unique IDs, per-category counts) - DONE
- [x] 12. Wired into lib.rs - DONE

**Current: 265 recipes across 7 files. Phase change module added.**

Breakdown:
- extraction.rs: 70 recipes (iron 6 paths, copper 6, gold 5, silver 3, tin, lead 2, zinc 3, aluminum 2, titanium 2, chromium 2, nickel 3, manganese, tungsten 2, silicon 2, mercury, uranium 2, lithium 2, magnesium 2, cobalt, molybdenum, vanadium, antimony 2, beryllium, zirconium, niobium, tantalum, cadmium, indium, gallium, germanium, selenium, tellurium, rhenium, platinum, sodium, calcium, rare earths, phosphorus, sulfur)
- alloys.rs: 48 recipes (14 steels, 10 copper alloys, 3 aluminum, 4 nickel, 1 titanium, 5 precious, 11 other)
- chemistry.rs: 48 recipes (major industrial + polymers + explosives + pharma + semiconductor + water treatment + fuels)
- construction.rs: 32 recipes (lime, cement, concrete, brick, mortar, plaster, glass, ceramics, asphalt, engineered wood, insulation)
- fuel.rs: 17 recipes (charcoal, coke, petroleum, biodiesel, ethanol, biogas, activated carbon, cryogenics, cracking, gasification, propellants)
- biological.rs: 30 recipes (bread, beer, wine, spirits, dairy, sugar, salt, vinegar, preservation, textiles, leather, paper, dyes, composting)
- phase_change.rs: 20 recipes (water states, heat treatment, glass processing, crystallization, casting, distillation)
