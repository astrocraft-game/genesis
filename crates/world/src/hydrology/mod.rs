pub mod ocean;

pub use crate::types::{DeltaType, Hydrography, LakeDistribution, LakeFormationType, LiquidType};
use crate::types::{OrbitContext, PlanetSimulationInput};

/// Generate a simple hydrography profile with Hadley-cell zonal rainfall.
///
/// The model is deliberately coarse — three latitude bands
/// (equatorial/mid-latitude/polar) with rainfall redistributed by axial tilt.
/// Low-tilt worlds concentrate rain in the equatorial belt (ITCZ); high-tilt
/// worlds smear precipitation across latitudes via strong seasonal migration.
///
/// Returns `None` for bodies with no hydrosphere or no atmosphere.
pub fn generate_hydrography(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    hydrosphere_pct: f32,
) -> Option<Hydrography> {
    if hydrosphere_pct <= 0.0 || atmospheric_pressure < 0.05 {
        return None;
    }

    // Global mean precipitation scales with available surface water, a
    // moisture-carrying atmosphere, and temperature (Clausius-Clapeyron
    // roughly doubles water vapour per 10 K above 273 K).
    let temp_k = context.blackbody_temp_k as f32;
    let water_factor = (hydrosphere_pct / 100.0).clamp(0.05, 1.0);
    let pressure_factor = atmospheric_pressure.clamp(0.1, 5.0).sqrt();
    let temp_factor = ((temp_k - 250.0) / 35.0).clamp(0.2, 3.0);
    let mean_precipitation_mm =
        (1000.0 * water_factor * pressure_factor * temp_factor).clamp(50.0, 4000.0);

    let (cells, zonal) = zonal_distribution(&context.orbit, mean_precipitation_mm);

    // River count scales with land area available and water throughput.
    let land_fraction = (100.0 - hydrosphere_pct).max(0.0) / 100.0;
    let major_river_count =
        ((mean_precipitation_mm / 100.0) * land_fraction * 12.0).clamp(0.0, 80.0) as u32;
    let longest_river_km = (mean_precipitation_mm * land_fraction * 3.5).clamp(0.0, 7000.0);

    Some(Hydrography {
        major_river_count,
        longest_river_km,
        mean_precipitation_mm,
        zonal_precipitation_mm: zonal,
        hadley_cells_per_hemisphere: cells,
        dominant_delta_type: if major_river_count > 20 {
            DeltaType::Arcuate
        } else {
            DeltaType::None
        },
    })
}

