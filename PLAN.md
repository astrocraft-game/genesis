# Implementation Plan

Detailed plan for completing the remaining roadmap items from README.md. Organized by priority and dependency order.

---

## 1. Galaxy Generation - Names

**Status:** Galaxy names hardcoded as `"Galaxy"` in `src/galaxy/generator.rs:49`

**What to do:**
- Implement a name generator for procedurally-created galaxies
- Use a **syllable-based concatenation** approach (simplest, fits the existing pattern)
  - Define syllable pools: prefixes (`And`, `Gal`, `Neb`, `Vir`), middles (`ro`, `ax`, `el`), suffixes (`ia`, `us`, `eda`, `on`)
  - Concatenate 2-4 syllables using `SeededDiceRoller`
  - Optionally prepend catalog-style designators (`NGC`, `IC`, `UGC`) for "hard sci-fi" feel
- Add `name_style: GalaxyNameStyle` to `GalaxySettings` (Catalog, Fantastical, Mixed)
- Integrate into `Galaxy::generate()` after category is determined

**Files to modify:**
- `src/galaxy/generator.rs` - add `generate_name()` function
- `src/galaxy/types.rs` - add `GalaxyNameStyle` enum if needed
- New file: `src/galaxy/constants.rs` - add syllable pools (or extend existing constants)

**Seeding:** `SeededDiceRoller::new(&seed, &format!("gal_{}_name", index))`

---

## 2. Galaxy Generation - Our Local Group Galaxies

**Status:** `LOCAL_GROUP_GALAXIES` in `src/galaxy/constants.rs` has 38 entries but indices 3-37 are placeholders with `name: "TODO"`

