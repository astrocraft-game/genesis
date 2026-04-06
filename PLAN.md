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

### F3 — Multi-stage processing chains ✅

- [x] `chain_length(from, to)` — shortest hop count between substances.
- [x] `processing_tier(substance)` — minimum hops from any raw material
      (raw = tier 0, pig iron = tier 1+, steel = tier 2+, etc.).
- [x] `find_bottleneck(from, to)` — recipe step with highest min_temp_c
      in the chain (the tech-gating constraint).
- [x] Existing `production_chain(from, to)` already returns the full
      recipe sequence (A* shortest path via petgraph).
- [x] Tests (5): chain length scales with complexity, raw materials are
      tier 0, processed substances have positive tier, bottleneck returns
      highest-temp step, every non-raw substance reachable from an input.

### F4 — Resource scanning & discovery ✅

- [x] `scanning.rs` with `ScanState` (Unknown, SurfaceScan, DeepScan,
      FullyMapped) and `ScanMap`.
- [x] `scan_tile` / `scan_region` advance state (never downgrade).
- [x] Visibility queries: `surface_visible()`, `underground_visible()`,
      `fully_mapped()` — game engine checks before showing data.
- [x] `explored_fraction()`, `unexplored_tiles()`, `counts()`.
- [x] Tests (9): fresh all unknown, scan advances, never downgrades,
      region scan, explored fraction, unexplored shrinks, fully mapped
      reveals all, surface hides underground, OOB safe.

---

## Track G — Environment & hazards

Goal: planets fight back — pollution, disasters, hostile zones.

### G1 — Pollution layer ✅

- [x] `pollution.rs` with `PollutionMap` (per-tile levels, width/height).
- [x] `emit(tile, amount)` adds pollution. `tick(diffusion, decay, vegetation)`
      spreads to 4-neighbours, decays naturally, and lets vegetation absorb.
- [x] `apply_pollution_degradation(grid, pollution)` degrades biomes:
      >0.7 → forest→grassland, savanna→xeric; >0.9 → barren.
      Ocean tiles unaffected.
- [x] Queries: `max_pollution`, `mean_pollution`, `polluted_tiles(threshold)`.
- [x] Tests (10): pristine fresh, emit increases, spreads to neighbours,
      decays over time, vegetation absorbs, forest→grassland, severe→barren,
      ocean unaffected, correct tile indices, OOB safe.

### G2 — Environmental hazard zones ✅

- [x] `hazards.rs` with `HazardFlags` (7 flags: toxic_atmosphere, radiation,
      acid_rain, extreme_cold, extreme_heat, seismic, high_altitude) and
      `HazardMap`.
- [x] `generate_hazards(grid, pollution)` derives flags from physics layers:
      tectonic boundary → seismic, temperature → cold/heat, elevation →
      altitude/radiation, volcanic biome → toxic, pollution → acid rain.
- [x] `danger_score()` (0.0–1.0), `is_safe()`, `tiles_with(predicate)`.
- [x] Tests (9): correct size, seismic at boundaries, cold matches temp,
      altitude matches elevation, some safe, some hazardous, score bounded,
      pollution triggers acid rain, filter works.

### G3 — Expanded natural disasters ✅

- [x] Added 4 new `EventKind` variants: `Supervolcano` (months-long,
      radius 5, volcanic tiles), `Tsunami` (offshore earthquake, radius 3),
      `SolarFlare` (global radiation spike), `AcidRainStorm` (arid regions).
- [x] Added `affected_radius: u16` field to `NaturalEvent` — tiles
      outward from epicentre affected (0 = epicentre only).
- [x] All existing events updated with appropriate radii.
- [x] Tests (4 new): supervolcanoes on volcanic tiles with long duration,
      tsunamis originate in ocean, solar flares bounded, radius bounded.

### G4 — Climate change simulation ✅

- [x] `climate_change.rs` with `ClimateShift` (temp delta, precip factor,
      sea level rise, tiles flooded) and `compute_climate_shift(pollution,
      deforestation)`.
- [x] `apply_climate_shift(grid, shift)` mutates temperature (annual +
      monthly), precipitation (annual + monthly), floods coastal tiles
      below new sea level, updates biome to Ocean.
- [x] Model: +6 °C per unit pollution, −20% precip per unit deforestation,
      +1 m sea level per °C warming.
- [x] Tests (8): zero pollution stable, high pollution warms, deforestation
      dries, temp shifts applied, precip scaled, coastal flooding, flooded
      tiles become ocean, sea level updates in grid.

---

## Track H — Alien biology

Goal: make alien ecosystems feel alive, dangerous, and worth studying.

### H1 — Species evolution timeline ✅

- [x] `evolution.rs` with `AdaptationEvent` (mya, kind, description),
      `AdaptationKind` (7 variants), `EvolutionHistory`.