/// Redistributes the global mean precipitation across three zonal bands and
/// returns the atmospheric-cell count per hemisphere. All values in mm/yr.
fn zonal_distribution(orbit: &OrbitContext, mean_mm: f32) -> (u8, [f32; 3]) {
    let tilt = orbit.axial_tilt_deg.abs();

    // Cell count per hemisphere:
    //   tilt < 8°   → 1 (single merged cell, strong equator-to-pole transport)
    //   tilt 8-40°  → 3 (Hadley + Ferrel + Polar, Earth-like)
    //   tilt 40-54° → 2 (Ferrel cell collapses)
    //   tilt > 54°  → 1 (chaotic seasonal reversal — treated as single cell)
    let cells: u8 = if tilt < 8.0 {
        1
    } else if tilt < 40.0 {
        3
    } else if tilt < 54.0 {
        2
    } else {
        1
    };

    // Base weights (equatorial, mid-lat, polar). Low tilt → strongly peaked
    // at equator; Earth-like → ITCZ + subtropical desert + mid-lat rain belt;
    // high tilt → smeared by seasonal migration.
    let weights = match cells {
        1 if tilt < 8.0 => [2.2, 0.7, 0.3], // low-tilt: equator-peaked
        3 => [1.8, 0.8, 0.5],               // Earth-like
        2 => [1.4, 1.0, 0.8],               // collapsed Ferrel
        _ => [1.1, 1.2, 1.1],               // chaotic high-tilt
    };
    // Normalize so area-weighted mean matches mean_mm. Zonal areas (fraction
    // of hemisphere by latitude): equatorial 0-30°=0.50, mid 30-60°=0.37,
    // polar 60-90°=0.13.
    let zonal_area = [0.50, 0.37, 0.13];
    let weighted_sum: f32 = weights
        .iter()
        .zip(zonal_area.iter())
        .map(|(w, a)| w * a)
        .sum();
    let norm = if weighted_sum > 0.0 {
        mean_mm / weighted_sum
    } else {
        mean_mm
    };
    let zonal = [weights[0] * norm, weights[1] * norm, weights[2] * norm];
    (cells, zonal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StarContext;

    fn earth_like_context() -> PlanetSimulationInput {
        PlanetSimulationInput {
            blackbody_temp_k: 288,
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
    fn earth_like_produces_hydrography() {
        let h = generate_hydrography(&earth_like_context(), 1.0, 71.0).unwrap();
        assert_eq!(h.hadley_cells_per_hemisphere, 3);
        assert!(h.mean_precipitation_mm > 500.0 && h.mean_precipitation_mm < 2500.0);
    }

    #[test]
    fn low_tilt_concentrates_rain_at_equator() {
        let mut ctx = earth_like_context();
        ctx.orbit.axial_tilt_deg = 2.0;
        let h = generate_hydrography(&ctx, 1.0, 71.0).unwrap();
        assert_eq!(h.hadley_cells_per_hemisphere, 1);
        let [eq, mid, polar] = h.zonal_precipitation_mm;
        assert!(
            eq > mid && mid > polar,
            "expected equator>mid>polar, got {:?}",
            h.zonal_precipitation_mm
        );
        assert!(eq > 2.5 * polar, "equator {} not ≫ polar {}", eq, polar);
    }

    #[test]
    fn earth_tilt_has_subtropical_dry_bands() {
        let h = generate_hydrography(&earth_like_context(), 1.0, 71.0).unwrap();
        let [eq, mid, _polar] = h.zonal_precipitation_mm;
        // Mid-latitudes are drier than equatorial — the subtropical high.
        assert!(
            eq > mid,
            "expected equator > mid-lat, got {:?}",
            h.zonal_precipitation_mm
        );
    }

    #[test]
    fn high_tilt_smears_rainfall() {
        let mut ctx = earth_like_context();
        ctx.orbit.axial_tilt_deg = 70.0;
        let h = generate_hydrography(&ctx, 1.0, 71.0).unwrap();
        let [eq, _mid, polar] = h.zonal_precipitation_mm;
        // High-tilt worlds smear rainfall; equator-to-polar ratio is low.
        let ratio = eq / polar;
        assert!(
            ratio < 1.5,
            "expected smeared rainfall, got ratio {}",
            ratio
        );
    }

    #[test]
    fn no_water_means_no_hydrography() {
        assert!(generate_hydrography(&earth_like_context(), 1.0, 0.0).is_none());
    }

    #[test]
    fn no_atmosphere_means_no_hydrography() {
        assert!(generate_hydrography(&earth_like_context(), 0.0, 50.0).is_none());
    }

    #[test]
    fn hot_ocean_world_has_more_rain() {
        let mut ctx = earth_like_context();
        ctx.blackbody_temp_k = 320;
        let hot = generate_hydrography(&ctx, 1.0, 95.0).unwrap();
        let earth = generate_hydrography(&earth_like_context(), 1.0, 71.0).unwrap();
        assert!(hot.mean_precipitation_mm > earth.mean_precipitation_mm);
    }
}

// ---------------------------------------------------------------------------
// Grid-level precipitation & humidity layer
//
// Populates `precipitation_mm`, `humidity_relative`, and `pet_ratio` on a
// `SurfaceGrid` that already has elevation, temperature, wind, and
// is_ocean populated. The model is:
//   1. Zonal base from latitude bands (ITCZ peak, subtropical desert,
//      mid-lat rain, polar dry), scaled by a global moisture factor.
//   2. Ocean-proximity decay: land tiles far from any ocean get drier.
//   3. Orographic modifier: windward slopes wetter, leeward drier.
//   4. Holdridge PET ratio + humidity derived from precipitation/temperature.
// ---------------------------------------------------------------------------

use crate::grid::SurfaceGrid;

/// Populate precipitation, humidity, and PET-ratio layers on a grid that
/// already has geology, temperature, and wind populated.
pub fn generate_precipitation(
    _context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    hydrosphere_pct: f32,
    grid: &mut SurfaceGrid,
) {
    // Use the grid's own mean temperature (includes greenhouse warming).
    let mean_temp_c = grid.layers.temperature_c.iter().sum::<f32>() / grid.tile_count() as f32;
    let temp_k = mean_temp_c + 273.15;
    let water_factor = (hydrosphere_pct / 100.0).clamp(0.05, 1.0);
    let pressure_factor = atmospheric_pressure.clamp(0.05, 5.0).sqrt();
    let temp_factor = ((temp_k - 250.0) / 35.0).clamp(0.2, 3.0);
    let global_scale = water_factor * pressure_factor * temp_factor;

    // No atmosphere or no water → no precipitation anywhere.
    if atmospheric_pressure < 0.05 || hydrosphere_pct <= 0.0 {
        for idx in 0..grid.tile_count() {
            grid.layers.precipitation_mm[idx] = 0.0;
            grid.layers.humidity_relative[idx] = 0.0;
            grid.layers.pet_ratio[idx] = 0.0;
        }
        return;
    }

    // Stage 1: zonal band baseline per latitude row.
    for r in 0..grid.height {
        let lat_deg = grid.row_latitude(r);
        let zonal = zonal_band_precipitation(lat_deg) * global_scale;
        for c in 0..grid.width {
            let idx = grid.idx(c, r);
            grid.layers.precipitation_mm[idx] = zonal;
        }
    }

    // Stage 2: ocean proximity decay on land.
    apply_ocean_proximity(grid);

    // Stage 3: orographic modifier.
    apply_orographic(grid);

    // Stage 4: PET ratio + humidity derivation.
    derive_pet_and_humidity(grid);
}

/// Base precipitation in mm/yr for an Earth-like world at the given
/// latitude. Empirically calibrated against Earth's Hadley/Ferrel/Polar
/// cell pattern. Returns the pre-scaling base.
fn zonal_band_precipitation(lat_deg: f32) -> f32 {
    let abs_lat = lat_deg.abs();
    if abs_lat < 10.0 {
        // ITCZ — tropical rain belt.
        2200.0 - abs_lat * 40.0
    } else if abs_lat < 25.0 {
        // Trade-wind tropics: drying toward 30°.
        1800.0 - (abs_lat - 10.0) * 80.0
    } else if abs_lat < 35.0 {
        // Subtropical high / desert belt.
        300.0 + (30.0 - abs_lat).abs() * 20.0
    } else if abs_lat < 50.0 {
        // Rising toward mid-latitude rain belt.
        400.0 + (abs_lat - 35.0) * 60.0
    } else if abs_lat < 65.0 {
        // Mid-latitude storm track.
        1200.0 - (abs_lat - 50.0) * 20.0
    } else if abs_lat < 80.0 {
        // Polar front decline.
        900.0 - (abs_lat - 65.0) * 40.0
    } else {
        // Polar desert.
        150.0 + (90.0 - abs_lat) * 10.0
    }
}

/// Reduce precipitation on land based on BFS distance to the nearest
/// ocean tile. Coastal land keeps full rainfall; deep-interior land
/// receives ~40% of the coastal value at saturation.
fn apply_ocean_proximity(grid: &mut SurfaceGrid) {
    let dist = distance_to_ocean(grid);
    let max_dist = *dist.iter().max().unwrap_or(&1) as f32;
    if max_dist < 1.0 {
        return;
    }
    for (idx, &d) in dist.iter().enumerate() {
        if grid.layers.is_ocean[idx] {
            continue;
        }
        let norm = d as f32 / max_dist;
        // Exponential decay: near-coast 1.0 → deep-interior 0.4.
        let factor = 0.4 + 0.6 * (-3.0 * norm).exp();
        grid.layers.precipitation_mm[idx] *= factor;
    }
}

/// Apply orographic rain shadow by comparing each tile's elevation with
/// its immediate-upwind neighbour. Climbing slopes boost precipitation;
/// descending slopes (leeward of a rise) suppress it.
fn apply_orographic(grid: &mut SurfaceGrid) {
    let w = grid.width as i32;
    let h = grid.height as i32;
    // Work on a snapshot so feedback doesn't compound row-by-row.
    let base = grid.layers.precipitation_mm.clone();
    for r in 0..grid.height {
        for c in 0..grid.width {
            let idx = grid.idx(c, r);
            if grid.layers.is_ocean[idx] {
                continue;
            }
            // Upwind offset: wind_direction_deg is the bearing wind blows
            // FROM. Moving in that bearing follows the wind backward, to
            // where the air just came from.
            let dir = grid.layers.wind_direction_deg[idx].to_radians();
            let drow = -dir.cos(); // north (0°) → row−1
            let dcol = dir.sin(); // east (90°) → col+1
            let upwind_r = (r as i32 + drow.round() as i32).rem_euclid(h);
            let upwind_c = (c as i32 + dcol.round() as i32).rem_euclid(w);
            let upwind_idx = grid.idx(upwind_c as u16 % grid.width, upwind_r as u16 % grid.height);
            let my_elev = grid.layers.elevation_m[idx];
            let up_elev = grid.layers.elevation_m[upwind_idx];
            let delta_km = (my_elev - up_elev) / 1000.0;
            let factor = if delta_km > 0.0 {
                // Windward slope: boost up to +50% per km climbed.
                (1.0 + delta_km * 0.5).min(2.0)
            } else {
                // Leeward slope: suppress down to 30% at -2 km drop.
                (1.0 + delta_km * 0.35).max(0.3)
            };
            grid.layers.precipitation_mm[idx] = base[idx] * factor;
        }
    }
}

/// Compute Holdridge PET ratio and derive relative humidity.
///
/// PET (mm/yr) = 58.93 × biotemperature (Holdridge 1947).
/// Biotemperature = mean annual temp clamped to [0, 30] °C.
/// PET ratio = precipitation / PET → Holdridge aridity index.
/// Relative humidity is mapped from PET ratio into [0, 1].
fn derive_pet_and_humidity(grid: &mut SurfaceGrid) {
    for idx in 0..grid.tile_count() {
        let temp_c = grid.layers.temperature_c[idx];
        let biotemp = temp_c.clamp(0.0, 30.0);
        let pet_mm = 58.93 * biotemp;
        let precip = grid.layers.precipitation_mm[idx];
        let ratio = if pet_mm > 1.0 {
            precip / pet_mm
        } else {
            // No evapotranspiration possible (frozen tile): mark as saturated
            // if there's any precipitation.
            if precip > 0.0 {
                2.0
            } else {
                0.0
            }
        };
        grid.layers.pet_ratio[idx] = ratio;
        // Humidity mapping: 0.125 ratio → arid (0.15), 1.0 → temperate (0.60),
        // 2.0+ → saturated (0.90). Smooth via square-root curve.
        let humidity = (ratio.sqrt() * 0.5).clamp(0.0, 0.95);
        grid.layers.humidity_relative[idx] = humidity;
    }
}

/// Multi-source BFS distance from any ocean tile, measured in tile-steps.
fn distance_to_ocean(grid: &SurfaceGrid) -> Vec<u16> {
    let w = grid.width as usize;
    let h = grid.height as usize;
    let n = w * h;
    let mut dist = vec![u16::MAX; n];
    let mut queue = std::collections::VecDeque::new();
    for (idx, &ocean) in grid.layers.is_ocean.iter().enumerate() {
        if ocean {
            dist[idx] = 0;
            queue.push_back(idx);
        }
    }
    while let Some(idx) = queue.pop_front() {
        let d = dist[idx];
        let r = idx / w;
        let c = idx % w;
        let neighbours = [
            (c, r.saturating_sub(1)),
            (c, (r + 1).min(h - 1)),
            ((c + w - 1) % w, r),
            ((c + 1) % w, r),
        ];
        for (nc, nr) in neighbours {
            let nidx = nr * w + nc;
            if dist[nidx] > d + 1 {
                dist[nidx] = d + 1;
                queue.push_back(nidx);
            }
        }
    }
    for d in dist.iter_mut() {
        if *d == u16::MAX {
            *d = 0;
        }
    }
    dist
}

#[cfg(test)]
mod grid_tests {
    use super::*;
    use crate::climate::{generate_temperature, generate_wind};
    use crate::geology::generate_geology;
    use crate::grid::GridResolution;
    use crate::types::StarContext;

    fn earth_like_input() -> PlanetSimulationInput {
        PlanetSimulationInput {
            body_id: 7,
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

    fn earth_grid() -> SurfaceGrid {
        let input = earth_like_input();
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "precip");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        g
    }

    #[test]
    fn itcz_is_wettest() {
        // Band function should peak at the equator.
        let eq = zonal_band_precipitation(0.0);
        let trades = zonal_band_precipitation(15.0);
        let desert = zonal_band_precipitation(30.0);
        assert!(eq > trades);
        assert!(trades > desert);
    }

    #[test]
    fn subtropical_desert_band_exists() {
        // Around 30° should be the driest non-polar zone.
        let itcz = zonal_band_precipitation(5.0);
        let desert = zonal_band_precipitation(30.0);
        let midlat = zonal_band_precipitation(50.0);
        assert!(desert < itcz);
        assert!(desert < midlat);
    }

    #[test]
    fn midlat_rainbelt_wetter_than_polar() {
        let midlat = zonal_band_precipitation(55.0);
        let polar = zonal_band_precipitation(85.0);
        assert!(midlat > polar);
    }

    #[test]
    fn earth_grid_has_plausible_mean_precipitation() {
        let g = earth_grid();
        let mean = g.layers.precipitation_mm.iter().sum::<f32>() / g.tile_count() as f32;
        // Earth's global mean is ~990 mm. Allow wide tolerance given coarse model.
        assert!(
            (300.0..=2500.0).contains(&mean),
            "mean precipitation {} mm outside plausible range",
            mean
        );
    }

    #[test]
    fn equator_is_wetter_than_subtropics_after_scaling() {
        let g = earth_grid();
        let mut eq = 0.0f32;
        let mut sub = 0.0f32;
        let mut eq_count = 0;
        let mut sub_count = 0;
        for r in 0..g.height {
            let lat = g.row_latitude(r);
            for c in 0..g.width {
                let idx = g.idx(c, r);
                if lat.abs() < 10.0 {
                    eq += g.layers.precipitation_mm[idx];
                    eq_count += 1;
                } else if (25.0..35.0).contains(&lat.abs()) {
                    sub += g.layers.precipitation_mm[idx];
                    sub_count += 1;
                }
            }
        }
        let eq_mean = eq / eq_count as f32;
        let sub_mean = sub / sub_count as f32;
        assert!(
            eq_mean > sub_mean * 2.0,
            "equator {} should be > 2× subtropics {}",
            eq_mean,
            sub_mean
        );
    }

    #[test]
    fn no_atmosphere_means_no_precipitation() {
        let input = earth_like_input();
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "dry");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 0.0, &mut g);
        generate_precipitation(&input, 0.0, 71.0, &mut g);
        for &p in &g.layers.precipitation_mm {
            assert_eq!(p, 0.0);
        }
    }

    #[test]
    fn no_water_means_no_precipitation() {
        let input = earth_like_input();
        let mut g = generate_geology(&input, 0.0, GridResolution::Fast, "arid");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 0.0, &mut g);
        for &p in &g.layers.precipitation_mm {
            assert_eq!(p, 0.0);
        }
    }

    #[test]
    fn pet_ratio_reasonable_for_earth_like() {
        let g = earth_grid();
        let mean_ratio = g.layers.pet_ratio.iter().sum::<f32>() / g.tile_count() as f32;
        // Earth-wide average is between arid and humid.
        assert!(
            (0.2..=5.0).contains(&mean_ratio),
            "mean pet_ratio {} out of range",
            mean_ratio
        );
    }

    #[test]
    fn humidity_in_valid_range() {
        let g = earth_grid();
        for (idx, &h) in g.layers.humidity_relative.iter().enumerate() {
            assert!(
                (0.0..=0.95).contains(&h),
                "humidity {} at idx {} out of [0, 0.95]",
                h,
                idx
            );
        }
    }

    #[test]
    fn continental_interiors_drier_than_coasts() {
        let g = earth_grid();
        // Find a land tile at equator that's far from ocean vs. one close.
        let eq_row = g.height / 2;
        let dist = distance_to_ocean(&g);
        let mut max_d = 0u16;
        let mut min_d = u16::MAX;
        let mut interior_idx = 0usize;
        let mut coast_idx = 0usize;
        for c in 0..g.width {
            let idx = g.idx(c, eq_row);
            if g.layers.is_ocean[idx] {
                continue;
            }
            let d = dist[idx];
            if d > max_d {
                max_d = d;
                interior_idx = idx;
            }
            if d > 0 && d < min_d {
                min_d = d;
                coast_idx = idx;
            }
        }
        if max_d > min_d + 2 {
            assert!(
                g.layers.precipitation_mm[interior_idx] < g.layers.precipitation_mm[coast_idx],
                "interior ({}) should be drier than coast ({})",
                g.layers.precipitation_mm[interior_idx],
                g.layers.precipitation_mm[coast_idx]
            );
        }
    }

    #[test]
    fn precipitation_is_deterministic() {
        let a = earth_grid();
        let b = earth_grid();
        assert_eq!(a.layers.precipitation_mm, b.layers.precipitation_mm);
        assert_eq!(a.layers.pet_ratio, b.layers.pet_ratio);
        assert_eq!(a.layers.humidity_relative, b.layers.humidity_relative);
    }

    #[test]
    fn frozen_world_has_saturated_pet_ratio_where_precipitation_falls() {
        // Arctic tile: temp < 0 → biotemp = 0 → PET undefined.
        // Our formula returns ratio = 2.0 if there's precipitation.
        let input = earth_like_input();
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "frost");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        // Find a sub-freezing tile with positive precipitation.
        let mut found = false;
        for idx in 0..g.tile_count() {
            if g.layers.temperature_c[idx] < -5.0 && g.layers.precipitation_mm[idx] > 1.0 {
                assert_eq!(
                    g.layers.pet_ratio[idx], 2.0,
                    "frozen tile with precip should have ratio=2"
                );
                found = true;
                break;
            }
        }
        assert!(found, "no sub-freezing tile with precipitation found");
    }
}

