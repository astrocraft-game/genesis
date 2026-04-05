# Genesis — Post-v0.2 Roadmap

**Current state:** genesis 0.1.0, world 0.2.0, life 0.2.0 — surface-maps
pipeline complete (10 phases delivered, ~417 tests passing).

This plan lays out the next round of work, grouped into four tracks:

- **Track A: Fidelity & quality** — refine the physics we already have.
- **Track B: New systems** — civilisation, resources, history, events.
- **Track C: Engine integration** — make grids easier to consume.
- **Track D: Developer experience** — examples, benchmarks, tooling.

Tracks are independent; phases within a track are roughly sequential.
Each phase lists concrete deliverables and test criteria.

---

## Track A — Fidelity & quality

Goal: deepen the existing surface-maps pipeline without adding new subsystems.

### A1 — Replace custom value noise with proper simplex ✅

- [x] Added `noise = "0.9"` crate dependency to `world`.
- [x] Replaced `hash_noise_2d` in `geology.rs` with `noise::SuperSimplex`.
- [x] Added domain warping: two independent noise fields (warp_x, warp_y)
      perturb input coordinates by ±0.25 × warp at frequency 2.0.
- [x] Re-tuned amplitude from 800 m → 900 m for simplex gradient characteristics.
- [x] Added test verifying sub-plate elevation variety (>500 m spread).

### A2 — Hydraulic + thermal erosion

- [ ] Feature-flag `erosion` on the `world` crate (opt-in, expensive).
- [ ] Particle-based hydraulic erosion (10k-50k particles): each droplet
      slides downhill, picking up sediment proportional to slope and
      velocity, depositing on lower-gradient cells.
- [ ] Thermal erosion: iteratively spread material where slope exceeds
      the talus angle (~33° for rock, ~45° for sand).
- [ ] Run after base elevation, before sea-level binary search.
- [ ] Tests: erosion creates valleys (max-to-min elevation delta widens
      on land), rivers carved into geology, coarsening of coastlines.

### A3 — Seasonal monthly climate grids

- [ ] Expand `temperature_c` → per-month array `Vec<[f32; 12]>` or add
      `temperature_monthly: Vec<[f32; 12]>` alongside existing summer/
      winter means.
- [ ] Compute monthly insolation from axial tilt + orbital eccentricity
      + current month (simple orbital-mechanics).
- [ ] Monthly precipitation: zonal bands shift seasonally with the
      migrating ITCZ (follows solar declination).
- [ ] Enable full Köppen seasonal subtypes (Cwa, Csa, Dwb, Dsa…).
- [ ] Tests: Earth's axial tilt reproduces approx. seasonal Indian
      monsoon pattern; at tilt=0 seasons flatten.

### A4 — Biome palette expansion

- [ ] Add new `BiomeType` variants (behind `#[non_exhaustive]` which
      we already have):
      - `MediterraneanShrubland` (Csa/Csb climates)
      - `XericShrubland` (arid but not full desert)
      - `Mangrove` (tropical coastal wetland)
      - `Chaparral` / `Maquis`
      - `Steppe` (mid-lat dry grassland)
      - `ColdDesert` (e.g. Gobi, Patagonia)
- [ ] Refine Whittaker lookup with secondary axes (humidity, continentality).
- [ ] Mirror additions to life's `Biome` enum + adapter map.
- [ ] Update biome affinity tables in life/habitat.rs.

### A5 — Refined ocean dynamics

- [ ] Per-basin gyre centres (currently global 30° latitude assumption).
- [ ] Properly compute western/eastern boundary relative to each basin's
      longitudinal extent rather than immediate east/west land.
- [ ] El-Niño-equivalent periodic modulation (optional, monthly climate).
- [ ] Thermohaline circulation hint (dense cold polar water sinks).
- [ ] Tests: isolated basins get their own gyres, multi-basin worlds
      show plausible current asymmetry.

### A6 — Grid serialisation ✅

- [x] Added `serde = { version = "1.0", optional = true }` + `serde` feature
      flag in `world/Cargo.toml`.
