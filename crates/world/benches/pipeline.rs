//! Criterion benchmarks for the world-generation pipeline.

use criterion::{criterion_group, criterion_main, Criterion};
use world::climate::{
    generate_biomes, generate_monthly_climate, generate_temperature, generate_wind,
};
use world::geology::generate_geology;
use world::grid::{generate_surface_grid, GridResolution};
use world::hydrology::{generate_hydrology, generate_precipitation};
use world::ocean::generate_ocean_dynamics;
use world::types::{OrbitContext, PlanetSimulationInput, StarContext};

fn earth_input() -> PlanetSimulationInput {
    PlanetSimulationInput {
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
    }
}

fn bench_geology(c: &mut Criterion) {
    let input = earth_input();
    c.bench_function("geology_fast", |b| {
        b.iter(|| generate_geology(&input, 71.0, GridResolution::Fast, "bench"))
    });
}

fn bench_temperature(c: &mut Criterion) {
    let input = earth_input();
    let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "bench");
    c.bench_function("temperature_fast", |b| {
        b.iter(|| generate_temperature(&input, 33.0, &mut g))
    });
}

fn bench_wind(c: &mut Criterion) {
    let input = earth_input();
    let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "bench");
    generate_temperature(&input, 33.0, &mut g);
    c.bench_function("wind_fast", |b| {
        b.iter(|| generate_wind(&input, 1.0, &mut g))
    });
}

fn bench_precipitation(c: &mut Criterion) {
    let input = earth_input();
    let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "bench");
    generate_temperature(&input, 33.0, &mut g);
    generate_wind(&input, 1.0, &mut g);
    c.bench_function("precipitation_fast", |b| {
        b.iter(|| generate_precipitation(&input, 1.0, 71.0, &mut g))
    });
}

fn bench_ocean(c: &mut Criterion) {
    let input = earth_input();
    let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "bench");
    generate_temperature(&input, 33.0, &mut g);
    generate_wind(&input, 1.0, &mut g);
    generate_precipitation(&input, 1.0, 71.0, &mut g);
    c.bench_function("ocean_dynamics_fast", |b| {
        b.iter(|| generate_ocean_dynamics(&mut g))
    });
}

fn bench_hydrology(c: &mut Criterion) {
    let input = earth_input();
    let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "bench");
    generate_temperature(&input, 33.0, &mut g);
    generate_wind(&input, 1.0, &mut g);
    generate_precipitation(&input, 1.0, 71.0, &mut g);
    generate_ocean_dynamics(&mut g);
    c.bench_function("hydrology_fast", |b| {
        b.iter(|| generate_hydrology(1.0, &mut g))
    });
}

fn bench_biomes(c: &mut Criterion) {
    let input = earth_input();
    let mut g = generate_geology(&input, 71.0, GridResolution::Fast, "bench");
    generate_temperature(&input, 33.0, &mut g);
    generate_wind(&input, 1.0, &mut g);
    generate_precipitation(&input, 1.0, 71.0, &mut g);
    generate_ocean_dynamics(&mut g);
    generate_hydrology(1.0, &mut g);
    generate_monthly_climate(&input, &mut g);
    c.bench_function("biomes_fast", |b| b.iter(|| generate_biomes(&mut g)));
}

fn bench_full_pipeline(c: &mut Criterion) {
    let input = earth_input();
    let mut group = c.benchmark_group("full_pipeline");
    group.sample_size(20);

    group.bench_function("fast_72x36", |b| {
        b.iter(|| generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "bench"))
    });

    group.bench_function("standard_144x72", |b| {
        b.iter(|| generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Standard, "bench"))
    });

    group.bench_function("detailed_360x180", |b| {
        b.iter(|| generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Detailed, "bench"))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_geology,
    bench_temperature,
    bench_wind,
    bench_precipitation,
    bench_ocean,
    bench_hydrology,
    bench_biomes,
    bench_full_pipeline,
);
criterion_main!(benches);
