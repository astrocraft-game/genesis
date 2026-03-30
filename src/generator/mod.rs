use crate::internal::*;
use crate::prelude::*;
pub mod types;

/// The generator itself, depending on the given settings, can generate a full blown universe with multiple galaxies, sectors, systems,
/// planets and the species living in those.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Generator {}

impl Generator {
    /// Generates a full blown universe with multiple galaxies, sectors, systems, planets and the species living in those following the
    /// given [GenerationSettings], in a deterministic way thanks to the given **seed**.
    pub fn generate(settings: GenerationSettings) -> GeneratedUniverse {
        let universe = Universe::generate(&settings);
        let galactic_neighborhood = GalacticNeighborhood::generate(universe, &settings);
        let galaxies: Vec<Galaxy> = generate_galaxies(galactic_neighborhood, settings);

        GeneratedUniverse {
            universe,
            galactic_neighborhood,
            galaxies,
        }
    }
}

/// Generates species for all habitable worlds in a star system.
/// Call this after system generation when `settings.populate` is true.
pub fn populate_system(system: &StarSystem, seed: &str) -> Vec<(u32, crate::life::species::Species, Vec<crate::life::history::HistoricalEvent>)> {
    let mut results = Vec::new();
    for obj in &system.all_objects {
        if let AstronomicalObject::TelluricBody(body) = &obj.object {
            if let CelestialBodyDetails::Telluric(details) = &body.details {
                // Check life level from climate and world conditions
                let life_level = if details.hydrosphere > 10.0
                    && details.atmospheric_pressure > 0.1
                    && matches!(details.magnetic_field, MagneticFieldStrength::Moderate | MagneticFieldStrength::Strong | MagneticFieldStrength::VeryStrong | MagneticFieldStrength::Extreme)
                    && matches!(details.temperature_category, WorldTemperatureCategory::Cool | WorldTemperatureCategory::Temperate | WorldTemperatureCategory::Warm | WorldTemperatureCategory::Chilly)
                {
                    LifeLevel::Sentient
                } else if details.hydrosphere > 0.0 && details.atmospheric_pressure > 0.01 {
                    LifeLevel::AnimalLike
                } else {
                    continue;
                };

                if life_level.as_u8() < LifeLevel::AnimalLike.as_u8() {
                    continue;
                }

                if let Some(species) = crate::life::generator::generate_species_from_world(
                    details.world_type,
                    details.climate,
                    details.temperature_category,
                    body.gravity,
                    details.atmospheric_pressure,
                    details.hydrosphere,
                    life_level,
                    seed,
                    SpaceCoordinates::new(0, 0, 0),
                    0,
                    0,
                    obj.id,
                ) {
                    let history = if let Some(tl) = species.tech_level {
                        crate::life::history::generate_species_history(
                            tl, species.lifespan_years, seed, &species.name,
                        )
                    } else {
                        Vec::new()
                    };
                    results.push((obj.id, species, history));
                }
            }
        }
    }
    results
}

/// Generates a list of [Galaxy] in the given **galactic_neighborhood** using the given **seed** and **settings**.
fn generate_galaxies(
    galactic_neighborhood: GalacticNeighborhood,
    settings: GenerationSettings,
) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = vec![];
    let to_generate: u16;
    match galactic_neighborhood.density {
        GalacticNeighborhoodDensity::Void(g, m) | GalacticNeighborhoodDensity::Group(g, m) => {
            to_generate = (g as u16) + m
        }
        GalacticNeighborhoodDensity::Cluster(d, g, m) => to_generate = (d as u16) + (g as u16) + m,
    }
    for i in 0..to_generate {
        galaxies.push(Galaxy::generate(galactic_neighborhood, i, &settings));
    }
    galaxies
}
