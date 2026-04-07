use crate::internal::*;
use crate::prelude::*;

pub mod generator;
pub mod types;
pub mod utils;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct OrbitalPoint {
    pub id: u32,
    pub own_orbit: Option<Orbit>,
    pub object: AstronomicalObject,
    pub orbits: Vec<Orbit>,
}

impl OrbitalPoint {
    pub fn new(
        id: u32,
        own_orbit: Option<Orbit>,
        object: AstronomicalObject,
        orbits: Vec<Orbit>,
    ) -> Self {
        Self {
            id,
            own_orbit,
            object,
            orbits,
        }
    }

    pub fn get_own_orbit(&self) -> Option<Orbit> {
        self.own_orbit.clone()
    }

    pub fn set_own_orbit(&mut self, orbit: Orbit) {
        self.own_orbit = Some(orbit.clone());
        match &mut self.object {
            AstronomicalObject::Void => {}
            AstronomicalObject::Star(ref mut star) => star.orbit = Some(orbit),
            AstronomicalObject::TelluricBody(ref mut body) => body.orbit = Some(orbit),
            AstronomicalObject::GaseousBody(ref mut body) => body.orbit = Some(orbit),
            AstronomicalObject::IcyBody(ref mut body) => body.orbit = Some(orbit),
            AstronomicalObject::TelluricDisk(_) => {}
            AstronomicalObject::GaseousDisk(_) => {}
            AstronomicalObject::IcyDisk(_) => {}
            AstronomicalObject::Spacecraft => {}
        }
    }

    pub fn update_object_own_orbit(&mut self) {
        let orbit = self.get_own_orbit();
        match &mut self.object {
            AstronomicalObject::Void => {}
            AstronomicalObject::Star(star) => {
                star.orbit = orbit;
                star.orbital_point_id = self.id;
            }
            AstronomicalObject::TelluricBody(body) => {
                body.orbit = orbit;
                body.orbital_point_id = self.id;
            }
            AstronomicalObject::GaseousBody(body) => {
                body.orbit = orbit;
                body.orbital_point_id = self.id;
            }
            AstronomicalObject::IcyBody(body) => {
                body.orbit = orbit;
                body.orbital_point_id = self.id;
            }
            AstronomicalObject::TelluricDisk(_) => {}
            AstronomicalObject::GaseousDisk(_) => {}
            AstronomicalObject::IcyDisk(_) => {}
            AstronomicalObject::Spacecraft => {}
        }
    }
}
