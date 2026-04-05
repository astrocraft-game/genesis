# Genesis — Surface Maps Roadmap

**Version target:** 0.1.0
**Last updated:** 2026-04-05

This plan lays out the full build-out of **tile-level surface grids** for
terrestrial worlds. It replaces the previous summary-stat approach
(`PlanetSurfaceMap`) with a stack of physically-motivated 2D layers that
downstream crates (life, apps) consume to decide where biomes, plants,
and species actually live.

---

## Guiding principles

### Separation of concerns

- **`world` crate** owns **physical** maps — everything derivable from
  physics, astronomy, and climate: elevation, temperature, pressure,
  humidity, wind, precipitation, sea-surface temperature, rivers, biome.
- **`life` crate** owns **biological** overlays — species habitability,
  territory, population density, plant-zone classification. It *consumes*
  the physical grid and produces biological interpretations.
- **No duplication**: the life crate never recomputes temperature or
  humidity; it reads the world grid and asks "is this tile habitable
  for species X?". Similarly the world crate never decides where jaguars
  go — it only produces the climate they'd need.
- **Adapter layer** (`src/adapters.rs`) holds any bridging logic that
  needs both crates.

### Determinism

Every layer is deterministic from the world's `(seed, body_id)`. Grids
at the same resolution produced from the same seed are byte-identical.

### Cost control

Grid generation is **opt-in** via a `surface_maps` feature flag. Default
world generation keeps producing the summary-stat `PlanetSurfaceMap` for
cheap batch runs. Apps that need tile maps pay the cost once per body.

### Grid layout

Equirectangular projection (lat/lon cells) at caller-chosen resolution:
- Fast: 72×36 (5° cells, 2,592 tiles)
- Standard: 144×72 (2.5° cells, 10,368 tiles)
- Detailed: 360×180 (1° cells, 64,800 tiles)

The cylindrical distortion is accepted — all callers agree on the
projection and handle polar area-weighting at query time.

---

## Data model

### `SurfaceGrid` (world crate)

```rust
pub struct SurfaceGrid {
    pub width: u16,   // longitude cells
    pub height: u16,  // latitude cells
    pub layers: SurfaceLayers,
}

pub struct SurfaceLayers {
    // Geology
    pub elevation_m: Vec<f32>,           // metres; negative for ocean
    pub plate_id: Vec<u8>,               // tectonic plate membership
    pub is_ocean: Vec<bool>,             // elevation_m < sea_level
    pub tectonic_boundary: Vec<BoundaryKind>, // divergent/convergent/transform/none

    // Climate — scalars
    pub temperature_c: Vec<f32>,         // mean annual surface temperature
    pub temperature_summer_c: Vec<f32>,  // peak summer month
    pub temperature_winter_c: Vec<f32>,  // coldest month
    pub precipitation_mm: Vec<f32>,      // annual mm
    pub humidity_relative: Vec<f32>,     // 0.0-1.0 annual mean
    pub pet_ratio: Vec<f32>,             // Precipitation / PET (Holdridge AI)

    // Climate — vectors
    pub wind_direction_deg: Vec<f32>,    // 0-360, meteorological convention
    pub wind_speed_ms: Vec<f32>,         // m/s
    pub ocean_current_direction_deg: Vec<f32>,
    pub ocean_current_speed_ms: Vec<f32>,
    pub sea_surface_temp_c: Vec<f32>,

    // Hydrology
    pub flow_accumulation: Vec<u32>,     // upstream tile count
    pub river_discharge_m3s: Vec<f32>,   // estimated volumetric flow
    pub drainage_basin_id: Vec<u16>,     // which basin this tile drains into

    // Classification
    pub biome: Vec<BiomeType>,
    pub koppen_class: Vec<KoppenClass>,
}
```

Index convention: `idx = lat_row * width + lon_col`, row 0 at the north
pole, increasing southward. Accessors `at(lon_col, lat_row)` and
`at_latlon(lat_deg, lon_deg)` handle wrapping and clamping.

