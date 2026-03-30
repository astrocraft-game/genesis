# Implementation Plan V2 - Natural Science & Physics

Focused on physical plausibility, real data distributions, and natural phenomena.

---

## 1. Atmospheric Escape Validation

**Status:** `escape_velocity()` and `jeans_parameter()` exist in `src/system/contents/utils.rs` but are never called during world generation. Bodies can be generated with physically impossible atmospheres.

**What to do:**
- After atmosphere generation in `world/generator.rs`, validate retention:
  - Compute Jeans parameter for each major atmospheric component
  - If `jeans_parameter < 6` for a gas, it escapes over Gyr timescales
  - If `jeans_parameter < 3`, hydrodynamic escape strips it rapidly
  - Remove components that can't be retained; reduce pressure accordingly
- Factor in magnetic field: unshielded bodies lose atmosphere to stellar wind sputtering
- Factor in age: cumulative loss over system lifetime

**Files to modify:**
- `src/system/celestial_body/world/generator.rs` - add validation pass after atmosphere generation

---

## 2. Tidal Locking from Physics

**Status:** `CelestialBodySpecialTrait::TideLocked` exists but locking is not computed from orbital mechanics.

**What to do:**
- Compute tidal locking timescale: `t_lock ~ (a^6 * Q) / (G * M_host^2 * k2 * R^5)`
  - a = semi-major axis, R = body radius, M_host = primary mass
  - Q ~ 100 for rocky, ~10-50 for icy bodies
  - k2 ~ 0.3 for rocky, ~0.05 for icy
- Compare `t_lock` to system age: if `t_lock < star_age`, body is tidally locked
- Apply `TideLocked` trait automatically when physics demands it
- For thick-atmosphere bodies (P > 10 atm) near M dwarfs, thermal atmospheric tides resist locking

**Files to modify:**
- `src/system/celestial_body/world/generator.rs` or `orbital_point/generator.rs` - add locking check
- `src/system/celestial_body/traits/` - ensure TideLocked trait is applied

---

## 3. Quantitative Tidal Heating

**Status:** `tidal_heating: u32` on `CelestialBody` exists but is not derived from orbital parameters.

**What to do:**
- Compute: `E_dot ~ (e^2 * R^5 * n) / (a^6 * Q)` where n = mean motion
- Use eccentricity, distance, body radius, primary mass from existing data
- Map to heat flux (W/m^2):
  - `< 0.01` -> negligible
  - `0.01 - 0.04` -> can maintain subsurface ocean
  - `0.04 - 0.5` -> active geology, possible cryovolcanism
  - `0.5 - 2.0` -> intense volcanism
  - `> 2.0` -> Io-like extreme volcanism
- Wire results into core heat, volcanism, and subsurface ocean generation

**Files to modify:**
- `src/system/contents/utils.rs` - add `calculate_tidal_heating_flux()` function
- `src/system/celestial_body/moon/generator.rs` - use formula for moons

---

## 4. Orbital Resonance Detection

**Status:** Orbital periods are calculated but resonances are not detected or applied.

**What to do:**
- After orbit generation, scan adjacent body pairs for near-integer period ratios
- Check ratios: 2:1, 3:2, 4:3, 5:3, 5:2 (within 5% tolerance)
- When detected:
  - Add `OrbitalResonance { ratio: (u8, u8), partner_id: u32 }` to orbit data
  - Force non-zero eccentricity on resonant bodies (resonances pump eccentricity)
  - Increase tidal heating for resonant moons
- Detect resonance chains (3+ bodies in linked resonances like Io-Europa-Ganymede)

**Files to modify:**
- `src/system/orbital_point/types.rs` - add resonance field to Orbit
- `src/system/contents/generator.rs` - add resonance detection pass

---

## 5. Ring and Belt Generation

**Status:** 3 empty generator files exist: `celestial_disk/generator.rs`, `ring/generator.rs`, `belt/generator.rs`. Types are fully defined.

