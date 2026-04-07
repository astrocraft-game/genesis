# Genesis — v0.4 Roadmap: Rare & Exotic Materials

**Current state:** v0.3 complete (27/27 phases, ~386 tests). All tracks
(underground, resources, hazards, alien biology, crafting/factory) delivered.

**Goal:** Make the planet worth exploring. 60+ rare and exotic materials
spread across 10 geological zones force the player to build multiple bases.
Each zone has unique resources that feed into distinct recipe chains,
creating natural bottlenecks and trade routes.

---

## Geological Zones (motivate multi-base play)

| Zone | Tectonic Setting | Key Resources |
|------|-----------------|---------------|
| **Carbonatite Pipe** | Alkaline igneous, continental rift | Rare earths, niobium, thorium |
| **Layered Mafic Intrusion** | Cratonic margin, convergent | PGMs, chromium, vanadium |
| **Pegmatite Field** | Continental interior, granitic | Lithium, tantalum, beryllium, tin |
| **Porphyry / Subduction** | Convergent boundary | Copper, molybdenum, rhenium, gold |
| **Laterite / Tropical** | Weathered continental, equatorial | Cobalt, nickel, scandium, ion-adsorption REEs |
| **Sedimentary Basin** | Passive continental | Uranium, manganese, phosphate, graphite |
| **Heavy Mineral Sands** | Coastal, beach placer | Zirconium, hafnium, titanium, monazite |
| **Brine Flat / Evaporite** | Arid continental interior | Lithium (brine), boron, potassium |
| **Volcanic Vent** | Divergent / hotspot | Sulfur, arsenic, bismuth, geothermal |
| **Impact Crater** | Meteorite impact sites | Iridium anomaly, shocked quartz, nickel-iron |

---

## Track M — Materials (substances + geological placement)

### M1 — Rare earth elements (individual) ✅

- [x] Added 10 individual REE substances: Neodymium, Cerium, Lanthanum,
      Praseodymium, Samarium, Europium, Dysprosium, Gadolinium, Yttrium,
      Scandium. Plus Boron (element) and Bastnaesite (ore).
- [x] `RareEarthMix` kept as intermediate — produced by Monazite and
      Bastnaesite extraction recipes.
- [x] 3 separation recipes split RareEarthMix into light REEs (La, Ce,
      Pr, Nd), medium REEs (Sm, Eu, Gd), and heavy REEs (Dy, Y, Sc).
- [x] 4 REE product recipes: NdFeB magnet, SmCo magnet, REE phosphor,
      Al-Sc aerospace alloy.
- [x] 4 new product substances: NdFeBMagnet, SmCoMagnet, REEPhosphor,
      AlScAlloy.
- [x] Tests (4): Nd reachable from monazite (2+ steps), NdFeB magnet
      reachable (3+ steps), all 10 REEs have incoming edges, bastnaesite
      produces RareEarthMix.

### M2 — Platinum group metals ✅

- [x] Added 5 PGM substances: Palladium, Rhodium, Iridium, Osmium, Ruthenium.
      Plus PGMConcentrate intermediate, CatalyticConverter, FuelCellMembrane.
- [x] Extraction chain: Ni + Cu + H₂SO₄ → PGMConcentrate (1200°C, 48h).
- [x] 3 separation recipes from PGMConcentrate: Pt+Pd, Rh+Ir, Os+Ru.
- [x] 2 product recipes: three-way catalytic converter (Pt+Pd+Rh),
      PEM fuel cell membrane (Pt+C+PE).
- [x] Tests (3): all 6 PGMs have incoming edges, catalytic converter
      reachable from concentrate, concentrate reachable from nickel.

### M3 — Battery metals ✅

- [x] Added 6 new substances: Graphite, LithiumCarbonate, NMCCathodeRaw,
      GraphiteAnode, BatteryCell, BatteryElectrolyte. (Lithium, Spodumene,
      Cobalt, Nickel, Manganese already existed.)
- [x] Graphite extraction from carbon ore (metamorphic).
- [x] Li → LithiumCarbonate conversion recipe.
- [x] 4-step battery chain: NMC cathode (Li+Ni+Mn+Co), graphite anode,
      electrolyte, cell assembly — requires inputs from 4+ geological zones.
- [x] Tests (3): battery cell reachable from lithium (3+ steps), reachable
      from graphite, needs 3+ direct inputs (multi-zone forcing).

### M4 — Semiconductor materials ✅

- [x] Added 4 new substances: Arsenic, Polysilicon, SiliconWafer, MicroChip.
- [x] 4-step silicon chain: SilicaSand → metallurgical Si (1900°C) →
      polysilicon (Siemens, 1100°C) → wafer (Czochralski, 1420°C) →
      microchip (doping + lithography, vacuum, 72h).