- [x] `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]` on
      `SurfaceGrid`, `SurfaceLayers`, `Plate`, `GridResolution`, `BoundaryKind`,
      `PlateKind`, `BiomeType`, `KoppenClass`.
- [x] Feature-gated tests: `grid_round_trips_through_json` and
      `resolution_round_trips` verify full round-trip equality.
- [x] Default build unchanged — serde dep only pulled in with `--features serde`.

---

## Track B — New systems

Goal: build on the existing physical/life grids to model
civilisations, resources, history, and dynamic events.

### B1 — Resource layer ✅

- [x] Added `world/src/resources.rs` with `Resource` enum (20 variants:
      ores, minerals, fossil fuels, chemicals, biological, fresh water)
      and `ResourceMap` struct.
- [x] Per-tile resource derivation from plate kind + tectonic boundary +
      biome + river discharge:
      - Continental convergent → iron/copper/gold ore
      - Divergent → iron/tin ore
      - Volcanic biomes + convergent boundaries → sulfur/obsidian/gemstones
      - Continental sedimentary basins → coal (forests) / oil+gas (arid)
      - Forest biomes → timber, spices, herbs
      - Savanna/grassland → livestock, grain
      - Ocean tiles → fish + salt + limestone
      - Arid land → evaporite salt
      - High-discharge river tiles → fresh water
- [x] Root adapters: `resource_to_substances()` maps each Resource to one or
      more `crafting::Substance`s; `resource_map_to_substance_set()`
      returns the union of craftable substances for a whole world.
- [x] `ResourceMap` supports `count(resource)` and `distinct_resources()`.
- [x] 9 world tests + 1 adapter test covering all major rules.

### B2 — Settlement placement ✅

- [x] Added `life/src/settlement.rs` with `Settlement` struct and two
      public functions.
- [x] `compute_settlement_suitability()` combines per-tile habitability,
      water access, resource density, biome moderation, and elevation
      penalty into a multiplicative score (0–1). Ocean/unhabitable tiles
      score 0; extreme elevation (>4500 m) scores 0.
- [x] `place_settlements()` — greedy top-N selection with Chebyshev
      minimum-separation radius (longitude-wrapped), ordered by
      suitability, with population order derived from score (3-9).
- [x] Biome moderation table: TemperateForest/Grassland 1.0 → IceCap 0.0.
- [x] Root adapters: `water_access_from_grid()` (coastal + riverine) and
      `resource_density_from_map()` (normalised unique-resource count).
- [x] 10 life tests + 2 adapter tests, including end-to-end Earth-like
      world settlement placement using real SurfaceGrid + ResourceMap.

### B3 — Trade routes

- [ ] A* pathfinding between settlement pairs through the grid.
- [ ] Cost function: ocean segments cheaper (water trade), mountains
      and deserts expensive, rivers follow channel downstream.
- [ ] Per-route value: product of source + destination resource
      complementarity (what one has, the other lacks).
- [ ] Output: `TradeRoute { from: SettlementId, to: SettlementId,
      path: Vec<(u16, u16)>, value: f32 }`.
- [ ] Tests: coastal settlements route by sea, mountain ranges avoided,
      closed basins (endorheic) don't reach other basins by water.

### B4 — History generation (Dwarf-Fortress-lite)

- [ ] New file `life/src/history_sim.rs` — event-driven sim.
- [ ] Core event types: `Founding`, `War`, `Migration`, `Discovery`,
      `Catastrophe`, `GoldenAge`, `Contact`, `Schism`, `DynastyChange`.
- [ ] Entity types: `Civilization`, `Settlement`, `HistoricalFigure`,
      `Dynasty`, `Artifact`.
- [ ] Simulate N years of timeline with random events, each event
      recording participants + cause + effect.
- [ ] Retroactive rationalisation (Caves-of-Qud style): events generated
      first, narrative explanations synthesised from context.
- [ ] Deterministic from seed; produces `History { events, civs, figures }`.
- [ ] Tests: history length scales with sim years, events have
      believable causal chains, no time-paradoxes.

