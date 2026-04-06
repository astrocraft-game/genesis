//! Planetary history and technology progression.
//!
//! Two complementary systems:
//!
//! 1. **Technology eras** — `HistoricalEra` maps a tech level to
//!    temperature/pressure capabilities, gating which crafting recipes
//!    the player's factory can execute.
//!
//! 2. **Planetary timeline** — `generate_planetary_timeline` produces the
//!    geological and biological history of a world: formation, volcanism,
//!    ocean appearance, first life, mass extinctions, etc. This is deep
//!    backstory that explains *why* resources are where they are and
//!    *why* alien species have certain traits.
//!
//! There is no civilisation simulation — the player IS the civilisation.

use seeded_dice_roller::SeededDiceRoller;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt::{self, Display};

// ---------------------------------------------------------------------------
// Technology eras (player factory progression)
// ---------------------------------------------------------------------------

/// Factory progression stages, from stone-age campfires to interstellar
/// plasma reactors. Each era gates crafting recipes via temperature and
/// pressure thresholds.
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum HistoricalEra {
    #[default]
    Origin,
    FirstTools,
    Agriculture,
    EarlyCivilization,
    Industrialization,
    InformationAge,
    SpaceExploration,
    Interplanetary,
    Interstellar,
}

impl Display for HistoricalEra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HistoricalEra::Origin => "Origin",
                HistoricalEra::FirstTools => "First Tools",
                HistoricalEra::Agriculture => "Agriculture",
                HistoricalEra::EarlyCivilization => "Early Civilization",
                HistoricalEra::Industrialization => "Industrialization",
                HistoricalEra::InformationAge => "Information Age",
                HistoricalEra::SpaceExploration => "Space Exploration",
                HistoricalEra::Interplanetary => "Interplanetary",
                HistoricalEra::Interstellar => "Interstellar",
            }
        )
    }
}

impl HistoricalEra {
    /// Minimum tech level needed to reach this era.
    pub fn min_tech_level(self) -> u8 {
        match self {
            HistoricalEra::Origin => 0,
            HistoricalEra::FirstTools => 1,
            HistoricalEra::Agriculture => 2,
            HistoricalEra::EarlyCivilization => 4,
            HistoricalEra::Industrialization => 6,
            HistoricalEra::InformationAge => 8,
            HistoricalEra::SpaceExploration => 9,
            HistoricalEra::Interplanetary => 10,
            HistoricalEra::Interstellar => 12,
        }
    }

    /// Which era a given tech level corresponds to.
    pub fn from_tech_level(tech_level: u8) -> Self {
        match tech_level {
            0 => HistoricalEra::Origin,
            1 => HistoricalEra::FirstTools,
            2..=3 => HistoricalEra::Agriculture,
            4..=5 => HistoricalEra::EarlyCivilization,
            6..=7 => HistoricalEra::Industrialization,
            8 => HistoricalEra::InformationAge,
            9 => HistoricalEra::SpaceExploration,
            10..=11 => HistoricalEra::Interplanetary,
            _ => HistoricalEra::Interstellar,
        }
    }

    /// Maximum temperature (°C) and pressure (atm) achievable at this era.
    pub fn capability_thresholds(self) -> (i32, f32) {
        if self == HistoricalEra::Origin {
            return (0, 0.0);
        }
        crate::expansion::tech_level_capabilities(self.min_tech_level())
    }

    /// Whether a factory at this era can execute a recipe requiring the
    /// given minimum temperature and pressure.
    pub fn can_achieve(self, min_temp_c: i32, pressure_atm: f32) -> bool {
        let (t, p) = self.capability_thresholds();
        min_temp_c <= t && pressure_atm <= p
    }

