//! # Genesis
//!
//! Procedural universe generation library for games. Produces galaxies,
//! star systems, planets with tile-level surface maps, species with
//! ecosystems, civilisation histories, and material-science crafting
//! recipes — all deterministic from a seed.
//!
//! ## Architecture
//!
//! ```text
//! cosmos          world              life            crafting
//! ┌──────────┐   ┌──────────────┐   ┌────────────┐  ┌───────────┐
//! │ Universe │   │ SurfaceGrid  │   │ Species    │  │ Recipe    │
//! │ Galaxy   │   │  elevation   │   │ Ecosystem  │  │ Substance │
//! │ StarSys  │──>│  temperature │──>│ Settlement │  │ Graph     │
//! │ Body     │   │  biome       │   │ History    │  │           │
//! └──────────┘   │  rivers ...  │   └────────────┘  └───────────┘
//!                └──────────────┘         │                │
//!                       └────────────────>│<───────────────┘
//!                            adapters (this crate)
//! ```
//!
//! - **cosmos** — universe, galaxies, star systems, celestial bodies.
//! - **world** — planetary interior, atmosphere, climate, geology, 20+
//!   tile-level layers (equirectangular, hex, cube-sphere).
//! - **life** — species, ecosystems, settlements, Markov naming,
//!   DF-lite world history, expansion footprints.
//! - **crafting** — 750+ real-world material recipes as a DAG.
//! - **genesis** (this crate) — bridges the four crates via `adapters`.
//!
//! ## Quick start: generate your first planet
//!
//! ```rust,no_run
//! use cosmos::prelude::*;
//! use world::grid::GridResolution;
//!
//! // 1. Create a celestial body (or pull one from a generated star system).
//! let body = CelestialBody::new(
//!     Some(Orbit {
//!         average_distance: 1.0,
//!         average_distance_from_system_center: 1.0,
//!         eccentricity: 0.017,
//!         axial_tilt: 23.4,
//!         rotation: 1.0,
//!         ..Default::default()
//!     }),
//!     7, "Gaia".into(), 1.0, 1.0, 5.5, 1.0, 288, 0,
//!     CelestialBodySize::Standard,
//!     CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
//!         TelluricBodyComposition::Rocky,
//!         CelestialBodyWorldType::Terrestrial,
//!         Vec::new(),
//!         CelestialBodyCoreHeat::ActiveCore,
//!         MagneticFieldStrength::Strong,
//!         Vec::new(), Vec::new(), 10.0, true, 65.0,
//!     )),
//! );
//!
//! // 2. Generate the full world (interior + detail + surface grid).
//! let result = genesis::generate_world_with_surface(
//!     &body, 4.6, 1, false,
//!     world::prelude::LifeLevel::Sentient,
//!     GridResolution::Fast,  // 72x36 tiles
//!     "my_seed",
//! ).expect("telluric body");
//!
//! // 3. Query the surface grid.
//! let grid = &result.surface;
//! println!("Tiles: {}", grid.tile_count());
//! println!("Sea level: {:.0} m", grid.sea_level_m);
//! let dist = grid.biome_distribution();
//! for (biome, frac) in &dist {
//!     println!("  {:?}: {:.1}%", biome, frac * 100.0);
//! }
//!
//! // 4. Generate an ecosystem and distribute it on the surface.
//! let eco_input = life::SpeciesGenerationInput {
//!     habitat: life::types::Habitat::Terrestrial,
//!     climate: life::types::Climate::Terrestrial,
//!     temperature: life::types::Temperature::Temperate,
//!     gravity: 1.0, atmospheric_pressure: 1.0, hydrosphere: 71.0,
//!     life_level: life::types::LifeLevel::Sentient,
//!     seed: "my_seed".into(), scope_key: "eco".into(),
//! };
//! let eco = life::generate_ecosystem_from_world(&eco_input);
//! let life_dist = genesis::generate_life_on_surface(
//!     grid, &eco, 1.0, life::LifeLevel::Sentient,
//! );
//! println!("Species: {}, Ranges: {}", eco.species_count(), life_dist.ranges.len());
//! ```
//!
//! See `examples/` for more: `single_planet`, `export_maps`,
//! `species_ecosystem`, `recipe_chain`, `visualise`, `grid_diff`.

pub use cosmos;
pub use crafting;
pub use life;
pub use world;

pub mod adapters;
pub use adapters::*;

use cosmos::generator::types::{GeneratedUniverse, GenerationSettings};
use cosmos::generator::Generator;
use cosmos::prelude::*;
use cosmos::system_generator::generate_star_system;

/// Generate a complete universe from settings (universe → neighborhood → galaxies).
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

/// Generate a species for a cosmos body by routing it through world
/// simulation and then into the life crate. Returns `None` if the body
/// is not telluric or if life has not reached the animal-like threshold.
pub fn generate_species_for_world(
    body: &CelestialBody,
    star_age_gyr: f32,
    moon_count: u32,
    has_rings: bool,
    world_life_level: world::prelude::LifeLevel,
    seed: &str,
    scope_key: &str,
) -> Option<life::Species> {
    let generated = adapters::generate_world_from_cosmos_body(
        body,
        star_age_gyr,
        moon_count,
        has_rings,
        world_life_level,
    )?;
    let species_input = adapters::planetary_detail_to_species_input(
        &generated.input,
        &generated.interior,
        seed,
        scope_key,
    );
    life::generator::generate_species_from_world(&species_input)
}

/// Generate a fully-populated surface grid for a telluric body.
///
/// Composes the full world pipeline (interior + detail + surface grid)
/// in one call. Returns `None` if the body is not telluric.
///
/// `resolution` controls grid size; `surface_seed` is the string seed used
/// for deterministic plate/noise generation (independent of the universe seed).
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

/// Composite output of `generate_world_with_surface`: the world's interior,
/// detail, input context, generation profile, and full surface grid.
#[derive(Clone, Debug)]
pub struct WorldWithSurface {
    pub input: world::prelude::PlanetSimulationInput,
    pub profile: world::prelude::PlanetGenerationProfile,
    pub interior: world::prelude::PlanetInterior,
    pub detail: world::prelude::PlanetaryDetail,
    pub surface: world::grid::SurfaceGrid,
}

/// Compute life distribution (vegetation + species ranges) over a surface
/// grid for a given ecosystem.
pub fn generate_life_on_surface(
    surface: &world::grid::SurfaceGrid,
    ecosystem: &life::Ecosystem,
    gravity_g: f32,
    life_level: life::LifeLevel,
) -> life::LifeDistribution {
    let habitat = adapters::surface_grid_to_habitat_grid(surface);
    life::distribute_ecosystem(ecosystem, gravity_g, &habitat, life_level)
}

/// Compute a single species' range over a surface grid.
pub fn generate_species_on_surface(
    surface: &world::grid::SurfaceGrid,
    species: &life::Species,
    gravity_g: f32,
    life_level: life::LifeLevel,
) -> life::SpeciesRange {
    let habitat = adapters::surface_grid_to_habitat_grid(surface);
    let (_, vegetation) = life::generate_vegetation(&habitat, life_level);
    life::compute_species_range(species, gravity_g, &habitat, &vegetation)
}