**What to do:**
- **Rings:** Generate within Roche limit of gas giants/large planets
  - Ring optical depth (0.001 to 5.0), width, inner/outer radius
  - Composition from `CelestialRingComposition` (Ice, Rock, Metal, Dust)
  - Gap structure at orbital resonances with shepherd moons
  - Ring age category (young < 500 Myr, intermediate, primordial)
  - Ring mass derived from optical depth and extent
- **Belts:** Generate between orbital zones (asteroid belt analog)
  - Size-frequency distribution: power law `N(>D) ~ D^(-2.34)`
  - Total mass, largest body diameter
  - Composition (C-type, S-type, M-type proportions)
  - Kirkwood gaps at resonances with nearby massive planet
- Use Roche limit formula already in `utils.rs`

**Files to modify:**
- `src/system/celestial_disk/generator.rs` - implement
- `src/system/celestial_disk/ring/generator.rs` - implement
- `src/system/celestial_disk/belt/generator.rs` - implement

---

## 6. Magnetopause and Radiation Belts

**Status:** `MagneticFieldStrength` enum exists but magnetopause distance and radiation belts are not computed.

**What to do:**
- Compute magnetopause standoff distance: `R_mp = R_planet * (B^2 / (2 * mu_0 * rho_sw * v_sw^2))^(1/6)`
- Stellar wind pressure from star type + distance (M dwarfs have denser winds at HZ)
- Add to `TelluricBodyDetails`:
  - `magnetopause_radii: f32` (in body radii, 0 if no field)
  - `has_radiation_belts: bool` (true if field moderate+ and stellar wind present)
  - `aurora_latitude: f32` (degrees from pole, ~arccos(sqrt(R/R_mp)))
- Radiation belt intensity affects habitability and satellite electronics

**Files to modify:**
- `src/system/celestial_body/telluric/mod.rs` - add fields
- `src/system/celestial_body/world/generator.rs` - compute values

---

## 7. Hadley Circulation Cells

**Status:** Climate is assigned as a single enum. No atmospheric circulation model.

**What to do:**
- Compute thermal Rossby number: `Ro = (g * H * Delta_T) / (Omega^2 * a^2)`
  - g = gravity, H = scale height, Delta_T = equator-pole temp diff
  - Omega = rotation rate (2*pi/rotation_period), a = planet radius
- Map to circulation regime:
  - Ro >> 1 (slow rotators like Venus): 1 cell per hemisphere
  - Ro ~ 1 (Earth-like): 3 cells per hemisphere
  - Ro << 1 (fast rotators like Jupiter): many narrow cells
- Derive: `circulation_cells: u8`, `jet_stream_count: u8`
- Use cell count to refine biome distribution in surface map

**Files to modify:**
- `src/system/celestial_body/telluric/types.rs` - add `AtmosphericCirculation` struct
- `src/system/celestial_body/world/generator.rs` - compute from rotation + gravity

---

## 8. Stellar Flare Activity

**Status:** `StarPeculiarity::VariableStar` and `StrongMagneticField` exist but no quantitative flare model.

**What to do:**
- Assign flare activity class based on spectral type + age:
  - M dwarfs (young): Hyperactive
  - M dwarfs (old): Active
  - K dwarfs: Moderate
  - G dwarfs: Quiet
  - F/A dwarfs: Very Quiet
- Add `flare_activity: FlareActivity` enum to `Star` (Quiet, Moderate, Active, Hyperactive)
- Superflare probability: `P(E > 10^33 erg) ~ 0.01-1%/year` for M dwarfs
- Wire into life level calculation (flares reduce habitability for M dwarf planets)

**Files to modify:**
- `src/system/star/mod.rs` - add field
- `src/system/star/generator.rs` - compute from type + age

---

## 9. Subsurface Ocean Estimation

**Status:** `CelestialBodySpecialTrait::SubSurfaceOceans` exists but depth and ice thickness are not computed.

**What to do:**
- For icy bodies with tidal heating > 0.01 W/m^2 or radiogenic heating:
  - Ice shell thickness from thermal equilibrium: `d_ice ~ k_ice * (T_melt - T_surface) / q_tidal`
  - Ocean depth from remaining water mass budget
  - Antifreeze agents: ammonia depresses melting point by ~100K
