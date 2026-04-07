//! Generate a single terrestrial world (cosmos body → world interior + detail
//! + full surface grid) and print a summary of its physical state.
//!
//! Run with:
//!     cargo run --example single_planet

use genesis::prelude::*;
use genesis::generate_world_with_surface;
use std::collections::HashMap;
use atlasis::world::grid::GridResolution;
use atlasis::world::types::BiomeType;

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

fn main() {
    let seed = std::env::args().nth(1).unwrap_or_else(|| "gaia_001".into());
    let body = earth_like_body();
    let result = generate_world_with_surface(
        &body,
        4.6,
        1,
        false,
        atlasis::world::types::LifeLevel::Sentient,
        GridResolution::Standard,
        &seed,
    )
    .expect("Earth-like body should be telluric");

    let surface = &result.surface;
    let land_count = surface.layers.is_ocean.iter().filter(|&&o| !o).count();
    let ocean_count = surface.tile_count() - land_count;
    let land_pct = 100.0 * land_count as f32 / surface.tile_count() as f32;

    let mean_temp = surface.layers.temperature_c.iter().sum::<f32>() / surface.tile_count() as f32;
    let max_elev = surface
        .layers
        .elevation_m
        .iter()
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max);
    let min_elev = surface
        .layers
        .elevation_m
        .iter()
        .cloned()
        .fold(f32::INFINITY, f32::min);
    let total_precip =
        surface.layers.precipitation_mm.iter().sum::<f32>() / surface.tile_count() as f32;
    let max_discharge = surface
        .layers
        .river_discharge_m3s
        .iter()
        .cloned()
        .fold(0.0f32, f32::max);

    // Biome distribution.
    let mut biome_counts: HashMap<BiomeType, usize> = HashMap::new();
    for &b in &surface.layers.biome {
        *biome_counts.entry(b).or_insert(0) += 1;
    }
    let mut biomes: Vec<_> = biome_counts.iter().collect();
    biomes.sort_by_key(|(_, &c)| std::cmp::Reverse(c));

    println!("Seed: {}", seed);
    println!(
        "Grid: {}×{} ({} tiles)",
        surface.width,
        surface.height,
        surface.tile_count()
    );
    println!(
        "Plates: {} (sea level: {:.0} m)",
        surface.plates.len(),
        surface.sea_level_m
    );
    println!("Land/Ocean: {:.1}% / {:.1}%", land_pct, 100.0 - land_pct);
    println!("Elevation: {:.0} m to {:.0} m", min_elev, max_elev);
    println!("Mean annual temperature: {:.1}°C", mean_temp);
    println!("Mean annual precipitation: {:.0} mm", total_precip);
    println!("Largest river discharge: {:.0} m³/s", max_discharge);
    println!();
    println!("Biome distribution:");
    for (biome, count) in biomes.iter().take(10) {
        let pct = 100.0 * **count as f32 / surface.tile_count() as f32;
        println!("  {:5.1}%  {:?}", pct, biome);
    }
    let _ = (ocean_count, min_elev, max_elev, total_precip);
}
