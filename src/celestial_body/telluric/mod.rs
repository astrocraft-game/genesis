use crate::internal::*;
use crate::prelude::*;

pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct TelluricBodyDetails {
    pub body_type: TelluricBodyComposition,
    pub world_type: CelestialBodyWorldType,
    pub special_traits: Vec<CelestialBodySpecialTrait>,
    pub core_heat: CelestialBodyCoreHeat,
    pub magnetic_field: MagneticFieldStrength,
    pub resources: Vec<PlanetaryResource>,
    pub points_of_interest: Vec<PointOfInterest>,
    pub magnetopause_radii: f32,
    pub has_radiation_belts: bool,
    pub aurora_latitude: f32,
}

impl TelluricBodyDetails {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        body_type: TelluricBodyComposition,
        world_type: CelestialBodyWorldType,
        special_traits: Vec<CelestialBodySpecialTrait>,
        core_heat: CelestialBodyCoreHeat,
        magnetic_field: MagneticFieldStrength,
        resources: Vec<PlanetaryResource>,
        points_of_interest: Vec<PointOfInterest>,
        magnetopause_radii: f32,
        has_radiation_belts: bool,
        aurora_latitude: f32,
    ) -> Self {
        Self {
            body_type,
            world_type,
            special_traits,
            core_heat,
            magnetic_field,
            resources,
            points_of_interest,
            magnetopause_radii,
            has_radiation_belts,
            aurora_latitude,
        }
    }
}