### B5 — Name generators (Markov chains) ✅

- [x] Added `life/src/naming.rs` with an order-2 Markov chain.
- [x] `NameStyle` enum with 5 bundled corpora: FantasyHuman (~50 words,
      medieval/Latin), Dwarvish (~45, consonant-heavy), Elvish (~47,
      vowel-rich), Norse (~47, saga names), Alien (~42, unusual clusters).
- [x] `MarkovNameGen::for_style()` builds a generator from the bundled
      corpus. `generate(&mut rng, min_len, max_len)` returns a capitalized
      name, retrying up to 8× to hit length bounds.
- [x] Deterministic from seed; empty corpus returns "Unnamed" gracefully.
- [x] 8 tests: all styles produce names, length bounds respected, same
      seed → same sequence, different seeds diverge, 100-sample diversity
      check (≥60 unique), corpus alphanumeric check, empty-corpus
      fallback, human vs alien phonology (vowel-ratio distinction).

### B6 — Named features ✅

- [x] Added `world/src/features.rs` with feature detection (no names,
      pure geography): `MountainRange`, `River`, `OceanBasin`, `Island`,
      `Desert`, all bundled in `Features`.
- [x] Algorithms:
      - Mountain ranges: flood-fill elevation > sea_level + 1500 m,
        minimum 3 tiles.
      - Rivers: trace from each ocean-adjacent high-discharge tile
        upstream via strictly-decreasing discharge, source-first.
      - Ocean basins: group ocean tiles by `drainage_basin_id`.
      - Islands: flood-fill contiguous land tiles, sorted by size.
      - Deserts: flood-fill tiles with biome=Desert, minimum 2 tiles.
- [x] All flood-fills 4-connected with longitude wrap.
- [x] Root adapter `name_features()` pairs each feature with a Markov-
      generated name (+ category suffix: "Mountains", "River", "Ocean",
      "Desert") using the chosen NameStyle. Deterministic from seed.
- [x] 9 world tests + 1 adapter test covering detection, river monotonic
      discharge, basin-id consistency, name suffixes, determinism.

### B7 — Weather events & disasters

- [ ] New file `world/src/events.rs` — episodic per-tile events.
- [ ] Event types:
      - **Volcanic eruption**: at convergent/divergent boundaries,
        chance scales with volcanism
      - **Earthquake**: at all tectonic boundaries
      - **Hurricane/cyclone**: tropical oceans >27°C SST
      - **Wildfire**: arid biomes during hot-summer seasons
      - **Drought**: below-average precipitation years
      - **Flood**: high-discharge rivers during wet seasons
      - **Meteorite impact**: random, low probability
- [ ] Event distribution: return Vec<Event> with location, magnitude,
      year, duration.
- [ ] Deterministic, based on grid state + seed.
- [ ] Tests: no volcanoes in plate interiors, hurricanes avoid cold SST,
      wildfires need both heat and fuel.

### B8 — Ecosystem dynamics

- [ ] Extend `life::ecosystem::Ecosystem` with:
      - `predator_prey_links: Vec<(SpeciesIdx, SpeciesIdx)>`
      - `trophic_pyramid_validity: bool`
- [ ] Migration routes: for each species, compute seasonal movement
      between summer and winter habitability peaks (when available).
- [ ] Keystone species: identify species whose removal would collapse
      the food web.
- [ ] Extinction triggers: sudden biome shift (from a B7 event) kills
      species whose habitability drops below 0.2.
- [ ] Tests: every carnivore has ≥1 prey, removal of keystone drops
      range counts, extinction events reproducible.

### B9 — Technology & culture (ties to crafting)

- [ ] Extend `life::HistoricalEra` with accessible tech list.
- [ ] Per-civilisation tech tree progression:
      - starting tech based on homeworld biome (metals available? wood?)
      - branching techs unlock based on available resources (from B1)
      - social structures (hunter-gatherer → agricultural → industrial)
