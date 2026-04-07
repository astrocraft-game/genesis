//! Generate an Earth-like world and write the surface layers as PPM images
//! to the current directory. PPM is a plain-text-header binary format that
//! every image viewer reads, with no external crate dependency.
//!
//! Run with:
//!     cargo run --example export_maps
//!
//! Output files: biome.ppm, elevation.pgm, temperature.ppm, precipitation.ppm

use genesis::prelude::*;
use genesis::generate_world_with_surface;
use std::fs::File;
use std::io::Write;
use atlasis::world::grid::{GridResolution, SurfaceGrid};

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

fn write_ppm_rgb(path: &str, width: u16, height: u16, rgb: &[u8]) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    write!(f, "P6\n{} {}\n255\n", width, height)?;
    f.write_all(rgb)?;
    Ok(())
}

fn write_pgm_grayscale(path: &str, width: u16, height: u16, gray: &[u8]) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    write!(f, "P5\n{} {}\n255\n", width, height)?;
    f.write_all(gray)?;
    Ok(())
}

fn export_grid(grid: &SurfaceGrid) -> std::io::Result<()> {
    let (w, h) = grid.dimensions();
    write_ppm_rgb("biome.ppm", w, h, &grid.export_biome_rgb())?;
    write_pgm_grayscale("elevation.pgm", w, h, &grid.export_elevation_grayscale())?;
    write_ppm_rgb("temperature.ppm", w, h, &grid.export_temperature_rgb())?;
    write_ppm_rgb("precipitation.ppm", w, h, &grid.export_precipitation_rgb())?;
    write_pgm_grayscale("ocean_mask.pgm", w, h, &grid.export_ocean_mask())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let seed = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "export_demo".into());
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
    .expect("telluric body");

    export_grid(&result.surface)?;

    println!(
        "Exported 5 map layers ({}×{}):",
        result.surface.width, result.surface.height
    );
    println!("  biome.ppm         — biome palette");
    println!("  elevation.pgm     — grayscale heightmap");
    println!("  temperature.ppm   — blue→red colormap");
    println!("  precipitation.ppm — tan→blue colormap");
    println!("  ocean_mask.pgm    — binary land/ocean mask");
    println!();
    println!("View with any image viewer (feh, eog, GIMP, etc.)");
    Ok(())
}