### `LifeDistribution` (life crate)

```rust
pub struct LifeDistribution {
    pub ranges: Vec<SpeciesRange>,
    pub vegetation_density: Vec<f32>,       // 0.0-1.0 per tile
    pub primary_productivity: Vec<f32>,     // relative biomass per tile
}

pub struct SpeciesRange {
    pub species_name: Rc<str>,
    pub habitability: Vec<f32>,   // 0.0-1.0 suitability per tile
    pub territory: Vec<bool>,     // where species currently inhabits
    pub population_density: Vec<f32>,  // 0.0-1.0
}
```

Life consumes a borrowed `&SurfaceGrid` plus a species/ecosystem and
fills habitability by matching the species' preferred ranges against the
tile's temperature, humidity, elevation, biome, and hydrosphere state.

---

## Phase 1 — Geology layer

**Goal:** produce elevation and tectonic structure.

### 1.1 Plate tectonics via Voronoi

- Generate 8-40 plate seed points via Fibonacci spiral on the sphere,
  then project to equirectangular for the grid.
- Flood-fill plate membership (Voronoi on the grid) into `plate_id`.
- Per plate, roll: velocity vector, density (continental/oceanic),
  age (affects elevation).
- Classify each tile's `tectonic_boundary` by comparing its plate's
  motion with its neighbour's plate motion (convergent/divergent/
  transform/none).

### 1.2 Base elevation

- Start with plate altitude: oceanic plates at −3000 m baseline,
  continental at +400 m.
- Boundary modifiers:
  - Convergent continental-continental → mountain range (+2000–6000 m).
  - Convergent oceanic-continental → coastal range + trench.
  - Divergent → mid-ocean ridge or rift valley.
  - Transform → fault scarps.
- Overlay domain-warped fractal simplex noise (4-6 octaves, amplitude
  decaying 0.5×/octave, warped by a low-frequency secondary noise).
- Apply continental-shelf falloff near ocean/continent boundaries.

### 1.3 Erosion (optional feature)

- **Thermal erosion**: talus-angle spreading, 50-200 iterations.
- **Hydraulic erosion**: particle-based simulation, ~10k particles,
  carves valleys.
- Feature-flag `erosion` — gate behind it since it's the heaviest step.

### 1.4 Sea level

- Compute sea level from target hydrosphere fraction (interior.hydrosphere).
- Binary-search a threshold such that tiles below it cover the target
  ocean percentage, then set `is_ocean` and adjust `elevation_m`.

**Deliverables:** elevation_m, plate_id, is_ocean, tectonic_boundary.
**Tests:** plate count matches seed, elevation distribution is
plausible (20%-80% land consistent with hydrosphere), boundaries are
contiguous, determinism.

---

## Phase 2 — Temperature layer

**Goal:** mean annual and seasonal surface temperature per tile.

### 2.1 Latitude insolation

- Base temperature from latitude using `cos(lat)` insolation with the
  body's `blackbody_temp_k`.
- Axial tilt `orbit.axial_tilt_deg` modulates equator-to-pole gradient:
  high tilt → weaker gradient (hot summers everywhere), low tilt →
  strong gradient.

### 2.2 Elevation lapse

- Apply lapse rate −6.5 °C per 1000 m above sea level.
- Elevations below sea level ignored (use SST).

### 2.3 Continentality

- Compute distance-to-ocean via BFS from ocean tiles.
- Interior continental tiles swing ±5–15 °C seasonally; ocean-adjacent
  tiles stay within ±2–5 °C of annual mean.
- Produces `temperature_summer_c` and `temperature_winter_c`.

### 2.4 Ocean surface temperature

- SST = base latitude profile + warm western-boundary currents −
  cold eastern-boundary currents.
- Interpolate gyres over ocean basins (see Phase 5).

**Deliverables:** temperature_c, temperature_summer_c, temperature_winter_c.
**Tests:** equator warmer than poles, high elevation colder than sea
level at same latitude, continentality widens seasonal swing, tilt
scaling (0° → strong gradient; 70° → chaotic).

