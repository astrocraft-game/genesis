# Implementation Plan - Planetary Detail Fixes

Every feature needs its unused enum variants wired in, placeholder logic replaced with real physics, and generation gaps filled.

---

## 1. Cloud Decks - Use ALL 8 compositions

**Currently:** Only 4 of 8 CloudComposition variants generated (Water, WaterIce, SulfuricAcid, Ammonia, Methane). Missing: AmmoniumHydrosulfide, OrganicHaze, SiliconDust.

**Fix:**
- Add AmmoniumHydrosulfide clouds for NH3+H2S atmospheres between 200-250K
- Add OrganicHaze for CH4+N2 atmospheres (Titan) - tholin layer at 100-300km
- Add SiliconDust for lava worlds (T>1500K) - silicate vapor condensing
- Add multi-deck logic for gas giant worlds: 3-layer (NH3 / NH4SH / H2O)
- Compute altitude from condensation temperature vs lapse rate: `z = (T_surface - T_condensation) / lapse_rate`

---

## 2. Sky Color - Use ALL 12 variants

**Currently:** Only 7 of 12 SkyColor variants generated. Missing: DeepBlue, Green, Pink, Red, Yellow.

**Fix:**
- DeepBlue: CH4 absorption in thick H2/He atmospheres (Uranus/Neptune analogs)
- Green: Cl2-rich atmospheres (exotic)
- Pink: fine silicate dust + very thin atmosphere
- Red: heavy iron oxide dust loading (extreme Mars storms)
- Yellow: sulfur aerosol or dense CO2 without acid clouds
- Make sunset color derive properly from daytime composition

---

## 3. Toxicity - Use ALL 9 variants

**Currently:** Missing Filterable, LethallyToxic. Gas checks are binary (present/absent).

**Fix:**
- Filterable: particulates, pollen, low-level SO2 (<10 ppm equivalent)
- LethallyToxic: HCN, high CO, high Cl2 presence
- Add partial pressure checks: CO2 >5% narcotic, >10% lethal. CO >100ppm. H2S >100ppm. SO2 >100ppm.
- Compute toxicity from actual composition fractions * pressure, not just presence

---

## 4. Lake Distribution - Use ALL 7 formation types + ALL 5 liquid types

**Currently:** Missing Impact, Endorheic lake types. Missing Brine, Magma liquids.

**Fix:**
- Impact: generate when crater_density is Heavy/Saturated and hydrosphere > 5%
- Endorheic: generate when precipitation < 300mm and temperature > 280K (arid + warm)
- Brine: hyper-saline lakes in endorheic basins or salty oceans
- Magma: lava lakes on volcanic worlds with volcanism > 70

---

## 5. Surface Material - Use ALL 9 types

**Currently:** Only 4 of 9 SurfaceMaterialType variants generated. Missing: SulfurDeposits, OrganicSediment, EvaporiteDeposits, SandDunes (wrong trigger), BarrenRock (fallback only).

**Fix:**
- SulfurDeposits: SO2-rich atmosphere + volcanism (Io-like)
- OrganicSediment: life >= PluriCellular + hydrosphere > 20 (dead organic matter)
- EvaporiteDeposits: low hydrosphere + warm + was once wetter (evaporated seas)
- SandDunes: arid worlds with wind (pressure > 0.001, land > 50%, hydrosphere < 20%)
- Fix: volcanism > 60 should NOT always produce sand dunes

---

## 6. Volcanic Profile - Use ALL 6 types

**Currently:** Missing Caldera, FloodBasalt as dominant types.

**Fix:**
- Caldera: dominant when volcanism > 60 AND tectonics > 30 (high viscosity magma)
- FloodBasalt: dominant when volcanism > 70 AND tectonics < 10 (mantle plume, no tectonics)
- Add eruption frequency estimate: `eruptions_per_year = volcanism * active_count / 5000`
- Add magma viscosity class: basaltic (shield), andesitic (strato), rhyolitic (caldera)

---

## 7. Ocean Chemistry - Use ALL iron levels + ALL liquid types

**Currently:** Only Negligible/High iron. Only Water/Ammonia/MethaneEthane liquids.

**Fix:**
- Low iron: partially oxygenated oceans (early oxygen rise)
- Moderate iron: reducing conditions with some oxygen
- Brine oceans: salinity > 100 g/kg (evaporating worlds, Europa subsurface)
- Magma oceans: lava worlds with T > 1500K
- Compute salinity from evaporation/precipitation ratio, not pure RNG
- pH from CO2 partial pressure: `pH ~ 8.1 - 0.8 * log10(pCO2/0.0004)`

