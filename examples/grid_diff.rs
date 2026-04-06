//! Grid diff tool: compare two procedurally-generated grids.
//!
//! Generates two grids from different seeds (or the same seed to verify
//! determinism) and prints a per-layer diff summary. Useful for checking
//! that refactors don't silently change output.
//!
//! Usage:
//!     cargo run --example grid_diff                    # default seeds
//!     cargo run --example grid_diff -- seed_a seed_b   # custom seeds
//!     cargo run --example grid_diff -- same same       # verify determinism

use cosmos::prelude::*;
use world::diff::diff_grids;
use world::grid::GridResolution;

fn earth_like_body() -> CelestialBody {
    CelestialBody::new(
        Some(Orbit {
            average_distance: 1.0,
            average_distance_from_system_center: 1.0,
            eccentricity: 0.0167,
            axial_tilt: 23.4,
            rotation: 1.0,
            ..Default::default()
        }),
        7,
        "Terra".into(),
        1.0,
        1.0,
        5.5,
        1.0,
        288,
        0,
        CelestialBodySize::Standard,
        CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
            TelluricBodyComposition::Rocky,
            CelestialBodyWorldType::Terrestrial,
            Vec::new(),
            CelestialBodyCoreHeat::ActiveCore,
            MagneticFieldStrength::Strong,
            Vec::new(),
            Vec::new(),
            10.0,
            true,
            65.0,
        )),
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed_a = args.get(1).map(|s| s.as_str()).unwrap_or("alpha");
    let seed_b = args.get(2).map(|s| s.as_str()).unwrap_or("beta");

    println!("Generating grid A (seed={})...", seed_a);
    let body = earth_like_body();
    let a = genesis::generate_world_with_surface(
        &body,
        4.6,
        1,
        false,
        world::prelude::LifeLevel::Sentient,
        GridResolution::Fast,
        seed_a,
    )
    .expect("telluric body");

    println!("Generating grid B (seed={})...", seed_b);
    let b = genesis::generate_world_with_surface(
        &body,
        4.6,
        1,
        false,
        world::prelude::LifeLevel::Sentient,
        GridResolution::Fast,
        seed_b,
    )
    .expect("telluric body");

    println!();
    let report = diff_grids(&a.surface, &b.surface);
    report.print_summary();
}
