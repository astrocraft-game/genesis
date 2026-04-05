//! Life distribution on a pre-computed habitat grid.
//!
//! `HabitatGrid` is life's read-only view of a world's surface climate —
//! the fields needed to decide *where* species can live. It contains no
//! physics; it just mirrors the per-tile climate facts that world computes.
//! The application layer (adapters) fills it from `world::SurfaceGrid`.
//!
//! From a `HabitatGrid`, life produces:
//!   - `vegetation_density` per tile (0.0–1.0) — how much plant biomass the
//!     climate supports, scaled by the world's life level.
//!   - `SpeciesRange` per species — tile-level habitability + territory +
//!     population density based on the species' preferred ranges, body plan,
//!     and locomotion.

use crate::ecosystem::Ecosystem;
use crate::species::{BodyPlan, LocomotionType, Species, TrophicLevel};
use crate::types::{Biome, LifeLevel};
use std::rc::Rc;

/// Read-only climate facts per tile. Life treats this as an opaque input
/// (no physics here); it's filled by a root adapter from `world::SurfaceGrid`.
#[derive(Clone, Debug, Default)]
pub struct HabitatGrid {
    pub width: u16,
    pub height: u16,
    pub temperature_c: Vec<f32>,
    pub humidity_relative: Vec<f32>,
    pub biome: Vec<Biome>,
    pub is_ocean: Vec<bool>,
    pub elevation_m: Vec<f32>,
}

impl HabitatGrid {
    pub fn tile_count(&self) -> usize {
        self.width as usize * self.height as usize
    }
}

/// A species' distribution over the grid: per-tile habitability, occupied
/// territory, and population density.
#[derive(Clone, Debug)]
pub struct SpeciesRange {
    pub species_name: Rc<str>,
    pub habitability: Vec<f32>,
    pub territory: Vec<bool>,
    pub population_density: Vec<f32>,
}

/// Biological occupancy of a habitat: per-tile vegetation and the ranges
/// of every species in the ecosystem.
#[derive(Clone, Debug, Default)]
pub struct LifeDistribution {
    pub vegetation_density: Vec<f32>,
    pub primary_productivity: Vec<f32>,
    pub ranges: Vec<SpeciesRange>,
}

/// Compute per-tile primary productivity and vegetation density from
/// climate. Productivity is a Whittaker-style min(thermal, moisture)
/// factor; vegetation density scales productivity by the world's life
/// level and biome suitability.
pub fn generate_vegetation(habitat: &HabitatGrid, life_level: LifeLevel) -> (Vec<f32>, Vec<f32>) {
    let n = habitat.tile_count();
    let mut productivity = vec![0.0f32; n];
    let mut density = vec![0.0f32; n];

    // Life below PlantLike has no meaningful vegetation.
    let life_factor = match life_level {
        LifeLevel::None | LifeLevel::UniCellular => 0.0,
        LifeLevel::PluriCellular => 0.2,
        LifeLevel::PlantLike => 0.7,
        LifeLevel::AnimalLike | LifeLevel::Sentient => 1.0,
    };

    for idx in 0..n {
        if habitat.is_ocean[idx] {
            // Oceans have primary productivity (phytoplankton) but we treat
            // vegetation as a land metric for this coarse model.
            let temp = habitat.temperature_c[idx];
            productivity[idx] = thermal_factor(temp) * 0.5;
            continue;
        }
        let temp = habitat.temperature_c[idx];
        let humidity = habitat.humidity_relative[idx];
        let thermal = thermal_factor(temp);
        let moisture = humidity.clamp(0.0, 1.0);
        let prod = thermal.min(moisture);
        productivity[idx] = prod;
        let biome_mod = biome_vegetation_modifier(habitat.biome[idx]);
        density[idx] = (prod * biome_mod * life_factor).clamp(0.0, 1.0);
    }
    (productivity, density)
}

/// Thermal productivity factor: peaks around 25 °C, falls off toward
/// freezing and extreme heat. Matches the envelope of the Whittaker model.
fn thermal_factor(temp_c: f32) -> f32 {
    if !(-10.0..=45.0).contains(&temp_c) {
        return 0.0;
    }
    // Gaussian-ish curve centred on 25 °C, width ~20 °C.
    let x = (temp_c - 25.0) / 20.0;
    (-x * x).exp().clamp(0.0, 1.0)
}

