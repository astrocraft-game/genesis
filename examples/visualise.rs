//! Visualisation CLI: ASCII maps, plate DOT graph, food web, trade routes.
//!
//! Run with:
//!     cargo run --example visualise
//!     cargo run --example visualise -- biome
//!     cargo run --example visualise -- elevation
//!     cargo run --example visualise -- temperature
//!     cargo run --example visualise -- plates
//!     cargo run --example visualise -- food_web
//!     cargo run --example visualise -- trade_routes
//!     cargo run --example visualise -- all

use cosmos::prelude::*;
use world::grid::GridResolution;
use world::types::BiomeType;

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

// ---------------------------------------------------------------------------
// ASCII map rendering
// ---------------------------------------------------------------------------

fn biome_char(b: BiomeType) -> char {
    match b {
        BiomeType::Ocean => '~',
        BiomeType::IceCap => '#',
        BiomeType::Tundra => 'T',
        BiomeType::Taiga => 't',
        BiomeType::TemperateForest => 'F',
        BiomeType::TropicalForest => 'R',
        BiomeType::Grassland => 'g',
        BiomeType::Steppe => 's',
        BiomeType::Desert => 'D',
        BiomeType::ColdDesert => 'd',
        BiomeType::Savanna => 'S',
        BiomeType::Wetland => 'W',
        BiomeType::Mangrove => 'M',
        BiomeType::MediterraneanShrubland => 'm',
        BiomeType::XericShrubland => 'x',
        BiomeType::Chaparral => 'c',
        BiomeType::Alpine => 'A',
        BiomeType::Volcanic => 'V',
        BiomeType::Barren => '.',
        _ => '?',
    }
}

fn print_ascii_biome(grid: &world::grid::SurfaceGrid) {
    println!("=== Biome Map ({}x{}) ===", grid.width, grid.height);
    println!("Legend: ~=Ocean #=Ice T=Tundra t=Taiga F=TempForest R=TropForest");
    println!("       g=Grass s=Steppe D=Desert d=ColdDesert S=Savanna W=Wetland");
    println!("       M=Mangrove m=Medit x=Xeric c=Chaparral A=Alpine V=Volcanic");
    println!();
    for r in 0..grid.height {
        for c in 0..grid.width {
            let idx = grid.idx(c, r);
            print!("{}", biome_char(grid.layers.biome[idx]));
        }
        println!();
    }
    println!();
}