---

## Phase 3 — Atmospheric circulation

**Goal:** prevailing wind vector per tile.

### 3.1 Hadley/Ferrel/Polar cells

- Reuse Phase 4.2's `hadley_cells_per_hemisphere` count.
- 3-cell (Earth-like): trade-easterlies 0–30°, westerlies 30–60°,
  polar-easterlies 60–90°.
- Cell count changes band layout:
  - 1 cell (low tilt) → single Hadley circulation, easterlies everywhere.
  - 2 cells → Hadley + combined polar, no Ferrel.
  - 3 cells (Earth-like) → classic bands.
  - Chaotic (>54° tilt) → seasonally flipped, treat as turbulent mean.

### 3.2 Coriolis deflection

- Winds deflect right in northern hemisphere, left in southern.
- Apply per-cell deflection angle based on latitude.

### 3.3 Wind speed

- Scale by pressure gradient (stronger near band boundaries).
- Modifier for atmospheric pressure (thin atmosphere → weaker surface
  winds).

**Deliverables:** wind_direction_deg, wind_speed_ms.
**Tests:** NH trade winds blow NE→SW, westerlies blow SW→NE, speed
scales with atmospheric pressure.

---

## Phase 4 — Precipitation & humidity

**Goal:** mean annual precipitation and relative humidity per tile.

### 4.1 Zonal base from atmospheric cells

- Start from Phase 4.2's `zonal_precipitation_mm` as per-hemisphere
  band baselines.
- Expand to per-tile: each tile samples the band for its latitude.

### 4.2 Ocean proximity (moisture source)

- Distance-to-ocean (reuse from 2.3) modulates moisture availability.
- Exponential decay with prevailing-wind-weighted distance — ocean
  upwind of tile contributes more than downwind.

### 4.3 Orographic rain shadow

- For each land tile, trace upwind along `wind_direction_deg` for a
  few cells.
- If elevation rises sharply upstream → enhanced rainfall windward,
  suppressed leeward.
- Formula: `precipitation_mm *= 1 + upwind_elevation_gain / scale`
  on windward, `*= max(0.3, 1 - leeward_drop)` on leeward.

### 4.4 Holdridge PET ratio

- Compute potential evapotranspiration from biotemperature:
  `PET_mm = 58.93 × biotemperature_c` (Holdridge formula).
- `pet_ratio = precipitation_mm / PET_mm`.
- Derive relative humidity from PET ratio bounded to 0.0–1.0.

**Deliverables:** precipitation_mm, humidity_relative, pet_ratio.
**Tests:** ITCZ wet, subtropical dry, windward > leeward across
mountain ranges, oceanic islands wetter than continental interiors.

---

## Phase 5 — Ocean dynamics

**Goal:** SST and ocean-current field.

### 5.1 Ocean basins

- Flood-fill connected ocean tiles into `drainage_basin_id`.
- Basins smaller than a threshold merge with the nearest large basin.

### 5.2 Gyres

- Per basin, identify subtropical gyre centres (~30° latitude).
- Assign current direction perpendicular to radial vector from gyre
  centre (clockwise in NH, counterclockwise in SH).

### 5.3 Western-boundary intensification

- Ocean tiles on the west side of a basin (warm currents flowing
  poleward) get +3–8 °C SST boost.
- East-side boundaries (cold currents equator-ward) get −3–8 °C.

**Deliverables:** ocean_current_direction_deg, ocean_current_speed_ms,
sea_surface_temp_c.
**Tests:** gyres rotate correctly per hemisphere, western boundaries
warmer, equatorial SST > polar SST.

---

## Phase 6 — Hydrology

**Goal:** rivers, lakes, drainage basins on land.

### 6.1 D8 flow direction

- Per land tile, find the neighbour with steepest downslope.
- Sinks flagged as potential lakes.

### 6.2 Flow accumulation

