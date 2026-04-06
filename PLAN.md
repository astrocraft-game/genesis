# Genesis — v0.3 Roadmap

**Current state:** genesis 0.1.0, world 0.2.0, life 0.2.0 — surface-maps
pipeline complete (all v0.2 phases delivered, ~240 tests passing).

**Context:** genesis generates alien worlds for a factory-management game
(Factorio / Satisfactory / Riftbreaker style). The player lands on a
procedurally-generated planet and must extract resources, build processing
chains, and expand. There are no human civilisations — "life" means alien
flora/fauna, environmental hazards, and the planet's deep geological
history. Technology comes from the player's progression through crafting
recipes, not from NPC societies.

This plan is grouped into five tracks:

- **Track E: Underground & geology** — depth, caves, ore distribution.
- **Track F: Resources & extraction** — depletion, fluids, processing chains.
- **Track G: Environment & hazards** — pollution, disasters, hazard zones.
- **Track H: Alien biology** — evolution, adaptation, ecological depth.
- **Track I: Crafting & factory** — recipe fidelity, energy, byproducts.

Plus a **Cleanup** section for gaps found in the v0.2 audit.

---

## Cleanup — Gaps from v0.2 audit

### Z1 — Unify settlement types ✅

- [x] Removed `HistoricSettlement` (was part of civ sim). Only
      `settlement::Settlement` (grid-based) remains.

### Z2 — History module rethink ✅

- [x] Removed: `Civilization`, `Dynasty`, `DynastyChange`, `HistoricalFigure`,
      `WorldEvent`, `History`, `HistoryParams`, `simulate_history`,
      `rationalise_events`.
- [x] Replaced `social_structure()` with `factory_stage()` on `HistoricalEra`
      (manual → kiln → furnace → blast-furnace → electric-arc → plasma → exotic).
- [x] Added `PlanetaryEventKind` (19 geological + biological event types),
      `PlanetaryEvent`, `PrecursorRuin`, `PlanetaryTimeline`.
- [x] `generate_planetary_timeline(star_age, has_oceans, has_life, tiles, seed)`
      produces chronological geological + biological history with optional
      precursor ruins (~20% on life-bearing worlds).
- [x] Kept: `HistoricalEra` + all tech/capability methods, `generate_species_history`.
- [x] 13 tests covering determinism, chronology, formation first, lifeless
      worlds skip bio events, precursor ruin validity, species history.

### Z3 — Name style variety ✅

- [x] Added 3 new `NameStyle` variants: `Crystalline` (mineral-sounding),
      `Fungal` (soft/sibilant), `Insectoid` (clicks/buzzes/staccato).
- [x] ~40 corpus words per new style.
- [x] `NameStyle::for_body_plan(BodyPlan)` maps species body plan to an
      appropriate name style (Arthropod→Insectoid, PlantLike→Fungal, etc.).
- [x] Tests: all 8 styles produce output, body plan selects distinct styles,
      insectoid is consonant-heavy, corpus words are alphabetic.

### Z4 — Recipe placeholder cleanup ✅

- [x] Added 24 new `Substance` variants: 13 elements (Antimony, Beryllium,
      Zirconium, Niobium, Tantalum, Cadmium, Indium, Gallium, Germanium,
      Selenium, Tellurium, Rhenium, Plutonium), 6 chemicals (Polystyrene,
      AcrylicResin, Aspirin, Penicillin, RareEarthMix), 5 fuels (Naphtha,
      LPG, PetroleumCoke, CoalGas, WoodPellets).
- [x] Fixed 20 critical recipe outputs in extraction.rs (13 metals),
      chemistry.rs (5: PP, PS, acrylic, aspirin, penicillin),
      fuel.rs (6: peat, wood pellets, coal gas, naphtha, LPG, petcoke),
      electrochemistry.rs (1: plutonium breeding).
- [x] Remaining ~20 minor placeholders (torrefied biomass → charcoal,
      DME → diesel, etc.) documented as intentional simplification where
      distinct output would add substance bloat without gameplay value.

---

## Track E — Underground & geology

Goal: give planets vertical depth — ore veins, cave networks, aquifers.

### E1 — Stratified geological layers ✅

- [x] `strata.rs` module with `RockType` (Sedimentary, Metamorphic, Ignite,
      Regolith, OreVein), `RockLayer`, `OreDeposit` (resource + purity +
      quantity_kt), `GeologicalColumn`, `StratifiedGeology`.
