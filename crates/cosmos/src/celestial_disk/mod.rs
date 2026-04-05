use crate::internal::*;
use crate::prelude::*;

pub mod belt;
pub mod generator;
pub mod ring;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CelestialDisk {
    stub: bool,
    pub name: Rc<str>,
    pub orbit: Option<Orbit>,
    pub orbital_point_id: u32,
    pub details: CelestialDiskType,
}

impl CelestialDisk {
    pub fn new(
        orbit: Option<Orbit>,
        orbital_point_id: u32,
        name: Rc<str>,
        details: CelestialDiskType,
    ) -> Self {
        Self {
            stub: false,
            orbit,
            orbital_point_id,
            name,
            details,
        }
    }

    pub fn is_stub(self) -> bool {
        self.stub
    }
}
