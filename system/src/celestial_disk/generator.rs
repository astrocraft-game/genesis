use crate::internal::*;
use crate::prelude::*;

impl CelestialDisk {
    /// Generate a ring disk for a gas giant or large planet.
    pub fn generate_ring(
        orbit: Option<Orbit>,
        orbital_point_id: u32,
        name: Rc<str>,
        ring: CelestialRingDetails,
    ) -> Self {
        Self::new(orbit, orbital_point_id, name, CelestialDiskType::Ring(ring))
    }

    /// Generate a belt disk (asteroid belt, Kuiper belt, etc.).
    pub fn generate_belt(
        orbit: Option<Orbit>,
        orbital_point_id: u32,
        name: Rc<str>,
        belt: CelestialBeltDetails,
    ) -> Self {
        Self::new(orbit, orbital_point_id, name, CelestialDiskType::Belt(belt))
    }

    /// Generate an Oort cloud shell.
    pub fn generate_shell(
        orbit: Option<Orbit>,
        orbital_point_id: u32,
        name: Rc<str>,
    ) -> Self {
        Self::new(orbit, orbital_point_id, name, CelestialDiskType::Shell)
    }
}