- [x] `generate_strata(grid, seed)` builds per-tile vertical columns.
- [x] Layer distribution driven by plate type (continental → thick
      sedimentary + granite; oceanic → thin crust + basalt), boundary
      kind (convergent → extra metamorphic; divergent → igneous intrusion),
      and elevation.
- [x] Ore placement: iron/copper/tin/aluminum/sulfur in igneous, coal/
      limestone/oil/salt in sedimentary, gems/gold in metamorphic.
- [x] Tests (8): every tile has column, positive depth, continental thicker
      than oceanic, ore in correct layers, deterministic, depth-ordered,
      purity 0–1, grid has >10 ore deposits.

### E2 — Cave system generation ✅

- [x] `caves.rs` module with `CaveRoom` (depth, volume, aquifer flag),
      `Tunnel` (from/to/length), `CaveOrigin` (Karst, LavaTube, Tectonic,
      Erosional), `CaveNetwork`, `CaveMap`.
- [x] `generate_caves(grid, strata, seed)` builds per-tile networks.
- [x] Generation driven by geology: karst in thick sedimentary, lava tubes
      near volcanic boundaries, tectonic fractures at transform boundaries.
- [x] Aquifers: water-filled rooms near tiles with significant river
      discharge (60% vs 5% base chance).
- [x] Spanning-tree connectivity with optional loop tunnels.
- [x] Tests (8): correct size, no ocean caves, some land caves, connected
      networks, aquifers near rivers, deterministic, depths within column,
      cave origin matches geology.

### E3 — Terrain mutability ledger ✅

- [x] `terrain_log.rs` with `ChangeKind` (8 variants: Mining, Dumping,
      Deforestation, Erosion, Construction, Pollution, Flooding, Terraforming),
      `ChangeEntry` (kind, tick, magnitude, note), `TileLog`, `TerrainLog`.
- [x] `TerrainLog::new(tile_count)` creates empty ledger.
      `record(tile, kind, tick, magnitude, note)` appends entries.
- [x] Query helpers: `total_magnitude(kind)`, `entries_of(kind)`,
      `last_tick()`, `modified_tiles()`, `global_total(kind)`.
- [x] Out-of-bounds records are silently ignored (safe for game engine).
- [x] Tests (8): fresh is empty, accumulation, magnitude per kind,
      global total, filter by kind, last tick, modified indices, OOB safety.

---

## Track F — Resources & extraction

Goal: make resources behave like real deposits — finite, variable quality,
requiring multi-step processing.

### F1 — Resource node model ✅

- [x] `resource_nodes.rs` with `ResourceNode` (resource, purity, quantity_kt,
      initial_quantity_kt, depth_m), `TileNodes`, `ResourceNodeMap`.
- [x] `extract(amount_kt) -> (usable_yield, waste)` depletes quantity,
      splits by purity. `is_spent()` / `remaining_fraction()` queries.
- [x] `generate_resource_nodes(strata)` lifts ore deposits from geological
      columns into a flat queryable map.
- [x] Query helpers: `global_remaining_kt`, `tiles_with_resource`,
      `active_node_count`, `total_node_count`.
- [x] Tests (8): node count matches strata, purity 0–1, quantity > 0,
      extract reduces quantity, extract clamps at zero, global sum correct,
      tile indices correct, planetary ore within plausible bounds.

### F2 — Fluid resources ✅

- [x] `fluids.rs` with `FluidKind` (Oil, NaturalGas, Geothermal),
      `FluidNode` (pressure, flow_rate, depth, remaining, permanent flag),
      `TileFluids`, `FluidMap`.
- [x] `generate_fluids(grid, strata, seed)` places geothermal vents at
      convergent/divergent boundaries (~40%), oil in thick sedimentary
      (~20%), gas co-located (~25%).
- [x] Geothermal: permanent, infinite supply, never depletes.
      Oil/gas: finite, pressure-driven depletion (flow rate drops with
      pressure as reservoir empties).
- [x] Tests (9): correct size, geothermal only near volcanics, oil only
      in sedimentary, geothermal is permanent, oil depletes, geothermal
      doesn't deplete, some tiles have fluids, deterministic, no ocean fluids.

### F3 — Multi-stage processing chains

- [ ] Enrich the `CraftingGraph` with explicit extraction→processing→output
      paths. Add `ExtractionRecipe` linking `ResourceNode` to `Substance`.