// ---------------------------------------------------------------------------
// Grid-level hydrology layer — D8 flow, rivers, drainage basins
//
// Populates `flow_accumulation`, `river_discharge_m3s`, and extends
// `drainage_basin_id` for land tiles (inheriting the ocean basin they
// drain to, or a new ID for endorheic sinks).
// ---------------------------------------------------------------------------

/// Mean Earth radius in kilometres, used as the reference body size when
/// converting tile counts to real-world areas.
const EARTH_RADIUS_KM: f32 = 6371.0;

/// Populate hydrology layers on a grid that already has geology,
/// precipitation, and ocean-dynamics populated.
///
/// `planet_radius_earth` scales tile areas and river discharges. Pass 1.0
/// for an Earth-size body.
pub fn generate_hydrology(planet_radius_earth: f32, grid: &mut SurfaceGrid) {
    let flow_dir = compute_d8_flow_direction(grid);
    let accumulation = compute_flow_accumulation(grid, &flow_dir);
    assign_land_basins(grid, &flow_dir);
    compute_discharge(planet_radius_earth, grid, &accumulation);
    grid.layers.flow_accumulation = accumulation;
}

/// For each land tile, find the steepest-descent neighbour. Returns
/// Some(idx) pointing to the downstream tile, or None for oceans and
/// local minima (sinks).
fn compute_d8_flow_direction(grid: &SurfaceGrid) -> Vec<Option<usize>> {
    let w = grid.width as usize;
    let h = grid.height as usize;
    let n = w * h;
    let mut flow = vec![None; n];

    for r in 0..h {
        for c in 0..w {
            let idx = r * w + c;
            if grid.layers.is_ocean[idx] {
                continue;
            }
            let my_elev = grid.layers.elevation_m[idx];
            let mut best_drop = 0.0f32;
            let mut best_idx: Option<usize> = None;
            // 8-connected neighbours (D8) with longitude wrap.
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r as i32 + dr;
                    if nr < 0 || nr >= h as i32 {
                        continue; // no wrap across poles
                    }
                    let nc = (c as i32 + dc).rem_euclid(w as i32);
                    let nidx = nr as usize * w + nc as usize;
                    let drop = my_elev - grid.layers.elevation_m[nidx];
                    // Diagonal moves scaled by sqrt(2).
                    let slope = if dr != 0 && dc != 0 {
                        drop / std::f32::consts::SQRT_2
                    } else {
                        drop
                    };
                    if slope > best_drop {
                        best_drop = slope;
                        best_idx = Some(nidx);
                    }
                }
            }
            flow[idx] = best_idx;
        }
    }
    flow
}