- Topologically sort tiles by elevation, propagate unit water downstream.
- `flow_accumulation[tile]` = number of upstream tiles draining into it.

### 6.3 River discharge estimation

- Scale flow accumulation by upstream mean precipitation:
  `discharge_m3s = accumulation × mean_upstream_precip_mm × tile_area_km2 × 1e-3 / (365×86400)`.
- Threshold: tiles with discharge > 10 m³/s form a visible river.

### 6.4 Basin assignment

- `drainage_basin_id[tile]` = ID of the ocean outlet or endorheic lake
  it drains to.

**Deliverables:** flow_accumulation, river_discharge_m3s,
drainage_basin_id.
**Tests:** discharge increases downstream, no water disappears,
basins partition the land area.

---

## Phase 7 — Biome classification

**Goal:** assign `BiomeType` and `KoppenClass` to every tile.

### 7.1 Whittaker lookup

- Build a 2D lookup table keyed by `(temperature_c, precipitation_mm)`:
  - **Tropical rain forest**: T > 20 °C, P > 2000 mm
  - **Tropical seasonal forest / savanna**: T > 20 °C, 500–2000 mm
  - **Subtropical desert**: T > 18 °C, P < 300 mm
  - **Temperate rain forest**: 8–20 °C, P > 1500 mm
  - **Temperate seasonal forest**: 5–20 °C, 500–1500 mm
  - **Woodland / shrubland**: 5–20 °C, 200–500 mm
  - **Temperate grassland / cold desert**: −5–20 °C, 100–300 mm
  - **Boreal forest (taiga)**: −5–5 °C, 200–750 mm
  - **Tundra**: T < −5 °C, P < 250 mm
  - **Polar / ice desert**: T < −15 °C

### 7.2 Köppen-Geiger full classification

- Implement full Köppen rule ladder from monthly temp+precip data:
  - **A (tropical)**: all months ≥18 °C
    - Af, Am, Aw based on dry-month rainfall
  - **B (arid)**: P < threshold(T, seasonal distribution)
    - BW (desert), BS (steppe), BWh/BWk/BSh/BSk hot/cold
  - **C (temperate)**: coldest month 0–18 °C
    - Cfa, Cfb, Cfc, Cwa, Cwb, Csa, Csb
  - **D (continental)**: coldest month <0 °C, ≥1 month >10 °C
    - Dfa, Dfb, Dfc, Dfd, Dwa..., Dsa...
  - **E (polar)**: all months <10 °C
    - ET (tundra), EF (ice cap)

### 7.3 Elevation overlays

- High elevation (>2500 m) tiles override to `Alpine`.
- Very high (>4500 m) → `IceCap` regardless of latitude.
- Active volcanism → `Volcanic`.

**Deliverables:** biome, koppen_class (new enum).
**Tests:** Earth-like world reproduces ~Earth's biome distribution
(±15% per biome), hot deserts at 30° latitude, ice caps at poles.

---

## Phase 8 — Life distribution (life crate)

**Goal:** given a SurfaceGrid, compute species habitability and vegetation
density.

### 8.1 Vegetation density

- Primary productivity from Whittaker-style formula:
  `productivity ∝ min(temperature_factor, moisture_factor)`
- `vegetation_density[tile] = productivity × (1 - ice_cover) × biome_modifier`.

### 8.2 Species habitability

Per species, score each tile 0.0–1.0:
- **Temperature fit**: gaussian curve centred on `preferred_temp_range`.
- **Gravity fit**: 1.0 at preferred, dropping outside the ±40% band.
- **Hydrosphere fit**: depends on locomotion (Swimmer wants ocean,
  Walker wants land, Flyer wants coasts/plains).
- **Biome affinity**: e.g. PlantLike prefers forests, Amorphous prefers
  wetlands/oceans.
- Final `habitability = product_of_factors.clamp(0, 1)`.

### 8.3 Territory and population