- [ ] Model processing tiers: raw ore → concentrate → metal → alloy → part.
- [ ] Each tier requires higher temperature/pressure (maps to tech level).
- [ ] Visualisation: `graph.shortest_chain(raw_iron, steel_plate)` returns
      the recipe sequence.
- [ ] Tests: every substance reachable from some raw resource, chain
      length increases with product complexity.

### F4 — Resource scanning & discovery

- [ ] `ScanState` per-tile: `Unknown | SurfaceScan | DeepScan | FullyMapped`.
- [ ] Surface scan reveals biome, elevation, surface resources.
      Deep scan reveals underground layers, ore nodes, caves.
- [ ] Progression mechanic: player starts with surface-only data,
      unlocks deep scanning via tech.
- [ ] Tests: fresh planet is all Unknown, scan reveals correct data.

---

## Track G — Environment & hazards

Goal: planets fight back — pollution, disasters, hostile zones.

### G1 — Pollution layer

- [ ] Per-tile `pollution: f32` that spreads via diffusion each tick.
- [ ] Sources: industrial settlements/factories (keyed to extraction rate
      and recipe byproducts).
- [ ] Effects: biome degradation (forest → grassland → barren at high
      pollution), species range contraction, ecosystem extinction cascade.
- [ ] Sinks: vegetation absorbs pollution (incentivises preserving forests).
- [ ] Tests: pollution spreads from source, decays with distance,
      high pollution triggers biome change.

### G2 — Environmental hazard zones

- [ ] `HazardMap` parallel to `ResourceMap`: per-tile flags for
      toxic atmosphere, radiation, acid rain, extreme cold/heat,
      seismic instability.
- [ ] Derived from existing layers: `photochemistry` → toxic/radiation,
      `temperature_c` → extreme cold/heat, `tectonic_boundary` → seismic.
- [ ] Affects: factory placement cost, equipment wear, species habitability.
- [ ] Tests: hazard flags consistent with underlying physics layers.

### G3 — Expanded natural disasters

- [ ] Extend `world::events` with richer disaster model:
      - Supervolcano: months-long eruption, global temperature drop.
      - Tsunami: triggered by offshore earthquake, affects coastal tiles.
      - Solar flare: radiation spike, electronics damage (high-tech hazard).
      - Acid rain storm: chemical damage in polluted regions.
- [ ] Each disaster has epicentre, radius, severity, duration.
- [ ] Disasters can destroy factory buildings (game-engine hook).
- [ ] Tests: disasters respect geographical constraints, severity bounded.

### G4 — Climate change simulation

- [ ] Given a pollution trajectory over N years, compute shifted
      temperature and precipitation maps.
- [ ] Greenhouse gas accumulation → global warming → ice cap retreat →
      sea level rise → coastal tile flooding.
- [ ] Deforestation → reduced precipitation in affected basin.
- [ ] Tests: zero pollution = stable climate, high pollution shifts
      temperature upward, sea level rises with warming.

---

## Track H — Alien biology

Goal: make alien ecosystems feel alive, dangerous, and worth studying.

### H1 — Species evolution timeline

- [ ] Per-species `EvolutionHistory`: sequence of adaptation events
      (body plan change, size shift, new locomotion, trophic shift).
- [ ] Generated from the planet's geological timeline: mass extinctions
      drive speciation bursts, climate shifts drive adaptation.
- [ ] Output: narrative text explaining why species X has trait Y.
- [ ] Tests: evolution events are chronological, traits match environment.

### H2 — Mutation and adaptation

- [ ] When environment changes (pollution, climate shift, disaster),
      species in affected tiles may mutate.
- [ ] `Mutation { trait: TraitChange, trigger: EventKind, generation: u32 }`.
- [ ] Mutations can make species more dangerous (larger, toxic, aggressive)
      or cause extinction (maladaptive).
- [ ] Tests: mutation only triggered by environmental change, traits shift
      in plausible direction.

### H3 — Ecological competition and niches

- [ ] Add competition links to ecosystem: two herbivores sharing the
      same biome compete for vegetation density.
- [ ] Niche differentiation: species with different `SizeClass` or
      `LocomotionType` compete less.
- [ ] Parasitism: parasite species reduce host population density.
- [ ] Tests: competition reduces both populations, niche separation
      allows coexistence.

### H4 — Alien flora detail

- [ ] Expand vegetation model: per-biome plant types (canopy trees,
      ground cover, aquatic, fungal) with growth rates.
- [ ] Player can harvest specific plant types for crafting inputs
      (timber, fiber, resin, spores).