fn print_ascii_elevation(grid: &world::grid::SurfaceGrid) {
    println!("=== Elevation Map ===");
    println!("Legend: ' '=deep .=mid ,=shallow _=low -=mid n=hill A=mountain ^=peak");
    println!();
    for r in 0..grid.height {
        for c in 0..grid.width {
            let idx = grid.idx(c, r);
            let elev = grid.layers.elevation_m[idx];
            let sl = grid.sea_level_m;
            let ch = if elev < sl {
                let d = sl - elev;
                if d > 3000.0 {
                    ' '
                } else if d > 1500.0 {
                    '.'
                } else {
                    ','
                }
            } else {
                let h = elev - sl;
                if h > 4000.0 {
                    '^'
                } else if h > 2500.0 {
                    'A'
                } else if h > 1000.0 {
                    'n'
                } else if h > 300.0 {
                    '-'
                } else {
                    '_'
                }
            };
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn print_ascii_temperature(grid: &world::grid::SurfaceGrid) {
    println!("=== Temperature Map ===");
    println!("Legend: *=<-20 -=cold .=cool o=mild +=warm #=hot");
    println!();
    for r in 0..grid.height {
        for c in 0..grid.width {
            let idx = grid.idx(c, r);
            let t = grid.layers.temperature_c[idx];
            let ch = if t < -20.0 {
                '*'
            } else if t < -5.0 {
                '-'
            } else if t < 5.0 {
                '.'
            } else if t < 15.0 {
                'o'
            } else if t < 25.0 {
                '+'
            } else {
                '#'
            };
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

// ---------------------------------------------------------------------------
// Plate DOT export
// ---------------------------------------------------------------------------

fn print_plate_dot(grid: &world::grid::SurfaceGrid) {
    println!("=== Tectonic Plates (DOT) ===");
    println!("// Paste into https://dreampuf.github.io/GraphvizOnline/");
    println!("graph plates {{");
    println!("  layout=neato;");
    for plate in &grid.plates {
        let kind = match plate.kind {
            world::grid::PlateKind::Continental => "Continental",
            world::grid::PlateKind::Oceanic => "Oceanic",
        };
        println!(
            "  plate_{} [label=\"Plate {} ({})\\nage={:.0} Myr\"];",
            plate.id, plate.id, kind, plate.age_myr
        );
    }
    let mut seen = std::collections::HashSet::new();
    let w = grid.width as usize;
    for r in 0..grid.height as usize {
        for c in 0..w {
            let idx = r * w + c;
            let pid = grid.layers.plate_id[idx];
            let rc = (c + 1) % w;
            let ridx = r * w + rc;
            let rpid = grid.layers.plate_id[ridx];
            if pid != rpid {
                let key = if pid < rpid { (pid, rpid) } else { (rpid, pid) };
                if seen.insert(key) {
                    let bk = grid.layers.tectonic_boundary[idx];
                    let label = match bk {
                        world::grid::BoundaryKind::Convergent => "convergent",
                        world::grid::BoundaryKind::Divergent => "divergent",
                        world::grid::BoundaryKind::Transform => "transform",
                        _ => "",
                    };
                    println!(
                        "  plate_{} -- plate_{} [label=\"{}\"];",
                        key.0, key.1, label
                    );
                }
            }
        }
    }
    println!("}}");
    println!();
}

// ---------------------------------------------------------------------------
// Food web
// ---------------------------------------------------------------------------

fn print_food_web(eco: &life::Ecosystem) {
    println!("=== Food Web ===");
    let species: Vec<_> = eco.all_species().collect();
    for (i, sp) in species.iter().enumerate() {
        println!("  [{}] {} ({})", i, sp.name, sp.trophic_level);
    }
    println!();
    if eco.predator_prey_links.is_empty() {
        println!("  (no predator-prey links)");
    } else {
        for &(pred, prey) in &eco.predator_prey_links {
            println!(
                "  {} --> {}",
                species.get(pred).map_or("?", |s| &s.name),
                species.get(prey).map_or("?", |s| &s.name),
            );
        }
    }
    if !eco.keystone_species.is_empty() {
        print!("  Keystone:");
        for &k in &eco.keystone_species {
            print!(" {}", species.get(k).map_or("?", |s| &s.name));
        }
        println!();
    }
    println!("  Trophic pyramid valid: {}", eco.trophic_pyramid_valid);
    println!();
}

// ---------------------------------------------------------------------------
// Trade routes
// ---------------------------------------------------------------------------

fn print_trade_routes(settlements: &[life::Settlement], routes: &[genesis::adapters::TradeRoute]) {
    println!("=== Settlements ({}) ===", settlements.len());
    for (i, s) in settlements.iter().enumerate() {
        println!(
            "  [{}] tile={} suitability={:.2} pop=1e{}",
            i, s.tile_idx, s.suitability, s.population_order
        );
    }
    println!();
    println!("=== Trade Routes ({}) ===", routes.len());
    for r in routes {
        println!(
            "  {} -> {} | cost={:.1} value={:.2} hops={}",
            r.from_settlement,
            r.to_settlement,
            r.total_cost,
            r.value,
            r.tiles.len()
        );
    }
    println!();
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("all");

    let body = earth_like_body();
    let result = genesis::generate_world_with_surface(
        &body,
        4.6,
        1,
        false,
        world::prelude::LifeLevel::Sentient,
        GridResolution::Fast,
        "vis",
    )
    .expect("Earth-like body should be telluric");

    let grid = &result.surface;
    let show_all = mode == "all";

    if show_all || mode == "biome" {
        print_ascii_biome(grid);
    }
    if show_all || mode == "elevation" {
        print_ascii_elevation(grid);
    }
    if show_all || mode == "temperature" {
        print_ascii_temperature(grid);
    }
    if show_all || mode == "plates" {
        print_plate_dot(grid);
    }

    if show_all || mode == "food_web" {
        let eco_input = life::SpeciesGenerationInput {
            habitat: life::types::Habitat::Terrestrial,
            climate: life::types::Climate::Terrestrial,
            temperature: life::types::Temperature::Temperate,
            gravity: 1.0,
            atmospheric_pressure: 1.0,
            hydrosphere: 71.0,
            life_level: life::types::LifeLevel::Sentient,
            seed: "vis".into(),
            scope_key: "vis_eco".into(),
        };
        let eco = life::generate_ecosystem_from_world(&eco_input);
        print_food_web(&eco);
    }

    if show_all || mode == "trade_routes" {
        use std::rc::Rc;
        let rm = world::resources::generate_resources(grid);
        let hg = genesis::adapters::surface_grid_to_habitat_grid(grid);
        let water = genesis::adapters::water_access_from_grid(grid);
        let density = genesis::adapters::resource_density_from_map(&rm);
        // Use uniform habitability = 1.0 for all land tiles as a simple proxy.
        let hab: Vec<f32> = hg
            .is_ocean
            .iter()
            .map(|&ocean| if ocean { 0.0 } else { 1.0 })
            .collect();
        let suitability = life::compute_settlement_suitability(&hg, &hab, &water, &density);
        let species_name: Rc<str> = Rc::from("Terrans");
        let settlements = life::place_settlements(&suitability, &hg, species_name, 8, 5);
        let routes = genesis::adapters::compute_trade_routes(grid, &rm, &settlements, 2);
        print_trade_routes(&settlements, &routes);
    }

    if !show_all {
        println!("Modes: biome | elevation | temperature | plates | food_web | trade_routes | all");
    }
}
