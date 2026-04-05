//! Genesis - procedural universe generation library.
//!
//! Re-exports four crates:
//! - `cosmos` - universe, galaxy, star system, and orbital mechanics
//! - `world` - planetary interior, atmosphere, climate, ocean, geology, and surfaces
//! - `life` - species generation, history, and expansion
//! - `crafting` - 750+ real-world material science recipes with dependency graph

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
