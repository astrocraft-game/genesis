use seeded_dice_roller::SeededDiceRoller;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt::{self, Display};

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
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
    /// Minimum tech level a civilisation needs to be *in* this era.
    /// Origin is pre-technological (tech 0); each subsequent era steps by one
    /// or two tech levels to align with `expansion::tech_level_capabilities`.
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

    /// Which era a civilisation with the given tech level is currently in.
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
    /// Returns `(0, 0.0)` for the pre-technological Origin era. Values are
    /// derived from `expansion::tech_level_capabilities`.
    pub fn capability_thresholds(self) -> (i32, f32) {
        if self == HistoricalEra::Origin {
            return (0, 0.0);
        }
        crate::expansion::tech_level_capabilities(self.min_tech_level())
    }

    /// Whether this era's civilisation can fire recipes at the given
    /// minimum temperature and pressure.
    pub fn can_achieve(self, min_temp_c: i32, pressure_atm: f32) -> bool {
        let (t, p) = self.capability_thresholds();
        min_temp_c <= t && pressure_atm <= p
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum HistoricalEventType {
    #[default]
    Milestone,
    War,
    Discovery,
    Catastrophe,
    GoldenAge,
    Schism,
    Contact,
    Migration,
}

impl Display for HistoricalEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HistoricalEventType::Milestone => "Milestone",
                HistoricalEventType::War => "War",
                HistoricalEventType::Discovery => "Discovery",
                HistoricalEventType::Catastrophe => "Catastrophe",
                HistoricalEventType::GoldenAge => "Golden Age",
                HistoricalEventType::Schism => "Schism",
                HistoricalEventType::Contact => "Contact",
                HistoricalEventType::Migration => "Migration",
            }
        )
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct HistoricalEvent {
    /// Which era this event occurred in.
    pub era: HistoricalEra,
    /// Type of event.
    pub event_type: HistoricalEventType,
    /// Years before present when this event occurred.
    pub years_ago: f64,
}

