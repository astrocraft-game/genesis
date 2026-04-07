//! # World
//!
//! Planetary interior, atmosphere, climate, ocean, geology, and surface
//! modelling. Produces tile-level equirectangular grids with 20+ physical
//! layers: elevation, tectonic plates, temperature (annual + monthly),
//! wind, precipitation, humidity, ocean currents, SST, rivers, drainage
//! basins, biome (19 types), and Koppen climate class (22 subtypes).
//!
//! ## Pipeline
//!
//! The full surface-generation pipeline runs in dependency order:
//!
//! 1. **Geology** — plate tectonics, elevation, fractal noise, sea-level.
//! 2. **Erosion** (opt-in `erosion` feature) — particle hydraulic + thermal.
//! 3. **Temperature** — latitude insolation, elevation lapse, continentality.
//! 4. **Wind** — Hadley-cell bands, pressure scaling.
//! 5. **Precipitation** — orographic + latitude bands.
//! 6. **Ocean dynamics** — basin flood-fill, gyre currents, SST modifiers.
//! 7. **Hydrology** — D8 flow, accumulation, river discharge, basins.
//! 8. **Monthly climate** — 12-month temperature/precipitation arrays.
//! 9. **Biome classification** — Whittaker + Koppen with seasonal subtypes.
//!
//! Call `grid::generate_surface_grid()` for the one-shot pipeline, or
//! invoke each stage individually for finer control.
//!
//! ## Alternative layouts
//!
//! The `grids` module provides `HexGrid` (icosahedron subdivision) and
//! `CubeSphereGrid` (6-face), both implementing the `SurfaceSampler`
//! trait alongside `SurfaceGrid`.
//!
//! ## Features
//!
//! - `serde` — Serialize/Deserialize on all grid types.
//! - `erosion` — Enable hydraulic + thermal erosion (expensive).

#![warn(clippy::all)]
#![allow(dead_code, unused_imports, unused)]
// Architectural and stylistic lints we don't gate on: simulation pipelines
// take many parameters; cleaning remaining stylistic warnings is not in
// scope for the current work.
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::ptr_arg,
    clippy::if_same_then_else,
    clippy::manual_clamp,
    clippy::match_like_matches_macro,
    clippy::needless_late_init,
    clippy::nonminimal_bool,
    clippy::unnecessary_unwrap,
    clippy::absurd_extreme_comparisons
)]
// Pedantic lints are useful but produce high noise in numerical/game code.
// They can be re-enabled locally with `cargo clippy -- -W clippy::pedantic`.

pub mod atmosphere;
pub mod caves;
pub mod climate;
pub mod climate_change;
pub mod detail;
pub mod diff;
#[cfg(feature = "erosion")]
pub mod erosion;
pub mod events;
pub mod features;
pub mod fluids;
pub mod geology;
pub mod grid;
pub mod grids;
pub mod hazards;
pub mod hydrology;
pub mod impacts;
pub mod interior;
pub mod logistics;
pub mod ocean;
pub mod photochemistry;
pub mod pollution;
pub mod resource_nodes;
pub mod resources;
pub mod routing;
pub mod scan_query;
pub mod scanning;
pub mod strata;
pub mod subsurface;
pub mod surface;
pub mod terrain_log;
pub mod types;
pub mod zone_ores;
pub mod zones;

pub mod prelude {
    pub use crate::atmosphere::*;
    pub use crate::climate::*;
    pub use crate::detail::*;
    pub use crate::geology::*;
    pub use crate::hydrology::*;
    pub use crate::impacts::*;
    pub use crate::interior::*;
    pub use crate::ocean::*;
    pub use crate::photochemistry::*;
    pub use crate::subsurface::*;
    pub use crate::surface::*;
    pub use crate::types::*;
}
