//! # World
//!
//! Planetary interior, atmosphere, climate, ocean, geology, and surface
//! modelling. Produces tile-level equirectangular grids with 20+ physical
//! layers: elevation, tectonic plates, temperature (annual + monthly),
//! wind, precipitation, humidity, ocean currents, SST, rivers, drainage
//! basins, biome (19 types), and Koppen climate class (22 subtypes).
//!
//! ## Module structure
//!
//! - `grid` — SurfaceGrid, alternative layouts (hex, cube-sphere), diff tool.
//! - `geology` — plate tectonics, elevation, erosion, strata, caves.
//! - `climate` — temperature, wind, precipitation, monthly, biomes, events.
//! - `hydrology` — rivers, drainage, ocean dynamics.
//! - `resources` — surface resources, resource nodes, fluids, zones, zone ores.
//! - `surface` — named features, hazards, routing.
//! - `interior` — atmosphere, photochemistry, impacts, subsurface, detail.
//! - `types` — shared type definitions.
//!
//! ## Features
//!
//! - `serde` — Serialize/Deserialize on all grid types.
//! - `erosion` — Enable hydraulic + thermal erosion (expensive).

#![warn(clippy::all)]
#![allow(dead_code, unused_imports, unused)]
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

pub mod climate;
pub mod geology;
pub mod grid;
pub mod hydrology;
pub mod interior;
pub mod resources;
pub mod surface;
pub mod types;

// Re-exports for backward compatibility
pub use climate::events;
pub use geology::caves;
#[cfg(feature = "erosion")]
pub use geology::erosion;
pub use geology::strata;
pub use grid::alt as grids;
pub use grid::diff;
pub use hydrology::ocean;
pub use interior::atmosphere;
pub use interior::detail;
pub use interior::impacts;
pub use interior::photochemistry;
pub use interior::subsurface;
pub use resources::fluids;
pub use resources::nodes as resource_nodes;
pub use resources::zone_ores;
pub use resources::zones;
pub use surface::features;
pub use surface::hazards;
pub use surface::routing;

pub mod prelude {
    pub use crate::climate::*;
    pub use crate::geology::*;
    pub use crate::hydrology::ocean::*;
    pub use crate::hydrology::*;
    pub use crate::interior::atmosphere::*;
    pub use crate::interior::detail::*;
    pub use crate::interior::impacts::*;
    pub use crate::interior::photochemistry::*;
    pub use crate::interior::subsurface::*;
    pub use crate::interior::*;
    pub use crate::surface::*;
    pub use crate::types::*;
}