- [ ] Deforestation: removing vegetation reduces precipitation downwind,
      increases erosion, opens tile for factory building.
- [ ] Tests: vegetation types match biome, harvesting depletes density.

### H5 — Creature behaviour tags

- [ ] Per-species `BehaviourProfile`: territorial, migratory, nocturnal,
      burrowing, swarming, venomous.
- [ ] Tags influence: factory defence requirements, resource access
      (burrowing creatures block mining), seasonal threat patterns.
- [ ] Generated from body plan + trophic level + biome.
- [ ] Tests: carnivores tend toward territorial, small arthropods toward
      swarming, tags consistent with biology.

---

## Track I — Crafting & factory

Goal: make the crafting system factory-ready — energy, waste, throughput.

### I1 — Energy model

- [ ] Per-recipe `energy_kj: f32` field — energy cost to execute.
- [ ] Energy sources: geothermal (permanent), fossil fuel (finite),
      solar (biome-dependent), nuclear (tech-gated).
- [ ] `EnergyBudget` struct: available power vs. factory demand.
- [ ] Deficit → recipes slow down or halt.
- [ ] Tests: total energy demand scales with factory size, deficit
      reduces throughput.

### I2 — Byproduct and waste tracking

- [ ] Every recipe already has `byproducts` field — ensure it's populated
      for all 750+ recipes (many are currently empty).
- [ ] Waste substances: slag, tailings, CO2, wastewater, toxic sludge.
- [ ] Unmanaged waste accumulates → feeds pollution layer (G1).
- [ ] Waste processing recipes: turn slag into aggregate, CO2 into
      carbon capture, wastewater into clean water.
- [ ] Tests: industrial recipes produce waste, waste processing reduces
      pollution.

### I3 — Recipe tech tiers

- [ ] Explicit `tech_tier: u8` on every recipe (currently implicit via
      temperature/pressure thresholds).
- [ ] Tiers: 0=manual, 1=kiln, 2=furnace, 3=blast furnace, 4=electric
      arc, 5=chemical reactor, 6=plasma, 7=nuclear, 8=exotic.
- [ ] Player's factory has a current max tier; filters available recipes.
- [ ] Tests: tier monotonically increases with temperature requirement,
      all recipes have a tier assigned.

### I4 — Throughput and logistics model

- [ ] Per-recipe `throughput_kg_per_hour: f32` — base production rate.
- [ ] Logistics: transport cost between factory nodes uses the existing
      `routing::trade_cost` pathfinding.
- [ ] Bottleneck detection: `graph.find_bottleneck(output_substance)`
      returns the rate-limiting recipe in the chain.
- [ ] Tests: bottleneck is the slowest recipe in the chain, throughput
      scales with parallel instances.

### I5 — Rare and exotic materials

- [ ] Add 15-20 new substances: rare earths (neodymium, cerium, lanthanum),
      semiconductors (silicon wafer, gallium arsenide), pharmaceuticals
      (antibiotic, stimulant), exotic (dark matter, zero-point crystal).
- [ ] Rare earths found only on specific geological configurations.
- [ ] Exotic materials found at precursor ruin sites or anomaly tiles.
- [ ] Tests: new substances reachable via recipe chains, exotic substances
      gated behind high tech tier.

---

## Execution order (suggested)

**Phase 1 — Foundation fixes (cleanup):**
1. Z1 (unify settlements), Z2 (rethink history), Z3 (name styles)

**Phase 2 — Underground & resources:**
2. E1 (geological layers) — prerequisite for everything underground
3. F1 (resource nodes) — depletion model
4. E2 (caves) — needs E1
5. F2 (fluid resources) — needs F1

**Phase 3 — Factory core:**
6. I1 (energy model)
7. I2 (byproducts/waste)
8. I3 (recipe tech tiers)
9. F3 (processing chains)

**Phase 4 — Environment:**
10. G1 (pollution) — needs I2
11. G2 (hazard zones)
12. G3 (expanded disasters)

**Phase 5 — Alien biology:**
13. H1 (evolution timeline)
14. H5 (creature behaviour)
15. H4 (flora detail)
16. H2 (mutation)

**Phase 6 — Polish:**
17. G4 (climate change)
18. F4 (resource scanning)
19. H3 (ecological competition)
20. I4 (throughput/logistics)
21. I5 (rare/exotic materials)
22. E3 (terrain ledger)
23. Z4 (recipe placeholders)