    /// Factory stage label (for UI/flavour text).
    pub fn factory_stage(self) -> &'static str {
        match self {
            HistoricalEra::Origin | HistoricalEra::FirstTools => "manual",
            HistoricalEra::Agriculture => "kiln",
            HistoricalEra::EarlyCivilization => "furnace",
            HistoricalEra::Industrialization => "blast-furnace",
            HistoricalEra::InformationAge => "electric-arc",
            HistoricalEra::SpaceExploration => "chemical-reactor",
            HistoricalEra::Interplanetary => "plasma",
            HistoricalEra::Interstellar => "exotic",
        }
    }

    /// Key technologies accessible at this era (flavour text).
    pub fn key_technologies(self) -> &'static [&'static str] {
        match self {
            HistoricalEra::Origin => &["fire", "stone tools"],
            HistoricalEra::FirstTools => &["fire", "stone tools", "bone tools", "tanning"],
            HistoricalEra::Agriculture => &[
                "pottery",
                "copper smelting",
                "bronze casting",
                "weaving",
                "irrigation",
            ],
            HistoricalEra::EarlyCivilization => &[
                "iron smelting",
                "glass blowing",
                "masonry",
                "wheel",
                "sailing",
            ],
            HistoricalEra::Industrialization => &[
                "steel production",
                "steam engine",
                "gunpowder",
                "textile machinery",
                "coal mining",
            ],
            HistoricalEra::InformationAge => &[
                "electronics",
                "computers",
                "petrochemistry",
                "nuclear fission",
            ],
            HistoricalEra::SpaceExploration => &[
                "rocketry",
                "advanced materials",
                "nuclear power",
                "gene editing",
            ],
            HistoricalEra::Interplanetary => &[
                "fusion power",
                "asteroid mining",
                "terraforming",
                "AI systems",
            ],
            HistoricalEra::Interstellar => &[
                "FTL drive",
                "antimatter reactors",
                "megastructures",
                "nanotechnology",
            ],
        }
    }
}

// ---------------------------------------------------------------------------
// Planetary timeline — geological and biological history
// ---------------------------------------------------------------------------

/// Categories of planetary history events.
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum PlanetaryEventKind {
    // Geological
    #[default]
    PlanetFormation,
    CoreDifferentiation,
    HeavyBombardment,
    FirstOceans,
    PlateOnset,
    SupervolcanicEruption,
    IceAge,
    GlobalWarming,
    MagneticFieldReversal,

    // Biological
    FirstLife,
    PhotosynthesisOnset,
    OxygenCatastrophe,
    FirstMulticellular,
    CambrianExplosion,
    MassExtinction,
    FirstLandLife,
    FirstFlight,
    IntelligenceEmergence,

    // Precursor (optional)
    PrecursorCivRise,
    PrecursorCivCollapse,
}

impl Display for PlanetaryEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PlanetaryEventKind::PlanetFormation => "Planet Formation",
            PlanetaryEventKind::CoreDifferentiation => "Core Differentiation",
            PlanetaryEventKind::HeavyBombardment => "Heavy Bombardment",
            PlanetaryEventKind::FirstOceans => "First Oceans",
            PlanetaryEventKind::PlateOnset => "Plate Tectonics Begin",
            PlanetaryEventKind::SupervolcanicEruption => "Supervolcanic Eruption",
            PlanetaryEventKind::IceAge => "Ice Age",
            PlanetaryEventKind::GlobalWarming => "Global Warming",
            PlanetaryEventKind::MagneticFieldReversal => "Magnetic Field Reversal",
            PlanetaryEventKind::FirstLife => "First Life",
            PlanetaryEventKind::PhotosynthesisOnset => "Photosynthesis Begins",
            PlanetaryEventKind::OxygenCatastrophe => "Oxygen Catastrophe",
            PlanetaryEventKind::FirstMulticellular => "First Multicellular Life",
            PlanetaryEventKind::CambrianExplosion => "Cambrian Explosion",
            PlanetaryEventKind::MassExtinction => "Mass Extinction",
            PlanetaryEventKind::FirstLandLife => "First Land Life",
            PlanetaryEventKind::FirstFlight => "First Flight",
            PlanetaryEventKind::IntelligenceEmergence => "Intelligence Emerges",
            PlanetaryEventKind::PrecursorCivRise => "Precursor Civilisation Rises",
            PlanetaryEventKind::PrecursorCivCollapse => "Precursor Civilisation Collapses",
        };
        write!(f, "{}", s)
    }
}

/// A single event in a planet's geological or biological history.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanetaryEvent {
    pub kind: PlanetaryEventKind,
    /// Millions of years ago (Mya). 0.0 = present.
    pub mya: f64,
    /// Machine-generated narrative description.
    pub description: String,
}

