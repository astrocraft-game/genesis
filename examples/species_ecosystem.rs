//! Generate an ecosystem for an Earth-like world and print the food web.
//!
//! Run with:
//!     cargo run --example species_ecosystem

use life::{
    generate_ecosystem_from_world, Climate, Habitat, LifeLevel, SpeciesGenerationInput, Temperature,
};

fn main() {
    let seed = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "ecosystem_demo".into());

    let input = SpeciesGenerationInput {
        habitat: Habitat::Terrestrial,
        climate: Climate::Terrestrial,
        temperature: Temperature::Temperate,
        gravity: 1.0,
        atmospheric_pressure: 1.0,
        hydrosphere: 71.0,
        life_level: LifeLevel::AnimalLike,
        seed: seed.clone(),
        scope_key: "homeworld".into(),
    };

    let ecosystem = generate_ecosystem_from_world(&input);

    println!("Seed: {}", seed);
    println!("Species count: {}", ecosystem.species_count());
    println!();

    if let Some(producer) = &ecosystem.producer {
        println!("Producer (autotroph):");
        print_species(producer);
    }

    if !ecosystem.herbivores.is_empty() {
        println!();
        println!("Primary consumers (herbivores/omnivores):");
        for s in &ecosystem.herbivores {
            print_species(s);
        }
    }

    if !ecosystem.predators.is_empty() {
        println!();
        println!("Secondary consumers (carnivores):");
        for s in &ecosystem.predators {
            print_species(s);
        }
    }

    if !ecosystem.detritivores.is_empty() {
        println!();
        println!("Filter-feeders / detritivores:");
        for s in &ecosystem.detritivores {
            print_species(s);
        }
    }
}

fn print_species(s: &life::Species) {
    println!(
        "  {:15} — {:?} {:?} {:?}, size {:?}, {} yr lifespan",
        &*s.name, s.biochemistry, s.body_plan, s.trophic_level, s.size_class, s.lifespan_years
    );
    if !s.special_traits.is_empty() {
        println!("                    traits: {:?}", s.special_traits);
    }
}
