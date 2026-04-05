#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub mod biological;
pub mod cheese;
pub mod fermentation;
pub mod oils;
pub mod spice;

use crate::recipes::types::Recipe;

pub fn all_food_recipes() -> Vec<&'static Recipe> {
    let mut all = Vec::new();
    all.extend(biological::BIOLOGICAL_RECIPES.iter());
    all.extend(cheese::CHEESE_RECIPES.iter());
    all.extend(fermentation::FERMENTATION_RECIPES.iter());
    all.extend(oils::OIL_RECIPES.iter());
    all.extend(spice::SPICE_RECIPES.iter());
    all
}
