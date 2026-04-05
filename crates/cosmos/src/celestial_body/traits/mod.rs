use crate::internal::*;
use crate::prelude::*;
use std::fmt::Display;

pub mod types;

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum CelestialBodySpecialTrait {
    #[default]
    NoPeculiarity,
    ProtoGiant,
    RetrogradeOrbit,
    SpecificGeologicActivity(TelluricGeologicActivity),
    SpecificTerrainRelief(TelluricTerrainRelief),
    TideLocked(TideLockTarget),
    UnusualVolatileDensity(TelluricVolatileDensityDifference),
    UnusualMagneticField(TelluricMagneticFieldDifference),
    UnusualAxialTilt(TelluricAxialTiltDifference),
    UnusualRotation(TelluricRotationDifference),
    UnusualCore(TelluricCoreDifference),
    SubSurfaceOceans(ChemicalComponent),
    Oceans(ChemicalComponent),
    Lakes(ChemicalComponent),
    UnusualElementPresence((ChemicalComponent, ElementPresenceOccurrence)),
}

impl Display for CelestialBodySpecialTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CelestialBodySpecialTrait::NoPeculiarity => write!(f, "No Peculiarity"),
            CelestialBodySpecialTrait::ProtoGiant => write!(f, "Proto-Giant"),
            CelestialBodySpecialTrait::RetrogradeOrbit => write!(f, "Retrograde Orbit"),
            CelestialBodySpecialTrait::SpecificGeologicActivity(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::SpecificTerrainRelief(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualVolatileDensity(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualMagneticField(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualAxialTilt(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualRotation(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualCore(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::TideLocked(s) => write!(f, "Tide-Locked {}", s),
            CelestialBodySpecialTrait::SubSurfaceOceans(s) => {
                write!(f, "{} Sub-Surface Oceans", s)
            }
            CelestialBodySpecialTrait::Oceans(s) => write!(f, "{} Oceans", s),
            CelestialBodySpecialTrait::Lakes(s) => write!(f, "{} Lakes", s),
            CelestialBodySpecialTrait::UnusualElementPresence(difference) => {
                write!(f, "{} of {}", difference.1, difference.0)
            }
        }
    }
}
