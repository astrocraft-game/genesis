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

    // === FERMENTED BEVERAGES (20) ===
    Mead, Sake, Cider, Perry, Pulque, Chicha, Kumiss, Ayran, Kvass,
    GingerBeer, Tepache, WaterKefir, MilkKefir, Kombucha, Jun,

    // === FERMENTED FOODS (20) ===
    Tempeh, Miso, Natto, Kimchi, Sauerkraut, Curtido, Tsukemono,
    DosaBatter, Idli, Injera, FermentedHotSauce, FermentedGarlic,
    Surstromming, Hakarl, ShrimpPaste,

    // === FERMENTED DAIRY (10) ===
    Kefir, Skyr, Labneh, CremeFraiche, CulturedButter, Buttermilk,

    // === CHEESE VARIETIES (20) ===
    CheddarCheese, GoudaCheese, BrieCheese, CamembertCheese,
    ParmesanCheese, MozzarellaCheese, RicottaCheese, FetaCheese,
    SwissCheese, BlueCheese, GruyereCheese, ManchegoCheese,
    RoquefortCheese, StiltonCheese, HalloumiCheese, Paneer,
    CottageCheese, CreamCheese, MascarponeCheese, ProvoloneCheese,

    // === OILS AND FATS (16) ===
    CoconutOil, PalmOil, SunflowerOil, RapeseedOil, SoybeanOil,
    PeanutOil, SesameOil, FlaxseedOil, WalnutOil, AvocadoOil,
    CornOil, CottonseedOil, CastorOil, JojobaOil, ArganOil,

    // === SPICE AND FLAVOR PRODUCTS (15) ===
    VanillaBean, Cinnamon, BlackPepper, WhitePepper, GreenPepper,
    CacaoBean, RoastedCacao, MapleSyrup, RefinedSugar,
    GreenTea, BlackTea, OolongTea, RoastedCoffee,

    // === TEXTILE RAW FIBERS (15) ===
    HempFiber, JuteFiber, SisalFiber, CoirFiber, RamieFiber,
    BambooFiber, KapokFiber,
    RayonFiber, ModalFiber, LyocellFiber, SpandexFiber,
    PolypropyleneFiber, AcrylicFiber,

    // === LEATHER AND HIDE (5) ===
    BrainTannedLeather, AlumTannedLeather, ChamoisLeather,
    SyntheticLeather, RawHide,

    // === PAPER AND PULP (10) ===
    SulfitePulp, MechanicalPulp, ThermoMechanicalPulp, RecycledPaper,
    RicePaper, HandmadePaper, Papyrus, BarkCloth,

    // === BIOLOGICAL MATERIALS (25) ===
    BoneMeal, BloodMeal, FishMeal, FeatherMeal,
    Gelatin, Isinglass, Carrageenan, AgarAgar, Pectin,
    Shellac, Beeswax, Lanolin,
    HideGlue, CaseinGlue, StarchPaste, BirchBarkTar,

    // === BIOLOGICAL INPUTS ===
    Milk, Cream, Whey, KojiMold, SCOBYCulture,
    KefirGrains, SoybeanRaw, RiceGrain, TeffGrain, CacaoFruit,
    CoffeeCherries, TeaLeaves, GreenCoffeeBeans,
    VanillaGreenPod, PineappleFruit, GingerRoot, MaresMilk,
    SugarcaneJuice, SugarBeet, MapleSap, FlaxStalk, HempStalk,
    JuteStalk, SilkCocoon, RawWool, RawCottonBoll,
    PepperDrupe, CinnamonBark, Seaweed, FruitPeel,
    AnimalBones, AnimalBlood, PoultryFeathers, FishOffal,
    AnimalHide, SwimBladder, LacInsectResin,
    BirchBark, CaseinProtein, Starch, WoodPulpRaw,
    ChromiumSulfate, BrainTissue,
    OilSeed, AgavePlant,

    // === ADVANCED MATERIALS ===
    CarbonNanotube, Graphene, Fullerene, QuantumDot, Aerogel, MOF,
    YBCO, MgB2Superconductor, NbTi,
    GalliumArsenide, GalliumNitride, SiliconCarbideSemi, IndiuPhosphide,
    CdTeSolar, CIGSSolar, PerovskiteSolar,
    SiliconNitrideCeramic, BoronCarbide, AluminumNitride, YSZ, PZT, BariumFerrite,
    Sapphire, OpticalFiber,
    CarbonFiber, CFRP, Kevlar, UHMWPE,
    Hydroxyapatite, BioactiveGlass, PLA, Chitosan,
    SiliconSolarCell, NMCCathode, SolidElectrolyte, MetalHydride, BiTe,
    Nanocellulose, SilverNanoparticle, GoldNanoparticle, TiO2Nano, IronOxideNano, SilicaNano,

    // === CHEMISTRY PRODUCTS ===
    SalicylicAcid, Crotonaldehyde, Cyclohexene, HDPE, SBRLatex, PVCBeads,
    Polyamide, Epoxide, Triazole, SilicaGelProduct,

    // === INORGANIC PRODUCTS (Extended) ===
    HydroxylRadical, Diol, Patina, SilverSulfide, CementGel, Zeolite,
    SilicaGel, WaterGlass, Potash, Borax, CalciumHypochlorite,
    CopperSulfate, IronSulfate, ZincSulfate, ChromeAlum,
    SodiumThiosulfate, SiliconTetrafluoride, Phosphine, Arsine, Diborane,
    Ferrocene, TitaniumIsopropoxide, AluminumIsopropoxide,

    // === NATURAL/GEO (Extended) ===
    Dolomite, Phosphorite, Chert, Lignite, Bituminite, Anthracite,
    Laterite, Saprolite, ManganeseNodule, BandedIronFormation,
    Guano, Amber, PetrifiedWood, Fossil, CoralReef, Pearl,
    Stromatolite, Biofilm, Humus, Peat, CalciumOxalite,
    Stalactite, Travertine, Tufa, Geode, Agate, DesertVarnish,
    RustScale, Saltpeter, Geyserite, IronPan, Calcrete,
}
