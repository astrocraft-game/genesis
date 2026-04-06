//! Climate change simulation — long-term temperature, precipitation, and
//! sea-level shifts driven by pollution and deforestation.
//!
//! Call `simulate_climate_change` with a cumulative pollution level and
//! deforestation fraction to get a `ClimateShift` describing global deltas.
//! Then call `apply_climate_shift` to mutate a `SurfaceGrid` in place.

use crate::grid::SurfaceGrid;
use crate::types::BiomeType;

/// Computed climate deltas from pollution and deforestation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClimateShift {
    /// Global mean temperature change in °C (positive = warming).
    pub temperature_delta_c: f32,
    /// Precipitation multiplier (1.0 = unchanged, <1.0 = drier).
    pub precipitation_factor: f32,
    /// Sea-level rise in metres (positive = rise).
    pub sea_level_rise_m: f32,
    /// Number of coastal tiles newly flooded.
    pub tiles_flooded: usize,
}

/// Compute climate shift from cumulative pollution and deforestation.
///
/// - `mean_pollution`: average pollution level across all tiles (0.0–1.0+).
/// - `deforestation_fraction`: fraction of original forest tiles that have
///   been cleared (0.0–1.0).
///
/// The model is deliberately simple:
/// - Temperature rises ~3 °C per 0.5 mean pollution (greenhouse forcing).
/// - Deforestation reduces precipitation by up to 20%.
/// - Sea level rises ~1 m per °C of warming (ice-sheet response).
pub fn compute_climate_shift(mean_pollution: f32, deforestation_fraction: f32) -> ClimateShift {
    let temp_delta = mean_pollution * 6.0; // 0.5 pollution → +3 °C
    let precip_factor = 1.0 - deforestation_fraction.clamp(0.0, 1.0) * 0.20;
    let sea_rise = temp_delta * 1.0; // 1 m per °C

    ClimateShift {
        temperature_delta_c: temp_delta,
        precipitation_factor: precip_factor,
        sea_level_rise_m: sea_rise,
        tiles_flooded: 0, // filled by apply_climate_shift
    }
}

/// Apply a climate shift to a surface grid in place.
///
/// - Temperatures shift by `temperature_delta_c` globally.
/// - Precipitation scales by `precipitation_factor`.
/// - Sea level rises: tiles below `sea_level + rise` become ocean.
/// - Newly flooded tiles get `BiomeType::Ocean`.
///
/// Returns the updated `ClimateShift` with `tiles_flooded` filled in.
pub fn apply_climate_shift(grid: &mut SurfaceGrid, shift: &ClimateShift) -> ClimateShift {
    let n = grid.tile_count();
    let new_sea_level = grid.sea_level_m + shift.sea_level_rise_m;
    let mut flooded = 0usize;

    for idx in 0..n {
        // Temperature shift.
        grid.layers.temperature_c[idx] += shift.temperature_delta_c;
        grid.layers.temperature_summer_c[idx] += shift.temperature_delta_c;
        grid.layers.temperature_winter_c[idx] += shift.temperature_delta_c;

        // Precipitation scaling.
        grid.layers.precipitation_mm[idx] *= shift.precipitation_factor;

        // Monthly arrays.
        for m in grid.layers.temperature_monthly_c[idx].iter_mut() {
            *m += shift.temperature_delta_c;
        }
        let precip_f = shift.precipitation_factor;
        for m in grid.layers.precipitation_monthly_mm[idx].iter_mut() {
            *m *= precip_f;
        }

        // Coastal flooding: land tiles below new sea level become ocean.
        if !grid.layers.is_ocean[idx] && grid.layers.elevation_m[idx] < new_sea_level {
            grid.layers.is_ocean[idx] = true;
            grid.layers.biome[idx] = BiomeType::Ocean;
            grid.layers.river_discharge_m3s[idx] = 0.0;
            grid.layers.flow_accumulation[idx] = 0;
            flooded += 1;
        }
    }

    grid.sea_level_m = new_sea_level;

    ClimateShift {
        tiles_flooded: flooded,
        ..*shift
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{generate_surface_grid, GridResolution};
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};

    fn earth_grid() -> SurfaceGrid {
        let input = PlanetSimulationInput {
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
        };
        generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "cc")
    }

    #[test]
    fn zero_pollution_no_change() {
        let shift = compute_climate_shift(0.0, 0.0);
        assert_eq!(shift.temperature_delta_c, 0.0);
        assert_eq!(shift.precipitation_factor, 1.0);
        assert_eq!(shift.sea_level_rise_m, 0.0);
    }

    #[test]
    fn high_pollution_warms_climate() {
        let shift = compute_climate_shift(0.5, 0.0);
        assert!(shift.temperature_delta_c > 2.0);
        assert!(shift.sea_level_rise_m > 0.0);
    }

    #[test]
    fn deforestation_reduces_precipitation() {
        let shift = compute_climate_shift(0.0, 0.8);
        assert!(shift.precipitation_factor < 1.0);
        assert!(shift.precipitation_factor > 0.7);
    }

    #[test]
    fn apply_shifts_temperature() {
        let mut g = earth_grid();
        let before_temp = g.layers.temperature_c[0];
        let shift = compute_climate_shift(0.3, 0.0);
        apply_climate_shift(&mut g, &shift);
        assert!((g.layers.temperature_c[0] - before_temp - shift.temperature_delta_c).abs() < 0.01,);
    }

    #[test]
    fn apply_scales_precipitation() {
        let mut g = earth_grid();
        // Find a land tile with positive precipitation.
        let idx = (0..g.tile_count())
            .find(|&i| !g.layers.is_ocean[i] && g.layers.precipitation_mm[i] > 100.0)
            .unwrap();
        let before = g.layers.precipitation_mm[idx];
        let shift = compute_climate_shift(0.0, 0.5);
        apply_climate_shift(&mut g, &shift);
        let after = g.layers.precipitation_mm[idx];
        assert!(
            (after - before * shift.precipitation_factor).abs() < 1.0,
            "precip {} should be {} * {}",
            after,
            before,
            shift.precipitation_factor
        );
    }

    #[test]
    fn sea_level_rise_floods_coastal_tiles() {
        let mut g = earth_grid();
        let land_before = g.layers.is_ocean.iter().filter(|&&o| !o).count();
        // Massive warming → large sea level rise.
        let shift = compute_climate_shift(1.0, 0.0);
        let result = apply_climate_shift(&mut g, &shift);
        let land_after = g.layers.is_ocean.iter().filter(|&&o| !o).count();
        assert!(
            land_after <= land_before,
            "land should not increase after flooding"
        );
        if shift.sea_level_rise_m > 1.0 {
            assert!(
                result.tiles_flooded > 0,
                "large sea rise should flood some tiles"
            );
        }
    }

    #[test]
    fn flooded_tiles_become_ocean() {
        let mut g = earth_grid();
        let shift = compute_climate_shift(1.0, 0.0);
        let result = apply_climate_shift(&mut g, &shift);
        // Verify all tiles marked as ocean have ocean biome.
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                assert_eq!(
                    g.layers.biome[idx],
                    BiomeType::Ocean,
                    "ocean tile {} has biome {:?}",
                    idx,
                    g.layers.biome[idx]
                );
            }
        }
        let _ = result;
    }

    #[test]
    fn sea_level_updates_in_grid() {
        let mut g = earth_grid();
        let before = g.sea_level_m;
        let shift = compute_climate_shift(0.5, 0.0);
        apply_climate_shift(&mut g, &shift);
        assert!((g.sea_level_m - before - shift.sea_level_rise_m).abs() < 0.01,);
    }
}
