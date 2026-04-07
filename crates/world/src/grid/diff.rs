//! Grid diff: compare two `SurfaceGrid`s layer-by-layer.
//!
//! Produces per-layer statistics (count of differing tiles, mean/max delta
//! for continuous layers, changed-variant count for discrete layers) so
//! refactors can be verified not to silently alter output.

use crate::grid::SurfaceGrid;
use crate::types::BiomeType;

/// Summary of differences for a single continuous (f32) layer.
#[derive(Clone, Debug)]
pub struct ContinuousDiff {
    pub layer_name: &'static str,
    /// Number of tiles where the value differs by more than `epsilon`.
    pub changed_count: usize,
    /// Mean absolute difference across all tiles.
    pub mean_abs_diff: f32,
    /// Maximum absolute difference.
    pub max_abs_diff: f32,
    /// Index of the tile with the largest difference.
    pub max_diff_tile: usize,
}

/// Summary of differences for a single discrete layer.
#[derive(Clone, Debug)]
pub struct DiscreteDiff {
    pub layer_name: &'static str,
    /// Number of tiles where the value differs.
    pub changed_count: usize,
}

/// Full diff report across all layers.
#[derive(Clone, Debug)]
pub struct GridDiff {
    pub tile_count: usize,
    pub dimensions_match: bool,
    pub continuous: Vec<ContinuousDiff>,
    pub discrete: Vec<DiscreteDiff>,
}

impl GridDiff {
    /// True if no layer has any difference.
    pub fn is_identical(&self) -> bool {
        self.dimensions_match
            && self.continuous.iter().all(|d| d.changed_count == 0)
            && self.discrete.iter().all(|d| d.changed_count == 0)
    }

    /// Total number of tile-layer pairs that differ.
    pub fn total_changes(&self) -> usize {
        let c: usize = self.continuous.iter().map(|d| d.changed_count).sum();
        let d: usize = self.discrete.iter().map(|d| d.changed_count).sum();
        c + d
    }

    /// Print a human-readable summary to stdout.
    pub fn print_summary(&self) {
        if !self.dimensions_match {
            println!("DIMENSIONS DIFFER — grids are not comparable.");
            return;
        }
        println!("Grid diff: {} tiles", self.tile_count);
        if self.is_identical() {
            println!("  IDENTICAL — no differences.");
            return;
        }
        println!();
        println!("Continuous layers:");
        for d in &self.continuous {
            if d.changed_count == 0 {
                continue;
            }
            println!(
                "  {:<30} changed={:>5}/{:<5} mean_diff={:>10.3} max_diff={:>10.3} (tile {})",
                d.layer_name,
                d.changed_count,
                self.tile_count,
                d.mean_abs_diff,
                d.max_abs_diff,
                d.max_diff_tile,
            );
        }
        println!();
        println!("Discrete layers:");
        for d in &self.discrete {
            if d.changed_count == 0 {
                continue;
            }
            println!(
                "  {:<30} changed={:>5}/{}",
                d.layer_name, d.changed_count, self.tile_count,
            );
        }
        println!();
        println!("Total changes: {}", self.total_changes());
    }
}