/// Per-biome ceiling on vegetation density. Deserts and ice caps cap low
/// even with ample thermal + moisture headroom.
fn biome_vegetation_modifier(biome: Biome) -> f32 {
    match biome {
        Biome::TropicalForest | Biome::TemperateForest | Biome::Mangrove => 1.0,
        Biome::Taiga | Biome::Savanna | Biome::Wetland => 0.85,
        Biome::MediterraneanShrubland | Biome::Chaparral => 0.7,
        Biome::Grassland | Biome::Steppe => 0.65,
        Biome::XericShrubland => 0.4,
        Biome::Tundra | Biome::Alpine => 0.3,
        Biome::Desert | Biome::ColdDesert => 0.1,
        Biome::Volcanic | Biome::Barren => 0.05,
        Biome::IceCap => 0.0,
        Biome::Ocean => 0.0,
    }
}

/// Compute a species' per-tile habitability, territory, and population
/// density on a habitat grid.
///
/// `gravity_g` is the planet's surface gravity (Earth g). The species'
/// temperature range is compared against tile `temperature_c + 273.15`.
pub fn compute_species_range(
    species: &Species,
    gravity_g: f32,
    habitat: &HabitatGrid,
    vegetation_density: &[f32],
) -> SpeciesRange {
    let n = habitat.tile_count();
    let mut habitability = vec![0.0f32; n];

    let grav_fit = gravity_fit(species, gravity_g);

    for (idx, slot) in habitability.iter_mut().enumerate() {
        let temp_k = habitat.temperature_c[idx] + 273.15;
        let temp_fit = temperature_fit(species, temp_k);
        let hydro_fit = hydrosphere_fit(species, habitat.is_ocean[idx]);
        let biome_fit = biome_affinity(species, habitat.biome[idx]);
        *slot = (temp_fit * hydro_fit * biome_fit * grav_fit).clamp(0.0, 1.0);
    }

    // Territory: threshold top tiles — keep tiles above 0.5 habitability.
    let territory: Vec<bool> = habitability.iter().map(|&h| h > 0.5).collect();

    // Population density: habitability × (vegetation for heterotrophs,
    // productivity for autotrophs — autotrophs create their own density).
    let is_autotroph = species.trophic_level == TrophicLevel::Autotroph;
    let population_density: Vec<f32> = habitability
        .iter()
        .zip(vegetation_density.iter())
        .map(|(&h, &v)| {
            let base = if is_autotroph { h } else { h * v };
            base.clamp(0.0, 1.0)
        })
        .collect();

    SpeciesRange {
        species_name: species.name.clone(),
        habitability,
        territory,
        population_density,
    }
}

/// Distribute an entire ecosystem onto a habitat grid. Returns vegetation
/// density + per-species ranges.
pub fn distribute_ecosystem(
    ecosystem: &Ecosystem,
    gravity_g: f32,
    habitat: &HabitatGrid,
    life_level: LifeLevel,
) -> LifeDistribution {
    let (productivity, vegetation_density) = generate_vegetation(habitat, life_level);
    let mut ranges = Vec::new();
    for species in ecosystem.all_species() {
        ranges.push(compute_species_range(
            species,
            gravity_g,
            habitat,
            &vegetation_density,
        ));
    }
    LifeDistribution {
        vegetation_density,
        primary_productivity: productivity,
        ranges,
    }
}

// ---------------------------------------------------------------------------
// Fit functions: how well a species matches a tile
// ---------------------------------------------------------------------------

/// Gaussian curve centred on the species' preferred temperature range.
fn temperature_fit(species: &Species, temp_k: f32) -> f32 {
    let (low, high) = species.preferred_temp_range;
    let centre = (low + high) * 0.5;
    let width = ((high - low) * 0.5).max(5.0);
    let x = (temp_k - centre) / width;
    (-x * x).exp().clamp(0.0, 1.0)
}

/// 1.0 inside the species' preferred gravity range, falls off outside.
fn gravity_fit(species: &Species, gravity_g: f32) -> f32 {
    let (low, high) = species.preferred_gravity_range;
    if (low..=high).contains(&gravity_g) {
        return 1.0;
    }
    let centre = (low + high) * 0.5;
    let half_width = ((high - low) * 0.5).max(0.1);
    let dev = (gravity_g - centre).abs() / half_width;
    (1.0 / (1.0 + (dev - 1.0).powi(2))).clamp(0.0, 1.0)
}

/// How well the tile's ocean/land status matches the species' locomotion.
fn hydrosphere_fit(species: &Species, is_ocean: bool) -> f32 {
    let swims = species.locomotion.contains(&LocomotionType::Swimmer);
    let walks = species.locomotion.contains(&LocomotionType::Walker);
    let flies = species.locomotion.contains(&LocomotionType::Flyer);

    if is_ocean {
        if swims {
            1.0
        } else if flies {
            0.4
        } else {
            0.0
        }
    } else {
        // Land tile
        if walks {
            1.0
        } else if flies {
            0.8
        } else if swims {
            0.1
        } else {
            0.5
        }
    }
}

