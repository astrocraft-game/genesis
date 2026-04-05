//! Episodic natural events and disasters on a SurfaceGrid.
//!
//! Simulates per-year rolls for volcanic eruptions, earthquakes, hurricanes,
//! wildfires, droughts, floods, and meteorite impacts. Each event type
//! requires specific tile conditions (tectonic boundary for quakes, warm
//! SST for hurricanes, arid summer for wildfires, etc.). Deterministic
//! from the seed.

use crate::grid::{BoundaryKind, SurfaceGrid};
use crate::types::BiomeType;
use seeded_dice_roller::SeededDiceRoller;

/// Categories of natural events. Non-exhaustive because more event types
/// (blizzards, ice storms, tsunamis, solar flares) may be added later.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum EventKind {
    VolcanicEruption,
    Earthquake,
    Hurricane,
    Wildfire,
    Drought,
    Flood,
    MeteoriteImpact,
}

/// A single discrete event at a tile and time.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NaturalEvent {
    pub kind: EventKind,
    pub tile_idx: usize,
    pub year: u32,
    /// 0.0 – 1.0 intensity (death toll, area scorched, etc. scale with this).
    pub magnitude: f32,
    /// Event duration in days (0 for instantaneous events).
    pub duration_days: u16,
}

/// Simulate `years` worth of natural events on the grid. Returns all
/// events in chronological order (year ascending).
pub fn generate_events(grid: &SurfaceGrid, years: u32, seed: &str) -> Vec<NaturalEvent> {
    let mut rng = SeededDiceRoller::new(seed, "natural_events");
    let mut events = Vec::new();
    let n = grid.tile_count();

    // Pre-classify tiles so we don't iterate whole grid per-year.
    let mut tectonic: Vec<usize> = Vec::new();
    let mut volcanic_candidates: Vec<usize> = Vec::new();
    let mut warm_ocean: Vec<usize> = Vec::new();
    let mut arid_land: Vec<usize> = Vec::new();
    let mut river_tiles: Vec<usize> = Vec::new();
    for idx in 0..n {
        if grid.layers.tectonic_boundary[idx] != BoundaryKind::None {
            tectonic.push(idx);
            if !grid.layers.is_ocean[idx]
                && matches!(
                    grid.layers.tectonic_boundary[idx],
                    BoundaryKind::Convergent | BoundaryKind::Divergent
                )
            {
                volcanic_candidates.push(idx);
            }
        }
        if grid.layers.is_ocean[idx] && grid.layers.sea_surface_temp_c[idx] > 27.0 {
            warm_ocean.push(idx);
        }
        if !grid.layers.is_ocean[idx]
            && matches!(
                grid.layers.biome[idx],
                BiomeType::Desert | BiomeType::Savanna
            )
            && grid.layers.temperature_summer_c[idx] > 28.0
        {
            arid_land.push(idx);
        }
        if !grid.layers.is_ocean[idx] && grid.layers.river_discharge_m3s[idx] > 500.0 {
            river_tiles.push(idx);
        }
    }

    // Basins (for droughts) — iterate unique drainage_basin_ids on land.
    // BTreeSet keeps iteration deterministic.
    let mut basins = std::collections::BTreeSet::new();
    for idx in 0..n {
        if !grid.layers.is_ocean[idx] {
            basins.insert(grid.layers.drainage_basin_id[idx]);
        }
    }
    basins.remove(&0);
    let basin_ids: Vec<u16> = basins.into_iter().collect();

    for year in 0..years {
        // Volcanic eruptions: 0.1% per candidate per year.
        for &tile in &volcanic_candidates {
            if roll_pct(&mut rng, 0.1) {
                events.push(NaturalEvent {
                    kind: EventKind::VolcanicEruption,
                    tile_idx: tile,
                    year,
                    magnitude: (rng.gen_f64() as f32 * 0.8 + 0.2).clamp(0.0, 1.0),
                    duration_days: 7 + (rng.gen_u32() % 120) as u16,
                });
            }
        }
        // Earthquakes: 0.4% per boundary tile per year.
        for &tile in &tectonic {
            if roll_pct(&mut rng, 0.4) {
                events.push(NaturalEvent {
                    kind: EventKind::Earthquake,
                    tile_idx: tile,
                    year,
                    magnitude: (rng.gen_f64() as f32 * 0.9 + 0.1).clamp(0.0, 1.0),
                    duration_days: 0,
                });
            }
        }
        // Hurricanes: 1% per warm ocean tile.
        for &tile in &warm_ocean {
            if roll_pct(&mut rng, 1.0) {
                events.push(NaturalEvent {
                    kind: EventKind::Hurricane,
                    tile_idx: tile,
                    year,
                    magnitude: (rng.gen_f64() as f32 * 0.7 + 0.3).clamp(0.0, 1.0),
                    duration_days: 3 + (rng.gen_u32() % 12) as u16,
                });
            }
        }
        // Wildfires: 2% per arid hot tile.
        for &tile in &arid_land {
            if roll_pct(&mut rng, 2.0) {
                events.push(NaturalEvent {
                    kind: EventKind::Wildfire,
                    tile_idx: tile,
                    year,
                    magnitude: (rng.gen_f64() as f32 * 0.7 + 0.2).clamp(0.0, 1.0),
                    duration_days: 1 + (rng.gen_u32() % 30) as u16,
                });
            }
        }
        // Droughts: 0.5% per basin per year.
        for &bid in &basin_ids {
            if roll_pct(&mut rng, 0.5) {
                // Pick any land tile in this basin as the anchor.
                let anchor = grid
                    .layers
                    .drainage_basin_id
                    .iter()
                    .position(|&b| b == bid)
                    .unwrap_or(0);
                events.push(NaturalEvent {
                    kind: EventKind::Drought,
                    tile_idx: anchor,
                    year,
                    magnitude: (rng.gen_f64() as f32 * 0.6 + 0.3).clamp(0.0, 1.0),
                    duration_days: 90 + (rng.gen_u32() % 275) as u16,
                });
            }
        }
        // Floods: 1% per major river tile.
        for &tile in &river_tiles {
            if roll_pct(&mut rng, 1.0) {
                events.push(NaturalEvent {
                    kind: EventKind::Flood,
                    tile_idx: tile,
                    year,
                    magnitude: (rng.gen_f64() as f32 * 0.6 + 0.3).clamp(0.0, 1.0),
                    duration_days: 1 + (rng.gen_u32() % 30) as u16,
                });
            }
        }
        // Meteorite impacts: ~0.05% per planet per year.
        if roll_pct(&mut rng, 0.05) {
            let tile = (rng.gen_usize()) % n;
            events.push(NaturalEvent {
                kind: EventKind::MeteoriteImpact,
                tile_idx: tile,
                year,
                magnitude: (rng.gen_f64() as f32).clamp(0.0, 1.0),
                duration_days: 0,
            });
        }
    }
    events
}