/// Compare two `SurfaceGrid`s and return a detailed diff report.
///
/// Continuous layers use `epsilon = 1e-6` for the changed-count threshold.
///
/// ```
/// use world::grid::{generate_surface_grid, GridResolution};
/// use world::diff::diff_grids;
/// use world::types::PlanetSimulationInput;
///
/// let input = PlanetSimulationInput { blackbody_temp_k: 255, ..Default::default() };
/// let a = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "a");
/// let b = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "a");
/// assert!(diff_grids(&a, &b).is_identical());
/// ```
pub fn diff_grids(a: &SurfaceGrid, b: &SurfaceGrid) -> GridDiff {
    let n_a = a.tile_count();
    let n_b = b.tile_count();
    if a.width != b.width || a.height != b.height {
        return GridDiff {
            tile_count: n_a,
            dimensions_match: false,
            continuous: Vec::new(),
            discrete: Vec::new(),
        };
    }
    let n = n_a;
    let eps = 1e-6f32;

    let continuous = vec![
        diff_f32(
            "elevation_m",
            &a.layers.elevation_m,
            &b.layers.elevation_m,
            n,
            eps,
        ),
        diff_f32(
            "temperature_c",
            &a.layers.temperature_c,
            &b.layers.temperature_c,
            n,
            eps,
        ),
        diff_f32(
            "temperature_summer_c",
            &a.layers.temperature_summer_c,
            &b.layers.temperature_summer_c,
            n,
            eps,
        ),
        diff_f32(
            "temperature_winter_c",
            &a.layers.temperature_winter_c,
            &b.layers.temperature_winter_c,
            n,
            eps,
        ),
        diff_f32(
            "precipitation_mm",
            &a.layers.precipitation_mm,
            &b.layers.precipitation_mm,
            n,
            eps,
        ),
        diff_f32(
            "humidity_relative",
            &a.layers.humidity_relative,
            &b.layers.humidity_relative,
            n,
            eps,
        ),
        diff_f32(
            "wind_speed_ms",
            &a.layers.wind_speed_ms,
            &b.layers.wind_speed_ms,
            n,
            eps,
        ),
        diff_f32(
            "wind_direction_deg",
            &a.layers.wind_direction_deg,
            &b.layers.wind_direction_deg,
            n,
            eps,
        ),
        diff_f32(
            "sea_surface_temp_c",
            &a.layers.sea_surface_temp_c,
            &b.layers.sea_surface_temp_c,
            n,
            eps,
        ),
        diff_f32(
            "river_discharge_m3s",
            &a.layers.river_discharge_m3s,
            &b.layers.river_discharge_m3s,
            n,
            eps,
        ),
    ];

    let discrete = vec![
        diff_eq("plate_id", &a.layers.plate_id, &b.layers.plate_id, n),
        diff_eq("is_ocean", &a.layers.is_ocean, &b.layers.is_ocean, n),
        diff_eq(
            "tectonic_boundary",
            &a.layers.tectonic_boundary,
            &b.layers.tectonic_boundary,
            n,
        ),
        diff_eq("biome", &a.layers.biome, &b.layers.biome, n),
        diff_eq(
            "koppen_class",
            &a.layers.koppen_class,
            &b.layers.koppen_class,
            n,
        ),
        diff_eq(
            "drainage_basin_id",
            &a.layers.drainage_basin_id,
            &b.layers.drainage_basin_id,
            n,
        ),
    ];

    GridDiff {
        tile_count: n,
        dimensions_match: true,
        continuous,
        discrete,
    }
}

fn diff_f32(name: &'static str, a: &[f32], b: &[f32], n: usize, eps: f32) -> ContinuousDiff {
    let mut changed = 0usize;
    let mut sum_diff = 0.0f64;
    let mut max_diff = 0.0f32;
    let mut max_tile = 0usize;
    for i in 0..n {
        let d = (a[i] - b[i]).abs();
        sum_diff += d as f64;
        if d > eps {
            changed += 1;
        }
        if d > max_diff {
            max_diff = d;
            max_tile = i;
        }
    }
    ContinuousDiff {
        layer_name: name,
        changed_count: changed,
        mean_abs_diff: (sum_diff / n as f64) as f32,
        max_abs_diff: max_diff,
        max_diff_tile: max_tile,
    }
}

fn diff_eq<T: PartialEq>(name: &'static str, a: &[T], b: &[T], n: usize) -> DiscreteDiff {
    let changed = (0..n).filter(|&i| a[i] != b[i]).count();
    DiscreteDiff {
        layer_name: name,
        changed_count: changed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{generate_surface_grid, GridResolution};
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};

    fn earth_input() -> PlanetSimulationInput {
        PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn same_seed_produces_identical_diff() {
        let input = earth_input();
        let a = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "diff_a");
        let b = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "diff_a");
        let d = diff_grids(&a, &b);
        assert!(d.is_identical());
        assert_eq!(d.total_changes(), 0);
    }

    #[test]
    fn different_seeds_produce_differences() {
        let input = earth_input();
        let a = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "diff_x");
        let b = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "diff_y");
        let d = diff_grids(&a, &b);
        assert!(!d.is_identical());
        assert!(d.total_changes() > 0);
    }

    #[test]
    fn different_dimensions_not_comparable() {
        let input = earth_input();
        let a = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "dim");
        let b = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Standard, "dim");
        let d = diff_grids(&a, &b);
        assert!(!d.dimensions_match);
    }

    #[test]
    fn diff_reports_correct_layer_count() {
        let input = earth_input();
        let a = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "cnt");
        let d = diff_grids(&a, &a);
        assert_eq!(d.continuous.len(), 10);
        assert_eq!(d.discrete.len(), 6);
    }
}