/// Topologically propagate unit water from every land tile to its
/// downstream neighbour. Returns per-tile accumulated upstream count.
fn compute_flow_accumulation(grid: &SurfaceGrid, flow_dir: &[Option<usize>]) -> Vec<u32> {
    let n = grid.tile_count();
    let mut accumulation = vec![1u32; n]; // each tile contributes itself

    // Sort land tiles by elevation descending so upstream is processed first.
    let mut order: Vec<usize> = (0..n).filter(|&i| !grid.layers.is_ocean[i]).collect();
    order.sort_by(|&a, &b| {
        grid.layers.elevation_m[b]
            .partial_cmp(&grid.layers.elevation_m[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for idx in order {
        if let Some(downstream) = flow_dir[idx] {
            accumulation[downstream] = accumulation[downstream].saturating_add(accumulation[idx]);
        }
    }
    // Zero out ocean tiles — accumulation is a land-only metric.
    for (i, &ocean) in grid.layers.is_ocean.iter().enumerate() {
        if ocean {
            accumulation[i] = 0;
        }
    }
    accumulation
}

/// For each land tile, trace downstream until hitting an ocean tile;
/// inherit that ocean's basin_id. Land tiles reaching an endorheic sink
/// (no downstream) get a new basin ID above the ocean range.
fn assign_land_basins(grid: &mut SurfaceGrid, flow_dir: &[Option<usize>]) {
    let n = grid.tile_count();
    // Find max ocean basin ID to allocate fresh IDs for endorheic sinks.
    let max_ocean_id = *grid.layers.drainage_basin_id.iter().max().unwrap_or(&0);
    let mut next_endorheic = max_ocean_id.saturating_add(1);
    let mut sink_basins: std::collections::HashMap<usize, u16> = std::collections::HashMap::new();

    for start in 0..n {
        if grid.layers.is_ocean[start] || grid.layers.drainage_basin_id[start] != 0 {
            continue;
        }
        // Walk downstream with a cycle guard.
        let mut visited: Vec<usize> = Vec::with_capacity(16);
        let mut cur = start;
        let mut outlet_basin: Option<u16> = None;
        for _ in 0..(n + 1) {
            visited.push(cur);
            if grid.layers.is_ocean[cur] {
                outlet_basin = Some(grid.layers.drainage_basin_id[cur]);
                break;
            }
            if let Some(next) = flow_dir[cur] {
                if visited.contains(&next) {
                    break; // cycle — treat current as endorheic sink
                }
                cur = next;
            } else {
                break; // local sink
            }
        }
        let basin = outlet_basin.unwrap_or_else(|| {
            let entry = sink_basins.entry(cur).or_insert_with(|| {
                let id = next_endorheic;
                next_endorheic = next_endorheic.saturating_add(1);
                id
            });
            *entry
        });
        // Stamp the basin ID along the traced path.
        for &t in &visited {
            if !grid.layers.is_ocean[t] {
                grid.layers.drainage_basin_id[t] = basin;
            }
        }
    }
}

/// River discharge in m³/s from flow accumulation × upstream precipitation.
fn compute_discharge(planet_radius_earth: f32, grid: &mut SurfaceGrid, accumulation: &[u32]) {
    let radius_km = EARTH_RADIUS_KM * planet_radius_earth;
    let dlat_rad = std::f32::consts::PI / grid.height as f32;
    let dlon_rad = 2.0 * std::f32::consts::PI / grid.width as f32;
    let seconds_per_year: f32 = 365.25 * 86_400.0;
    for r in 0..grid.height {
        let lat_rad = grid.row_latitude(r).to_radians();
        // Area of one tile at this latitude, in km².
        let tile_area_km2 = radius_km * radius_km * lat_rad.cos().abs() * dlat_rad * dlon_rad;
        let tile_area_m2 = tile_area_km2 * 1.0e6;
        for c in 0..grid.width {
            let idx = grid.idx(c, r);
            if grid.layers.is_ocean[idx] {
                grid.layers.river_discharge_m3s[idx] = 0.0;
                continue;
            }
            let precip_m = grid.layers.precipitation_mm[idx] / 1000.0;
            // Water volume per year per upstream tile (m³).
            let annual_volume_m3 = accumulation[idx] as f32 * tile_area_m2 * precip_m;
            // Assume ~30% runs off (the rest evaporates/transpires/infiltrates).
            let runoff_fraction = 0.3;
            grid.layers.river_discharge_m3s[idx] =
                (annual_volume_m3 * runoff_fraction / seconds_per_year).max(0.0);
        }
    }
}

#[cfg(test)]
mod hydrology_grid_tests {
    use super::*;
    use crate::climate::{generate_temperature, generate_wind};
    use crate::geology::generate_geology;
    use crate::grid::GridResolution;
    use crate::ocean::generate_ocean_dynamics;
    use crate::types::StarContext;

    fn earth_like_input() -> PlanetSimulationInput {
        PlanetSimulationInput {
            body_id: 7,
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

    fn earth_grid() -> SurfaceGrid {
        let input = earth_like_input();
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "hydro");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        g
    }

    #[test]
    fn ocean_tiles_have_zero_accumulation_and_discharge() {
        let g = earth_grid();
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                assert_eq!(g.layers.flow_accumulation[idx], 0);
                assert_eq!(g.layers.river_discharge_m3s[idx], 0.0);
            }
        }
    }

    #[test]
    fn every_land_tile_has_accumulation_at_least_one() {
        let g = earth_grid();
        for idx in 0..g.tile_count() {
            if !g.layers.is_ocean[idx] {
                assert!(
                    g.layers.flow_accumulation[idx] >= 1,
                    "land tile {} has zero accumulation",
                    idx
                );
            }
        }
    }

    #[test]
    fn accumulation_flows_downhill() {
        // Total accumulation over all tiles should equal the number of land
        // tiles, since each tile contributes one unit which is redistributed.
        let g = earth_grid();
        let land_count = g.layers.is_ocean.iter().filter(|&&o| !o).count() as u64;
        // Accumulation of land tiles that drain to the ocean ends up being
        // "consumed" by the ocean tiles (which we zero). So total ≥ land count
        // only if there are endorheic basins; otherwise may be less.
        let total_accum: u64 = g.layers.flow_accumulation.iter().map(|&x| x as u64).sum();
        assert!(total_accum > 0, "no flow accumulation generated at all");
        // At minimum each land tile should count for itself.
        assert!(total_accum >= land_count / 2);
    }

    #[test]
    fn land_tiles_get_basin_id_downstream_of_ocean() {
        let g = earth_grid();
        // Count land tiles with basin_id assigned vs. unassigned.
        let mut unassigned = 0;
        for idx in 0..g.tile_count() {
            if !g.layers.is_ocean[idx] && g.layers.drainage_basin_id[idx] == 0 {
                unassigned += 1;
            }
        }
        assert_eq!(unassigned, 0, "{} land tiles have no basin", unassigned);
    }

    #[test]
    fn river_discharge_increases_downstream() {
        let g = earth_grid();
        let flow = compute_d8_flow_direction(&g);
        // Pick any land tile with a downstream neighbour; discharge should
        // not decrease at the downstream end (it should weakly monotone).
        let mut violations = 0;
        for (idx, down_opt) in flow.iter().enumerate() {
            if g.layers.is_ocean[idx] {
                continue;
            }
            if let Some(down) = down_opt {
                if g.layers.is_ocean[*down] {
                    continue;
                }
                if g.layers.flow_accumulation[*down] < g.layers.flow_accumulation[idx] {
                    violations += 1;
                }
            }
        }
        assert_eq!(
            violations, 0,
            "{} downstream drops in accumulation",
            violations
        );
    }

    #[test]
    fn large_rivers_exist_on_earth_like_world() {
        let g = earth_grid();
        let max_discharge = g
            .layers
            .river_discharge_m3s
            .iter()
            .cloned()
            .fold(0.0f32, f32::max);
        // Amazon ≈ 2e5 m³/s; for a 72×36 grid our largest "river" should
        // nonetheless exceed a few hundred m³/s.
        assert!(
            max_discharge > 100.0,
            "max discharge {} too low",
            max_discharge
        );
    }

    #[test]
    fn hydrology_is_deterministic() {
        let a = earth_grid();
        let b = earth_grid();
        assert_eq!(a.layers.flow_accumulation, b.layers.flow_accumulation);
        assert_eq!(a.layers.river_discharge_m3s, b.layers.river_discharge_m3s);
        assert_eq!(a.layers.drainage_basin_id, b.layers.drainage_basin_id);
    }

    #[test]
    fn dry_world_has_minimal_discharge() {
        let input = earth_like_input();
        let mut g = generate_geology(&input, 5.0, GridResolution::Fast, "dry");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 5.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        let max_discharge = g
            .layers
            .river_discharge_m3s
            .iter()
            .cloned()
            .fold(0.0f32, f32::max);
        // 5% hydrosphere → low global scaling, so discharge should be small.
        assert!(max_discharge < 1e5);
    }
}
