use crate::internal::*;
use crate::prelude::*;
pub mod celestial_body;
pub mod celestial_disk;
pub mod contents;
mod display;
pub mod generator;
pub mod neighborhood;
pub mod orbital_point;
pub mod star;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct StarSystem {
    /// That star's name.
    #[default("default")]
    pub name: Rc<str>,
    /// The id of the [OrbitalPoint] at the center of the system.
    pub center_id: u32,
    /// The id of the [OrbitalPoint] containing the main star of the system.
    pub main_star_id: u32,
    /// The list of [OrbitalPoint]s that can be found in the system.
    pub all_objects: Vec<OrbitalPoint>,
    /// What are the pecularities of this system.
    pub special_traits: Vec<SystemPeculiarity>,
    /// Estimated Oort cloud mass in Earth masses (0 if no gas giants to scatter material).
    pub oort_cloud_mass: f32,
    /// Estimated Kuiper belt analog mass in Earth masses.
    pub kuiper_belt_mass: f32,
    /// Estimated long-period comet injection rate (new comets per century).
    pub comet_injection_rate: f32,
}

impl StarSystem {
    /// Creates a new star system with the given array of [OrbitalPoint], and the id of the system's main star.
    pub fn new(
        name: Rc<str>,
        center_id: u32,
        main_star_id: u32,
        all_objects: Vec<OrbitalPoint>,
        special_traits: Vec<SystemPeculiarity>,
    ) -> Self {
        // Estimate outer system from gas giant presence
        let gas_giant_mass_sum: f64 = all_objects.iter().map(|o| {
            if let AstronomicalObject::TelluricBody(body) = &o.object {
                if body.mass > 10.0 { body.mass } else { 0.0 }
            } else { 0.0 }
        }).sum();
        let has_gas_giants = gas_giant_mass_sum > 10.0;
        let oort_cloud_mass = if has_gas_giants {
            (gas_giant_mass_sum as f32 / 300.0 * 10.0).clamp(1.0, 100.0)
        } else { 0.0 };
        let kuiper_belt_mass = if has_gas_giants {
            (gas_giant_mass_sum as f32 / 300.0 * 0.1).clamp(0.001, 0.5)
        } else { 0.0 };
        let comet_injection_rate = oort_cloud_mass * 0.5;

        Self {
            name,
            center_id,
            main_star_id,
            all_objects,
            special_traits,
            oort_cloud_mass,
            kuiper_belt_mass,
            comet_injection_rate,
        }
    }

    /// Returns a reference to the [OrbitalPoint] at the center of the system. It can either be a [Star] or the barycentre of a binary pair.
    pub fn get_center(&self) -> &OrbitalPoint {
        self.get_point(self.center_id)
            .expect("There should always be a center point.")
    }

    /// Returns a mutable reference to the [OrbitalPoint] at the center of the system. It can either be a [Star] or the barycentre
    /// of a binary pair.
    pub fn get_center_mut(&mut self) -> &mut OrbitalPoint {
        self.get_point_mut(self.center_id)
            .expect("There should always be a center point.")
    }

    /// Returns a reference to the [OrbitalPoint] containing the main [Star] of the system.
    pub fn get_main_star(&self) -> &OrbitalPoint {
        self.get_point(self.main_star_id)
            .expect("There should always be a main star.")
    }

    /// Returns a mutable reference to the [OrbitalPoint] containing the main [Star] of the system.
    pub fn get_main_star_mut(&mut self) -> &mut OrbitalPoint {
        self.get_point_mut(self.main_star_id)
            .expect("There should always be a main star.")
    }

    /// Returns an [Option] that might contain a reference to the object with the given id.
    pub fn get_point(&self, id: u32) -> Option<&OrbitalPoint> {
        self.all_objects.iter().find(|p| p.id == id)
    }

    /// Returns an [Option] that might contain a mutable reference to the object with the given id.
    pub fn get_point_mut(&mut self, id: u32) -> Option<&mut OrbitalPoint> {
        self.all_objects.iter_mut().find(|p| p.id == id)
    }
}