- Add to relevant trait or new struct:
  - `ice_shell_thickness_km: f32`
  - `ocean_depth_km: f32`
  - `ocean_composition: Vec<ChemicalComponent>` (water, ammonia, salts)

**Files to modify:**
- `src/system/celestial_body/world/generator.rs` - compute after tidal heating
- `src/system/celestial_body/traits/types.rs` - extend SubSurfaceOceans data

---

## 10. Crater Population

**Status:** `POIType::ImpactCrater` exists as a single POI. No crater density model.

**What to do:**
- Compute crater density from surface age and impactor flux:
  - `crater_density_class`: Pristine (young, resurfaced), Light, Moderate, Heavy, Saturated (ancient)
  - `largest_crater_diameter_km`: scales with body size and surface age
  - `impact_basin_count`: very large impacts (D > 0.3 * body diameter)
- Surface age proxy: volcanism and tectonics reduce crater density
- Add to `PlanetSurfaceMap`:
  - `crater_density: CraterDensity` enum
  - `largest_crater_km: f32`

**Files to modify:**
- `src/system/celestial_body/telluric/types.rs` - add crater types
- `src/system/celestial_body/world/generator.rs` - compute in surface map

---

## 11. Spin-Orbit Resonance

**Status:** `TelluricRotationDifference::Resonant` exists for 3:2 but is not systematically computed.

**What to do:**
- For non-tidally-locked bodies with eccentricity > 0.1:
  - 3:2 resonance probability increases with eccentricity
  - Mercury capture probability: ~55% at e=0.206
- For thick-atmosphere bodies (Venus analog):
  - Thermal tides can produce retrograde or very slow rotation
- Add `spin_orbit_resonance: Option<(u8, u8)>` to Orbit
- Wire into rotation period calculation

**Files to modify:**
- `src/system/orbital_point/types.rs` - add field
- `src/system/orbital_point/generator.rs` - compute in rotation generation

---

## 12. Eccentricity from Beta Distribution

**Status:** Current eccentricity uses 3d6 + modifier table in `orbital_point/generator.rs`.

**What to do:**
- Replace with physics-informed distributions:
  - Hot planets (a < 0.1 AU): circularized, e ~ 0.01
  - Warm giants: Beta(1.0, 2.79) distribution
  - Multi-planet systems: Rayleigh(sigma=0.05)
  - Single giants: Rayleigh(sigma=0.3)
- Keep the modifier system for gas giant arrangement effects
- Sample: `e = rng.gen_f64().powf(1.0/alpha) * (1.0 - rng.gen_f64().powf(1.0/beta))`

**Files to modify:**
- `src/system/orbital_point/generator.rs` - replace eccentricity table

---

## 13. Cryovolcanism

**Status:** `POIType::IceGeysers` exists. Tidal heating exists. Not connected.

**What to do:**
- For icy bodies with tidal heating flux > 0.01 W/m^2:
  - Generate cryovolcanic features: geyser fields, plumes, cryolava flows
  - Plume height from eruption velocity: `h ~ v^2 / (2*g)`
  - Eruption rate from tidal heating flux and ice shell permeability
- Add `CryovolcanicActivity` struct:
  - `activity_level: f32` (0-100, like volcanism)
  - `plume_height_km: f32`
  - `geyser_count: u16`

**Files to modify:**
- `src/system/celestial_body/world/generator.rs` - derive from tidal heating for icy bodies

---

## 14. Frost Cycles

**Status:** `ice_over_land` and `ice_over_water` exist as static percentages. No seasonal variation.

**What to do:**
- From axial tilt and orbital eccentricity, compute seasonal temperature range:
  - `T_range ~ T_avg * sin(axial_tilt) * (1 + eccentricity)`
- For bodies with atmospheric species near condensation point:
  - CO2 frost: condenses at ~195K (Mars)
  - N2 frost: condenses at ~63K (Triton, Pluto)
  - CH4 frost: condenses at ~91K (Pluto)
