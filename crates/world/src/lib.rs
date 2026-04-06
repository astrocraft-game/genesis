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
pub mod climate;
pub mod detail;
pub mod diff;
#[cfg(feature = "erosion")]
pub mod erosion;
pub mod events;
pub mod features;
pub mod geology;
pub mod grid;
pub mod grids;
pub mod hydrology;
pub mod impacts;
pub mod interior;
pub mod ocean;
pub mod photochemistry;
pub mod resources;
pub mod routing;
pub mod subsurface;
pub mod surface;
pub mod types;

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
