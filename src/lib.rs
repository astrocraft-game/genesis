/// Genesis - procedural universe generation library
///
/// Re-exports four crates:
/// - `cosmos` - universe, galaxy, star system, and orbital mechanics
/// - `world` - planetary interior, atmosphere, climate, ocean, geology, and surfaces
/// - `life` - species generation, history, and expansion
/// - `crafting` - 750+ real-world material science recipes with dependency graph

pub use cosmos;
pub use crafting;
pub use life;
pub use world;

pub mod adapters;
pub use adapters::*;

use cosmos::generator::Generator;
use cosmos::generator::types::{GeneratedUniverse, GenerationSettings};
use cosmos::system_generator::generate_star_system;
use cosmos::prelude::*;

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