- Add to `PlanetSurfaceMap`:
  - `seasonal_frost_type: Option<ChemicalComponent>`
  - `frost_cap_latitude: f32` (equatorward extent at maximum)
  - `temperature_range_k: f32` (seasonal swing)

**Files to modify:**
- `src/system/celestial_body/telluric/types.rs` - add frost/seasonal fields
- `src/system/celestial_body/world/generator.rs` - compute from axial tilt + orbit

---

## 15. Wire Life System into Generation

**Status:** Species generator, history, expansion all built but never called. `populate: bool` setting defined but unchecked. This is the biggest dead code issue.

**What to do:**
- In world generation, after life level is computed:
  - If `life_level >= AnimalLike` and `settings.populate == true`:
    - Call `generate_species_from_world()` with world parameters
    - If species generated, call `generate_species_history()` with tech level
    - Store species + history alongside world data
- In `Generator::generate()`:
  - If `settings.populate`, trigger species generation for each system
  - Apply expansion reach to mark neighboring systems as colonized
- Wire `do_not_generate_*` body settings into orbit filling
- Wire `fixed_spectral_type` / `fixed_luminosity_class` into star generation

**Files to modify:**
- `src/generator/mod.rs` - check populate flag
- `src/system/celestial_body/world/generator.rs` - call species generator
- `src/system/contents/generator.rs` - check do_not_generate settings
- `src/system/star/generator.rs` - check fixed spectral/luminosity settings

---

## 16. Photochemical Hazes

**Status:** Atmospheric composition generated but haze formation not modeled.

**What to do:**
- For atmospheres with CH4 + N2 and significant UV flux:
  - Compute haze optical depth from CH4 fraction, UV flux, pressure
  - Titan: CH4 > 1%, N2 dominant, sufficient UV -> opaque haze
- Add `haze_optical_depth: f32` to atmosphere data (0 = clear, > 1 = opaque)
- Affects surface visibility and greenhouse effect

**Files to modify:**
- `src/system/celestial_body/world/generator.rs` - compute after atmosphere

---

## 17. Lagrange Trojans

**Status:** Not modeled.

**What to do:**
- For planets with mass ratio to star > 1:25 (i.e., most gas giants):
  - Generate trojan population at L4/L5 points
  - Population size proportional to planet mass and system age
  - Composition similar to nearby belt material
- Add `trojan_population: Option<u32>` to planet data

**Files to modify:**
- `src/system/contents/generator.rs` - add trojan check after orbit filling

---

## 18. Oort Cloud and Comet Reservoirs

**Status:** Not modeled. Outer system structure doesn't exist.

**What to do:**
- For systems with gas giants:
  - Generate Kuiper belt analog beyond outermost giant (mass ~ 0.01-0.1 M_earth)
  - Generate Oort cloud (mass ~ 1-100 M_earth, extent ~ 1000-100000 AU)
  - Comet injection rate from stellar perturbations and galactic tides
- Add as system-level properties to `StarSystem`

**Files to modify:**
- `src/system/mod.rs` - add outer system fields
- `src/system/generator.rs` - generate after planet formation

---

## Suggested Implementation Order

**Tier 0 - Fix plausibility (generation currently wrong):**
- [ ] 1. Atmospheric escape validation
- [ ] 2. Tidal locking from physics
- [ ] 3. Quantitative tidal heating
- [ ] 12. Beta distribution for eccentricity

**Tier 1 - Wire existing dead code:**
- [ ] 15. Wire life system into generation

**Tier 2 - Missing physics (high impact):**
- [ ] 4. Orbital resonance detection
- [ ] 5. Ring and belt generation
- [ ] 6. Magnetopause and radiation belts
- [ ] 8. Stellar flare activity

**Tier 3 - Natural phenomena (enrichment):**
- [ ] 7. Hadley circulation cells
- [ ] 9. Subsurface ocean estimation
- [ ] 10. Crater population
- [ ] 11. Spin-orbit resonance
- [ ] 13. Cryovolcanism
- [ ] 14. Frost cycles
- [ ] 16. Photochemical hazes
- [ ] 17. Lagrange trojans
- [ ] 18. Oort cloud and comet reservoirs
