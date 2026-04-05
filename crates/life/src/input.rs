use crate::types::{Climate, Habitat, LifeLevel, Temperature};
use serde::{Deserialize, Serialize};

/// All inputs required to generate a species from its homeworld conditions.
///
/// This struct is the sole entry point for `generate_species_from_world`. It
/// carries only primitives and life-owned enums so the crate has no dependency
/// on `cosmos` or `world`. Callers using those crates should construct this via
/// an adapter in the application layer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeciesGenerationInput {
    /// Broad homeworld classification (rocky, ocean, ammonia, etc.).
    pub habitat: Habitat,
    /// Prevailing climate on the homeworld surface.
    pub climate: Climate,
    /// Temperature band used to derive thermal preferences.
    pub temperature: Temperature,
    /// Surface gravity in Earth g (1.0 = Earth).
    pub gravity: f32,
    /// Atmospheric pressure in Earth atm (1.0 = Earth).
    pub atmospheric_pressure: f32,
    /// Hydrosphere coverage as percentage of surface (0.0 – 100.0).
    pub hydrosphere: f32,
    /// How advanced life is on this world.
    pub life_level: LifeLevel,
    /// Seed for deterministic generation.
    pub seed: String,
    /// Unique scope key identifying this body within the seed's universe.
    /// Callers typically build this from galactic coordinates + body id.
    pub scope_key: String,
}
