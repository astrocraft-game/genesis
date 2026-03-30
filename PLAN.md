# Implementation Plan V3 - Planetary Detail

Deep planetary science: atmosphere, surface, geology, hazards. All derivable from existing data.

---

## 1. Atmospheric Layers

**What:** Scale height, tropopause, stratosphere presence, exobase altitude.
**Formula:** `H = R*T / (M*g)` where R=8.314, T=temp K, M=molar mass kg/mol, g=gravity m/s^2.
**Key:** Stratosphere only exists if UV absorber present (O3, tholin haze).

**Struct:** `AtmosphericLayers { scale_height_km, tropopause_km, has_stratosphere, exobase_km }`

**Files:** `telluric/types.rs` (struct), `world/generator.rs` (compute)

---

## 2. Atmosphere Breathability & Toxicity

**What:** Classify atmosphere for habitability: Vacuum through Superdense pressure, plus Benign through Insidious toxicity.
**Key thresholds:** CO2>5% narcotic, CO>100ppm lethal, H2S>100ppm lethal, O2<16% hypoxia, >50% toxic.

**Enums:** `AtmosphereBreathability`, `AtmosphereToxicity`

**Files:** `telluric/types.rs` (enums), `world/generator.rs` (derive from pressure + composition)

---

## 3. Cloud Decks

**What:** Multiple cloud layers with composition, altitude, optical depth, coverage.
**Key:** Clouds form where T_atmosphere = T_condensation for each compound. Jupiter has 3 decks.

**Struct:** `CloudDeck { composition, base_altitude_km, top_altitude_km, optical_depth, coverage_fraction }`
**Enum:** `CloudComposition` (Water, WaterIce, SulfuricAcid, Ammonia, Methane, OrganicHaze, etc.)

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 4. Greenhouse Effect Quantification

**What:** Equilibrium temp, actual surface temp, greenhouse delta, CO2 partial pressure, runaway flag, bond albedo.
**Formula:** `deltaT ~ 10 * ln(P_CO2 / 0.0004)` for CO2; runaway at >1.4x Earth flux.

**Struct:** `GreenhouseEffect { equilibrium_temp_k, surface_temp_k, greenhouse_delta_k, co2_partial_pressure_bar, bond_albedo, is_runaway }`

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 5. Sky Appearance

**What:** Daytime color, sunset color, haze depth, daytime star visibility.
**Key:** Rayleigh=blue, iron dust=butterscotch, tholins=orange, H2SO4=amber, CH4 absorption=deep blue.

**Enum:** `SkyColor` (Black, Blue, PaleBlue, White, Yellow, Amber, Orange, Butterscotch, Red, Green, Pink)
**Struct:** `SkyAppearance { daytime_color, sunset_color, daytime_stars_visible, haze_optical_depth }`

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 6. Wind Profile

**What:** Mean/max surface winds, superrotation, equator-pole delta.
**Key:** Neptune 580 m/s from internal heat. Venus superrotation 60x surface speed.

**Struct:** `WindProfile { mean_surface_wind_ms, max_wind_ms, superrotation, superrotation_factor }`

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 7. Hydrography (Rivers)

**What:** Drainage density, major river count, longest river, mean precipitation, delta type.
**Formula:** Hack's law `L = 1.4 * A^0.57`. Peak drainage at 250-360mm precipitation.

**Struct:** `Hydrography { drainage_density, major_river_count, longest_river_km, mean_precipitation_mm, dominant_delta_type }`

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 8. Lake Distribution

**What:** Count, density, formation type, largest lake, liquid type.
**Key:** Glaciated regions: 0.56 lakes/km^2. Titan: methane/ethane lakes at poles.

**Struct:** `LakeDistribution { lake_count, lake_density, dominant_type, largest_lake_km2, liquid_type }`

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 9. Glaciation State

**What:** Ice coverage, glacial period flag, snowball state, Milankovitch cycles, ice cap location.
**Key:** Tilt>40deg = equatorial ice. Snowball below ~260K mean. Obliquity cycle ~41kyr.

**Struct:** `GlaciationState { ice_coverage_fraction, in_glacial_period, snowball_state, ice_cap_location }`
**Enum:** `IceCapLocation` (None, Polar, Equatorial, DarkSide, Global)

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 10. Ocean Chemistry

**What:** Liquid type, salinity, pH, anoxic flag, iron content, hydrothermal vents, subsurface flag.
**Key:** Archean Earth: green iron oceans. Europa: salty subsurface. Titan: methane seas.

**Struct:** `OceanChemistry { liquid_type, salinity_g_per_kg, ph, anoxic, iron_content, hydrothermal_vents }`

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 11. Volcanic Profile

**What:** Active count, dominant type, flood basalt history, tallest volcano, supervolcano flag.
**Key:** No tectonics = only shield volcanoes (grow huge). Subduction = stratovolcanoes.

