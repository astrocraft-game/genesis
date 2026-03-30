# Implementation Plan - Expand Recipe System to 500+

Add 4 new recipe modules from research: organic chemistry, inorganic reactions, electrochemistry/nuclear, and natural processes (geo/atmo/bio/pigments/water).

## Current State: 265 recipes in 7 files
## Target: 500+ recipes in 11 files

---

## New Modules

### 1. `organic.rs` (~60 recipes)
Hydrocarbons, alcohols, aldehydes, ketones, acids, esters, amines, ethers, halogenated.

Key recipes:
- Steam cracking: naphtha → ethylene + propylene (850°C)
- Catalytic reforming: naphtha → BTX (benzene/toluene/xylene, 500°C, Pt)
- Methanol from syngas (250°C, 80atm, Cu/ZnO)
- Ethanol from ethylene hydration (300°C, 70atm, H3PO4)
- Formaldehyde from methanol oxidation (600°C, Ag)
- Acetaldehyde Wacker process (ethylene+O2, PdCl2)
- Acetone cumene process (benzene+propylene → phenol+acetone)
- Acetic acid Monsanto process (methanol+CO, Rh/Ir)
- Citric acid fermentation (Aspergillus niger)
- Adipic acid from cyclohexane (for nylon)
- Terephthalic acid from p-xylene (for PET)
- Ethyl acetate esterification
- Aniline from nitrobenzene (Bechamp reduction)
- Caprolactam for nylon-6
- Acrylonitrile Sohio process
- Diethyl ether from ethanol
- Ethylene oxide (Ag catalyst)
- Vinyl chloride monomer
- Chloroform, CCl4 from methane chlorination
- Tetrafluoroethylene for Teflon

### 2. `inorganic.rs` (~50 recipes)
Acid-base, precipitation, redox, complexation reactions.

Key recipes:
- 10 acid-base neutralizations (HCl+NaOH, H2SO4+CaCO3, etc.)
- 8 precipitation reactions (BaSO4, AgCl, Fe(OH)3, PbI2, CaCO3, Cu(OH)2, etc.)
- 10 redox (thermite, H2O2 decomposition, KMnO4 oxidations, Na+water, Mg burning, Fe displacement)
- 5 complexation (Prussian blue, Cu-ammonia, Fe-thiocyanate, Tollens, EDTA)
- Phase transitions and color-change reactions

### 3. `electrochemistry.rs` (~40 recipes)
Batteries (charge/discharge), electroplating, electrolysis.

Key recipes:
- 10 battery types (lead-acid, Li-ion, NiCd, NiMH, zinc-carbon, alkaline, zinc-air, Na-S, Fe-air, LFP)
- 8 electroplating (Cr, Au, Ni, Zn, Ag, Sn, Cu, anodizing Al)
- 5 electrolysis (water, brine, molten NaCl, Hall-Héroult, Cu/Zn electrowinning)
- Nuclear: 4 fission, 6 fusion, 5 decay chains, 5 transmutation, 5 nucleosynthesis

### 4. `natural.rs` (~60 recipes)
Geochemistry, atmospheric, biochemistry, pigments, water treatment.

Key recipes:
- 4 weathering (feldspar, olivine, calcite, pyrite oxidation)
- 4 metamorphic (limestone→marble, shale→slate→schist, sandstone→quartzite, serpentinization)
- 4 hydrothermal (black smoker pyrite, gold deposition, silica sinter, sulfide chimneys)
- 3 diagenesis (sandstone, limestone, kerogen→oil)
- 4 ozone cycle + 5 smog/acid rain
- 3 photosynthesis/respiration + 4 nitrogen cycle + 3 methane cycle + 3 sulfur cycle
- 15 pigments (ochres, Prussian blue, chrome yellow, cobalt blue, ultramarine, titanium white, Egyptian blue, vermilion, lead white, cadmium yellow/red, verdigris, carbon black)
- 5 water treatment (lime softening, ion exchange, RO, chlorination, ozonation)

---

## Implementation Order

- [x] 1. Add new Substance variants (organic, inorganic, pigment, natural) - DONE
- [x] 2. organic.rs (28 recipes: steam cracking, BTX, alcohols, aldehydes, acids, esters, amines, ethers, halogenated, polymers) - DONE
- [x] 3. inorganic.rs (16 recipes: acid-base, precipitation, redox, complexation) - DONE
- [x] 4. electrochemistry.rs (15 recipes: batteries, electroplating, nuclear fission/fusion/nucleosynthesis) - DONE
- [x] 5. natural.rs (40 recipes: weathering, metamorphic, hydrothermal, atmospheric, biochemistry, 16 pigments, water treatment) - DONE
- [x] 6. Wired into mod.rs, all tests pass - DONE

## Final Count: 364 recipes across 11 files

| File | Count | Categories |
|---|---|---|
| extraction.rs | 70 | ore → metal (40+ metals, 13 cross-recipe groups) |
| alloys.rs | 48 | metal + metal → alloy (steels, bronzes, Ni/Ti/precious) |
| chemistry.rs | 48 | industrial synthesis (Haber, Contact, polymers, explosives) |
| construction.rs | 32 | building materials (cement, concrete, brick, glass, wood) |
| fuel.rs | 17 | energy carriers (charcoal, petroleum, biodiesel, cryogenics) |
| biological.rs | 30 | food/textile/leather/paper/dyes |
| phase_change.rs | 20 | melting, heat treatment, casting, crystallization |
| organic.rs | 28 | hydrocarbons, alcohols, acids, polymers, halogenated |
| inorganic.rs | 16 | acid-base, precipitation, redox, complexation |
| electrochemistry.rs | 15 | batteries, electroplating, nuclear |
| natural.rs | 40 | geo/atmo/bio + 16 pigments + water treatment |
