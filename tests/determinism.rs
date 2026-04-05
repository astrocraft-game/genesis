//! Determinism regression test.
//!
//! Pins a small set of observable properties from a fixed seed so that
//! refactors that silently alter generator output (reordered RNG calls,
//! changed dice roll sequences, etc.) are caught before merge.
//!
//! When generator behaviour changes *intentionally*, update the expected
//! values below after manually verifying the new output is correct.

use cosmos::galaxy::map::division_level::GalacticMapDivisionLevel;
use cosmos::prelude::*;
use genesis::generate_system;

const SEED: &str = "determinism_snapshot_v1";
const COORD: (i64, i64, i64) = (3, 7, 0);

fn make_galaxy() -> Galaxy {
    let mut galaxy = Galaxy::default();
    galaxy.settings.seed = SEED.into();
    galaxy.division_levels = GalacticMapDivisionLevel::generate_division_levels(&galaxy.settings);
    galaxy
}

#[test]
fn fixed_seed_produces_stable_system() {
    let (x, y, z) = COORD;
    let coord = SpaceCoordinates::new(x, y, z);
    let hex = GalacticHex::default();
    let div = GalacticMapDivision::default();
    let mut galaxy = make_galaxy();

    let system = generate_system(0, coord, &hex, &div, &mut galaxy);

    // These values were captured on the initial run and pinned here.
    // Update only after verifying the new output is intentional.
    assert!(
        !system.all_objects.is_empty(),
        "system should have at least one object"
    );
    assert!(system.name.len() >= 3, "system name should be generated");

    // Count how many stars and bodies of each type the system contains —
    // these totals are structural fingerprints of the generator pipeline.
    let (stars, telluric, gaseous, icy) = classify(&system);
    assert!(stars >= 1, "must have at least one star, got {stars}");
    // The fixed seed must produce a stable count. If you change the seed
    // or the generator, update these numbers.
    let counts = (stars, telluric, gaseous, icy);
    assert_eq!(
        counts,
        snapshot_counts(),
        "generation drifted — if intentional, update SNAPSHOT to {counts:?}"
    );
}

/// Prints current counts when run with `cargo test fixed_seed_print_snapshot
/// -- --nocapture --ignored`. Useful for regenerating SNAPSHOT after an
/// intentional generator change.
#[test]
#[ignore]
fn fixed_seed_print_snapshot() {
    let (x, y, z) = COORD;
    let coord = SpaceCoordinates::new(x, y, z);
    let hex = GalacticHex::default();
    let div = GalacticMapDivision::default();
    let mut galaxy = make_galaxy();
    let system = generate_system(0, coord, &hex, &div, &mut galaxy);
    let counts = classify(&system);
    println!("SNAPSHOT = {counts:?};");
    println!("name = {}", system.name);
    println!("objects = {}", system.all_objects.len());
}

#[test]
fn fixed_seed_is_reproducible_across_runs() {
    let (x, y, z) = COORD;
    let coord = SpaceCoordinates::new(x, y, z);
    let hex = GalacticHex::default();
    let div = GalacticMapDivision::default();

    let mut g1 = make_galaxy();
    let s1 = generate_system(0, coord, &hex, &div, &mut g1);

    let mut g2 = make_galaxy();
    let s2 = generate_system(0, coord, &hex, &div, &mut g2);

    assert_eq!(
        s1.all_objects.len(),
        s2.all_objects.len(),
        "object counts must match"
    );
    assert_eq!(s1.name, s2.name, "names must match");
    assert_eq!(classify(&s1), classify(&s2), "body-type counts must match");
}

fn classify(system: &StarSystem) -> (u32, u32, u32, u32) {
    let mut stars = 0;
    let mut telluric = 0;
    let mut gaseous = 0;
    let mut icy = 0;
    for op in &system.all_objects {
        match &op.object {
            AstronomicalObject::Star(_) => stars += 1,
            AstronomicalObject::TelluricBody(_) => telluric += 1,
            AstronomicalObject::GaseousBody(_) => gaseous += 1,
            AstronomicalObject::IcyBody(_) => icy += 1,
            _ => {}
        }
    }
    (stars, telluric, gaseous, icy)
}

/// Snapshot of body-type counts for the fixed seed at the time this test
/// was written. Update **only** after confirming a change to the generator
/// is intentional and the new output is correct.
fn snapshot_counts() -> (u32, u32, u32, u32) {
    // (stars, telluric, gaseous, icy)
    SNAPSHOT
}

// Captured from `cargo test --test determinism fixed_seed_print_snapshot
// -- --nocapture --ignored`. The default GalacticHex/GalacticMapDivision
// produces a minimal system; this is sufficient as a drift sentinel.
const SNAPSHOT: (u32, u32, u32, u32) = (1, 0, 0, 0);

#[test]
fn fixed_seed_produces_stable_star_properties() {
    let (x, y, z) = COORD;
    let coord = SpaceCoordinates::new(x, y, z);
    let hex = GalacticHex::default();
    let div = GalacticMapDivision::default();
    let mut galaxy = make_galaxy();

    let system = generate_system(0, coord, &hex, &div, &mut galaxy);

    // Pin the main star's name as a deterministic fingerprint.
    assert_eq!(
        &*system.name, EXPECTED_SYSTEM_NAME,
        "system name drifted — if intentional, update EXPECTED_SYSTEM_NAME"
    );
    assert_eq!(
        system.all_objects.len(),
        EXPECTED_OBJECT_COUNT,
        "object count drifted"
    );
}

const EXPECTED_SYSTEM_NAME: &str = "Mcclucas";
const EXPECTED_OBJECT_COUNT: usize = 1;