**What to do:**
- Populate all 38 entries with real local group galaxy data:
  - Names: Sagittarius Dwarf, Large/Small Magellanic Cloud, Ursa Minor Dwarf, Draco Dwarf, Carina Dwarf, Sextans Dwarf, Sculptor Dwarf, Fornax Dwarf, Leo I, Leo II, NGC 6822 (Barnard's Galaxy), NGC 185, NGC 147, IC 10, IC 1613, Phoenix Dwarf, Cetus Dwarf, Tucana Dwarf, Pegasus Dwarf, Aquarius Dwarf, Sagittarius Dwarf Irregular, Leo A, Pisces Dwarf, Antlia Dwarf, Wolf-Lundmark-Melotte, etc.
  - Real ages, categories (mostly `Irregular` or `DwarfElliptical`), subcategories, special traits
- Source data from NASA/IPAC Extragalactic Database entries for the Local Group

**Files to modify:**
- `src/galaxy/constants.rs` - fill in `LOCAL_GROUP_GALAXIES` array entries 3-37

---

## 3. Sector Generation - Region Mapping

**Status:** `generate_region()` in `src/galaxy/map/division/generator.rs:75` is a placeholder returning only `Ellipse` or `Void`

### 3a. Temporary Region Mapping

**What to do:**
- Implement basic region assignment using distance-from-center heuristics:
  - **Core/Nucleus:** < 5% of galaxy radius
  - **Bulge:** 5-15% of radius (for spirals/lenticulars)
  - **Bar:** Within bar dimensions (for `BarredSpiral`), elongated ellipsoid check
  - **Arm:** Use logarithmic spiral equation `r = a * e^(b*theta)` to check if position is near a spiral arm (for `Spiral` subcategories). Number of arms from galaxy parameters
  - **Disk:** Within disk height but not in arm/bulge/bar (for spirals/lenticulars)
  - **Ellipse:** Within ellipsoidal boundary (for `Elliptical` categories)
  - **Halo:** Beyond main body but within 2x radius
  - **Aura:** Outskirts of `Irregular` galaxies
  - **Void:** Outside galaxy bounds
- Use the galaxy's `GalaxyCategory` size parameters (already stored as `(u32, u32)` or `(u32, u32, u32)`) to define boundaries
- Use `SpaceCoordinates` distance calculations already available

**Files to modify:**
- `src/galaxy/map/division/generator.rs` - rewrite `generate_region()`

**Seeding:** `SeededDiceRoller::new(&seed, &format!("div_{}_{}_reg", level, index))`

### 3b. Proper Region Mapping

**What to do (after temporary mapping works):**
- Add probabilistic boundaries instead of hard cutoffs (fuzzy transitions between regions)
- Add special region types:
  - **GlobularCluster:** Rare, seeded randomly in halo regions
  - **OpenCluster:** In disk/arm regions, young star-forming areas
  - **Association:** Loose groupings in arms
  - **Stream:** Tidal streams from satellite galaxies (connect to `GalaxySpecialTrait::Interacting` or `Tail`)
  - **Exile:** Rare isolated stars far from any region
- Factor in `GalaxySpecialTrait` modifiers:
  - `Compact(u8)` shrinks all boundaries
  - `Expansive(u8)` expands all boundaries
  - `ExtendedHalo` enlarges halo region
  - `Dead`/`Dormant` reduces arm prominence
  - `Starburst` increases cluster frequency

**Files to modify:**
- `src/galaxy/map/division/generator.rs` - enhance `generate_region()`
- Possibly `src/galaxy/map/types.rs` - add region metadata structs

---

## 4. Sector Generation - Names

**Status:** Division names default to `"default"` in `src/galaxy/map/division/mod.rs`

**What to do:**
- Generate names based on `GalacticRegion` type:
  - **Named regions** (Arm, Bulge, Bar): use thematic name pools (e.g., "Orion Arm", "Perseus Arm")
  - **Numbered sectors**: catalog-style designation `{Region}-{Level}-{X}.{Y}.{Z}` (e.g., "Arm-2-15.7.0")
  - **Special names** for clusters: draw from a name generator
- Lower-level divisions inherit parent region name as prefix
- Add `fixed_name: Option<Rc<str>>` to settings for user overrides

**Files to modify:**
- `src/galaxy/map/division/generator.rs` - add name generation
- `src/galaxy/map/division/mod.rs` - use generated name instead of "default"

---

## 5. Star System - Subdwarfs

**Status:** `StarLuminosityClass::VI` exists. `StellarEvolution::Subdwarf` (Population II) exists. Luminosity multiplier at `src/system/star/generator.rs:544` is `1.25x` which is inverted for cool subdwarfs.

**What to do:**
- Fix luminosity multiplier for cool subdwarfs (sdF through sdM): should be ~0.3-0.6x, not 1.25x
- Keep higher luminosity for hot subdwarfs (sdO, sdB) which are core-helium-burning evolved stars
- Split the logic by spectral type:
  - `sdO/sdB`: luminosity 1.0-10x (these are unusual evolved objects, not just metal-poor main sequence)
  - `sdF/sdG/sdK`: luminosity 0.4-0.6x, radius 0.75-0.90x
  - `sdM/sdL`: luminosity 0.2-0.5x, radius 0.7-0.85x
- Add proper temperature adjustment: subdwarfs are slightly hotter for the same color (shift spectral subtype by -1 to -2)
- Ensure planets around subdwarfs have appropriately shifted habitable zones

**Files to modify:**
- `src/system/star/generator.rs` - fix luminosity/radius multipliers in the `Subdwarf` branch (~line 540)

---

## 6. Star System - Configurable Stars

**Status:** `StarSettings` in `src/system/star/types.rs` has `fixed_age`, `fixed_mass`, `use_ours`, and spectral type generation chances. Limited configuration.

**What to do:**
- Add to `StarSettings`:
  - `fixed_spectral_type: Option<StarSpectralType>`
  - `fixed_luminosity_class: Option<StarLuminosityClass>`
  - `fixed_population: Option<StellarEvolution>`
  - `fixed_special_traits: Option<Vec<StarPeculiarity>>`
  - `forbidden_special_traits: Option<Vec<StarPeculiarity>>`
  - `min_stars: Option<u8>`, `max_stars: Option<u8>` - control multiplicity
- Respect these in `Star::generate()` by checking fixed values before rolling

**Files to modify:**
- `src/system/star/types.rs` - extend `StarSettings`
- `src/system/star/generator.rs` - check fixed values in generation functions

---

## 7. Star System - Peculiarities (Stars + System)

**Status:** Both `SystemPeculiarity` and `StarPeculiarity` enums are fully defined with rich variants. But generation is empty vectors (TODO at `src/system/generator.rs:24` and `src/system/star/generator.rs:160`).

### 7a. System Peculiarities

**What to do:**
- Implement `generate_system_peculiarities()` in `src/system/generator.rs`:
  - Roll for each peculiarity type with weighted probabilities
  - `CarbonRich`: ~2-5% chance, higher for older systems and higher metallicity populations
  - `Cataclysm`: ~5-10% chance, severity weighted (Minor most common, Ultimate very rare)
  - `UnusualDebrisDensity`: ~10-15% chance, roughly equal distribution
  - `Nebulae`: ~5% chance, weighted toward Tiny/Small
  - Apply `GalacticRegion` modifiers: Core regions = higher Nebulae chance, Arm regions = higher Cataclysm chance
- Wire into `StarSystem::generate()` replacing the empty vector

**Files to modify:**
- `src/system/generator.rs` - implement peculiarity generation at line 24

### 7b. Star Peculiarities

**What to do:**
- Implement `generate_star_peculiarities()` in `src/system/star/generator.rs`:
  - Roll for each peculiarity type:
    - `ChaoticOrbits`: ~3% base, +5% if system has Cataclysm
    - `ExcessiveRadiation`: ~5% for O/B stars, ~1% for others
    - `AgeDifference`: ~5% in multi-star systems only
    - `RotationAnomaly`: ~5%, speed inversely correlated with age
    - `UnusualMetallicity`: ~10%, direction weighted by population
    - `PowerfulStellarWinds`: ~5% for hot stars (O/B/A)
    - `StrongMagneticField`: ~5%, more common for fast rotators
    - `VariableStar`: ~5-10%, interval correlated with spectral type
    - `CircumstellarDisk`: ~10% for young systems (age < 1 Gyr)
    - `NoMetals`: automatic for Paleodwarf (already implemented)
    - `UnusualElementPresence`: ~15%, element type weighted by star composition
  - Allow max 2-3 peculiarities per star
- Factor peculiarities into downstream generation (orbital stability, planet habitability)

**Files to modify:**
- `src/system/star/generator.rs` - implement peculiarity generation at line 160

---

## 8. Star System - Multiple Star Orbit Eccentricity and Inclination

**Status:** Planet orbit eccentricity/inclination fully implemented in `src/system/orbital_point/generator.rs`. Star-to-star orbits may use fixed/simplified values.

**What to do:**
- Implement `calculate_star_orbit_eccentricity()` separate from planet eccentricity:
  - Short period binaries (P < 10 days): e ~ 0 (tidally circularized)
  - Medium period (10-1000 days): e ~ 0.0-0.6 (uniform-ish)
  - Long period (>1000 days): thermal distribution f(e) = 2e, range 0.0-0.95
  - Use `SeededDiceRoller` to sample from appropriate distribution
- Implement `generate_star_inclination()`:
  - Roughly isotropic: uniform in cos(i), so i = arccos(uniform(0,1)) for 0-90, with chance of retrograde
- Wire into the star orbit generation in `src/system/generator.rs:76` where the TODO for "dynamic parameters for star orbits" exists
- Update `update_existing_orbits()` to compute min/max separation from eccentricity

**Files to modify:**
- `src/system/orbital_point/generator.rs` - add star-specific eccentricity/inclination functions
- `src/system/generator.rs` - wire into star orbit generation at line 76

---

## 9. Star System - General Orbit Eccentricity and Inclination

**Status:** Already implemented for planets in `src/system/orbital_point/generator.rs:656-725`. The README checkbox may refer to refinements.

**What to do:**
- Review and validate existing planet eccentricity distribution against exoplanet data:
  - Rayleigh distribution with sigma ~0.05-0.1 for multi-planet systems
  - Wider distribution for single-planet systems and hot Jupiters
- Add gas giant arrangement effects on eccentricity (some already exists as modifiers)
- Ensure `SystemPeculiarity::Cataclysm` properly increases eccentricity for all orbits
- Address TODO at `src/system/contents/generator.rs:53-54` about planets rotating too fast

**Files to modify:**
- `src/system/orbital_point/generator.rs` - refine eccentricity distribution
- `src/system/contents/generator.rs` - implement rotation validation

---

## 10. Planet Generation - World Parameters and Climate

**Status:** ~90% implemented in `src/system/celestial_body/world/generator.rs` (3,881 lines). Has temperature categories, climate types, atmospheric pressure, hydrosphere, volcanism, tectonics.

**What to do:**
- Address TODO at line 311: **Planetary imbalances** - implement asymmetric climate effects (axial tilt causing extreme seasons, tidal locking causing eyeball worlds)
- Address TODO at line 368: **Atmospheric composition** - refine the atmospheric composition model:
  - Implement proper greenhouse factor calculation: `greenhouse_factor = f(atmospheric_mass, CO2_fraction, CH4_fraction, H2O_vapor)`
  - Add albedo calculation from cloud cover and surface type
  - Surface temp formula: `T_surface = T_blackbody * absorption_factor * greenhouse_factor`
- Address TODO at line 1537: sublimation effects on ice world atmospheres
- Address TODO at line 3627: fix core heat distribution for older systems
- Add tidal locking detection for close-in planets (eyeball world climate)

**Files to modify:**
- `src/system/celestial_body/world/generator.rs` - address 4 TODOs

---

## 11. Planet Generation - Resources

**Status:** Not implemented. `ChemicalComponent` enum and `ElementPresenceOccurrence` enum exist but are only used for star-level element abundance.

**What to do:**
- Create a resource system with three dimensions (matching README: Accessibility, Rarity, Quantity):

```
PlanetaryResource {
    resource_type: ResourceType,
    abundance: ResourceAbundance,      // Absent/Trace/Poor/Average/Rich/Motherlode
    accessibility: ResourceAccess,     // Inaccessible/Deep/Subsurface/Surface/Atmospheric
    quality: f32,                      // 0.0-1.0 purity
}
```

- Define `ResourceType` enum: CommonMetals, PreciousMetals, Radioactives, IndustrialMinerals, Volatiles, OrganicCompounds, ExoticMaterials
- Generation algorithm:
  1. Base abundance from `TelluricBodyComposition` (Metallic = rich metals, Icy = rich volatiles, Rocky = balanced)
  2. Modify by `StellarEvolution` population (higher metallicity = richer mineral resources)
  3. Modify by `volcanism` level (more volcanism = more accessible minerals via cycling)
  4. Modify by `CelestialBodySize` (larger = more total resources)
  5. Modify by system `UnusualElementPresence` traits
  6. Accessibility from atmospheric pressure, gravity, and volcanic activity
  7. Roll per-resource type using weighted tables

- Add `resources: Vec<PlanetaryResource>` to `TelluricBodyDetails`

**Files to create:**
- `src/system/celestial_body/resource/mod.rs` - structs and enums
- `src/system/celestial_body/resource/generator.rs` - generation logic

**Files to modify:**
- `src/system/celestial_body/telluric/mod.rs` - add resources field
- `src/system/celestial_body/mod.rs` - add resource module
- `src/system/celestial_body/telluric/generator.rs` - call resource generation

---

## 12. Planet Generation - Life Presence

**Status:** `LifeLevel` enum exists (None through Sentient). Hardcoded as `Sentient` at `src/system/celestial_body/world/generator.rs:1628`. Detailed bonus/malus comments exist but no logic.

**What to do:**
- Implement `generate_life_level()` using the commented specification:
  - **Bonuses (increase life chance):**
    - System age (older = more time for life to evolve): +1 per 2 Gyr
    - Liquid water present (scale with hydrosphere %): +1 to +3
    - Yellow/orange/red star (F/G/K/M spectral type): +1 to +2
    - Stable orbit (low eccentricity): +1
    - Magnetic field present: +1 to +2
    - Moderate atmosphere: +1
  - **Maluses (decrease life chance):**
    - Not in biozone: -3
    - Star not main sequence: -2
    - No oxygen/carbon dioxide/methane in atmosphere: -2
    - No magnetosphere: -2
    - No/trace atmosphere: -3
    - Carbon-rich system: -1
    - Planet younger than thresholds: -1 to -4 (100Myr/-4, 500Myr/-3, 2Gyr/-2, 4Gyr/-1)
    - Star population II or III: -1 to -2
    - Variable/flare star: -1
    - High debris density: -1
    - Gas planet: -3
  - Sum bonuses and maluses, map to `LifeLevel`:
    - < 0: None
    - 0-2: UniCellular
    - 3-5: PluriCellular
    - 6-8: PlantLike
    - 9-11: AnimalLike
    - 12+: Sentient
  - Add randomness via `SeededDiceRoller` (roll 2d6 + modifier)
- Life level affects downstream climate generation (already partially wired at line 2449+)

**Files to modify:**
- `src/system/celestial_body/world/generator.rs` - implement at line 1628

---

## 13. Planet Generation - Points of Interest

**Status:** Not implemented. No POI system exists.

**What to do:**
- Define POI types relevant to the generation scope:

```
PointOfInterest {
    poi_type: POIType,
    location: POILocation,     // Surface/Subsurface/Orbital/Atmospheric
    significance: POISignificance,  // Minor/Notable/Major/Unique
    description_seed: u64,     // For deterministic description generation
}
```

- `POIType` enum:
  - **Geological:** MassiveCanyon, SuperVolcano, ImpactCrater, CrystalFormation, LavaLake, GeyserField, CaveSystem
  - **Hydrological:** SubterraneanOcean, MassiveWaterfall, ThermalVents, IceGeysers
  - **Atmospheric:** PermanentStorm, AuroraField, FloatingIslands (gas giants)
  - **Biological:** FossilSite, ExtremeLifeColony, MassiveBiostructure (if life present)
  - **Anomalous:** GravityAnomaly, MagneticAnomaly, RadioactiveZone, UnusualMineral
- Generation based on:
  - `volcanism` level -> geological POIs
  - `hydrosphere` -> hydrological POIs
  - `atmospheric_pressure` -> atmospheric POIs
  - `life_level` -> biological POIs
  - Random anomalous POIs with low probability
- Number of POIs scales with planet size: 0-2 for small, 2-5 for medium, 3-8 for large

**Files to create:**
- `src/system/celestial_body/poi/mod.rs` - structs and enums
- `src/system/celestial_body/poi/generator.rs` - generation logic

**Files to modify:**
- `src/system/celestial_body/telluric/mod.rs` - add POI field
- `src/system/celestial_body/mod.rs` - add poi module

---

## 14. Planet Generation - Map Generation

**Status:** Not implemented. No terrain/map system exists.

**What to do:**
- Implement a data-oriented surface map (not visual rendering):

```
PlanetSurfaceMap {
    continent_count: u8,
    land_distribution: Vec<(BiomeType, f32)>,  // biome -> fraction of land
    ocean_distribution: Vec<(OceanType, f32)>,  // ocean type -> fraction of water
    highest_elevation_km: f32,
    deepest_ocean_km: f32,
    tectonic_plate_count: u8,
    notable_features: Vec<GeographicFeature>,
}
```

- `BiomeType` enum: Tundra, Taiga, TemperateForest, TropicalForest, Grassland, Desert, Savanna, Wetland, Alpine, Volcanic
- Derive distributions from existing world parameters:
  - `climate` type determines dominant biomes
  - `temperature_category` sets latitude-band proportions
  - `humidity` shifts forest/desert balance
  - `volcanism` adds volcanic terrain
  - `hydrosphere` determines ocean/land ratio
  - `ice_over_land`/`ice_over_water` adds polar coverage
- Continent count: roll based on planet size and tectonic activity
- Elevation extremes: scale with gravity, tectonic activity, and planet size
- Optional: use simplex noise on an icosphere for hex-grid map data (requires `noise` crate dependency)

**Files to create:**
- `src/system/celestial_body/map/mod.rs` - structs and enums
- `src/system/celestial_body/map/generator.rs` - generation logic

**Files to modify:**
- `src/system/celestial_body/telluric/mod.rs` - add map field
- `src/system/celestial_body/mod.rs` - add map module
- `Cargo.toml` - optionally add `noise` crate

---

## 15. Planet Generation - Exotic Planets

**Status:** `Exotic(ExoticBodyDetails)` and `CelestialBodyComposition::Exotic` are commented out in `src/system/celestial_body/types.rs`. Skeleton references exist in `src/system/contents/generator.rs:2219-2220, 2272-2273`.

**What to do:**
- Define `ExoticBodyDetails` struct:

```
ExoticBodyDetails {
    exotic_type: ExoticPlanetType,
    special_traits: Vec<CelestialBodySpecialTrait>,
    // Include subset of TelluricBodyDetails fields as needed
    core_heat: CelestialBodyCoreHeat,
    atmospheric_pressure: f32,
    atmospheric_composition: Vec<(f32, ChemicalComponent)>,
}
```

- Define `ExoticPlanetType` enum:
  - **CarbonWorld:** Diamond mantle, graphite crust, tar/hydrocarbon oceans. Trigger: `SystemPeculiarity::CarbonRich`
  - **LavaWorld:** Permanent magma ocean, silicate vapor atmosphere. Trigger: extreme proximity to star
  - **HyceanWorld:** Hot ocean + hydrogen atmosphere. Trigger: large water world near inner habitable zone
  - **EyeballWorld:** Tidally locked habitable. Trigger: M/K dwarf star + close-in habitable zone
  - **RoguePlanet:** No parent star, subsurface ocean possible. Trigger: system Cataclysm or special generation
  - **IronWorld:** Stripped mantle, massive iron core. Trigger: close to star + small size
  - **MiniNeptune:** Thick H/He envelope over rocky core. Trigger: size between rocky and gas giant
  - **PuffyGiant:** Inflated hot Jupiter. Trigger: gas giant very close to star
- Generation conditions per type (check orbit zone, size, system traits, stellar type)
- Uncomment the Exotic variants in `types.rs`
- Wire into `get_world_type()` in `src/system/celestial_body/generator.rs`
- Address TODO at line 118 for rogue planet handling

**Files to create:**
- `src/system/celestial_body/exotic/mod.rs` - ExoticBodyDetails struct
- `src/system/celestial_body/exotic/generator.rs` - generation logic

**Files to modify:**
- `src/system/celestial_body/types.rs` - uncomment Exotic variants
- `src/system/celestial_body/generator.rs` - add exotic planet generation paths
- `src/system/contents/generator.rs` - uncomment exotic references at lines 2219-2273

---

## 16. Species Generation

**Status:** Only `LifeLevel` enum exists in `src/life/types.rs`. No species data structures or generation.

**What to do (large feature - break into phases):**

### Phase 1: Species Data Model

```
Species {
    name: Rc<str>,
    homeworld_conditions: HomeworldPreferences,
    biochemistry: Biochemistry,          // CarbonWater, Ammonia, Silicon, Exotic
    body_plan: BodyPlan,                 // Vertebrate, Arthropod, Mollusk, PlantLike
    symmetry: BodySymmetry,              // Bilateral, Radial, Asymmetric
    locomotion: Vec<LocomotionType>,     // Walk, Swim, Fly, Burrow, Sessile
    trophic_level: TrophicLevel,         // Autotroph, Herbivore, Omnivore, Carnivore
    size_class: SizeClass,              // Tiny, Small, Medium, Large, Huge
    reproduction: ReproductionType,      // Sexual, Asexual, Hermaphroditic, Budding
    social_structure: SocialStructure,   // Solitary, Pair, Pack, Herd, Hive
    intelligence: LifeLevel,             // Reuse existing enum
    tech_level: Option<u8>,              // 0-15, None if non-sapient
    lifespan_years: f32,
    preferred_temp_range: (f32, f32),    // Kelvin
    preferred_gravity_range: (f32, f32), // g
    special_traits: Vec<SpeciesTrait>,   // Psionic, HiveMind, Metamorphic, Amphibious
}
```

### Phase 2: Add Species Using Settings

- Add `SpeciesSettings` to `GenerationSettings`:
  - `predefined_species: Vec<SpeciesTemplate>` - user-defined species to place
  - `max_species_per_sector: Option<u8>`
  - `min_tech_level: Option<u8>`, `max_tech_level: Option<u8>`
  - `biochemistry_weights: Option<Vec<(Biochemistry, f32)>>`

### Phase 3: Spawn Species Using System Conditions

- After planet generation, evaluate habitability score per planet
- If `life_level >= Sentient`, roll for sapient species origin
- Species attributes derived from homeworld:
  - High gravity -> stocky body plan, strong locomotion
  - Aquatic world -> swimming locomotion, possible amphibious
  - Low light (M dwarf) -> enhanced non-visual senses
  - Ammonia world -> ammonia biochemistry
  - High atmospheric pressure -> dense body plan
- Tech level derived from species age, homeworld resources, and intelligence

### Phase 4: Writing Species History

- Generate key historical milestones:
  - Origin event, first tools, agriculture, industrialization, space travel
  - Timeline based on tech level and species lifespan
  - Conflicts, alliances, migrations (seeded narrative events)
- Store as `Vec<HistoricalEvent>` with era, description seed, and impact level

### Phase 5: Filling Systems with Life

- Propagate species outward from homeworld based on tech level:
  - TL 0-3: homeworld only
  - TL 4-7: home system colonization
  - TL 8-10: nearby system colonization
  - TL 11+: sector-wide presence
- Generate colony worlds with population levels and development stages

**Files to create:**
- `src/life/species/mod.rs` - Species struct and enums
- `src/life/species/types.rs` - supporting enums
- `src/life/species/generator.rs` - species generation
- `src/life/species/history.rs` - history generation
- `src/life/species/expansion.rs` - system colonization logic

**Files to modify:**
- `src/life/mod.rs` - add species module
- `src/generator/types.rs` - add SpeciesSettings to GenerationSettings

---

## 17. Populated Sectors/Systems/Planets

**Status:** TODO comment at `src/galaxy/map/mod.rs:12` mentions adding a `populate` parameter. No population system exists.

**What to do:**
- Add `populate: bool` parameter to `Galaxy::get_hex()` (as noted in existing TODO)
- When `populate = true`:
  1. Generate all star systems in hex normally
  2. Evaluate life levels on all habitable planets
  3. For planets with `LifeLevel::Sentient`, check if species should originate here
  4. Apply species expansion from Phase 5 above to determine which systems are colonized
- Add population data to celestial bodies:
  - `population: Option<Population>` field on `TelluricBodyDetails`
  - `Population { species_id, count_order: u8, development_level, settlement_type }`
- Add convenience methods:
  - `Generator::generate_populated(settings)` - full generation with life
  - `StarSystem::generate_populated(...)` - single system with life evaluation
  - `Galaxy::get_populated_hex(coord)` - hex with population

**Files to modify:**
- `src/galaxy/map/mod.rs` - add populate parameter
- `src/system/celestial_body/telluric/mod.rs` - add population field
- `src/generator/mod.rs` - add populated generation entry point

---

## 18. Physics & Constants Improvements (from Skyfield research)

**Status:** Various constants and formulas scattered across the codebase use imprecise values or fudge factors.

### 18a. IAU 2015 Nominal Constants

**What to do:**
- Create a centralized constants file with IAU 2015 nominal values:

```rust
// Solar values
const SOLAR_LUMINOSITY_W: f64 = 3.828e26;
const SOLAR_EFFECTIVE_TEMP_K: f64 = 5772.0;
const SOLAR_RADIUS_M: f64 = 6.957e8;
const SOLAR_RADIUS_KM: f64 = 695_700.0;
const SOLAR_MASS_KG: f64 = 1.989e30;
const SOLAR_ABSOLUTE_MAG_V: f64 = 4.83;
const SOLAR_ABSOLUTE_MAG_BOL: f64 = 4.74;

// Fundamental constants
const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;    // m^3 kg^-1 s^-2
const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;      // W m^-2 K^-4
const SPEED_OF_LIGHT: f64 = 299_792_458.0;           // m/s

// Distance units
const AU_M: f64 = 149_597_870_700.0;                 // exact, IAU 2012
const AU_KM: f64 = 149_597_870.700;
const PARSEC_AU: f64 = 206_264.806_247;
const PARSEC_LY: f64 = 3.261_563_777;
const LIGHT_YEAR_M: f64 = 9.460_730_472_58e15;

// Planetary reference values
const EARTH_MASS_KG: f64 = 5.972_17e24;
const EARTH_RADIUS_KM: f64 = 6_371.0;
const EARTH_MASS_SOLAR: f64 = 3.003_46e-6;           // 1/332,946 (more precise than current 1/333,000)
const JUPITER_MASS_KG: f64 = 1.898_2e27;
const JUPITER_RADIUS_KM: f64 = 69_911.0;
const JUPITER_MASS_SOLAR: f64 = 9.545_8e-4;
```

- Replace ad-hoc constants throughout the codebase with references to this file
- Fix Earth mass ratio in `src/utils/conversion.rs` (1/332,946 instead of 1/333,000)

**Files to create:**
- `src/utils/constants.rs` - centralized IAU constants

**Files to modify:**
- `src/utils/mod.rs` - add constants module
- `src/utils/conversion.rs` - use precise constants
- `src/system/star/constants.rs` - reference centralized values

### 18b. Stefan-Boltzmann Fix

**Status:** `calculate_temperature_using_luminosity` in `src/system/star/generator.rs` has a comment: "I have no idea why but I need to put 10^-17 instead of 10^-8." This is because it mixes SI units with solar units.

**What to do:**
- Replace the fudge-factor formulation with the solar-unit version:

```rust
// Solar-unit Stefan-Boltzmann (no fudge factors needed):
// L/L_sun = (R/R_sun)^2 * (T/T_sun)^4

fn temperature_from_luminosity_solar(luminosity_solar: f64, radius_solar: f64) -> f64 {
    // T = T_sun * (L/L_sun)^(1/4) / (R/R_sun)^(1/2)
    5772.0 * luminosity_solar.powf(0.25) / radius_solar.sqrt()
}

fn luminosity_from_temperature_solar(temperature_k: f64, radius_solar: f64) -> f64 {
    radius_solar.powi(2) * (temperature_k / 5772.0).powi(4)
}

fn radius_from_luminosity_temperature(luminosity_solar: f64, temperature_k: f64) -> f64 {
    luminosity_solar.sqrt() / (temperature_k / 5772.0).powi(2)
}
```

- This gives exact results for the Sun (L=1, R=1 -> T=5772K) with zero correction factors
- Verify all downstream calculations still produce valid results after the fix

**Files to modify:**
- `src/system/star/generator.rs` - replace `calculate_temperature_using_luminosity` and related functions (~lines 332-369)

### 18c. Magnitude System

**Status:** No magnitude calculations exist in the codebase.

**What to do:**
- Add absolute and apparent visual magnitude to the `Star` struct
- Formulas:

```rust
fn absolute_magnitude(apparent_mag: f64, distance_parsec: f64) -> f64 {
    apparent_mag - 5.0 * distance_parsec.log10() + 5.0
}

fn luminosity_from_absolute_magnitude(abs_mag: f64) -> f64 {
    10.0_f64.powf((4.83 - abs_mag) / 2.5)  // relative to Sun (M_V = 4.83)
}

fn absolute_magnitude_from_luminosity(luminosity_solar: f64) -> f64 {
    4.83 - 2.5 * luminosity_solar.log10()
}
```

- Add `absolute_magnitude: f64` and `apparent_magnitude: Option<f64>` to `Star` struct
- Compute during star generation from luminosity

**Files to modify:**
- `src/system/star/mod.rs` - add magnitude fields
- `src/system/star/generator.rs` - compute magnitude from luminosity

### 18d. Enhanced Spectral Type Table

**Status:** `TEMPERATURE_TO_SPECTRAL_TYPE_DATASET` in `src/system/star/constants.rs` maps temperature to spectral type only.

**What to do:**
- Extend the reference table to include mass, radius, luminosity, B-V color, and lifetime per spectral type:

```
(spectral_type, T_eff_K, mass_solar, radius_solar, luminosity_solar, B_V_color, lifetime_Myr)
```

- Key entries (45 rows, O3V through Y2):
  - O3V: 44900K, 59.0M, 15.0R, 790000L, -0.33 B-V, 1 Myr
  - G2V: 5770K, 1.00M, 1.00R, 1.0L, 0.63 B-V, 10000 Myr (Sun)
  - M5V: 3060K, 0.16M, 0.20R, 0.0032L, 1.61 B-V, 2000000 Myr
  - L0: 2200K, 0.07M, 0.10R, 0.0001L (brown dwarf)
  - T5: 1100K, 0.04M, 0.08R, 0.000003L (cool brown dwarf)
  - Y0: 450K, 0.02M, 0.08R, 0.0000001L (coldest brown dwarf)
- Use this table for cross-validation during star generation and for deriving properties from spectral type when configuring stars

**Files to modify:**
- `src/system/star/constants.rs` - extend or add parallel table

### 18e. B-V Color Index to RGB

**Status:** No star color rendering data exists.

**What to do:**
- Add B-V color index field to `Star` struct (derive from temperature using Ballesteros 2012 formula)
- Add `bv_to_rgb()` utility for downstream visual rendering:

```rust
fn temperature_to_bv(temp_k: f64) -> f64 {
    // Inverse Ballesteros (2012) - approximate
    // B-V ~ 0.865 * (5601/T)^1.7 - 0.396  (for 3000K-30000K range)
    if temp_k < 3000.0 { 2.0 }
    else if temp_k > 30000.0 { -0.33 }
    else { 0.865 * (5601.0 / temp_k).powf(1.7) - 0.396 }
}

fn bv_to_rgb(bv: f64) -> (u8, u8, u8) {
    // Ballesteros (2012) blackbody approximation
    // Returns sRGB (0-255) tuple
    // ... (temperature-based piecewise calculation)
}
```

**Files to modify:**
- `src/system/star/mod.rs` - add `color_bv: f64` field
- `src/system/star/generator.rs` - compute B-V from temperature
- `src/utils/conversion.rs` - add `bv_to_rgb()` utility

### 18f. Habitable Zone (Kopparapu et al. 2013)

**Status:** Current `BioZone` uses equilibrium temperature thresholds (344K inner, 244K outer) in `src/system/contents/zones.rs`.

**What to do:**
- Replace or supplement with the Kopparapu et al. (2013) formulas which account for stellar spectral type:

```rust
fn habitable_zone_au(luminosity_solar: f64, temperature_k: f64) -> (f64, f64) {
    let t_star = temperature_k - 5780.0;

    // Recent Venus / Early Mars limits (optimistic HZ)
    let s_inner = 1.7763 + 1.4335e-4 * t_star + 3.3954e-9 * t_star.powi(2);
    let s_outer = 0.3179 + 5.4513e-5 * t_star + 1.5313e-9 * t_star.powi(2);

    let inner_au = (luminosity_solar / s_inner).sqrt();
    let outer_au = (luminosity_solar / s_outer).sqrt();
    (inner_au, outer_au)
}

fn snow_line_au(luminosity_solar: f64) -> f64 {
    2.7 * luminosity_solar.sqrt()
}
```

- This gives more accurate HZ for M dwarfs (wider than simple temperature model) and hot stars (narrower)
- Keep existing zone system structure, just improve the boundary calculations

**Files to modify:**
- `src/system/contents/zones.rs` - update BioZone calculation

### 18g. White Dwarf Improvements

**Status:** White dwarf generation exists but neutron star temperature has a TODO: "doesn't seem right at all."

**What to do:**
- Add Mestel's cooling law for white dwarf luminosity vs age:

```rust
fn white_dwarf_luminosity(mass_solar: f64, cooling_age_gyr: f64) -> f64 {
    let l_initial = 10.0_f64.powf(-2.15) * mass_solar.powf(3.95);
    l_initial * cooling_age_gyr.powf(-1.4)
}
```

- Add Chandrasekhar mass-radius relation:

```rust
fn white_dwarf_radius(mass_solar: f64) -> f64 {
    const CHANDRASEKHAR_MASS: f64 = 1.44;
    let ratio = mass_solar / CHANDRASEKHAR_MASS;
    0.0127 * ratio.powf(-1.0 / 3.0) * (1.0 - ratio.powf(4.0 / 3.0)).sqrt()
}
```

- Improve initial-final mass relation (extend Kalirai/Cummings for 3.65-7.2 M_sun range):

```rust
fn white_dwarf_final_mass(initial_mass: f64) -> f64 {
    if initial_mass < 2.7 { 0.096 * initial_mass + 0.429 }
    else if initial_mass < 3.65 { 0.137 * initial_mass + 0.318 }
    else if initial_mass < 7.2 { 0.0873 * initial_mass + 0.476 }
    else { (0.123 * initial_mass + 0.200).min(1.38) }
}
```

- Review and fix `calculate_neutron_star_temperature` (the TODO at `src/system/star/generator.rs:896`)

**Files to modify:**
- `src/system/star/generator.rs` - improve WD cooling, NS temperature, mass-radius relations
- `src/system/star/constants.rs` - add Chandrasekhar mass constant

---

## Suggested Implementation Order

**Tier 0 - Physics fixes (should come first, affects downstream accuracy):**
- [x] 0a. IAU 2015 Constants (item 18a)
- [x] 0b. Stefan-Boltzmann fix (item 18b)
- [x] 0c. Habitable zone formula (item 18f)
- [x] 0d. White dwarf/neutron star fixes (item 18g) - added Mestel cooling law
- [x] 0e. Subdwarf luminosity fix (item 5)

**Tier 1 - Quick wins (standalone, no dependencies):**
- [x] 1. Local Group Galaxies data (item 2) - 35 galaxies populated
- [x] 2. Galaxy Names (item 1) - syllable-based generation
- [x] 3. Sector Names (item 4) - region-based naming
- [x] 4. Star/System Peculiarities (item 7) - both star and system
- [x] 5. Magnitude system (item 18c) - absolute_magnitude field
- [x] 6. Enhanced spectral type table (item 18d) - 26-entry reference
- [x] 7. B-V color index (item 18e) - color_bv field + bv_to_rgb()

**Tier 2 - Medium effort (build on existing systems):**
- [x] 8. Configurable Stars (item 6) - settings fields + wiring
- [x] 9. Binary Star Orbits (item 8) - eccentricity, inclination, period
- [x] 10. Orbit Refinements (item 9) - rotation clamping
- [x] 11. World Parameters/Climate TODOs (item 10) - sublimation + age-based core cooling
- [x] 12. Life Presence (item 12) - score-based life level

**Tier 3 - New subsystems:**
- [x] 13. Resources (item 11) - 6 resource types with abundance/accessibility
- [ ] 14. Exotic Planets (item 15)
- [x] 15. Points of Interest (item 13) - 18 POI types with significance
- [x] 16. Region Mapping - temporary (item 3a) - nucleus/bulge/bar/arm/disk/halo
- [x] 17. Region Mapping - proper (item 3b) - clusters, associations, tidal streams
- [x] 18. Map Generation (item 14) - biome distribution, elevation, tectonic plates

**Tier 4 - Major features (depend on Tier 2-3):**
- [x] 19. Species Generation phases 1-3 (item 16) - data model + condition-based generation
- [ ] 20. Species History phase 4 (item 16)
- [ ] 21. Species Expansion phase 5 (item 16)
- [ ] 22. Populated Objects (item 17)
