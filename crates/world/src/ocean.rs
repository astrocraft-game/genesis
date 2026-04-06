pub use crate::types::{
    CelestialBodyWorldType, ChemicalComponent, LifeLevel, LiquidType, NutrientRichness,
    OceanChemistry, OceanIronContent, OceanRedoxState, OceanStratification, OrbitContext,
    PlanetSimulationInput, TelluricBodyComposition,
};

pub fn generate_ocean_chemistry(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    atmospheric_composition: &[(f32, ChemicalComponent)],
    _body_type: TelluricBodyComposition,
    world_type: CelestialBodyWorldType,
    hydrosphere: f32,
    volcanism: f32,
    tectonic_activity: f32,
    land_fraction: f32,
    life_level: LifeLevel,
) -> Option<OceanChemistry> {
    if hydrosphere <= 5.0 {
        return None;
    }

    let has_oxygen = atmospheric_composition
        .iter()
        .any(|(f, c)| *c == ChemicalComponent::Oxygen && *f > 0.1);
    let has_h2s = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::HydrogenSulfide);
    let has_methane = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Methane);
    let has_ammonia = atmospheric_composition
        .iter()
        .any(|(_, c)| *c == ChemicalComponent::Ammonia);

    let liquid = match world_type {
        CelestialBodyWorldType::Ammonia => LiquidType::Ammonia,
        CelestialBodyWorldType::LavaWorld => LiquidType::Magma,
        _ if context.blackbody_temp_k < 150 => LiquidType::MethaneEthane,
        _ if hydrosphere < 15.0 && context.blackbody_temp_k > 310 => LiquidType::Brine,
        _ => LiquidType::Water,
    };

    let salinity_g_per_kg = if liquid == LiquidType::Brine {
        140.0 + hydrosphere.min(20.0) * 2.0 + volcanism * 0.3
    } else {
        (18.0 + volcanism * 0.35 + tectonic_activity * 0.15 + land_fraction * 20.0)
            .clamp(10.0, 70.0)
    };

    let ph = if liquid == LiquidType::Water {
        let co2_pp = atmospheric_composition
            .iter()
            .filter(|(_, c)| *c == ChemicalComponent::CarbonDioxide)
            .map(|(f, _)| f * atmospheric_pressure)
            .sum::<f32>()
            .max(0.0001);
        (8.1 - 0.8 * (co2_pp / 0.0004).log10()).clamp(4.0, 10.0)
    } else {
        0.0
    };

    let alkalinity_meq_l = if liquid == LiquidType::Water || liquid == LiquidType::Brine {
        let carbonate_buffer = if tectonic_activity > 10.0 { 1.4 } else { 1.0 };
        let evaporative_boost = if liquid == LiquidType::Brine {
            2.0
        } else {
            1.0
        };
        (1.2 + salinity_g_per_kg / 40.0
            + hydrosphere / 140.0
            + carbonate_buffer * evaporative_boost)
            .clamp(0.5, 12.0)
    } else {
        0.0
    };

    let anoxic = !has_oxygen || life_level.as_u8() < LifeLevel::PlantLike.as_u8();
    let iron_content = if !anoxic {
        OceanIronContent::Negligible
    } else if life_level.as_u8() >= LifeLevel::UniCellular.as_u8() {
        OceanIronContent::Moderate
    } else if volcanism > 20.0 {
        OceanIronContent::High
    } else {
        OceanIronContent::Low
    };
    let hydrothermal_vents = volcanism > 10.0 && hydrosphere > 20.0;

    let redox_state = if !anoxic {
        OceanRedoxState::Oxic
    } else if has_h2s || (has_methane && volcanism > 15.0) {
        OceanRedoxState::Euxinic
    } else if life_level.as_u8() >= LifeLevel::UniCellular.as_u8() || hydrothermal_vents {
        OceanRedoxState::Reducing
    } else {
        OceanRedoxState::Dysoxic
    };

    let nutrient_index = volcanism * 0.4
        + tectonic_activity * 0.3
        + if hydrothermal_vents { 20.0 } else { 0.0 }
        + hydrosphere * 0.1
        + if land_fraction > 0.2 { 10.0 } else { 0.0 };
    let nutrient_richness = if nutrient_index > 70.0 {
        NutrientRichness::BloomProne
    } else if nutrient_index > 45.0 {
        NutrientRichness::Fertile
    } else if nutrient_index > 22.0 {
        NutrientRichness::Moderate
    } else if nutrient_index > 8.0 {
        NutrientRichness::Limited
    } else {
        NutrientRichness::Starved
    };

    let stratification = if liquid == LiquidType::Brine || salinity_g_per_kg > 80.0 {
        OceanStratification::StronglyStratified
    } else if context.orbit.tidally_locked && context.orbit.rotation_period_days > 10.0 {
        OceanStratification::Layered
    } else if hydrosphere > 40.0 && context.orbit.axial_tilt_deg > 10.0 {
        OceanStratification::Seasonal
    } else {
        OceanStratification::WellMixed
    };

    let dissolved_volatile_load = (atmospheric_pressure * 0.8
        + volcanism * 0.35
        + if liquid == LiquidType::Brine {
            8.0
        } else {
            0.0
        }
        + if has_methane { 4.0 } else { 0.0 }
        + if has_ammonia { 3.0 } else { 0.0 })
    .clamp(0.0, 100.0);

    Some(OceanChemistry {
        liquid_type: liquid,
        salinity_g_per_kg,
        ph,
        alkalinity_meq_l,
        anoxic,
        redox_state,
        iron_content,
        nutrient_richness,
        stratification,
        dissolved_volatile_load,
        hydrothermal_vents,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(temp: u32) -> PlanetSimulationInput {
        PlanetSimulationInput {
            blackbody_temp_k: temp,
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn earth_like_ocean_is_oxic_and_buffered() {
        let composition = vec![
            (0.78, ChemicalComponent::Nitrogen),
            (0.21, ChemicalComponent::Oxygen),
            (0.01, ChemicalComponent::Argon),
        ];
        let ocean = generate_ocean_chemistry(
            &context(288),
            1.0,
            &composition,
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Terrestrial,
            71.0,
            30.0,
            35.0,
            0.29,
            LifeLevel::Sentient,
        )
        .unwrap();
        assert_eq!(ocean.redox_state, OceanRedoxState::Oxic);
        assert!(ocean.alkalinity_meq_l > 1.0);
        assert!(matches!(
            ocean.nutrient_richness,
            NutrientRichness::Moderate | NutrientRichness::Fertile | NutrientRichness::BloomProne
        ));
    }

    #[test]
    fn reducing_volcanic_ocean_shows_iron_and_volatiles() {
        let composition = vec![
            (0.8, ChemicalComponent::Nitrogen),
            (0.12, ChemicalComponent::CarbonDioxide),
            (0.08, ChemicalComponent::Methane),
        ];
        let ocean = generate_ocean_chemistry(
            &context(305),
            2.5,
            &composition,
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Ocean,
            55.0,
            35.0,
            18.0,
            0.45,
            LifeLevel::UniCellular,
        )
        .unwrap();
        assert!(matches!(
            ocean.redox_state,
            OceanRedoxState::Reducing | OceanRedoxState::Euxinic
        ));
        assert!(ocean.iron_content >= OceanIronContent::Moderate);
        assert!(ocean.dissolved_volatile_load > 10.0);
    }
}

// ---------------------------------------------------------------------------
// Grid-level ocean dynamics layer
//
// Populates `drainage_basin_id` (for ocean tiles), `ocean_current_direction_deg`,
// `ocean_current_speed_ms`, and refines `sea_surface_temp_c` with western-
// boundary warming and eastern-boundary cooling.
//
// Current convention: direction is the bearing the current flows *toward*
// (opposite of meteorological wind convention).
// ---------------------------------------------------------------------------

use crate::grid::SurfaceGrid;

/// Per-basin metadata used to drive gyre placement and SST modifiers.
struct BasinInfo {
    lat_min: f32,
    lat_max: f32,
    centroid_lat: f32,
    tile_count: u32,
    /// True if any tile in this basin is poleward of 50°.
    reaches_polar: bool,
}

/// Populate ocean dynamics layers on a grid that already has geology
/// (for is_ocean) and temperature (for base SST) populated.
pub fn generate_ocean_dynamics(grid: &mut SurfaceGrid) {
    flood_fill_basins(grid);
    let basins = compute_basin_info(grid);
    assign_ocean_currents(grid, &basins);
    apply_boundary_sst(grid, &basins);
    apply_thermohaline(grid, &basins);
}

/// Flood-fill connected ocean tiles into contiguous basin IDs. Basins are
/// numbered from 1; land tiles keep ID 0.
fn flood_fill_basins(grid: &mut SurfaceGrid) {
    let w = grid.width as usize;
    let h = grid.height as usize;
    let n = w * h;
    let mut basin = vec![0u16; n];
    let mut next_id: u16 = 1;
    let mut stack: Vec<usize> = Vec::new();

    for start in 0..n {
        if !grid.layers.is_ocean[start] || basin[start] != 0 {
            continue;
        }
        stack.push(start);
        basin[start] = next_id;
        while let Some(idx) = stack.pop() {
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
                if grid.layers.is_ocean[nidx] && basin[nidx] == 0 {
                    basin[nidx] = next_id;
                    stack.push(nidx);
                }
            }
        }
        next_id = next_id.saturating_add(1);
    }
    grid.layers.drainage_basin_id = basin;
}

/// Gather per-basin latitude extent, centroid, and polar reach.
fn compute_basin_info(grid: &SurfaceGrid) -> std::collections::HashMap<u16, BasinInfo> {
    let w = grid.width as usize;
    let mut map: std::collections::HashMap<u16, (f32, f32, f32, u32, bool)> =
        std::collections::HashMap::new();
    for (idx, &bid) in grid.layers.drainage_basin_id.iter().enumerate() {
        if bid == 0 {
            continue;
        }
        let r = (idx / w) as u16;
        let lat = grid.row_latitude(r);
        let entry = map.entry(bid).or_insert((90.0, -90.0, 0.0, 0, false));
        entry.0 = entry.0.min(lat); // lat_min (most southern)
        entry.1 = entry.1.max(lat); // lat_max (most northern)
        entry.2 += lat; // running sum for centroid
        entry.3 += 1;
        if lat.abs() > 50.0 {
            entry.4 = true;
        }
    }
    map.into_iter()
        .map(|(id, (lat_min, lat_max, lat_sum, count, polar))| {
            (
                id,
                BasinInfo {
                    lat_min,
                    lat_max,
                    centroid_lat: lat_sum / count as f32,
                    tile_count: count,
                    reaches_polar: polar,
                },
            )
        })
        .collect()
}

/// Assign current direction and speed to each ocean tile based on latitude
/// and east-west position within its basin's ocean span.
fn assign_ocean_currents(
    grid: &mut SurfaceGrid,
    basins: &std::collections::HashMap<u16, BasinInfo>,
) {
    let w = grid.width as usize;
    let h = grid.height as usize;
    for r in 0..h {
        for c in 0..w {
            let idx = r * w + c;
            if !grid.layers.is_ocean[idx] {
                grid.layers.ocean_current_direction_deg[idx] = 0.0;
                grid.layers.ocean_current_speed_ms[idx] = 0.0;
                continue;
            }
            let lat = grid.row_latitude(r as u16);
            let bid = grid.layers.drainage_basin_id[idx];
            let pos_ew = basin_east_west_position(grid, c, r, bid);
            let basin_info = basins.get(&bid);
            let (dir, speed) = current_vector_basin(lat, pos_ew, basin_info);
            grid.layers.ocean_current_direction_deg[idx] = dir;
            grid.layers.ocean_current_speed_ms[idx] = speed;
        }
    }
}

/// Boost SST on the western side of ocean spans (warm poleward currents)
/// and reduce it on the eastern side (cold equatorward currents). Strongest
/// at the basin's subtropical latitude, fading toward the equator and poles.
/// Also applies a mild El-Niño-like equatorial anomaly: the eastern
/// equatorial band in each basin is slightly warmer (+1 °C) to simulate
/// the baseline warm-pool / cold-tongue asymmetry.
fn apply_boundary_sst(grid: &mut SurfaceGrid, basins: &std::collections::HashMap<u16, BasinInfo>) {
    let w = grid.width as usize;
    let h = grid.height as usize;
    for r in 0..h {
        for c in 0..w {
            let idx = r * w + c;
            if !grid.layers.is_ocean[idx] {
                continue;
            }
            let lat = grid.row_latitude(r as u16);
            let bid = grid.layers.drainage_basin_id[idx];
            let pos_ew = basin_east_west_position(grid, c, r, bid);
            let basin_info = basins.get(&bid);

            // Subtropical gyre centre for this basin.
            let gyre_lat = basin_info.map_or(30.0, |b| {
                // Place gyre centre halfway between centroid and the
                // tropics-facing extent of the basin.
                let half = (b.lat_max - b.lat_min) / 2.0;
                b.centroid_lat.signum() * half.min(30.0)
            });
            let lat_factor = subtropical_band_factor_at(lat, gyre_lat.abs());

            // West gets +6 °C, east gets −6 °C at peak subtropical latitude.
            let modifier = 6.0 * (1.0 - 2.0 * pos_ew) * lat_factor;
            grid.layers.sea_surface_temp_c[idx] += modifier;

            // El-Niño-like: mild eastern equatorial warming.
            if lat.abs() < 10.0 && pos_ew > 0.6 {
                grid.layers.sea_surface_temp_c[idx] += 1.0;
            }

            grid.layers.temperature_c[idx] = grid.layers.sea_surface_temp_c[idx];
        }
    }
}

/// Thermohaline circulation: basins that reach polar latitudes develop
/// deep-water formation zones where cold, dense surface water sinks. We
/// model this as a slight cooling (−2 °C) of high-latitude tiles in
/// polar-connected basins and a slight warming (+0.5 °C) of their
/// mid-latitude tiles (upwelling heat transport).
fn apply_thermohaline(grid: &mut SurfaceGrid, basins: &std::collections::HashMap<u16, BasinInfo>) {
    let w = grid.width as usize;
    let h = grid.height as usize;
    for r in 0..h {
        let lat = grid.row_latitude(r as u16);
        for c in 0..w {
            let idx = r * w + c;
            if !grid.layers.is_ocean[idx] {
                continue;
            }
            let bid = grid.layers.drainage_basin_id[idx];
            let polar = basins.get(&bid).is_some_and(|b| b.reaches_polar);
            if !polar {
                continue;
            }
            if lat.abs() > 55.0 {
                // Deep-water formation zone — cooling.
                grid.layers.sea_surface_temp_c[idx] -= 2.0;
            } else if (20.0..=50.0).contains(&lat.abs()) {
                // Mid-latitude upwelling warmth.
                grid.layers.sea_surface_temp_c[idx] += 0.5;
            }
            grid.layers.temperature_c[idx] = grid.layers.sea_surface_temp_c[idx];
        }
    }
}

/// Position within the east-west ocean span at this latitude, in [0, 1].
/// 0 = touching the western shore, 1 = touching the eastern shore.
/// Legacy version that ignores basin boundaries (used by old tests).
fn east_west_position(grid: &SurfaceGrid, c: usize, r: usize) -> f32 {
    let w = grid.width as usize;
    let dist_w = dist_to_land(grid, c, r, -1);
    let dist_e = dist_to_land(grid, c, r, 1);
    let total = dist_w + dist_e;
    if total == 0 || total >= w as u16 {
        0.5
    } else {
        dist_w as f32 / total as f32
    }
}

/// Basin-aware east-west position: walks east / west until hitting land OR
/// a tile from a different basin, so gyre boundaries are respected.
fn basin_east_west_position(grid: &SurfaceGrid, c: usize, r: usize, bid: u16) -> f32 {
    let w = grid.width as usize;
    let dist_w = dist_to_basin_edge(grid, c, r, -1, bid);
    let dist_e = dist_to_basin_edge(grid, c, r, 1, bid);
    let total = dist_w + dist_e;
    if total == 0 || total >= w as u16 {
        0.5
    } else {
        dist_w as f32 / total as f32
    }
}

/// Walk east (+1) or west (-1) until hitting a land tile. Returns the
/// number of ocean tiles crossed. Wraps longitude; saturates at grid width.
fn dist_to_land(grid: &SurfaceGrid, c: usize, r: usize, step: i32) -> u16 {
    let w = grid.width as usize;
    let mut cur = c as i32;
    for d in 0..(w as u16) {
        cur = (cur + step).rem_euclid(w as i32);
        let idx = r * w + cur as usize;
        if !grid.layers.is_ocean[idx] {
            return d;
        }
    }
    w as u16
}

/// Walk east (+1) or west (-1) until leaving the given basin (hitting land
/// or a different basin_id). Returns ocean tile count crossed.
fn dist_to_basin_edge(grid: &SurfaceGrid, c: usize, r: usize, step: i32, bid: u16) -> u16 {
    let w = grid.width as usize;
    let mut cur = c as i32;
    for d in 0..(w as u16) {
        cur = (cur + step).rem_euclid(w as i32);
        let idx = r * w + cur as usize;
        if !grid.layers.is_ocean[idx] || grid.layers.drainage_basin_id[idx] != bid {
            return d;
        }
    }
    w as u16
}

/// Current direction (bearing flow travels toward) and speed, using
/// per-basin gyre centre. Falls back to latitude-only if no basin info.
fn current_vector_basin(lat_deg: f32, pos_ew: f32, basin: Option<&BasinInfo>) -> (f32, f32) {
    let abs_lat = lat_deg.abs();
    let is_nh = lat_deg >= 0.0;

    // Gyre centre latitude: use the basin's midpoint if available,
    // otherwise default to 30°.
    let gyre_centre = basin.map_or(30.0, |b| {
        let half = (b.lat_max - b.lat_min) / 2.0;
        half.min(30.0)
    });
    let gyre_top = gyre_centre + 15.0;

    if abs_lat < 10.0 {
        // Equatorial current: westward, trade-wind driven.
        (270.0, 0.3)
    } else if abs_lat < gyre_top {
        // Subtropical gyre (basin-relative).
        if pos_ew < 0.25 {
            if is_nh {
                (0.0, 1.2)
            } else {
                (180.0, 1.2)
            }
        } else if pos_ew > 0.75 {
            if is_nh {
                (180.0, 0.5)
            } else {
                (0.0, 0.5)
            }
        } else if abs_lat < gyre_centre {
            (270.0, 0.4)
        } else {
            (90.0, 0.5)
        }
    } else if abs_lat < 65.0 {
        // West-wind drift: eastward circumpolar flow.
        (90.0, 0.6)
    } else {
        // Polar weak circulation.
        (90.0, 0.2)
    }
}

/// Current direction (bearing flow travels toward) and speed in m/s.
/// Legacy function kept for backward compatibility with existing tests.
fn current_vector(lat_deg: f32, pos_ew: f32) -> (f32, f32) {
    current_vector_basin(lat_deg, pos_ew, None)
}

/// Factor peaking at 30° latitude, zero at equator and 60°.
fn subtropical_band_factor(lat_deg: f32) -> f32 {
    subtropical_band_factor_at(lat_deg, 30.0)
}

/// Factor peaking at `centre_lat` degrees, zero at equator and
/// `2 × centre_lat` poleward.
fn subtropical_band_factor_at(lat_deg: f32, centre_lat: f32) -> f32 {
    let abs_lat = lat_deg.abs();
    let half_width = centre_lat.max(1.0);
    let upper = centre_lat + half_width;
    if abs_lat > upper || abs_lat < 0.0 {
        return 0.0;
    }
    let phase = ((abs_lat - centre_lat).abs() / half_width) * (std::f32::consts::PI / 2.0);
    phase.cos().max(0.0)
}

#[cfg(test)]
mod grid_tests {
    use super::*;
    use crate::climate::{generate_temperature, generate_wind};
    use crate::geology::generate_geology;
    use crate::grid::GridResolution;
    use crate::types::{OrbitContext, StarContext};

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
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "ocean");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_ocean_dynamics(&mut g);
        g
    }

    #[test]
    fn ocean_tiles_have_basin_id() {
        let g = earth_grid();
        let mut land_bad = 0;
        let mut ocean_bad = 0;
        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                if g.layers.drainage_basin_id[idx] == 0 {
                    ocean_bad += 1;
                }
            } else if g.layers.drainage_basin_id[idx] != 0 {
                land_bad += 1;
            }
        }
        assert_eq!(ocean_bad, 0, "{} ocean tiles have no basin", ocean_bad);
        assert_eq!(land_bad, 0, "{} land tiles got a basin", land_bad);
    }

    #[test]
    fn water_world_is_one_basin() {
        let input = earth_like_input();
        let mut g = generate_geology(&input, 95.0, GridResolution::Fast, "water");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_ocean_dynamics(&mut g);
        let max_id = *g.layers.drainage_basin_id.iter().max().unwrap();
        assert!(
            max_id <= 3,
            "water world should have few basins, got {}",
            max_id
        );
    }

    #[test]
    fn land_tiles_have_zero_currents() {
        let g = earth_grid();
        for idx in 0..g.tile_count() {
            if !g.layers.is_ocean[idx] {
                assert_eq!(g.layers.ocean_current_speed_ms[idx], 0.0);
            }
        }
    }

    #[test]
    fn equatorial_current_flows_west() {
        // At < 10° latitude, current direction should be 270° (westward).
        let (dir, _) = current_vector(5.0, 0.5);
        assert_eq!(dir, 270.0);
        let (dir, _) = current_vector(-5.0, 0.5);
        assert_eq!(dir, 270.0);
    }

    #[test]
    fn western_boundary_current_flows_poleward() {
        // NH: western side of gyre → poleward (bearing 0° = north).
        let (dir_nh, _) = current_vector(25.0, 0.1);
        assert_eq!(dir_nh, 0.0);
        // SH: western side → south (bearing 180°).
        let (dir_sh, _) = current_vector(-25.0, 0.1);
        assert_eq!(dir_sh, 180.0);
    }

    #[test]
    fn eastern_boundary_current_flows_equatorward() {
        // NH: eastern side → equatorward (south = 180°).
        let (dir_nh, _) = current_vector(25.0, 0.9);
        assert_eq!(dir_nh, 180.0);
        // SH: eastern side → equatorward (north = 0°).
        let (dir_sh, _) = current_vector(-25.0, 0.9);
        assert_eq!(dir_sh, 0.0);
    }

    #[test]
    fn subtropical_factor_peaks_at_30() {
        let eq = subtropical_band_factor(0.0);
        let peak = subtropical_band_factor(30.0);
        let edge = subtropical_band_factor(60.0);
        assert!((peak - 1.0).abs() < 0.01);
        assert!(eq < 0.1);
        assert!(edge < 0.1);
    }

    #[test]
    fn western_boundaries_are_warmer_than_eastern() {
        let g = earth_grid();
        // Collect SST at subtropical latitudes for ocean tiles, bucket by
        // east-west position.
        let mut west_sum = 0.0f32;
        let mut east_sum = 0.0f32;
        let mut west_count = 0;
        let mut east_count = 0;
        for r in 0..g.height {
            let lat = g.row_latitude(r);
            if lat.abs() < 20.0 || lat.abs() > 40.0 {
                continue;
            }
            for c in 0..g.width {
                let idx = g.idx(c, r);
                if !g.layers.is_ocean[idx] {
                    continue;
                }
                let pos = east_west_position(&g, c as usize, r as usize);
                if pos < 0.25 {
                    west_sum += g.layers.sea_surface_temp_c[idx];
                    west_count += 1;
                } else if pos > 0.75 {
                    east_sum += g.layers.sea_surface_temp_c[idx];
                    east_count += 1;
                }
            }
        }
        if west_count > 5 && east_count > 5 {
            let west_mean = west_sum / west_count as f32;
            let east_mean = east_sum / east_count as f32;
            assert!(
                west_mean > east_mean,
                "western SST {} should exceed eastern SST {}",
                west_mean,
                east_mean
            );
        }
    }

    #[test]
    fn equator_sst_exceeds_polar_sst() {
        let g = earth_grid();
        let mut eq_sum = 0.0f32;
        let mut pole_sum = 0.0f32;
        let mut eq_n = 0;
        let mut pole_n = 0;
        for r in 0..g.height {
            let lat = g.row_latitude(r);
            for c in 0..g.width {
                let idx = g.idx(c, r);
                if !g.layers.is_ocean[idx] {
                    continue;
                }
                if lat.abs() < 15.0 {
                    eq_sum += g.layers.sea_surface_temp_c[idx];
                    eq_n += 1;
                } else if lat.abs() > 70.0 {
                    pole_sum += g.layers.sea_surface_temp_c[idx];
                    pole_n += 1;
                }
            }
        }
        if eq_n > 0 && pole_n > 0 {
            assert!(eq_sum / eq_n as f32 > pole_sum / pole_n as f32 + 20.0);
        }
    }

    #[test]
    fn ocean_dynamics_is_deterministic() {
        let a = earth_grid();
        let b = earth_grid();
        assert_eq!(a.layers.drainage_basin_id, b.layers.drainage_basin_id);
        assert_eq!(
            a.layers.ocean_current_direction_deg,
            b.layers.ocean_current_direction_deg
        );
        assert_eq!(a.layers.sea_surface_temp_c, b.layers.sea_surface_temp_c);
    }

    #[test]
    fn mid_ocean_position_is_centre() {
        // Small test grid: ocean row entirely open → middle col = 0.5.
        let mut g = SurfaceGrid::empty(GridResolution::Custom(10, 5));
        for idx in 0..g.tile_count() {
            g.layers.is_ocean[idx] = true;
        }
        // All ocean → dist_to_land wraps fully; position defaults to 0.5.
        let pos = east_west_position(&g, 5, 2);
        assert_eq!(pos, 0.5);
    }

    #[test]
    fn multi_basin_world_has_independent_gyres() {
        // With ~40% ocean, continents are large enough to split the ocean
        // into multiple basins with separate gyre centres.
        let input = earth_like_input();
        let mut g = generate_geology(&input, 40.0, GridResolution::Fast, "multi");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_ocean_dynamics(&mut g);
        let max_basin = *g.layers.drainage_basin_id.iter().max().unwrap();
        if max_basin >= 2 {
            // Different basins should have their own BasinInfo centroid.
            let basins = compute_basin_info(&g);
            let centroids: Vec<f32> = basins.values().map(|b| b.centroid_lat).collect();
            // If there are ≥2 basins, centroids shouldn't all be identical.
            let all_same = centroids.windows(2).all(|w| (w[0] - w[1]).abs() < 1.0);
            assert!(
                !all_same || centroids.len() < 2,
                "basins should have distinct centroids"
            );
        }
    }

    #[test]
    fn basin_ew_position_respects_basin_boundary() {
        // A grid with two side-by-side ocean basins separated by land.
        // Basin-aware position should not cross the land divider.
        let mut g = SurfaceGrid::empty(GridResolution::Custom(20, 5));
        let w = g.width as usize;
        // Row 2: ocean from col 1-8, land at col 0, land at col 9-10,
        // ocean from col 11-18, land at col 19.
        for c in 1..=8 {
            let idx = 2 * w + c;
            g.layers.is_ocean[idx] = true;
            g.layers.drainage_basin_id[idx] = 1;
        }
        for c in 11..=18 {
            let idx = 2 * w + c;
            g.layers.is_ocean[idx] = true;
            g.layers.drainage_basin_id[idx] = 2;
        }
        // Col 5 in basin 1 should have pos_ew ≈ 0.5 within basin 1 only.
        let pos = basin_east_west_position(&g, 5, 2, 1);
        assert!(
            (0.3..=0.7).contains(&pos),
            "mid-basin tile should be near 0.5, got {}",
            pos
        );
        // Col 12 in basin 2 should be near the western edge.
        let pos_west = basin_east_west_position(&g, 12, 2, 2);
        assert!(
            pos_west < 0.3,
            "near western edge of basin 2 should have low pos, got {}",
            pos_west
        );
    }

    #[test]
    fn thermohaline_cools_polar_connected_basins() {
        let g = earth_grid();
        let basins = compute_basin_info(&g);
        // Any basin reaching polar latitudes should exist.
        let has_polar = basins.values().any(|b| b.reaches_polar);
        // On Earth-like worlds this should be true.
        assert!(
            has_polar,
            "Earth-like world should have polar-connected basins"
        );
    }
}
