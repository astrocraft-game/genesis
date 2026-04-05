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

/// Populate ocean dynamics layers on a grid that already has geology
/// (for is_ocean) and temperature (for base SST) populated.
pub fn generate_ocean_dynamics(grid: &mut SurfaceGrid) {
    flood_fill_basins(grid);
    assign_ocean_currents(grid);
    apply_boundary_sst(grid);
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

/// Assign current direction and speed to each ocean tile based on latitude
/// and east-west position within its local ocean span.
fn assign_ocean_currents(grid: &mut SurfaceGrid) {
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
            let pos_ew = east_west_position(grid, c, r);
            let (dir, speed) = current_vector(lat, pos_ew);
            grid.layers.ocean_current_direction_deg[idx] = dir;
            grid.layers.ocean_current_speed_ms[idx] = speed;
        }
    }
}

/// Boost SST on the western side of ocean spans (warm poleward currents)
/// and reduce it on the eastern side (cold equatorward currents). Strongest
/// at subtropical latitudes (~30°), fading to zero at equator and 60°.
fn apply_boundary_sst(grid: &mut SurfaceGrid) {
    let w = grid.width as usize;
    let h = grid.height as usize;
    for r in 0..h {
        for c in 0..w {
            let idx = r * w + c;
            if !grid.layers.is_ocean[idx] {
                continue;
            }
            let lat = grid.row_latitude(r as u16);
            let pos_ew = east_west_position(grid, c, r);
            let lat_factor = subtropical_band_factor(lat);
            // pos_ew: 0 = western boundary, 1 = eastern boundary.
            // West gets +6 °C, east gets −6 °C at peak subtropical latitude.
            let modifier = 6.0 * (1.0 - 2.0 * pos_ew) * lat_factor;
            grid.layers.sea_surface_temp_c[idx] += modifier;
            grid.layers.temperature_c[idx] = grid.layers.sea_surface_temp_c[idx];
        }
    }
}

/// Position within the east-west ocean span at this latitude, in [0, 1].
/// 0 = touching the western shore, 1 = touching the eastern shore.
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

/// Current direction (bearing flow travels toward) and speed in m/s.
fn current_vector(lat_deg: f32, pos_ew: f32) -> (f32, f32) {
    let abs_lat = lat_deg.abs();
    let is_nh = lat_deg >= 0.0;

    if abs_lat < 10.0 {
        // Equatorial current: westward, trade-wind driven.
        (270.0, 0.3)
    } else if abs_lat < 45.0 {
        // Subtropical gyre. Direction depends on position within the gyre:
        // western boundary → poleward, eastern → equatorward,
        // plus east-west bands at the top and bottom.
        if pos_ew < 0.25 {
            // Western boundary current (warm poleward): e.g. Gulf Stream.
            if is_nh {
                (0.0, 1.2)
            } else {
                (180.0, 1.2)
            }
        } else if pos_ew > 0.75 {
            // Eastern boundary current (cold equatorward).
            if is_nh {
                (180.0, 0.5)
            } else {
                (0.0, 0.5)
            }
        } else if abs_lat < 25.0 {
            // Equatorward half of gyre: westward flow.
            (270.0, 0.4)
        } else {
            // Poleward half: eastward flow (North Atlantic Drift).
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

/// Factor peaking at 30° latitude, zero at equator and 60°.
fn subtropical_band_factor(lat_deg: f32) -> f32 {
    let abs_lat = lat_deg.abs();
    if !(0.0..=60.0).contains(&abs_lat) {
        return 0.0;
    }
    let phase = ((abs_lat - 30.0).abs() / 30.0) * (std::f32::consts::PI / 2.0);
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
}
