#![allow(dead_code)]
use crate::internal::*;

/// ~200 real substances: elements, ores, alloys, chemicals, fuels, construction, food, textiles.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum Substance {
    // === ELEMENTS (30) ===
    Iron, Copper, Gold, Silver, Platinum, Tin, Lead, Zinc, Nickel, Cobalt,
    Chromium, Manganese, Tungsten, Molybdenum, Vanadium, Titanium, Aluminum,
    Silicon, Carbon, Sulfur, Phosphorus, Nitrogen, Oxygen, Hydrogen, Uranium,
    Lithium, Magnesium, Sodium, Calcium, Mercury, Potassium, Thorium, Argon,

    // === ORES (30) ===
    Hematite, Magnetite, Chalcopyrite, Galena, Sphalerite, Cassiterite, Bauxite,
    Chromite, Pentlandite, Rutile, Ilmenite, Wolframite, Scheelite, MolybdeniteOre,
    Cinnabar, Monazite, Uraninite, Spodumene, PyriteOre, CopperOxideOre, GoldOre,
    SilverOre, CobaltiteOre, StibniteOre, Limestone, SilicaSand, Clite, FeldsparOre,
    ite, PhosphateRock,

    // === ALLOYS (25) ===
    LowCarbonSteel, MediumCarbonSteel, HighCarbonSteel, StainlessSteel304,
    StainlessSteel316, CastIron, ToolSteel, TinBronze, PhosphorBronze,
    AluminumBronze, Brass, CuproNickel, BerylliumCopper, Duralumin, Inconel,
    Monel, Nichrome, TiAlloy, SterlingSilver, Electrum, Solder, Pewter,
    Ferromanganese, Ferrochrome, TungstenCarbide,

    // === CHEMICALS (30) ===
    SulfuricAcid, NitricAcid, HydrochloricAcid, Ammonia, SodiumHydroxide,
    ChlorineGas, SodaAsh, Ethanol, Methanol, Acetone, Benzene, Toluene,
    Polyethylene, NylonResin, PVC, Glycerol, SoapProduct, BleachSolution,
    Gunpowder, TNT, Dynamite, Nitroglycerin, CalciumCarbide, Acetylene,
    PhosphoricAcid, AmmoniumNitrate, Urea, SyntheticRubber, Formaldehyde,
    HydrogenPeroxide,

    // === FUELS (15) ===
    Charcoal, Coke, Gasoline, Diesel, Kerosene, FuelOil, NaturalGas,
    HydrogenGas, Biodiesel, Biogas, Bitumen, RocketFuelRP1, LiquidOxygen,
    LiquidHydrogen, NuclearFuelRod,

    // === CONSTRUCTION (20) ===
    PortlandCite, Concrete, Brick, FireBrick, AdobeBrick, LimeMortar,
    CementMortar, PlasterOfParis, Quicklite, SlakedLite, Glass,
    BorosilicateGlass, Asphalt, Plywood, MDF, Fiberglass, RockWool,
    ReinforcedConcrete, Stucco, Porcelain,

    // === FOOD/BIO (20) ===
    Flour, Bread, Beer, Wine, Cheese, Butter, Yogurt, Sugar, Salt, Vinegar,
    OliveOil, SoySauce, FishSauce, DriedMeat, Pickles, Spirits, Chocolate,
    Coffee, Tea, Honey,

    // === TEXTILES/ORGANIC (15) ===
    CottonFiber, WoolFiber, SilkFiber, LinenFiber, LeatherProduct,
    NylonFiber, PolyesterFiber, PaperProduct, ActivatedCarbon, NaturalRubber,
    VulcanizedRubber, IndigoDye, Tannin, LyeSolution, AlumCompound,

    // === INTERMEDIATES (15) ===
    PigIron, WroughtIron, BlisterSteel, CopperMatte, BlisterCopper, Alumina,
    Clinker, SynGas, CrudeOil, Slag, Tailings, RedMud, FlyAsh, Yellowcake,
    TitaniumTetrachloride,

    // === MISC INPUTS ===
    Water, Steam, Air, Clay, Sand, Gravel, WoodLogs, WoodChips, StrawFiber,
    Yeast, RennetEnzyme, StarterCulture, Cryolite, DiatomaceousEarth,
    SodiumCyanide, Flux,

    // === ORGANIC CHEMISTRY ===
    Ethylene, Propylene, Butadiene, Xylene, Naphthalene,
    Isopropanol, Butanol, EthyleneGlycol, EthyleneOxide, PropyleneOxide,
    AceticAcid, FormicAcid, CitricAcid, AdipicAcid, TerephthalicAcid,
    EthylAcetate, MethylMethacrylate,
    Aniline, Caprolactam, Acrylonitrile, HydrogenCyanide, Melamine,
    DiethylEther, MTBE, VinylChloride, Chloroform,
    Phenol, Styrene, Acetaldehyde,
    OxalicAcid, MethylEthylKetone, SecButanol, SodiumFormate,
    CarbonTetrachloride, Freon12, Tetrafluoroethylene, PTFE, Polypropylene,
    Nitrobenzene, Cyclohexanone, Ethylbenzene,
    MethylChloride, Dichloromethane, AllylChloride, Epichlorohydrin,
    AcrylicAcid, MaleicAnhydride, PhthalicAnhydride, Ethylhexanol,
    VinylAcetate, DimethylTerephthalate,

    // === INORGANIC PRODUCTS ===
    SodiumChloride, SodiumSulfate, PotassiumNitrate, CalciumChloride,
    BariumSulfate, SilverChloride, IronHydroxide, LeadIodide,
    CopperHydroxide, MagnesiumHydroxide,
    PrussianBlue, IronOxideRed, MagnesiumOxide,

    // === ELECTROCHEMISTRY ===
    LeadAcidBattery, LithiumIonBattery, ZincCarbonBattery,
    NiMHBattery, SodiumSulfurBattery, IronAirBattery, LFPBattery, VanadiumFlowBattery,
    ChromePlating, GoldPlating, NickelPlating, ZincPlating,
    SilverPlating, TinPlating, CopperPlating,

    // === PIGMENTS ===
    YellowOchre, RedOchre, RawUmber, BurntUmber, RawSienna, BurntSienna,
    CarbonBlack, BoneBlack, ChromeYellow, CobaltBluePigment,
    SyntheticUltramarine, TitaniumWhite, EgyptianBlue, Vermilion,
    LeadWhite, NaplesYellow, ChromeGreen, CadmiumYellow, CadmiumRed,
    Verdigris, ZincWhite, MarsYellow, MarsRed, MarsBlack,

    // === NATURAL/GEO ===
    Ite, Marble, Slate, Quartzite, SerpentineMinite,
    Ozone, NitrogenDioxide, SulfurDioxide, CarbonDioxide,
    Glucose, LacticAcid, Cellulose, Chitin,
    Nitrate, Ammonium, Methane,
}