/// Generate a species history based on tech level and lifespan.
pub fn generate_species_history(
    tech_level: u8,
    lifespan_years: f32,
    seed: &str,
    species_name: &str,
) -> Vec<HistoricalEvent> {
    let mut rng = SeededDiceRoller::new(seed, &format!("species_{}_history", species_name));
    let mut events = Vec::new();

    // Scale based on tech level, adjusted by lifespan
    // Longer-lived species develop slower (more conservative), shorter-lived faster
    let lifespan_factor = (lifespan_years / 80.0) as f64; // 80 years = human baseline
    let history_length_years = lifespan_factor
        * match tech_level {
            0..=1 => 10_000.0,
            2..=3 => 50_000.0,
            4..=5 => 200_000.0,
            6..=7 => 500_000.0,
            8..=9 => 1_000_000.0,
            10..=11 => 5_000_000.0,
            _ => 10_000_000.0,
        };

    // Origin event
    events.push(HistoricalEvent {
        era: HistoricalEra::Origin,
        event_type: HistoricalEventType::Milestone,
        years_ago: history_length_years,
    });

    // Generate milestones based on tech level progression
    let eras_to_reach: Vec<HistoricalEra> = match tech_level {
        0 => vec![],
        1 => vec![HistoricalEra::FirstTools],
        2..=3 => vec![HistoricalEra::FirstTools, HistoricalEra::Agriculture],
        4..=5 => vec![
            HistoricalEra::FirstTools,
            HistoricalEra::Agriculture,
            HistoricalEra::EarlyCivilization,
        ],
        6..=7 => vec![
            HistoricalEra::FirstTools,
            HistoricalEra::Agriculture,
            HistoricalEra::EarlyCivilization,
            HistoricalEra::Industrialization,
        ],
        8..=9 => vec![
            HistoricalEra::FirstTools,
            HistoricalEra::Agriculture,
            HistoricalEra::EarlyCivilization,
            HistoricalEra::Industrialization,
            HistoricalEra::InformationAge,
            HistoricalEra::SpaceExploration,
        ],
        10..=11 => vec![
            HistoricalEra::FirstTools,
            HistoricalEra::Agriculture,
            HistoricalEra::EarlyCivilization,
            HistoricalEra::Industrialization,
            HistoricalEra::InformationAge,
            HistoricalEra::SpaceExploration,
            HistoricalEra::Interplanetary,
        ],
        _ => vec![
            HistoricalEra::FirstTools,
            HistoricalEra::Agriculture,
            HistoricalEra::EarlyCivilization,
            HistoricalEra::Industrialization,
            HistoricalEra::InformationAge,
            HistoricalEra::SpaceExploration,
            HistoricalEra::Interplanetary,
            HistoricalEra::Interstellar,
        ],
    };

    let era_count = eras_to_reach.len();
    for (i, era) in eras_to_reach.into_iter().enumerate() {
        let fraction = (i + 1) as f64 / (era_count + 1) as f64;
        let years = history_length_years * (1.0 - fraction);
        events.push(HistoricalEvent {
            era,
            event_type: HistoricalEventType::Milestone,
            years_ago: years,
        });

        // Roll for additional events in each era
        let extra_events = rng.roll(1, 3, -1).max(0) as usize;
        for _ in 0..extra_events {
            let event_type = match rng.roll(1, 7, 0) {
                1 => HistoricalEventType::War,
                2 => HistoricalEventType::Discovery,
                3 => HistoricalEventType::Catastrophe,
                4 => HistoricalEventType::GoldenAge,
                5 => HistoricalEventType::Schism,
                6 => HistoricalEventType::Migration,
                _ => HistoricalEventType::Contact,
            };
            let jitter = rng.gen_f64() * 0.08 - 0.04;
            events.push(HistoricalEvent {
                era,
                event_type,
                years_ago: (years * (1.0 + jitter)).max(0.0),
            });
        }
    }

    // Sort by years ago (most ancient first)
    events.sort_by(|a, b| b.years_ago.partial_cmp(&a.years_ago).unwrap());
    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_length_scales_with_tech_level() {
        let h_low = generate_species_history(1, 50.0, "seed", "TestA");
        let h_high = generate_species_history(12, 200.0, "seed", "TestB");
        assert!(h_high.len() > h_low.len());
    }

    #[test]
    fn history_is_deterministic() {
        let h1 = generate_species_history(8, 80.0, "seed42", "SpeciesX");
        let h2 = generate_species_history(8, 80.0, "seed42", "SpeciesX");
        assert_eq!(h1.len(), h2.len());
        assert_eq!(h1[0].years_ago, h2[0].years_ago);
    }

    #[test]
    fn history_ordered_by_time() {
        let h = generate_species_history(10, 100.0, "seed", "TestC");
        for w in h.windows(2) {
            assert!(w[0].years_ago >= w[1].years_ago);
        }
    }

    #[test]
    fn era_from_tech_level_is_monotonic() {
        // Era index should never decrease as tech level rises.
        let mut prev = HistoricalEra::from_tech_level(0);
        for tech in 1u8..=12 {
            let cur = HistoricalEra::from_tech_level(tech);
            assert!(
                cur >= prev,
                "tech {}: era {:?} regressed from {:?}",
                tech,
                cur,
                prev
            );
            prev = cur;
        }
    }

    #[test]
    fn era_thresholds_increase_with_progression() {
        // Each era admits at least as much temp/pressure as the previous.
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
            assert!(
                cur.0 >= prev.0 && cur.1 >= prev.1,
                "{:?}: thresholds {:?} regressed from {:?}",
                era,
                cur,
                prev
            );
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
        // Steel casting requires ~1500 °C.
        assert!(HistoricalEra::Industrialization.can_achieve(1500, 1.0));
        // Bronze age cannot.
        assert!(!HistoricalEra::Agriculture.can_achieve(1500, 1.0));
    }

    #[test]
    fn interstellar_era_admits_plasma_recipes() {
        assert!(HistoricalEra::Interstellar.can_achieve(5000, 10_000.0));
    }

    #[test]
    fn era_min_tech_matches_from_tech_level() {
        // If era X requires tech N, from_tech_level(N) should return X or later.
        for era in [
            HistoricalEra::FirstTools,
            HistoricalEra::Agriculture,
            HistoricalEra::EarlyCivilization,
            HistoricalEra::Industrialization,
            HistoricalEra::InformationAge,
            HistoricalEra::SpaceExploration,
            HistoricalEra::Interplanetary,
            HistoricalEra::Interstellar,
        ] {
            let derived = HistoricalEra::from_tech_level(era.min_tech_level());
            assert!(
                derived >= era,
                "era {:?} requires tech {} but from_tech_level returns {:?}",
                era,
                era.min_tech_level(),
                derived
            );
        }
    }
}
