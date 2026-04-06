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

### M4 — Semiconductor materials

- [ ] Add Arsenic substance (Silicon, Gallium, Germanium, Indium exist).
- [ ] Silicon wafer recipe: quartz → metallurgical Si → polysilicon →
      Czochralski crystal → wafer (4-step chain, electric arc tier).
- [ ] GaAs, GaN recipes fixed to use correct Gallium input (currently
      use Tin as placeholder — was flagged in Z4 audit).
- [ ] Tests: semiconductor chain reachable from quartz ore.

### M5 — Nuclear fuels

- [ ] Add Deuterium, Tritium, Helium3 substances (Uranium, Thorium,
      Plutonium exist).
- [ ] Deuterium extracted from water (electrolysis, high volume, low yield).
- [ ] Tritium bred from lithium-6 + neutron (reactor byproduct).
- [ ] Helium-3: available only on airless moons (special zone flag).
- [ ] Fusion fuel recipe: Deuterium + Tritium → energy (ties to Fusion
      power source in energy.rs).
- [ ] Tests: tritium needs lithium, He-3 restricted to airless bodies.

### M6 — Superalloy and refractory metals

- [ ] Add Hafnium, Bismuth substances (Rhenium, Niobium, Tantalum,
      Tungsten, Vanadium, Beryllium exist but lack placement).
- [ ] Place in correct zones: rhenium in porphyry Cu-Mo, niobium in
      carbonatites, tungsten in skarn/quartz veins.
- [ ] Superalloy recipes: Ni + Re + Hf → single-crystal turbine blade,
      Nb + Ti → superconducting wire, W + C → cemented carbide.
- [ ] Tests: refractory metals in correct geological zones.

### M7 — Advanced engineered materials

- [ ] Add 5 advanced substances: Graphene, CarbonNanotubes, Aerogel,
      SiliconCarbide, BoronNitride.
- [ ] Graphene: refined from ultra-pure graphite (plasma tier).
- [ ] Carbon nanotubes: CVD synthesis from methane + Fe catalyst (plasma).
- [ ] Aerogel: vacuum-processed silica gel (chemical reactor tier).
- [ ] SiC / BN: high-performance ceramics from silicon/boron (electric arc).
- [ ] These are endgame materials — require multi-zone supply chains.
- [ ] Tests: advanced materials require highest real-world tech tiers.

---

## Track N — Geological zone placement

### N1 — Zone classification per tile

- [ ] Add `GeologicalZone` enum (10 zones from table above) to strata.
- [ ] `classify_zone(grid, strata, tile_idx) -> GeologicalZone` using
      plate type, boundary, biome, elevation, latitude.
- [ ] `ZoneMap: Vec<GeologicalZone>` parallel to grid tiles.
- [ ] Tests: carbonatite only at rift/alkaline, laterite only tropical,
      brine only arid interior, anomaly only at precursor ruin tiles.

### N2 — Zone-aware ore placement

- [ ] Rewrite `strata::generate_strata` ore placement to use zone:
      each zone has its own ore probability table (replacing the current
      rock-type-only system).
- [ ] Zone → ore mapping follows the table above.
- [ ] Multiple ore types per tile (e.g., mafic intrusion gets PGMs +
      chromium + nickel together).
- [ ] Tests: ore types match zone, no REEs outside carbonatite/laterite,
      PGMs only in mafic intrusions.

### N3 — Resource scanning integration

- [ ] Surface scan reveals zone classification.
- [ ] Deep scan reveals specific ore types + quantities.
- [ ] Full scan reveals purity and exact deposit depth.
- [ ] Tests: scan level gates data visibility correctly.

---

## Track O — Recipe chains (connecting materials to products)

### O1 — REE processing chain

- [ ] Monazite ore → cracked concentrate → REE chloride solution →
      individual REE oxides (solvent extraction, 3-4 steps).
- [ ] Products: NdFeB magnets (Nd + Fe + B → permanent magnet),
      SmCo magnets (Sm + Co → high-temp magnet), phosphors (Eu, Tb, Y).
- [ ] Tests: magnets reachable from monazite, chain length ≥ 4.

### O2 — PGM processing chain

- [ ] Ni-Cu sulfide ore → matte → base-metal removal → PGM concentrate
      → individual PGM separation.
- [ ] Products: catalytic converters (Pt + Pd + Rh), fuel cells (Pt),
      crucibles (Ir), data storage (Ru).
- [ ] Tests: all 6 PGMs reachable from Ni-Cu ore.

### O3 — Battery manufacturing chain

- [ ] Lithium brine → Li₂CO₃ → LiCoO₂ cathode (+ cobalt).
- [ ] Graphite → anode material.
- [ ] Assembly: cathode + anode + electrolyte → battery cell.
- [ ] Tests: battery cell reachable from lithium + cobalt + graphite.

### O4 — Semiconductor fabrication chain

- [ ] Quartz → metallurgical Si → polysilicon → single crystal →
      wafer → doped wafer → chip.
- [ ] GaAs chain: gallium + arsenic → GaAs boule → wafer → LED/RF chip.
- [ ] Tests: chip reachable from quartz, GaAs from gallium.

### O5 — Fusion fuel chain

- [ ] Water → electrolysis → hydrogen + deuterium (isotope separation).
- [ ] Lithium-6 + neutron → tritium (breeder blanket recipe).
- [ ] D + T → fusion energy (connects to `Fusion` power source).
- [ ] Tests: fusion fuel reachable, energy output connects to budget.

### O6 — Advanced material recipes

- [ ] Graphene: graphite → exfoliation at plasma tier → graphene sheet.
- [ ] Carbon nanotubes: methane + Fe catalyst → CVD at plasma tier.
- [ ] Aerogel: silica + solvent → supercritical drying (chemical reactor).
- [ ] SiC: silicon + carbon → Acheson furnace (electric arc).
- [ ] BN: boron + nitrogen at high temp (blast furnace+).
- [ ] Tests: advanced material chain lengths are longest in the game,
      require inputs from multiple geological zones.

---

## Track P — Multi-base gameplay integration

### P1 — Zone scarcity balancing

- [ ] Tuning pass: ensure no single base location has access to all
      10 geological zones within a 3-tile radius.
- [ ] Each zone appears on ≤30% of land tiles.
- [ ] At least 2 zones require ocean-adjacent or polar locations.
- [ ] Tests: zone distribution per planet, no zone covers >30%.

### P2 — Inter-base logistics

- [ ] Extend `routing` module: `logistics_cost(zone_a, zone_b)` returns
      transport cost for moving materials between bases.
- [ ] Transport recipes: bulk hauler (low cost, slow), cargo drone
      (high cost, fast), pipeline (fluids only, permanent).
- [ ] Tests: logistics cost scales with distance, drone faster than hauler.

### P3 — Tech tree gating by materials

- [ ] Map each `TechTier` to the materials it requires:
      - Manual/Kiln: iron, copper, tin (common)
      - Furnace/Blast: steel alloys, tungsten (rare)
      - Electric Arc: silicon, nickel superalloys (very-rare)
      - Chemical Reactor: REEs, PGMs, lithium (very-rare, multi-zone)
      - Plasma: graphene, carbon nanotubes (ultra-rare, multi-zone)
      - Nuclear: deuterium, tritium, hafnium (requires fusion chain)
- [ ] Player cannot reach higher tiers without establishing bases in
      the correct geological zones.
- [ ] Tests: tier N+1 requires at least one material from a new zone.

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