- [x] Fixed GaAs recipe: Tin→Gallium + Sulfur→Arsenic (correct inputs).
- [x] Fixed GaN recipe: Tin→Gallium (correct input).
- [x] Tests (3): microchip reachable from SilicaSand (3+ steps), GaAs
      uses Gallium not Tin, GaN uses Gallium.

### M5 — Nuclear fuels ✅

- [x] Added 4 substances: Deuterium, Tritium, Helium3, FusionFuelPellet.
- [x] Deuterium extraction: 6400 L water → 1 unit deuterium (48h, bulk).
- [x] Tritium breeding: lithium + neutron (catalyst: NuclearFuelRod) →
      tritium + trace He-3 (720h, reactor-only).
- [x] Fusion fuel pellet: D + T → cryogenic pellet (−250°C, 100 atm).
- [x] Fixed D-T fusion reaction: uses FusionFuelPellet instead of
      HydrogenGas placeholder. Outputs He-3 + 500 steam (energy).
- [x] Tests (4): deuterium from water, tritium needs lithium, fuel pellet
      reachable from deuterium, fusion reaction uses pellet.

### M6 — Superalloy and refractory metals ✅

- [x] Added 5 substances: Hafnium, Bismuth, TurbineBlade,
      SuperconductingWire, CementedCarbideTool.
- [x] Single-crystal turbine blade: Ni + Re + Hf + Cr + Al → blade
      (1500°C, vacuum, 48h) — requires porphyry zone (Re) + heavy mineral
      sands (Hf).
- [x] NbTi superconducting wire: Nb + Ti + Cu (2000°C, 24h) — requires
      carbonatite zone (Nb).
- [x] Cemented carbide cutting tool: WC + Co (1400°C, 6h) — requires
      skarn zone (W) + laterite zone (Co).
- [x] Tests (3): turbine blade needs rhenium + hafnium, superconducting
      wire needs niobium, cemented carbide needs tungsten carbide.

### M7 — Advanced engineered materials ✅

- [x] Added 2 new substances: BoronNitride, SiliconCarbideCeramic.
      (Graphene, CarbonNanotube, Aerogel already existed with recipes.)
- [x] Graphene from graphite (exfoliation, 25°C, low yield) — second
      path alongside existing CVD from methane.
- [x] Boron nitride ceramic: B + N₂ at 1800°C, 50 atm (blast furnace+).
- [x] Silicon carbide ceramic: Si + C at 2500°C (electric arc tier).
- [x] Existing CNT (850°C methane + Fe) and Aerogel (supercritical
      drying) verified correct — no changes needed.
- [x] Tests (4): graphene from graphite, BN needs boron, SiC needs
      silicon, BN/SiC at blast furnace/electric arc tech tier.

---

## Track N — Geological zone placement

### N1 — Zone classification per tile ✅

- [x] `zones.rs` with `GeologicalZone` enum (11 variants: CarbonatitePipe,
      MaficIntrusion, PegmatiteField, PorphyrySubduction, LateriteTropical,
      SedimentaryBasin, HeavyMineralSands, BrineFlat, VolcanicVent,
      ImpactCrater, Common).
- [x] `classify_zones(grid)` assigns each land tile using plate type,
      boundary, biome, elevation, latitude, and coastal proximity.
      Priority-based first-match rules.
- [x] `ZoneMap` with `zone_counts()`, `tiles_in_zone()`, `distinct_zones()`.
- [x] Tests (9): correct size, ocean=Common, Earth has 4+ zones, no zone
      dominates, volcanic at divergent, porphyry at convergent+continental,
      laterite in tropics, brine in arid, tile indices correct.

### N2 — Zone-aware ore placement ✅

- [x] `zone_ores.rs` with `ZoneOreDeposit`, `ZoneOreMap`, and per-zone
      probability tables. Layered on top of common strata ores.
- [x] 10 zone ore tables: carbonatite (REE/Nb hosts), mafic (PGM hosts),
      pegmatite (Sn/gems), porphyry (Cu/Au/Mo), laterite (Ni/Co/Al),
      sedimentary (coal/oil/gas/salt), heavy sands (Ti/Zr), brine (salt),
      volcanic (S/Cu/Au), impact (Ni-Fe/gems). Common = no rare ores.
- [x] `generate_zone_ores(zone_map, seed)` rolls per-zone table per tile.
- [x] `deposits_of(resource)` query across entire map.
- [x] Tests (8): correct size, Common=empty, some deposits exist, zones
      match, purity bounded, deterministic, filter works, sedimentary
      has fossil fuels.