- Select top N% habitable tiles as `territory`.
- `population_density` proportional to habitability × vegetation_density
  (for heterotrophs) or × productivity (for autotrophs).

### 8.4 Ecosystem range stacking

- For a full `Ecosystem`, stack ranges: producers first, herbivores
  constrained by producer density, predators constrained by herbivore
  density.

**Deliverables:** `LifeDistribution`, `SpeciesRange`, `distribute_ecosystem()`.
**Tests:** aquatic species avoid land, plant-like prefers wet warm
tiles, predator range ⊆ herbivore range, density conserved.

---

## Phase 9 — Adapter layer

**Goal:** wire surface grids through the root `generate_*` pipeline.

### 9.1 Feature-flag rollout

- Add `surface_maps` feature to `world` and `life` Cargo.toml.
- Add `surface_maps` passthrough to root `genesis` crate.

### 9.2 Root helpers

- `generate_world_with_surface(body, ...) -> (PlanetInterior, PlanetaryDetail, SurfaceGrid)`.
- `generate_life_on_surface(grid, ecosystem) -> LifeDistribution`.
- `generate_species_on_surface(grid, species) -> SpeciesRange`.

### 9.3 Grid resolution config

- Add `SurfaceGridResolution::{Fast, Standard, Detailed}` enum.
- Default to `Fast` (72×36) for interactive use.

**Deliverables:** feature flag, root helpers, resolution enum.
**Tests:** end-to-end Earth-like test producing a full grid + species
distribution, resolution scaling.

---

## Phase 10 — API stabilisation (road to 0.1.0)

- Freeze public types after Phases 1–9 complete.
- Add `#[non_exhaustive]` to `BiomeType`, `KoppenClass`, `BoundaryKind`.
- Version-bump each crate from 0.1.0 → 0.2.0 (surface maps addition).
- Update README with surface-maps feature examples.

---

## Execution order

1. **Phase 1** (geology) — foundation every other layer depends on.
2. **Phase 2** (temperature) — needed for precipitation, biome.
3. **Phase 3** (wind) — needed for rain shadow.
4. **Phase 4** (precipitation) — needed for biome, rivers.
5. **Phase 5** (ocean) — can run in parallel with 3–4.
6. **Phase 6** (hydrology) — needs 1, 2, 4.
7. **Phase 7** (biome) — needs 2, 4, 1.
8. **Phase 8** (life) — needs 7.
9. **Phase 9** (adapters) — wires it all together.
10. **Phase 10** (stabilisation) — final gate.

Phases 2/3/5 are parallelisable after 1. Phases 4/6/7 form a sequential
chain from precipitation to biome classification.

---

## Open design questions

- **Hex vs. equirectangular grid?** Starting with equirectangular
  (simplest math, standard image format). Hex/HEALPix can be added
  as an alternative layout later if distortion becomes a problem.
- **Single-frame or seasonal simulation?** Starting with annual means
  + summer/winter extremes. Monthly grids are a future extension.
- **How to hand off grids to game engines?** Grids are `Vec<T>` in
  row-major order — callers can map directly to image textures or
  game tile arrays. No wrapper abstractions in v1.
- **Plate-count sensitivity?** Too few plates → boring continents,
  too many → no coherent structure. Target 8–24 for Earth-size bodies,
  scaled by surface area.

---

## Glossary

- **Biotemperature**: mean annual temperature with values clamped to
  [0, 30] °C; used by Holdridge. Freezing months contribute 0.
- **PET**: potential evapotranspiration — how much water *could*
  evaporate given unlimited supply. `Precipitation/PET` is the
  Holdridge aridity index.
- **ITCZ**: Intertropical Convergence Zone — equatorial low-pressure
  belt where trade winds converge and rainfall peaks.
- **Rain shadow**: dry region on the leeward (downwind) side of a
  mountain range.
- **Gyre**: large rotating ocean current system; subtropical gyres
  centred at ~30° latitude.
- **Orographic lift**: air forced upward by a mountain, cooling and
  releasing moisture as rain on the windward side.