---

## 8. Lightning - Use ALL 5 mechanisms

**Currently:** Missing AcidCloud mechanism.

**Fix:**
- AcidCloud: Venus-like worlds with H2SO4 cloud decks and high pressure
- Scale flash rate from convective energy: `rate ~ CAPE * cloud_coverage`
- Dust triboelectric rate should be higher (0.1, not 0.01)

---

## 9. Greenhouse - Add H2O + CH4 feedback

**Currently:** Only CO2 greenhouse. No water vapor feedback, no methane.

**Fix:**
- H2O feedback: if surface_temp > 300K, each +1K increases H2O vapor which adds +0.5K
- CH4 contribution: `deltaT_CH4 ~ 0.5 * ln(pCH4/0.000002)` per ppm above background
- Runaway threshold: stellar flux > 1.4x Earth AND water available
- Proper equilibrium temp from stellar luminosity + distance (not just blackbody)

---

## 10. Wind Profile - Use rotation physics + connect AtmosphericCirculation

**Currently:** Arbitrary sqrt formula. AtmosphericCirculation (Hadley cells) exists but is computed separately and not connected to winds.

**Fix:**
- Derive mean wind from Hadley cell count (already computed): more cells = stronger zonal winds
- Superrotation: specifically for slow rotators (Rossby number >> 1), not just rotation > 10 days
- Max wind: scale with internal heat for gas-giant-type atmospheres
- Connect wind profile to AtmosphericCirculation already computed in world generator

---

## 11. Radiation - Add distance scaling + stellar type

**Currently:** Fixed base 400 mSv/yr at 1 AU. No distance scaling.

**Fix:**
- Scale by `1/distance^2` from star
- Scale by stellar UV output: M dwarfs have high flare UV, F stars have strong steady UV
- Magnetic field strength: use numeric B-field ratio, not binary
- Atmospheric shielding: continuous `exp(-column_density)` not 3 tiers
- Add in-radiation-belt check for moons of gas giants

---

## 12. Atmospheric Layers - Add temperature profile

**Currently:** Only scale height + tropopause. No temp profile.

**Fix:**
- Compute lapse rate: `dT/dz = -g/cp` (dry adiabatic) or `-g*M/(R)` simplified
- Stratosphere height: if has_stratosphere, stratopause at ~3-5x scale height
- Mesopause at ~8-10x scale height
- Exobase from `H * ln(column_density)` properly

---

## 13. Seismic - Add Gutenberg-Richter

**Currently:** Max magnitude and quakes are independent RNG. No G-R law.

**Fix:**
- Set b-value (~1.0 for tectonic, ~1.5 for volcanic, ~0.8 for tidal)
- Derive quakes from: `log10(N_m4) = a - b*(4 - max_mag_offset)`
- Subduction zones: allow M9+ for TectonicExtreme
- Tidal: compute from actual tidal_heating value, not threshold

---

## 14. Hydrography - Use ALL 5 delta types + rain shadow

**Currently:** Missing Estuarine deltas. No rain shadow.

**Fix:**
- Estuarine: tidal range > 4m (compute from moon mass + distance)
- Rain shadow effect: reduce precipitation on leeward side of mountains (connect to tectonic_activity)
- Drainage density from Langbein-Schumm curve (peak at 250-360mm precip)

---

## Suggested Implementation Order

- [ ] 1. Cloud decks (all 8 compositions)
- [ ] 2. Sky color (all 12 variants)
- [ ] 3. Toxicity (all 9 variants with partial pressures)
- [ ] 4. Lake distribution (all 7 types + 5 liquids)
- [ ] 5. Surface material (all 9 types)
- [ ] 6. Volcanic profile (all 6 types + eruption frequency)
- [ ] 7. Ocean chemistry (all iron levels + liquid types + pH formula)
- [ ] 8. Lightning (all 5 mechanisms)
- [ ] 9. Greenhouse (H2O + CH4 feedback)
- [ ] 10. Wind profile (connect to Hadley cells)
- [ ] 11. Radiation (distance + stellar type scaling)
- [ ] 12. Atmospheric layers (temperature profile + lapse rate)
- [ ] 13. Seismic (Gutenberg-Richter law)
- [ ] 14. Hydrography (all 5 deltas + rain shadow)