/// An extinct precursor civilisation whose ruins may be discoverable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrecursorRuin {
    pub name: String,
    /// Tile indices where ruins are located.
    pub ruin_tiles: Vec<usize>,
    /// When the precursor civilisation collapsed (Mya).
    pub collapse_mya: f64,
    /// Tech level they reached before collapse.
    pub tech_level: u8,
}

/// Complete planetary history: formation through present day.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlanetaryTimeline {
    pub events: Vec<PlanetaryEvent>,
    /// Optional extinct precursor civilisation(s).
    pub precursor_ruins: Vec<PrecursorRuin>,
    /// Planet age in billions of years.
    pub planet_age_gyr: f32,
}

/// Generate the geological and biological timeline of a planet.
///
/// - `star_age_gyr`: age of the host star (planet is slightly younger).
/// - `has_oceans`: whether the planet has liquid surface water.
/// - `has_life`: whether the planet hosts life above unicellular level.
/// - `tile_count`: total tiles on the surface grid (for ruin placement).
/// - `seed`: deterministic seed string.
pub fn generate_planetary_timeline(
    star_age_gyr: f32,
    has_oceans: bool,
    has_life: bool,
    tile_count: usize,
    seed: &str,
) -> PlanetaryTimeline {
    let mut rng = SeededDiceRoller::new(seed, "planetary_timeline");
    let mut events = Vec::new();

    let planet_age_gyr = star_age_gyr * (0.85 + rng.gen_f64() as f32 * 0.1);
    let age_mya = planet_age_gyr as f64 * 1000.0;

    // Geological events (always present)
    events.push(PlanetaryEvent {
        kind: PlanetaryEventKind::PlanetFormation,
        mya: age_mya,
        description: format!(
            "The planet coalesced from the protoplanetary disk {:.1} billion years ago.",
            planet_age_gyr
        ),
    });

    events.push(PlanetaryEvent {
        kind: PlanetaryEventKind::CoreDifferentiation,
        mya: age_mya * 0.98,
        description: "Heavy elements sank to form a metallic core.".into(),
    });

    events.push(PlanetaryEvent {
        kind: PlanetaryEventKind::HeavyBombardment,
        mya: age_mya * 0.90 + rng.gen_f64() * 100.0,
        description: "Intense meteorite bombardment scarred the young surface.".into(),
    });

    if has_oceans {
        let ocean_mya = age_mya * 0.80 + rng.gen_f64() * 200.0;
        events.push(PlanetaryEvent {
            kind: PlanetaryEventKind::FirstOceans,
            mya: ocean_mya,
            description: "Volcanic outgassing and comet delivery formed the first oceans.".into(),
        });
    }

    events.push(PlanetaryEvent {
        kind: PlanetaryEventKind::PlateOnset,
        mya: age_mya * 0.75 + rng.gen_f64() * 200.0,
        description: "Convective mantle currents drove the first tectonic plates.".into(),
    });

    // Random geological disruptions
    let n_disruptions = 2 + (rng.gen_u32() % 4) as usize;
    for _ in 0..n_disruptions {
        let kind = match rng.roll(1, 4, 0) {
            1 => PlanetaryEventKind::SupervolcanicEruption,
            2 => PlanetaryEventKind::IceAge,
            3 => PlanetaryEventKind::GlobalWarming,
            _ => PlanetaryEventKind::MagneticFieldReversal,
        };
        let mya = rng.gen_f64() * age_mya * 0.6;
        events.push(PlanetaryEvent {
            kind,
            mya,
            description: format!("{} occurred {:.0} million years ago.", kind, mya),
        });
    }

    // Biological events (if life exists)
    if has_life {
        let life_start = age_mya * 0.65 + rng.gen_f64() * 500.0;

        events.push(PlanetaryEvent {
            kind: PlanetaryEventKind::FirstLife,
            mya: life_start,
            description: "Primitive single-celled organisms appeared.".into(),
        });

        if has_oceans {
            events.push(PlanetaryEvent {
                kind: PlanetaryEventKind::PhotosynthesisOnset,
                mya: life_start * 0.85,
                description: "Photosynthetic organisms began producing oxygen.".into(),
            });
            events.push(PlanetaryEvent {
                kind: PlanetaryEventKind::OxygenCatastrophe,
                mya: life_start * 0.70,
                description:
                    "Rising oxygen levels poisoned anaerobic life, reshaping the biosphere.".into(),
            });
        }

        events.push(PlanetaryEvent {
            kind: PlanetaryEventKind::FirstMulticellular,
            mya: life_start * 0.50 + rng.gen_f64() * 100.0,
            description: "Multicellular organisms emerged.".into(),
        });

        let explosion_mya = life_start * 0.35 + rng.gen_f64() * 100.0;
        events.push(PlanetaryEvent {
            kind: PlanetaryEventKind::CambrianExplosion,
            mya: explosion_mya,
            description: "Rapid diversification of complex body plans.".into(),
        });

        // Mass extinctions
        let n_extinctions = 1 + (rng.gen_u32() % 4) as usize;
        for _ in 0..n_extinctions {
            let mya = rng.gen_f64() * explosion_mya * 0.8;
            events.push(PlanetaryEvent {
                kind: PlanetaryEventKind::MassExtinction,
                mya,
                description: format!(
                    "A mass extinction event wiped out {:.0}% of species.",
                    30.0 + rng.gen_f64() * 60.0
                ),
            });
        }

        events.push(PlanetaryEvent {
            kind: PlanetaryEventKind::FirstLandLife,
            mya: explosion_mya * 0.7,
            description: "Life colonised the land.".into(),
        });

        if rng.gen_f64() > 0.3 {
            events.push(PlanetaryEvent {
                kind: PlanetaryEventKind::FirstFlight,
                mya: explosion_mya * 0.4 + rng.gen_f64() * 50.0,
                description: "Flying organisms evolved.".into(),
            });
        }

        if rng.gen_f64() > 0.5 {
            events.push(PlanetaryEvent {
                kind: PlanetaryEventKind::IntelligenceEmergence,
                mya: rng.gen_f64() * 5.0,
                description: "A species developed tool use and problem solving.".into(),
            });
        }
    }

    // Precursor ruins (rare — ~20% chance on life-bearing worlds)
    let mut precursor_ruins = Vec::new();
    if has_life && rng.gen_f64() < 0.20 {
        let name_gen = crate::naming::MarkovNameGen::for_style(crate::naming::NameStyle::Alien);
        let name = name_gen.generate(&mut rng, 5, 10);
        let collapse_mya = 0.5 + rng.gen_f64() * 50.0;
        let tech = 6 + (rng.gen_u32() % 7) as u8;
        let n_ruins = 2 + (rng.gen_u32() % 5) as usize;
        let ruin_tiles: Vec<usize> = (0..n_ruins)
            .map(|_| {
                if tile_count > 0 {
                    rng.gen_usize() % tile_count
                } else {
                    0
                }
            })
            .collect();

        events.push(PlanetaryEvent {
            kind: PlanetaryEventKind::PrecursorCivRise,
            mya: collapse_mya + 1.0 + rng.gen_f64() * 5.0,
            description: format!("The {} civilisation arose.", name),
        });
        events.push(PlanetaryEvent {
            kind: PlanetaryEventKind::PrecursorCivCollapse,
            mya: collapse_mya,
            description: format!(
                "The {} civilisation collapsed, leaving {} ruin sites.",
                name, n_ruins
            ),
        });

        precursor_ruins.push(PrecursorRuin {
            name,
            ruin_tiles,
            collapse_mya,
            tech_level: tech,
        });
    }

    // Sort events: oldest first.
    events.sort_by(|a, b| {
        b.mya
            .partial_cmp(&a.mya)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    PlanetaryTimeline {
        events,
        precursor_ruins,
        planet_age_gyr,
    }
}

// ---------------------------------------------------------------------------
// Species evolution history (kept from v0.2)
// ---------------------------------------------------------------------------

/// A single event in a species' evolutionary history.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct SpeciesHistoryEvent {
    pub era: HistoricalEra,
    pub description: &'static str,
    pub years_ago: f64,
}

/// Generate an evolutionary history for a species based on its tech level.
pub fn generate_species_history(
    tech_level: u8,
    lifespan_years: f32,
    seed: &str,
    species_name: &str,
) -> Vec<SpeciesHistoryEvent> {
    let mut rng = SeededDiceRoller::new(seed, &format!("species_{}_history", species_name));
    let mut events = Vec::new();

    let lifespan_factor = (lifespan_years / 80.0) as f64;
    let history_length = lifespan_factor
        * match tech_level {
            0..=1 => 10_000.0,
            2..=3 => 50_000.0,
            4..=5 => 200_000.0,
            6..=7 => 500_000.0,
            8..=9 => 1_000_000.0,
            10..=11 => 5_000_000.0,
            _ => 10_000_000.0,
        };

    events.push(SpeciesHistoryEvent {
        era: HistoricalEra::Origin,
        description: "Species emerged",
        years_ago: history_length,
    });

    let eras: Vec<(HistoricalEra, &'static str)> = match tech_level {
        0 => vec![],
        1 => vec![(HistoricalEra::FirstTools, "First tool use observed")],
        2..=3 => vec![
            (HistoricalEra::FirstTools, "First tool use observed"),
            (HistoricalEra::Agriculture, "Cultivation began"),
        ],
        4..=5 => vec![
            (HistoricalEra::FirstTools, "First tool use observed"),
            (HistoricalEra::Agriculture, "Cultivation began"),
            (HistoricalEra::EarlyCivilization, "Settlements formed"),
        ],
        6..=7 => vec![
            (HistoricalEra::FirstTools, "First tool use observed"),
            (HistoricalEra::Agriculture, "Cultivation began"),
            (HistoricalEra::EarlyCivilization, "Settlements formed"),
            (
                HistoricalEra::Industrialization,
                "Industrial methods developed",
            ),
        ],
        _ => vec![
            (HistoricalEra::FirstTools, "First tool use observed"),
            (HistoricalEra::Agriculture, "Cultivation began"),
            (HistoricalEra::EarlyCivilization, "Settlements formed"),
            (
                HistoricalEra::Industrialization,
                "Industrial methods developed",
            ),
            (HistoricalEra::InformationAge, "Computing emerged"),
        ],
    };

    let n = eras.len();
    for (i, (era, desc)) in eras.into_iter().enumerate() {
        let frac = (i + 1) as f64 / (n + 1) as f64;
        let years = history_length * (1.0 - frac);
        let jitter = rng.gen_f64() * 0.08 - 0.04;
        events.push(SpeciesHistoryEvent {
            era,
            description: desc,
            years_ago: (years * (1.0 + jitter)).max(0.0),
        });
    }

    events.sort_by(|a, b| b.years_ago.partial_cmp(&a.years_ago).unwrap());
    events
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Era / tech mapping tests ---

    #[test]
    fn era_from_tech_level_is_monotonic() {
        let mut prev = HistoricalEra::from_tech_level(0);
        for tech in 1u8..=12 {
            let cur = HistoricalEra::from_tech_level(tech);
            assert!(
                cur >= prev,
                "tech {}: {:?} regressed from {:?}",
                tech,
                cur,
                prev
            );
            prev = cur;
        }
    }

    #[test]
    fn era_thresholds_increase_with_progression() {
        let order = [
            HistoricalEra::FirstTools,
            HistoricalEra::Agriculture,
            HistoricalEra::EarlyCivilization,
            HistoricalEra::Industrialization,
            HistoricalEra::InformationAge,
            HistoricalEra::SpaceExploration,
            HistoricalEra::Interplanetary,
            HistoricalEra::Interstellar,
        ];
        let mut prev = order[0].capability_thresholds();
        for era in &order[1..] {
            let cur = era.capability_thresholds();
            assert!(cur.0 >= prev.0 && cur.1 >= prev.1);
            prev = cur;
        }
    }

    #[test]
    fn origin_era_has_no_capability() {
        let (t, p) = HistoricalEra::Origin.capability_thresholds();
        assert_eq!(t, 0);
        assert_eq!(p, 0.0);
        assert!(!HistoricalEra::Origin.can_achieve(100, 1.0));
    }

    #[test]
    fn industrialization_can_make_steel() {
        assert!(HistoricalEra::Industrialization.can_achieve(1500, 1.0));
        assert!(!HistoricalEra::Agriculture.can_achieve(1500, 1.0));
    }

    #[test]
    fn factory_stage_labels_are_distinct() {
        let stages: Vec<&str> = [
            HistoricalEra::Origin,
            HistoricalEra::Agriculture,
            HistoricalEra::Industrialization,
            HistoricalEra::Interstellar,
        ]
        .iter()
        .map(|e| e.factory_stage())
        .collect();
        // At least 3 of 4 should be distinct.
        let unique: std::collections::HashSet<_> = stages.iter().collect();
        assert!(unique.len() >= 3);
    }

    // --- Planetary timeline tests ---

    #[test]
    fn timeline_is_deterministic() {
        let a = generate_planetary_timeline(4.6, true, true, 2592, "det");
        let b = generate_planetary_timeline(4.6, true, true, 2592, "det");
        assert_eq!(a.events.len(), b.events.len());
        for (x, y) in a.events.iter().zip(b.events.iter()) {
            assert_eq!(x.kind, y.kind);
            assert_eq!(x.mya, y.mya);
        }
    }

    #[test]
    fn timeline_events_are_chronological() {
        let t = generate_planetary_timeline(4.6, true, true, 2592, "chrono");
        for w in t.events.windows(2) {
            assert!(
                w[0].mya >= w[1].mya,
                "event at {:.1} Mya followed by {:.1} Mya",
                w[0].mya,
                w[1].mya
            );
        }
    }

    #[test]
    fn timeline_starts_with_formation() {
        let t = generate_planetary_timeline(4.6, true, true, 2592, "form");
        assert_eq!(t.events[0].kind, PlanetaryEventKind::PlanetFormation);
    }

    #[test]
    fn lifeless_world_has_no_biological_events() {
        let t = generate_planetary_timeline(4.6, false, false, 2592, "dead");
        let bio_events = t.events.iter().filter(|e| {
            matches!(
                e.kind,
                PlanetaryEventKind::FirstLife
                    | PlanetaryEventKind::FirstMulticellular
                    | PlanetaryEventKind::CambrianExplosion
                    | PlanetaryEventKind::MassExtinction
            )
        });
        assert_eq!(bio_events.count(), 0);
    }

    #[test]
    fn living_world_has_first_life() {
        let t = generate_planetary_timeline(4.6, true, true, 2592, "alive");
        assert!(t
            .events
            .iter()
            .any(|e| e.kind == PlanetaryEventKind::FirstLife));
    }

    #[test]
    fn all_events_have_descriptions() {
        let t = generate_planetary_timeline(4.6, true, true, 2592, "desc");
        for e in &t.events {
            assert!(!e.description.is_empty(), "{:?} has no description", e.kind);
        }
    }

    #[test]
    fn precursor_ruins_have_valid_tiles() {
        // Run many seeds to hit the 20% precursor chance.
        for i in 0..50 {
            let t = generate_planetary_timeline(4.6, true, true, 2592, &format!("pre_{}", i));
            for ruin in &t.precursor_ruins {
                assert!(!ruin.name.is_empty());
                assert!(!ruin.ruin_tiles.is_empty());
                for &tile in &ruin.ruin_tiles {
                    assert!(tile < 2592, "ruin tile {} out of bounds", tile);
                }
                assert!(ruin.tech_level >= 6);
            }
        }
    }

    // --- Species history tests ---

    #[test]
    fn species_history_length_scales_with_tech() {
        let low = generate_species_history(1, 50.0, "seed", "A");
        let high = generate_species_history(8, 200.0, "seed", "B");
        assert!(high.len() > low.len());
    }

    #[test]
    fn species_history_is_deterministic() {
        let a = generate_species_history(8, 80.0, "s42", "X");
        let b = generate_species_history(8, 80.0, "s42", "X");
        assert_eq!(a.len(), b.len());
        assert_eq!(a[0].years_ago, b[0].years_ago);
    }

    #[test]
    fn species_history_ordered_by_time() {
        let h = generate_species_history(10, 100.0, "seed", "C");
        for w in h.windows(2) {
            assert!(w[0].years_ago >= w[1].years_ago);
        }
    }
}