/// How well the biome matches the species' body plan.
fn biome_affinity(species: &Species, biome: Biome) -> f32 {
    use BodyPlan::*;
    let plan = species.body_plan;
    match biome {
        Biome::TropicalForest | Biome::TemperateForest => match plan {
            Vertebrate | Arthropod | PlantLike => 1.0,
            Mollusk => 0.5,
            Amorphous | Crystalline => 0.6,
        },
        Biome::Taiga => match plan {
            Vertebrate | Arthropod | PlantLike => 0.9,
            Mollusk => 0.4,
            _ => 0.6,
        },
        Biome::Grassland | Biome::Savanna | Biome::Steppe => match plan {
            Vertebrate | Arthropod => 1.0,
            PlantLike => 0.8,
            Mollusk => 0.3,
            _ => 0.5,
        },
        Biome::Wetland | Biome::Mangrove => match plan {
            Mollusk | Amorphous | Arthropod => 1.0,
            Vertebrate | PlantLike => 0.7,
            _ => 0.5,
        },
        Biome::Desert => match plan {
            Arthropod => 0.8,
            Vertebrate => 0.6,
            Crystalline => 0.9,
            _ => 0.3,
        },
        Biome::ColdDesert | Biome::XericShrubland => match plan {
            Arthropod => 0.7,
            Vertebrate => 0.5,
            Crystalline => 0.8,
            _ => 0.2,
        },
        Biome::MediterraneanShrubland | Biome::Chaparral => match plan {
            Vertebrate | Arthropod | PlantLike => 0.9,
            Mollusk => 0.3,
            _ => 0.5,
        },
        Biome::Tundra | Biome::Alpine => match plan {
            Vertebrate => 0.7,
            Arthropod => 0.5,
            PlantLike => 0.4,
            _ => 0.3,
        },
        Biome::Ocean => match plan {
            Mollusk | Amorphous => 1.0,
            Vertebrate => 0.9,
            Arthropod => 0.7,
            _ => 0.2,
        },
        Biome::Volcanic | Biome::Barren => match plan {
            Crystalline => 0.9,
            _ => 0.1,
        },
        Biome::IceCap => match plan {
            Vertebrate => 0.3,
            _ => 0.1,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::species::{Biochemistry, BodyPlan, LocomotionType, SizeClass, TrophicLevel};

    fn simple_habitat() -> HabitatGrid {
        // 4-tile row: tropical forest, temperate forest, desert, ocean.
        HabitatGrid {
            width: 4,
            height: 1,
            temperature_c: vec![27.0, 15.0, 30.0, 20.0],
            humidity_relative: vec![0.9, 0.6, 0.1, 0.8],
            biome: vec![
                Biome::TropicalForest,
                Biome::TemperateForest,
                Biome::Desert,
                Biome::Ocean,
            ],
            is_ocean: vec![false, false, false, true],
            elevation_m: vec![300.0, 200.0, 500.0, -2000.0],
        }
    }

    fn walker_vertebrate() -> Species {
        Species {
            name: "Walker".into(),
            biochemistry: Biochemistry::CarbonWater,
            body_plan: BodyPlan::Vertebrate,
            locomotion: vec![LocomotionType::Walker],
            trophic_level: TrophicLevel::Herbivore,
            size_class: SizeClass::Medium,
            intelligence: 4,
            tech_level: None,
            lifespan_years: 50.0,
            preferred_temp_range: (280.0, 310.0),
            preferred_gravity_range: (0.7, 1.3),
            ..Default::default()
        }
    }

    fn ocean_swimmer() -> Species {
        Species {
            name: "Swimmer".into(),
            biochemistry: Biochemistry::CarbonWater,
            body_plan: BodyPlan::Mollusk,
            locomotion: vec![LocomotionType::Swimmer],
            trophic_level: TrophicLevel::Carnivore,
            size_class: SizeClass::Large,
            intelligence: 4,
            tech_level: None,
            lifespan_years: 80.0,
            preferred_temp_range: (275.0, 300.0),
            preferred_gravity_range: (0.7, 1.5),
            ..Default::default()
        }
    }

    #[test]
    fn vegetation_density_zero_without_plantlife() {
        let habitat = simple_habitat();
        let (_, density) = generate_vegetation(&habitat, LifeLevel::UniCellular);
        for &d in &density {
            assert_eq!(d, 0.0);
        }
    }

    #[test]
    fn tropical_forest_has_high_density_on_plantlike_world() {
        let habitat = simple_habitat();
        let (_, density) = generate_vegetation(&habitat, LifeLevel::AnimalLike);
        assert!(density[0] > density[2], "tropical > desert");
        assert!(density[0] > 0.5);
    }

    #[test]
    fn desert_has_low_density() {
        let habitat = simple_habitat();
        let (_, density) = generate_vegetation(&habitat, LifeLevel::AnimalLike);
        assert!(density[2] < 0.2);
    }

    #[test]
    fn ocean_tiles_have_zero_land_vegetation() {
        let habitat = simple_habitat();
        let (_, density) = generate_vegetation(&habitat, LifeLevel::AnimalLike);
        assert_eq!(density[3], 0.0);
    }

    #[test]
    fn walker_cannot_live_in_ocean() {
        let habitat = simple_habitat();
        let species = walker_vertebrate();
        let (_, veg) = generate_vegetation(&habitat, LifeLevel::AnimalLike);
        let range = compute_species_range(&species, 1.0, &habitat, &veg);
        assert_eq!(range.habitability[3], 0.0); // ocean tile
    }

    #[test]
    fn swimmer_prefers_ocean() {
        let habitat = simple_habitat();
        let species = ocean_swimmer();
        let (_, veg) = generate_vegetation(&habitat, LifeLevel::AnimalLike);
        let range = compute_species_range(&species, 1.0, &habitat, &veg);
        assert!(range.habitability[3] > range.habitability[0]);
        assert!(range.habitability[3] > 0.5);
    }

    #[test]
    fn walker_prefers_temperate_biome() {
        let habitat = simple_habitat();
        let species = walker_vertebrate();
        let (_, veg) = generate_vegetation(&habitat, LifeLevel::AnimalLike);
        let range = compute_species_range(&species, 1.0, &habitat, &veg);
        // Temperate forest (tile 1) should score higher than desert (tile 2).
        assert!(range.habitability[1] > range.habitability[2]);
    }

    #[test]
    fn species_gravity_mismatch_reduces_habitability() {
        let habitat = simple_habitat();
        let species = walker_vertebrate();
        let (_, veg) = generate_vegetation(&habitat, LifeLevel::AnimalLike);
        // Species prefers 0.7–1.3 g. Test at 3 g.
        let range_high_g = compute_species_range(&species, 3.0, &habitat, &veg);
        let range_normal_g = compute_species_range(&species, 1.0, &habitat, &veg);
        let h_high: f32 = range_high_g.habitability.iter().sum();
        let h_normal: f32 = range_normal_g.habitability.iter().sum();
        assert!(h_high < h_normal);
    }

    #[test]
    fn territory_is_threshold_of_habitability() {
        let habitat = simple_habitat();
        let species = walker_vertebrate();
        let (_, veg) = generate_vegetation(&habitat, LifeLevel::AnimalLike);
        let range = compute_species_range(&species, 1.0, &habitat, &veg);
        for idx in 0..habitat.tile_count() {
            let expect_in = range.habitability[idx] > 0.5;
            assert_eq!(range.territory[idx], expect_in);
        }
    }

    #[test]
    fn heterotroph_density_scales_with_vegetation() {
        let habitat = simple_habitat();
        let species = walker_vertebrate(); // Herbivore
        let (_, veg) = generate_vegetation(&habitat, LifeLevel::AnimalLike);
        let range = compute_species_range(&species, 1.0, &habitat, &veg);
        // Tropical forest (tile 0): if habitable AND vegetation > 0, density > 0.
        if range.habitability[0] > 0.1 && veg[0] > 0.1 {
            assert!(range.population_density[0] > 0.0);
        }
        // Desert (tile 2): vegetation ~0.1, density should be low.
        assert!(range.population_density[2] < range.population_density[0]);
    }

    #[test]
    fn distribute_ecosystem_produces_one_range_per_species() {
        use crate::ecosystem::generate_ecosystem_from_world;
        use crate::input::SpeciesGenerationInput;
        use crate::types::{Climate, Habitat, Temperature};

        let input = SpeciesGenerationInput {
            habitat: Habitat::Terrestrial,
            climate: Climate::Terrestrial,
            temperature: Temperature::Temperate,
            gravity: 1.0,
            atmospheric_pressure: 1.0,
            hydrosphere: 70.0,
            life_level: LifeLevel::AnimalLike,
            seed: "test".into(),
            scope_key: "world".into(),
        };
        let ecosystem = generate_ecosystem_from_world(&input);
        let habitat = simple_habitat();
        let dist = distribute_ecosystem(&ecosystem, 1.0, &habitat, LifeLevel::AnimalLike);
        assert_eq!(dist.ranges.len(), ecosystem.species_count());
    }
}