- [ ] Output: `Civilisation { tech_level, known_recipes: Vec<&Recipe> }`.
- [ ] Bridge to `crafting::PlanetaryConditions` (already exists) and
      filter recipes by tech tier + available substances.
- [ ] Tests: stone-age civs can't craft steel, civs without iron ore
      can't progress past bronze age until trade unlocks it.

---

## Track C — Engine integration

Goal: make grids trivially consumable by game engines.

### C1 — Alternative grid layouts

- [ ] Add `HexGrid` using an icosahedron subdivision (offset coords).
- [ ] Add `CubeSphere` grid with 6 square faces.
- [ ] Trait `SurfaceSampler` with `sample_xyz(x,y,z) -> TileData`.
- [ ] Implement the trait for both equirectangular SurfaceGrid AND hex/
      cube variants.
- [ ] Conversion helpers: `equirect_to_hex(&SurfaceGrid) -> HexGrid`.
- [ ] Tests: same seed produces sphere-equivalent coverage across layouts.

### C2 — Texture export ✅

- [x] `grid.export_biome_rgb() -> Vec<u8>` with fixed 13-colour biome palette.
- [x] `grid.export_elevation_grayscale() -> Vec<u8>` (split scaling at sea level).
- [x] `grid.export_temperature_rgb() -> Vec<u8>` (blue-white-red colormap).
- [x] `grid.export_precipitation_rgb() -> Vec<u8>` (tan-to-blue colormap).
- [x] `grid.export_ocean_mask() -> Vec<u8>` (0 = land, 255 = ocean).
- [x] `grid.dimensions() -> (u16, u16)` helper for buffer shape.
- [x] No new dependencies — raw `Vec<u8>` outputs compatible with `image`
      crate, Bevy textures, or raw GL uploads at the caller's discretion.
- [x] 11 tests covering buffer sizes, colour invariants, and palette coverage
      of all biome variants.

### C3 — Sphere sampling API

- [ ] `SurfaceGrid::sample_xyz(x, y, z) -> usize` — normalized 3D → tile.
- [ ] `SurfaceGrid::sample_cube_face(face: CubeFace, u: f32, v: f32) -> usize`.
- [ ] `SurfaceGrid::tile_area_km2(row, planet_radius_km) -> f32`
      (already used internally, expose publicly).
- [ ] Tests: sample at known XYZ returns the expected tile; area sum
      equals planet surface area to ±1%.

### C4 — LOD system

- [ ] Hierarchical grids: start at Fast (72×36), refine to Detailed
      (360×180) on demand per region.
- [ ] `grid.zoom_region(lon_min, lat_min, lon_max, lat_max, factor)`
      returns a sub-grid with sub-tile detail.
- [ ] Uses noise from the original seed, deterministic.
- [ ] Tests: zoom region preserves climate continuity with parent grid.

### C5 — Query helpers

- [ ] `grid.biome_distribution() -> HashMap<BiomeType, f32>` — land frac.
- [ ] `grid.nearest_ocean_tile(lat, lon) -> (u16, u16)`.
- [ ] `grid.longest_river() -> (Vec<(u16,u16)>, f32_discharge)`.
- [ ] `grid.largest_mountain_range() -> Vec<(u16,u16)>`.
- [ ] `grid.total_land_area_km2(planet_radius_km) -> f32`.
- [ ] Tests: Earth-like distribution roughly 29% land, match human
      geography intuitions.

---

## Track D — Developer experience

Goal: make the library easier to learn, test, and contribute to.

### D1 — Example binaries ✅

- [x] `examples/single_planet.rs` — generates an Earth-like world,
      prints plate count, land/ocean split, elevation range, mean temp,
      precipitation, biome distribution.
- [x] `examples/export_maps.rs` — generates a Standard-resolution grid
      and writes 5 PPM/PGM files (biome, elevation, temperature,
      precipitation, ocean mask) using a built-in PPM writer (no image
      crate dependency).
- [x] `examples/species_ecosystem.rs` — generates ecosystem + prints
      food web by trophic level (producer/herbivores/predators/filter).
- [x] `examples/recipe_chain.rs` — looks up crafting recipes that produce
      or consume a given substance.
