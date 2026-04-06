//! # Crafting
//!
//! Real-world material science recipes organised as a directed graph.
//! 750+ recipes across extraction, alloys, chemistry, construction,
//! fuel, and phase-change categories. Each recipe specifies inputs,
//! outputs, byproducts, minimum temperature, pressure, optional
//! catalyst, and duration.
//!
//! ## Key types
//!
//! - `Substance` — 100+ materials (elements, ores, alloys, compounds).
//! - `Recipe` — transformation with inputs/outputs and conditions.
//! - `PlanetaryConditions` — temperature/pressure/substance constraints
//!   that filter which recipes a civilisation can execute.
//! - `CraftingGraph` — petgraph DAG linking substances via recipes.
//!
//! Use `recipes::recipes_in_conditions()` to get the recipes available
//! at a given tech level and resource base.

#![allow(dead_code, unused_imports)]

pub mod energy;
pub mod food;
pub mod graph;
pub mod recipes;
pub mod waste;

pub use graph::CraftingGraph;
pub use recipes::substance::Substance;
pub use recipes::types::{Recipe, RecipeCategory};