**Struct:** `VolcanicProfile { active_count, dominant_type, flood_basalt_history, tallest_volcano_km, supervolcano_present }`
**Enum:** `VolcanoType` (Shield, Stratovolcano, Caldera, Cinder, Fissure, FloodBasalt, Cryovolcano)

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 12. Mineral Diversity

**What:** Mineral count, evolution stage, free oxygen flag.
**Key:** No water=500, with water=1500, with O2=4000+, with life=5800.

**Struct:** `MineralDiversity { mineral_count, evolution_stage }`
**Enum:** `MineralEvolutionStage` (Primordial, Differentiated, Hydrated, TectonicallyActive, Oxidized, Biogenic)

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 13. Surface Material

**What:** Primary type, depth, perchlorates, oxidation state, space weathering.
**Key:** Mars: iron oxide fines + perchlorates (toxic). Moon: regolith 4-15m from impacts.

**Struct:** `SurfaceMaterial { primary_type, depth_m, perchlorates, oxidized, space_weathering }`
**Enum:** `SurfaceMaterialType` (Regolith, IronOxideFines, Soil, SulfurDeposits, IceCrust, OrganicSediment, etc.)

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 14. Radiation Environment

**What:** Surface dose mSv/yr, UV index, cosmic ray flux, shielding flags, hazard class.
**Key:** Earth: 2.4 mSv/yr. Mars: 240. Europa: 5,400,000 (Jupiter belts).

**Struct:** `RadiationEnvironment { surface_dose_msv_yr, uv_index_peak, radiation_hazard }`
**Enum:** `RadiationHazard` (Negligible, Low, Moderate, High, Extreme)

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 15. Seismic Profile

**What:** Gutenberg-Richter a/b values, max magnitude, quake rate, source type.
**Formula:** `log10(N) = a - b*M`, b~1.0. Tidal: Mw_max from e^2*R^5/a^6 scaling.

**Struct:** `SeismicProfile { max_magnitude, quakes_per_year_m4, seismicity_source }`
**Enum:** `SeismicitySource` (None, Residual, TidalOnly, TectonicModerate, TectonicExtreme, TidalExtreme)

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 16. Dust Storm Profile

**What:** Global storm possibility, recurrence interval, peak winds, dust fraction, dust devils.
**Key:** Mars: planet-encircling every ~3 Mars years. Needs fine dust + thin atmosphere.

**Struct:** `DustStormProfile { global_storms_possible, global_storm_interval_years, peak_wind_ms, dust_devils_active }`

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## 17. Lightning Profile

**What:** Presence, flash rate, mechanism, energy.
**Key:** Jupiter 1000-10000x Earth per flash. Mars: triboelectric in dust. Volcanic lightning universal.

**Struct:** `LightningProfile { present, flash_rate_relative, mechanism }`
**Enum:** `LightningMechanism` (None, WaterCloud, VolcanicPlume, DustTriboelectric, AcidCloud)

**Files:** `telluric/types.rs`, `world/generator.rs`

---

## Suggested Implementation Order

**Tier 1 - Atmosphere (builds on existing atmospheric_pressure + composition):**
- [x] 1. Atmospheric layers - scale height, tropopause, stratosphere, exobase
- [x] 2. Breathability & toxicity - 8 breathability + 9 toxicity categories
- [x] 3. Cloud decks - multi-layer with composition, altitude, optical depth
- [x] 4. Greenhouse quantification - CO2 partial pressure, delta T, runaway flag
- [x] 5. Sky appearance - color from Rayleigh/Mie/absorption, sunset color
- [x] 6. Wind profile - surface/max winds, superrotation detection

**Tier 2 - Surface water/ice (builds on existing hydrosphere + climate):**
- [x] 7. Hydrography (rivers) - Hack's law river count/length, precipitation, deltas
- [x] 8. Lake distribution - count, formation type, liquid composition
- [x] 9. Glaciation state - ice coverage, snowball, Milankovitch, cap location
- [x] 10. Ocean chemistry - salinity, pH, iron content, hydrothermal vents

**Tier 3 - Geology (builds on existing volcanism + tectonics):**
- [x] 11. Volcanic profile - type (shield/strato/caldera), count, tallest, supervolcano
- [x] 12. Mineral diversity - 60 to 5800 minerals, evolution stage from water/O2/life
- [x] 13. Surface material - regolith/soil/iron fines, depth, perchlorates

**Tier 4 - Hazards (builds on everything above):**
- [x] 14. Radiation environment - dose mSv/yr, UV index, hazard class
- [x] 15. Seismic profile - Gutenberg-Richter, max magnitude, source type
- [x] 16. Dust storm profile - global storms, interval, peak winds, dust devils
- [x] 17. Lightning profile - mechanism (water/volcanic/dust), flash rate