- [ ] `examples/universe_walk.rs` — deferred.

### D2 — Benchmarks

- [ ] Add `criterion = "0.5"` as dev-dependency.
- [ ] Benchmarks for: plate generation, flood-fill, D8 drainage,
      biome classification, full pipeline (Fast/Standard/Detailed).
- [ ] Track regressions per-commit in CI.

### D3 — Visualisation CLI

- [ ] New binary crate `genesis-cli` (or `examples/cli.rs`):
      - Print ASCII map of any layer (biome, temperature, etc.)
      - Print plate diagram with DOT export
      - Print species food web
      - Print trade routes between settlements
- [ ] No GUI, plain terminal output.

### D4 — Property-based testing

- [ ] Add `proptest = "1.5"` as dev-dependency.
- [ ] Generate random `PlanetSimulationInput`s, verify invariants:
      - temperature_c within plausible Kelvin conversion bounds
      - biome always set (no defaults left unassigned)
      - river discharge monotonically increases downstream
      - population density ≤ habitability
      - every enum variant reachable with some seed
- [ ] Run proptest in CI (separate job, longer runtime).

### D5 — Grid diff tool

- [ ] CLI command: `genesis diff grid_a.bin grid_b.bin`.
- [ ] Highlights cells where layers differ, summary statistics.
- [ ] Used for verifying refactors don't silently change output.

### D6 — Documentation

- [ ] Crate-level `//!` docs with architecture overview per crate.
- [ ] Example in every public function's docstring.
- [ ] Architectural diagrams in README.
- [ ] Tutorial-style walkthrough: "generate your first planet".

---

## Execution order (suggested)

Immediate (weeks 1-2):
1. **A1** (real simplex noise) — low-risk upgrade, visible quality bump.
2. **A6** (serde) — unlocks saving/loading grids for experimentation.
3. **C2** (texture export) — enables visual feedback.
4. **D1** (example binaries) — documents the API surface.

Short-term (weeks 3-6):
5. **A2** (erosion) — classic PCG upgrade, behind feature flag.
6. **A3** (seasonal climate) — unlocks full Köppen, monsoons.
7. **B1** (resources) — bridges crafting ↔ surface.
8. **B5** (name generators) — prerequisite for B6, B4.

Medium-term (weeks 7-12):
9. **B6** (named features) — makes worlds tangible.
10. **B2** (settlements) — foundation of civilisation layer.
11. **B7** (events & disasters) — adds dynamism.
12. **B9** (tech & culture) — ties crafting to civilisations.

Long-term (weeks 13+):
13. **B3** (trade routes) — needs settlements.
14. **B4** (history sim) — needs civilisations, events.
15. **B8** (ecosystem dynamics) — needs more species variety.
16. **C1** (alt grid layouts) — major refactor.
17. **C4** (LOD) — major refactor.

---

## Open questions

- **Crate for civilisation** — does the civilisation/history layer
  belong in a new `civilisation` crate, or extend `life`? Leaning
  toward new crate to keep life's scope tight.
- **Optional dependencies everywhere** — should every new subsystem
  be behind a feature flag? Probably yes for large additions (erosion,
  noise, texture export) but no for API surface additions (resources,
  named features).
- **Art direction for names** — do we bundle only real-world-language-
  based corpora or add fully invented phonologies?
- **How deep should history go?** Dwarf Fortress generates 100s of
  years. Is 50 events enough? 500?
- **GPU acceleration** — any phase worth porting to GPU (noise,
  erosion)? Deferred until performance is a bottleneck.

---

## What we are NOT planning to do

These are explicitly out of scope (feel free to reopen later):
- **Per-meter walkable surface detail** — that's the game engine's job.
- **GPU-accelerated generation** — CPU determinism is more important.
- **Real-time weather simulation** — events are discrete, episodic.
- **AI-generated text or names** — all generators remain deterministic.
- **Network sync / multiplayer** — single-client library only.
- **Save file format for game state** — grids are regeneratable from
  seed; users persist seeds, not grids.
