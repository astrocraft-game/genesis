use crate::internal::*;
use crate::prelude::*;

pub mod gaseous;
pub mod generator;
pub mod icy;
pub mod moon;
pub mod telluric;
pub mod traits;
pub mod types;
pub mod world;

#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct CelestialBody {
    stub: bool,
    #[default("default")]
    pub name: Rc<str>,
    pub orbit: Option<Orbit>,
    pub orbital_point_id: u32,
    pub mass: f64,
    pub radius: f64,
    pub density: f32,
    pub gravity: f32,
    pub blackbody_temperature: u32,
    pub size: CelestialBodySize,
    pub details: CelestialBodyDetails,
    pub tidal_heating: u32,
}

impl CelestialBody {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        orbit: Option<Orbit>,
        orbital_point_id: u32,
        name: Rc<str>,
        mass: f64,
        radius: f64,
        density: f32,
        gravity: f32,
        blackbody_temperature: u32,
        tidal_heating: u32,
        size: CelestialBodySize,
        details: CelestialBodyDetails,
    ) -> Self {
        Self {
            stub: false,
            orbit,
            orbital_point_id,
            name,
            mass,
            radius,
            density,
            gravity,
            blackbody_temperature,
            tidal_heating,
            size,
            details,
        }
    }

    pub fn is_stub(self) -> bool {
        self.stub
    }

    pub fn as_stub(mut self) -> Self {
        self.stub = true;
        self
    }

    pub fn external_facts(
        &self,
        star_age_gyr: f32,
        moon_count: u32,
        has_rings: bool,
    ) -> ExternalBodyFacts {
        let orbit = self.orbit.clone().unwrap_or_default();
        let is_tidally_locked = match &self.details {
            CelestialBodyDetails::Telluric(details) => details
                .special_traits
                .iter()
                .any(|trait_| matches!(trait_, CelestialBodySpecialTrait::TideLocked(_))),
            _ => false,
        };

        ExternalBodyFacts {
            body_id: self.orbital_point_id,
            mass: self.mass,
            radius: self.radius,
            density: self.density,
            gravity: self.gravity,
            blackbody_temperature: self.blackbody_temperature,
            star_age: star_age_gyr,
            distance_from_star: orbit
                .average_distance_from_system_center
                .max(orbit.average_distance),
            eccentricity: orbit.eccentricity,
            axial_tilt: orbit.axial_tilt,
            rotation_days: orbit.rotation,
            is_tidally_locked,
            tidal_heating: self.tidal_heating,
            moon_count,
            has_rings,
        }
    }
}
