#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code, unused_imports, unused)]

pub mod atmosphere;
pub mod climate;
pub mod detail;
pub mod geology;
pub mod hydrology;
pub mod impacts;
pub mod interior;
pub mod ocean;
pub mod photochemistry;
pub mod subsurface;
pub mod surface;
pub mod types;

pub mod prelude {
    pub use crate::atmosphere::*;
    pub use crate::climate::*;
    pub use crate::detail::*;
    pub use crate::geology::*;
    pub use crate::hydrology::*;
    pub use crate::interior::*;
    pub use crate::impacts::*;
    pub use crate::ocean::*;
    pub use crate::photochemistry::*;
    pub use crate::subsurface::*;
    pub use crate::surface::*;
    pub use crate::types::*;
}