fn roll_pct(rng: &mut SeededDiceRoller, percent: f32) -> bool {
    (rng.gen_f64() as f32) < percent / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::climate::{generate_biomes, generate_temperature, generate_wind};
    use crate::geology::generate_geology;
    use crate::grid::GridResolution;
    use crate::hydrology::{generate_hydrology, generate_precipitation};
    use crate::ocean::generate_ocean_dynamics;
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
        let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "events");
        generate_temperature(&input, 33.0, &mut g);
        generate_wind(&input, 1.0, &mut g);
        generate_precipitation(&input, 1.0, 71.0, &mut g);
        generate_ocean_dynamics(&mut g);
        generate_hydrology(1.0, &mut g);
        generate_biomes(&mut g);
        g
    }

    #[test]
    fn events_generated_over_years() {
        let g = earth_grid();
        let events = generate_events(&g, 1000, "run_a");
        assert!(!events.is_empty());
    }

    #[test]
    fn events_are_deterministic() {
        let g = earth_grid();
        let a = generate_events(&g, 100, "det");
        let b = generate_events(&g, 100, "det");
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x.kind, y.kind);
            assert_eq!(x.tile_idx, y.tile_idx);
            assert_eq!(x.year, y.year);
        }
    }

    #[test]
    fn volcanoes_only_on_active_boundaries() {
        let g = earth_grid();
        let events = generate_events(&g, 2000, "volc");
        for e in &events {
            if e.kind == EventKind::VolcanicEruption {
                let b = g.layers.tectonic_boundary[e.tile_idx];
                assert!(
                    matches!(b, BoundaryKind::Convergent | BoundaryKind::Divergent),
                    "volcano on boundary {:?}",
                    b
                );
                assert!(!g.layers.is_ocean[e.tile_idx]);
            }
        }
    }

    #[test]
    fn earthquakes_only_on_tectonic_boundaries() {
        let g = earth_grid();
        let events = generate_events(&g, 500, "quake");
        for e in &events {
            if e.kind == EventKind::Earthquake {
                assert_ne!(g.layers.tectonic_boundary[e.tile_idx], BoundaryKind::None);
            }
        }
    }

    #[test]
    fn hurricanes_avoid_cold_oceans() {
        let g = earth_grid();
        let events = generate_events(&g, 500, "hurr");
        for e in &events {
            if e.kind == EventKind::Hurricane {
                assert!(g.layers.is_ocean[e.tile_idx]);
                assert!(g.layers.sea_surface_temp_c[e.tile_idx] > 27.0);
            }
        }
    }

    #[test]
    fn wildfires_need_heat_and_dry_biome() {
        let g = earth_grid();
        let events = generate_events(&g, 500, "fire");
        for e in &events {
            if e.kind == EventKind::Wildfire {
                assert!(g.layers.temperature_summer_c[e.tile_idx] > 28.0);
                assert!(matches!(
                    g.layers.biome[e.tile_idx],
                    BiomeType::Desert | BiomeType::Savanna
                ));
            }
        }
    }

    #[test]
    fn floods_happen_on_rivers() {
        let g = earth_grid();
        let events = generate_events(&g, 1000, "flood");
        for e in &events {
            if e.kind == EventKind::Flood {
                assert!(!g.layers.is_ocean[e.tile_idx]);
                assert!(g.layers.river_discharge_m3s[e.tile_idx] > 500.0);
            }
        }
    }

    #[test]
    fn meteorite_impacts_are_very_rare() {
        let g = earth_grid();
        let events = generate_events(&g, 10_000, "impact");
        let impacts = events
            .iter()
            .filter(|e| e.kind == EventKind::MeteoriteImpact)
            .count();
        // ~0.05% per year × 10k years ≈ 5 expected. Allow generous bounds.
        assert!(impacts < 50, "too many impacts: {}", impacts);
    }

    #[test]
    fn magnitude_is_in_unit_range() {
        let g = earth_grid();
        let events = generate_events(&g, 100, "mag");
        for e in &events {
            assert!(
                (0.0..=1.0).contains(&e.magnitude),
                "magnitude {} out of range",
                e.magnitude
            );
        }
    }

    #[test]
    fn years_are_bounded_by_simulation_length() {
        let g = earth_grid();
        let events = generate_events(&g, 100, "years");
        for e in &events {
            assert!(e.year < 100);
        }
    }

    #[test]
    fn more_years_means_more_events() {
        let g = earth_grid();
        let short = generate_events(&g, 10, "short").len();
        let long = generate_events(&g, 100, "long").len();
        assert!(long > short);
    }
}
