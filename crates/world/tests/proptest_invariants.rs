//! Property-based tests for the world-generation pipeline.
//!
//! Uses `proptest` to generate random `PlanetSimulationInput`s and verify
//! that key invariants hold for any plausible input combination.

use proptest::prelude::*;
use world::climate::generate_temperature;
use world::geology::generate_geology;
use world::grid::{generate_surface_grid, GridResolution};
use world::types::{BiomeType, OrbitContext, PlanetSimulationInput, StarContext};

/// Strategy for a plausible `PlanetSimulationInput`.
fn arb_input() -> impl Strategy<Value = PlanetSimulationInput> {
    (
        1u32..1000,   // body_id
        0.3f64..3.0,  // radius_earth
        150u32..400,  // blackbody_temp_k
        0.0f32..90.0, // axial_tilt_deg
        0.5f32..10.0, // star_age_gyr
    )
        .prop_map(|(id, radius, bbtemp, tilt, age)| PlanetSimulationInput {
            body_id: id,
            body_radius_earth: radius,
            blackbody_temp_k: bbtemp,
            star: StarContext {
                age_gyr: age,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: tilt,
                ..Default::default()
            },
            ..Default::default()
        })
}

/// Strategy for pipeline parameters alongside the input.
fn arb_pipeline() -> impl Strategy<Value = (PlanetSimulationInput, f32, f32, f32, u64)> {
    (
        arb_input(),
        0.0f32..50.0,  // greenhouse_delta_k
        0.1f32..5.0,   // atmospheric_pressure
        0.0f32..100.0, // hydrosphere_pct
        0u64..10000,   // seed suffix
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]

    #[test]
    fn temperature_within_plausible_bounds(
        (input, greenhouse, _atm_pressure, hydro, seed_n) in arb_pipeline()
    ) {
        let seed = format!("prop_{}", seed_n);
        let mut g = generate_geology(&input, hydro, GridResolution::Fast, &seed);
        generate_temperature(&input, greenhouse, &mut g);

        for idx in 0..g.tile_count() {
            let t = g.layers.temperature_c[idx];
            // Plausible range: -200 °C to +200 °C for any terrestrial body.
            // (blackbody_temp 150K → mean -123°C before lapse/latitude).
            prop_assert!(
                (-200.0..=200.0).contains(&t),
                "temperature {} out of plausible range at tile {}",
                t, idx
            );
        }
    }

    #[test]
    fn biome_always_assigned(
        (input, greenhouse, atm_pressure, hydro, seed_n) in arb_pipeline()
    ) {
        let seed = format!("prop_{}", seed_n);
        let g = generate_surface_grid(
            &input, greenhouse, atm_pressure, hydro,
            GridResolution::Fast, &seed,
        );

        for idx in 0..g.tile_count() {
            let b = g.layers.biome[idx];
            if g.layers.is_ocean[idx] {
                prop_assert_eq!(b, BiomeType::Ocean,
                    "ocean tile {} has biome {:?}", idx, b);
            } else {
                prop_assert_ne!(b, BiomeType::Ocean,
                    "land tile {} has Ocean biome", idx);
            }
        }
    }

    #[test]
    fn discharge_is_non_negative(
        (input, greenhouse, atm_pressure, hydro, seed_n) in arb_pipeline()
    ) {
        let seed = format!("prop_{}", seed_n);
        let g = generate_surface_grid(
            &input, greenhouse, atm_pressure, hydro,
            GridResolution::Fast, &seed,
        );

        for idx in 0..g.tile_count() {
            let d = g.layers.river_discharge_m3s[idx];
            prop_assert!(d >= 0.0,
                "negative discharge {} at tile {}", d, idx);
        }
    }

    #[test]
    fn ocean_tiles_have_zero_discharge(
        (input, greenhouse, atm_pressure, hydro, seed_n) in arb_pipeline()
    ) {
        let seed = format!("prop_{}", seed_n);
        let g = generate_surface_grid(
            &input, greenhouse, atm_pressure, hydro,
            GridResolution::Fast, &seed,
        );

        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] {
                prop_assert_eq!(g.layers.river_discharge_m3s[idx], 0.0,
                    "ocean tile {} has nonzero discharge", idx);
            }
        }
    }

    #[test]
    fn precipitation_is_non_negative(
        (input, greenhouse, atm_pressure, hydro, seed_n) in arb_pipeline()
    ) {
        let seed = format!("prop_{}", seed_n);
        let g = generate_surface_grid(
            &input, greenhouse, atm_pressure, hydro,
            GridResolution::Fast, &seed,
        );

        for idx in 0..g.tile_count() {
            let p = g.layers.precipitation_mm[idx];
            prop_assert!(p >= 0.0,
                "negative precipitation {} at tile {}", p, idx);
        }
    }

    #[test]
    fn elevation_is_finite(
        (input, greenhouse, atm_pressure, hydro, seed_n) in arb_pipeline()
    ) {
        let seed = format!("prop_{}", seed_n);
        let g = generate_surface_grid(
            &input, greenhouse, atm_pressure, hydro,
            GridResolution::Fast, &seed,
        );

        for idx in 0..g.tile_count() {
            let e = g.layers.elevation_m[idx];
            prop_assert!(e.is_finite(),
                "non-finite elevation at tile {}", idx);
        }
    }

    #[test]
    fn monthly_precipitation_sums_to_annual(
        (input, greenhouse, atm_pressure, hydro, seed_n) in arb_pipeline()
    ) {
        let seed = format!("prop_{}", seed_n);
        let g = generate_surface_grid(
            &input, greenhouse, atm_pressure, hydro,
            GridResolution::Fast, &seed,
        );

        for idx in 0..g.tile_count() {
            let annual = g.layers.precipitation_mm[idx];
            let monthly_sum: f32 = g.layers.precipitation_monthly_mm[idx].iter().sum();
            if annual > 0.0 {
                prop_assert!(
                    (monthly_sum - annual).abs() < 1.0,
                    "monthly sum {} != annual {} at tile {}", monthly_sum, annual, idx
                );
            }
        }
    }

    #[test]
    fn high_accumulation_implies_positive_discharge(
        (input, greenhouse, atm_pressure, hydro, seed_n) in arb_pipeline()
    ) {
        let seed = format!("prop_{}", seed_n);
        let g = generate_surface_grid(
            &input, greenhouse, atm_pressure, hydro,
            GridResolution::Fast, &seed,
        );

        for idx in 0..g.tile_count() {
            if g.layers.is_ocean[idx] { continue; }
            let acc = g.layers.flow_accumulation[idx];
            let discharge = g.layers.river_discharge_m3s[idx];
            if acc > 50 {
                prop_assert!(discharge > 0.0,
                    "tile {} has accumulation {} but zero discharge", idx, acc);
            }
        }
    }
}
