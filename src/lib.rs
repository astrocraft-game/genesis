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
