//! # Genesis
//!
//! Procedural universe and planet generation library. Produces galaxies,
//! star systems, and planets with tile-level surface maps — all
//! deterministic from a seed.

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
    clippy::filter_map_identity,
    clippy::absurd_extreme_comparisons,
    clippy::format_in_format_args,
    clippy::write_with_newline,
    clippy::print_with_newline
)]

#[macro_use]
extern crate lazy_static;

pub mod celestial_body;
pub mod celestial_disk;
pub mod contents;
mod display;
pub mod galaxy;
pub mod generator;
pub mod neighborhood;
pub mod orbital_point;
pub mod star;
pub mod system_generator;
pub mod system_types;
pub mod universe;
pub mod utils;

pub mod adapters;
pub use adapters::*;

pub mod prelude {
    pub use crate::celestial_body::gaseous::types::*;
    pub use crate::celestial_body::gaseous::GaseousBodyDetails;
    pub use crate::celestial_body::icy::types::*;
    pub use crate::celestial_body::icy::IcyBodyDetails;
    pub use crate::celestial_body::telluric::types::*;
    pub use crate::celestial_body::telluric::TelluricBodyDetails;
    pub use crate::celestial_body::traits::types::*;
    pub use crate::celestial_body::traits::*;
    pub use crate::celestial_body::types::ExternalBodyFacts;
    pub use crate::celestial_body::types::*;
    pub use crate::celestial_body::world::types::*;
    pub use crate::celestial_body::world::WorldGenerator;
    pub use crate::celestial_body::CelestialBody;
    pub use crate::celestial_disk::belt::types::*;
    pub use crate::celestial_disk::belt::CelestialBeltDetails;
    pub use crate::celestial_disk::ring::types::*;
    pub use crate::celestial_disk::ring::CelestialRingDetails;
    pub use crate::celestial_disk::types::*;
    pub use crate::celestial_disk::CelestialDisk;
    pub use crate::contents::elements::*;
    pub use crate::contents::types::*;
    pub use crate::galaxy::map::division::GalacticMapDivision;
    pub use crate::galaxy::map::division_level::GalacticMapDivisionLevel;
    pub use crate::galaxy::map::hex::types::SpaceCoordinates;
    pub use crate::galaxy::map::hex::GalacticHex;
    pub use crate::galaxy::map::types::*;
    pub use crate::galaxy::neighborhood::types::*;
    pub use crate::galaxy::neighborhood::GalacticNeighborhood;
    pub use crate::galaxy::types::*;
    pub use crate::galaxy::Galaxy;
    pub use crate::generator::types::*;
    pub use crate::neighborhood::types::*;
    pub use crate::neighborhood::StellarNeighborhood;
    pub use crate::orbital_point::types::*;
    pub use crate::orbital_point::OrbitalPoint;
    pub use crate::star::types::*;
    pub use crate::star::Star;
    pub use crate::system_types::*;
    pub use crate::universe::types::*;
    pub use crate::universe::Universe;
}

pub mod internal {
    pub use crate::celestial_body::moon::*;
    pub use crate::utils::conversion::ConversionUtils;
    pub use crate::utils::harmonics::OrbitalHarmonicsUtils;
    pub use crate::utils::math::MathUtils;
    pub use crate::utils::string::StringUtils;
    pub use log::*;
    pub use ordered_float::OrderedFloat;
    pub use seeded_dice_roller::*;
    pub use serde::{Deserialize, Serialize};
    pub use smart_default::SmartDefault;
    pub use std::fmt::Display;
    pub use std::mem::discriminant;
    pub use std::rc::Rc;
    pub use strum::IntoEnumIterator;
    pub use strum_macros::EnumIter;
}

pub mod types {
    pub use crate::system_types::*;
}

use generator::types::{GeneratedUniverse, GenerationSettings};
use generator::Generator;
use prelude::*;
use system_generator::generate_star_system;

pub fn generate_universe(settings: GenerationSettings) -> GeneratedUniverse {
    Generator::generate(settings)
}

pub fn generate_system(
    system_index: u16,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    sub_sector: &GalacticMapDivision,
    galaxy: &mut Galaxy,
) -> StarSystem {
    generate_star_system(system_index, coord, hex, sub_sector, galaxy)
}

pub fn generate_world_with_surface(
    body: &CelestialBody,
    star_age_gyr: f32,
    moon_count: u32,
    has_rings: bool,
    life_level: atlasis::world::types::LifeLevel,
    resolution: atlasis::world::grid::GridResolution,
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
    let surface = atlasis::world::grid::generate_surface_grid(
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
    pub input: atlasis::world::types::PlanetSimulationInput,
    pub profile: atlasis::world::types::PlanetGenerationProfile,
    pub interior: atlasis::world::types::PlanetInterior,
    pub detail: atlasis::world::types::PlanetaryDetail,
    pub surface: atlasis::world::grid::SurfaceGrid,
}
