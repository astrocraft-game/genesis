//! # Genesis
//!
//! Procedural universe generation library. Produces galaxies, star systems,
//! and planets with tile-level surface maps — all deterministic from a seed.
//!
//! ## Architecture
//!
//! ```text
//! cosmos              world
//! ┌──────────┐   ┌──────────────┐
//! │ Universe │   │ SurfaceGrid  │
//! │ Galaxy   │   │  elevation   │
//! │ StarSys  │──>│  temperature │
//! │ Body     │   │  biome       │
//! └──────────┘   │  rivers ...  │
//!                └──────────────┘
//! ```
//!
//! - **cosmos** — universe, galaxies, star systems, celestial bodies.
//! - **world** — planetary interior, atmosphere, climate, geology, 20+
//!   tile-level layers (equirectangular, hex, cube-sphere).
//! - **genesis** (this crate) — bridges cosmos→world via `adapters`.

pub use cosmos;
pub use world;

pub mod adapters;
pub use adapters::*;

use cosmos::generator::types::{GeneratedUniverse, GenerationSettings};
use cosmos::generator::Generator;
use cosmos::prelude::*;
use cosmos::system_generator::generate_star_system;

/// Generate a complete universe from settings.
pub fn generate_universe(settings: GenerationSettings) -> GeneratedUniverse {
    Generator::generate(settings)
}

/// Generate a star system at a given coordinate within a galaxy.
pub fn generate_system(
    system_index: u16,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    sub_sector: &GalacticMapDivision,
    galaxy: &mut Galaxy,
) -> StarSystem {
    generate_star_system(system_index, coord, hex, sub_sector, galaxy)
}

/// Generate a fully-populated surface grid for a telluric body.
pub fn generate_world_with_surface(
    body: &cosmos::prelude::CelestialBody,
    star_age_gyr: f32,
    moon_count: u32,
    has_rings: bool,
    life_level: world::prelude::LifeLevel,
    resolution: world::grid::GridResolution,
    surface_seed: &str,
) -> Option<WorldWithSurface> {
    let generated = adapters::generate_world_from_cosmos_body(
        body,
        star_age_gyr,
        moon_count,
        has_rings,
        life_level,
    )?;
    let greenhouse_delta = generated
        .detail
        .greenhouse
        .as_ref()
        .map(|g| g.greenhouse_delta_k)
        .unwrap_or(0.0);
    let surface = world::grid::generate_surface_grid(
        &generated.input,
        greenhouse_delta,
        generated.interior.atmospheric_pressure,
        generated.interior.hydrosphere,
        resolution,
        surface_seed,
    );
    Some(WorldWithSurface {
        input: generated.input,
        profile: generated.profile,
        interior: generated.interior,
        detail: generated.detail,
        surface,
    })
}

#[derive(Clone, Debug)]
pub struct WorldWithSurface {
    pub input: world::prelude::PlanetSimulationInput,
    pub profile: world::prelude::PlanetGenerationProfile,
    pub interior: world::prelude::PlanetInterior,
    pub detail: world::prelude::PlanetaryDetail,
    pub surface: world::grid::SurfaceGrid,
}
