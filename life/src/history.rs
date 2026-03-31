use serde::{Serialize, Deserialize};
use smart_default::SmartDefault;
use seeded_dice_roller::SeededDiceRoller;
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

    // Scale based on tech level (higher TL = longer history)
    let history_length_years = match tech_level {
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
}