- [x] `generate_evolution_history(species, timeline)` rationalises each
      species trait as a response to planetary events: body plan from
      Cambrian explosion, size from ice ages (Bergmann's rule), locomotion
      from land colonisation, trophic shift, special traits.
- [x] Post-extinction speciation bursts, near-extinction bottlenecks.
- [x] Tests (8): chronological, body plan event, trophic event, locomotion
      events, trait events, descriptions filled, name recorded, autotroph.

### H2 — Mutation and adaptation ✅

- [x] `mutation.rs` with `MutationTrigger` (6: Pollution, ClimateWarming,
      ClimateCooling, Disaster, RadiationExposure, HabitatLoss),
      `TraitChange` (7: GainedTrait, LostTrait, SizeIncrease, SizeDecrease,
      TempToleranceWidened, AggressionIncrease, Maladaptive), `Mutation`,
      `MutationLog`.
- [x] `roll_mutations(species, trigger, severity, gen, seed)` — severity
      scales mutation chance (0–60%). Per-trigger outcome tables.
- [x] `apply_mutations(species, mutations)` modifies traits/size/temp range.
- [x] `MutationLog` tracks maladaptive count (extinction risk) and gained traits.
- [x] Tests (9): zero severity = no mutation, high severity produces mutations,
      descriptions reference species name, gained trait applies, size shifts,
      temp tolerance widens, maladaptive count, size bounds, gained trait log.

### H3 — Ecological competition and niches ✅

- [x] `competition_links: Vec<(usize, usize, f32)>` on Ecosystem — species
      at the same trophic category (herbivore/omnivore, carnivore, filter)
      compete with overlap 0.0–1.0.
- [x] `niche_overlap(a, b)` reduces overlap by size class difference (−0.2
      per step) and non-shared locomotion (−0.4).
- [x] `parasitism_links: Vec<(usize, usize)>` — Parasite trophic level
      targets herbivore/omnivore/carnivore hosts.
- [x] Tests (4): herbivores compete, overlap bounded, niche differentiation
      reduces overlap, producers don't compete.

### H4 — Alien flora detail ✅

- [x] `flora.rs` with `PlantType` (6: CanopyTree, GroundCover, Aquatic,
      Fungal, MossLichen, Succulent), `HarvestYield` (6: Timber, Fiber,
      Resin, Spores, Fruit, Algae), `PlantLayer`, `TileFlora`, `FloraMap`.
- [x] `generate_flora(biomes, is_ocean)` assigns biome-appropriate plant
      mixes with densities and growth rates.
- [x] `harvest(fraction)` depletes density; `grow()` regenerates toward 1.0.
- [x] `available_yields()` lists what the player can harvest per tile.
- [x] Tests (9): tropical has canopy+ground, desert minimal, ocean empty,
      harvest depletes, growth regenerates, caps at 1.0, yields available,
      cleared layer excluded, tick growth across map.

### H5 — Creature behaviour tags ✅

- [x] `behaviour.rs` with `BehaviourProfile` (8 flags: territorial,
      migratory, nocturnal, burrowing, swarming, venomous, ambush, docile).
- [x] `generate_behaviour(species)` derives tags from body plan, trophic
      level, size class, locomotion, and special traits.
- [x] `threat_level()` counts danger tags; `is_dangerous()` threshold.
- [x] Rules: carnivores→territorial+ambush(large)/swarming(small arthropod),
      herbivores→migratory(large)/docile(small), venomous trait→venomous,
      burrower locomotion→burrowing, amorphous/mollusk/bioluminescent→nocturnal.
- [x] Tests (10): carnivore territorial, small arthropod swarms, large
      herbivore migratory, small herbivore docile, venomous maps, burrower
      maps, autotroph docile, amorphous nocturnal, bioluminescent nocturnal,
      threat level counts.

---

## Track I — Crafting & factory

Goal: make the crafting system factory-ready — energy, waste, throughput.

### I1 — Energy model ✅

- [x] `energy.rs` with `PowerSourceKind` (5: Geothermal, FossilFuel, Solar,
      Nuclear, Manual), `PowerSource` (capacity_kw + availability), `EnergyBudget`.
- [x] `estimate_recipe_energy_kj(recipe)` — heuristic from temperature,
      pressure, and duration (no schema change to 750+ recipes needed).
- [x] `EnergyBudget`: `add_source`, `add_recipe_demand`, `supply_kj`,
      `throughput_factor` (1.0 if balanced, <1.0 in deficit), `surplus_kj`.
- [x] Tests (10): empty no deficit, supply scales, deficit reduces
      throughput, throughput capped, availability reduces power, energy
      scales with temp/pressure, demand accumulates, reset, surplus.

### I2 — Byproduct and waste tracking ✅

- [x] Added `Wastewater` and `ToxicSludge` to Substance enum.
- [x] `waste.rs` with `is_waste(substance)` classifier, `WasteTracker`
      (HashMap stockpile), `record_recipe`, `add`, `remove`.
- [x] `pollution_pressure()` converts waste mass to 0.0–1.0+ value
      (toxic sludge weighted 3×), feeds into pollution layer.
- [x] 5 built-in `WasteProcessingRecipe`s: slag→concrete, CO₂→carbon
      capture, wastewater→clean water, tailings neutralisation, toxic
      sludge incineration.
- [x] Tests (10): empty tracker, accumulation, non-waste rejected, remove
      depletes, clamps to available, pressure scales, toxic weight higher,
      record captures byproducts, processing recipes exist, processing
      reduces pollution.

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
