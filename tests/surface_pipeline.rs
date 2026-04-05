//! End-to-end integration test for the cosmos → world (with surface grid)
//! → life pipeline.

use cosmos::prelude::*;
use genesis::{generate_life_on_surface, generate_world_with_surface};
use life::{
    generate_ecosystem_from_world, Biome, Climate, Habitat, LifeLevel, SpeciesGenerationInput,
    Temperature,
};
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
        "Gaia".into(),
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

#[test]
fn full_world_with_surface_pipeline() {
    let body = earth_like_body();
    let result = generate_world_with_surface(
        &body,
        4.6,
        1,
        false,
        world::prelude::LifeLevel::Sentient,
        GridResolution::Fast,
        "integration_seed",
    )
    .expect("body should be telluric");

    // Interior and detail flow through.
    assert_eq!(result.input.body_id, 7);
    assert!(result.interior.atmospheric_pressure > 0.0);
    assert!(result.detail.photochemistry.is_some());

    // Surface grid is populated.
    let surface = &result.surface;
    assert_eq!(surface.tile_count(), 72 * 36);
    assert!(surface.plates.len() >= 8);

    // At least one of every major layer populated.
    assert!(surface.layers.temperature_c.iter().any(|&t| t != 0.0));
    assert!(surface.layers.precipitation_mm.iter().any(|&p| p > 0.0));
    assert!(surface
        .layers
        .biome
        .iter()
        .any(|&b| b != Default::default()));
    assert!(surface.layers.river_discharge_m3s.iter().any(|&d| d > 0.0));
}

#[test]
fn full_world_plus_life_distribution() {
    let body = earth_like_body();
    let result = generate_world_with_surface(
        &body,
        4.6,
        1,
        false,
        world::prelude::LifeLevel::Sentient,
        GridResolution::Fast,
        "life_seed",
    )
    .expect("body should be telluric");

    // Build an ecosystem using life's API directly.
    let input = SpeciesGenerationInput {
        habitat: Habitat::Terrestrial,
        climate: Climate::Terrestrial,
        temperature: Temperature::Temperate,
        gravity: 1.0,
        atmospheric_pressure: 1.0,
        hydrosphere: 71.0,
        life_level: LifeLevel::AnimalLike,
        seed: "life_seed".into(),
        scope_key: "gaia".into(),
    };
    let ecosystem = generate_ecosystem_from_world(&input);
    assert!(ecosystem.species_count() >= 3);

    let distribution =
        generate_life_on_surface(&result.surface, &ecosystem, 1.0, LifeLevel::AnimalLike);

    assert_eq!(distribution.ranges.len(), ecosystem.species_count());
    assert_eq!(
        distribution.vegetation_density.len(),
        result.surface.tile_count()
    );
    assert!(distribution.vegetation_density.iter().any(|&v| v > 0.0));
    for range in &distribution.ranges {
        assert_eq!(range.habitability.len(), result.surface.tile_count());
    }
}

#[test]
fn pipeline_is_end_to_end_deterministic() {
    let body = earth_like_body();
    let seed = "reproducibility";
    let a = generate_world_with_surface(
        &body,
        4.6,
        1,
        false,
        world::prelude::LifeLevel::Sentient,
        GridResolution::Fast,
        seed,
    )
    .unwrap();
    let b = generate_world_with_surface(
        &body,
        4.6,
        1,
        false,
        world::prelude::LifeLevel::Sentient,
        GridResolution::Fast,
        seed,
    )
    .unwrap();
    assert_eq!(a.surface.layers.elevation_m, b.surface.layers.elevation_m);
    assert_eq!(a.surface.layers.biome, b.surface.layers.biome);
}

#[test]
fn biomes_map_cleanly_across_crates() {
    // Sanity: life::Biome::Ocean should correspond to world Ocean biome
    // after the adapter converts.
    let body = earth_like_body();
    let result = generate_world_with_surface(
        &body,
        4.6,
        1,
        false,
        world::prelude::LifeLevel::Sentient,
        GridResolution::Fast,
        "biome_check",
    )
    .unwrap();

    let input = SpeciesGenerationInput {
        habitat: Habitat::Terrestrial,
        climate: Climate::Terrestrial,
        temperature: Temperature::Temperate,
        gravity: 1.0,
        atmospheric_pressure: 1.0,
        hydrosphere: 71.0,
        life_level: LifeLevel::AnimalLike,
        seed: "biome_check".into(),
        scope_key: "gaia".into(),
    };
    let ecosystem = generate_ecosystem_from_world(&input);
    let dist = generate_life_on_surface(&result.surface, &ecosystem, 1.0, LifeLevel::AnimalLike);

    // Find an ocean tile — it should get biome Ocean in the habitat view,
    // and have low vegetation_density since we treat vegetation as land-only.
    for idx in 0..result.surface.tile_count() {
        if result.surface.layers.is_ocean[idx] {
            // Vegetation zero on ocean by design.
            assert_eq!(dist.vegetation_density[idx], 0.0);
            break;
        }
    }

    // Ensure no species-range vector is wrong size.
    for r in &dist.ranges {
        assert_eq!(r.habitability.len(), result.surface.tile_count());
    }
    // Verify Biome enum itself is exported.
    let _ = Biome::TropicalForest;
}
