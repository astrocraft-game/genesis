# Genesis

Procedural universe and planet generation library in Rust. Produces galaxies, star systems, and terrestrial worlds with tile-level surface maps — all deterministic from a seed.

## Crates

| Crate | Description |
|-------|-------------|
| `cosmos` | Universe, galaxies, star systems, celestial bodies, orbits |
| `world` | Planetary interior, atmosphere, climate, geology, hydrology, biomes, resources, geological zones |
| `genesis` (root) | Bridges cosmos → world via adapters |

Related standalone repos:

| Repo | Description |
|------|-------------|
| `vitaeis` | Species, ecosystems, evolution, settlements, planetary timelines |
| `poiesis` | 750+ real-world material science recipes as a directed graph |

## Usage

```rust
use genesis::generate_world_with_surface;
use world::grid::GridResolution;

let world = generate_world_with_surface(
    &cosmos_body, 4.6, 1, false,
    world::prelude::LifeLevel::Sentient,
    GridResolution::Fast,
    "my_seed",
).expect("terrestrial body");

// world.surface has 20+ layers: elevation, temperature, biome,
// rivers, ocean currents, precipitation, koppen class, ...
```

## World surface layers

Each tile carries: elevation, plate ID, tectonic boundary, temperature (annual + summer + winter + 12-month), precipitation (annual + 12-month), humidity, wind, ocean currents, SST, flow accumulation, river discharge, drainage basin, biome (19 types), Koppen class (22 subtypes).

## Underground

Stratified geological columns per tile (regolith → sedimentary → metamorphic → igneous), ore deposits with purity and quantity, cave networks with aquifers, fluid resources (oil, gas, geothermal).

## Geological zones

10 zones (carbonatite, mafic intrusion, pegmatite, porphyry, laterite, sedimentary basin, heavy mineral sands, brine flat, volcanic vent, impact crater) drive rare resource placement. No single location accesses all zones.

## License

MIT