### N3 — Resource scanning integration ✅

- [x] `scan_query.rs` with `TileScanResult`, `OreDetail`, `query_tile`,
      `query_all`, `discovered_zones`, `discovered_ore_types`.
- [x] Unknown → nothing. SurfaceScan → zone. DeepScan → ore types.
      FullyMapped → purity + quantity.
- [x] Progressive discovery: `discovered_zones()` and `discovered_ore_types()`
      return only what the player has actually scanned.
- [x] Tests (8): unknown reveals nothing, surface reveals zone, deep
      reveals ore types (not details), full reveals purity+quantity,
      unscanned zones empty, progressive scanning, query_all count,
      unscanned ores empty.

---

## Track O — Recipe chains (connecting materials to products)

### O1 — REE processing chain ✅

- [x] Delivered in M1: Monazite/Bastnaesite → RareEarthMix → 3 separation
      recipes → 10 individual REEs → NdFeB magnet, SmCo magnet, phosphor.
- [x] Tests: Nd reachable from monazite (2+ steps), NdFeB (3+ steps),
      all 10 REEs have incoming edges, bastnaesite produces RareEarthMix.

### O2 — PGM processing chain ✅

- [x] Delivered in M2: Ni+Cu → PGMConcentrate → 3 separation recipes →
      Pt, Pd, Rh, Ir, Os, Ru → catalytic converter, fuel cell membrane.
- [x] Tests: all 6 PGMs reachable, catalytic converter from concentrate,
      concentrate from nickel.

### O3 — Battery manufacturing chain ✅

- [x] Delivered in M3: Li→LiCO₃, graphite→anode, NMC cathode (Li+Ni+Mn+Co),
      electrolyte, 4-step assembly → BatteryCell.
- [x] Tests: cell from lithium (3+ steps), cell from graphite, 3+ inputs.

### O4 — Semiconductor fabrication chain ✅

- [x] Delivered in M4: SilicaSand→Si→Polysilicon→Wafer→MicroChip (3+ steps).
      GaAs/GaN fixed to use Gallium+Arsenic.
- [x] Tests: chip from silica, GaAs uses gallium, GaN uses gallium.

### O5 — Fusion fuel chain ✅

- [x] Delivered in M5: Water→Deuterium, Li→Tritium, D+T→FusionFuelPellet,
      pellet→fusion reaction (5000°C, 10k atm).
- [x] Tests: deuterium from water, tritium needs lithium, pellet reachable,
      fusion uses pellet.

### O6 — Advanced material recipes ✅

- [x] Delivered in M7: Graphene from graphite (exfoliation), BN ceramic
      (1800°C, 50 atm), SiC ceramic (2500°C). CNT and Aerogel pre-existing.
- [x] Tests: graphene from graphite, BN needs boron, SiC needs silicon,
      BN/SiC at blast furnace/electric arc tier.

---

## Track P — Multi-base gameplay integration

### P1 — Zone scarcity balancing ✅

- [x] `max_zone_fraction(is_ocean)` — verifies no zone exceeds 30% of land.
- [x] `zones_within_radius(centre, radius, width)` — counts distinct zones
      reachable from a tile via Chebyshev distance with longitude wrap.
- [x] `scarcity_maintained(radius, width)` — verifies no tile can access
      all zones within the given radius (forces multi-base expansion).
- [x] Tests (3): no zone exceeds 30%, scarcity maintained at radius 3,
      larger radius finds >= zones.

### P2 — Inter-base logistics (DEFERRED — game engine concern)

Not part of genesis. Transport modes, convoys, and route costs belong in
the game engine, not the planet generation library.

### P3 — Tech tree gating by materials (DEFERRED — game engine concern)

Not part of genesis. Tech progression, unlocking, and gating belong in
the game engine, not the planet generation library.

---

## Execution order

**Phase 1 — Substances and zones:**
1. M1 (individual REEs)
2. M2 (PGMs)
3. M3 (battery metals)
4. M4 (semiconductors)
5. M5 (nuclear fuels)
6. M6 (superalloys)
7. M7 (exotics)
8. N1 (zone classification)
9. N2 (zone-aware ore placement)

**Phase 2 — Recipe chains:**
10. O1 (REE chain)
11. O2 (PGM chain)
12. O3 (battery chain)
13. O4 (semiconductor chain)
14. O5 (fusion fuel chain)
15. O6 (exotic chain)

**Phase 3 — Gameplay balance:**
16. N3 (scanning integration)
17. P1 (zone scarcity)
18. P2 (inter-base logistics)
19. P3 (tech tree gating)
