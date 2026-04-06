use seeded_dice_roller::SeededDiceRoller;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt::{self, Display};

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

    /// Social structure label for the era.
    pub fn social_structure(self) -> &'static str {
        match self {
            HistoricalEra::Origin | HistoricalEra::FirstTools => "hunter-gatherer",
            HistoricalEra::Agriculture => "agricultural",
            HistoricalEra::EarlyCivilization => "early-state",
            HistoricalEra::Industrialization => "industrial",
            HistoricalEra::InformationAge => "information",
            HistoricalEra::SpaceExploration | HistoricalEra::Interplanetary => "spacefaring",
            HistoricalEra::Interstellar => "interstellar",
        }
    }

    /// Key technologies accessible at this era. Returns a list of
    /// human-readable technology names. Callers can use this for flavour
    /// text; actual recipe filtering uses temperature/pressure thresholds.
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
                "animal husbandry",
            ],
            HistoricalEra::EarlyCivilization => &[
                "iron smelting",
                "glass blowing",
                "masonry",
                "writing",
                "wheel",
                "sailing",
            ],
            HistoricalEra::Industrialization => &[
                "steel production",
                "steam engine",
                "gunpowder",
                "printing press",
                "textile machinery",
                "coal mining",
            ],
            HistoricalEra::InformationAge => &[
                "electronics",
                "computers",
                "petrochemistry",
                "nuclear fission",
                "telecommunications",
            ],
            HistoricalEra::SpaceExploration => &[
                "rocketry",
                "satellites",
                "advanced materials",
                "nuclear power",
                "gene editing",
            ],
            HistoricalEra::Interplanetary => &[
                "fusion power",
                "space habitats",
                "asteroid mining",
                "terraforming",
                "AI systems",
            ],
            HistoricalEra::Interstellar => &[
                "FTL drive",
                "antimatter reactors",
                "megastructures",
                "quantum computing",
                "nanotechnology",
            ],
        }
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum HistoricalEventType {
    #[default]
    Milestone,
    Founding,
    War,
    Discovery,
    Catastrophe,
    GoldenAge,
    Schism,
    Contact,
    Migration,
    DynastyChange,
}

impl Display for HistoricalEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HistoricalEventType::Milestone => "Milestone",
                HistoricalEventType::Founding => "Founding",
                HistoricalEventType::War => "War",
                HistoricalEventType::Discovery => "Discovery",
                HistoricalEventType::Catastrophe => "Catastrophe",
                HistoricalEventType::GoldenAge => "Golden Age",
                HistoricalEventType::Schism => "Schism",
                HistoricalEventType::Contact => "Contact",
                HistoricalEventType::Migration => "Migration",
                HistoricalEventType::DynastyChange => "Dynasty Change",
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

// ---------------------------------------------------------------------------
// World-history simulation (Dwarf-Fortress-lite)
//
// Entities (civilisations, settlements, figures, dynasties, artifacts) are
// created and destroyed by a year-by-year event loop. Each event records
// participants, cause, and effect. After the timeline is generated a
// retroactive "rationalisation" pass adds narrative descriptions.
// ---------------------------------------------------------------------------

/// Unique ID for any entity in the history.
pub type EntityId = u32;

/// A named civilisation that controls settlements.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Civilization {
    pub id: EntityId,
    pub name: String,
    pub founded_year: u32,
    pub collapsed_year: Option<u32>,
    pub settlements: Vec<EntityId>,
    pub tech_level: u8,
}

/// A named settlement belonging to a civilisation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoricSettlement {
    pub id: EntityId,
    pub name: String,
    pub civ_id: EntityId,
    pub founded_year: u32,
    pub destroyed_year: Option<u32>,
    pub tile_idx: usize,
}

/// A historical figure who participates in events.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoricalFigure {
    pub id: EntityId,
    pub name: String,
    pub civ_id: EntityId,
    pub birth_year: u32,
    pub death_year: Option<u32>,
}

/// A dynasty: a named lineage of rulers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dynasty {
    pub id: EntityId,
    pub name: String,
    pub civ_id: EntityId,
    pub founded_year: u32,
    pub ended_year: Option<u32>,
}

/// A legendary artifact created during a Golden Age or Discovery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
    pub id: EntityId,
    pub name: String,
    pub creator_id: EntityId,
    pub year: u32,
}

/// A single event in the world-history timeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEvent {
    pub year: u32,
    pub kind: HistoricalEventType,
    /// IDs of entities involved (civs, figures, settlements).
    pub participants: Vec<EntityId>,
    /// Machine-generated narrative explanation.
    pub description: String,
}

/// Complete world history produced by `simulate_history`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct History {
    pub events: Vec<WorldEvent>,
    pub civilizations: Vec<Civilization>,
    pub settlements: Vec<HistoricSettlement>,
    pub figures: Vec<HistoricalFigure>,
    pub dynasties: Vec<Dynasty>,
    pub artifacts: Vec<Artifact>,
}

/// Configuration for the world-history simulation.
#[derive(Clone, Debug)]
pub struct HistoryParams {
    /// Number of years to simulate.
    pub years: u32,
    /// Number of initial civilisations to seed.
    pub initial_civs: u8,
    /// Available tile indices where settlements can be placed.
    pub habitable_tiles: Vec<usize>,
}

impl Default for HistoryParams {
    fn default() -> Self {
        Self {
            years: 500,
            initial_civs: 3,
            habitable_tiles: (0..100).collect(),
        }
    }
}

/// Simulate `params.years` of world history, producing a deterministic
/// `History` from the given seed. Events are generated year-by-year; after
/// the timeline is complete a retroactive pass fills in narrative
/// descriptions (Caves-of-Qud style).
pub fn simulate_history(params: &HistoryParams, seed: &str) -> History {
    use crate::naming::{MarkovNameGen, NameStyle};

    let mut rng = SeededDiceRoller::new(seed, "world_history");
    let name_gen = MarkovNameGen::for_style(NameStyle::FantasyHuman);

    let mut history = History::default();
    let mut next_id: EntityId = 1;

    let mut alloc_id = || {
        let id = next_id;
        next_id += 1;
        id
    };

    // Seed initial civilisations.
    let n_civs = (params.initial_civs as usize).min(params.habitable_tiles.len());
    for i in 0..n_civs {
        let civ_id = alloc_id();
        let civ_name = name_gen.generate(&mut rng, 4, 9);

        // Each civ starts with one settlement and one founding figure.
        let sett_id = alloc_id();
        let tile = params.habitable_tiles[i % params.habitable_tiles.len()];
        let sett_name = name_gen.generate(&mut rng, 3, 8);

        let fig_id = alloc_id();
        let fig_name = name_gen.generate(&mut rng, 4, 9);

        let dyn_id = alloc_id();
        let dyn_name = name_gen.generate(&mut rng, 4, 8);

        history.civilizations.push(Civilization {
            id: civ_id,
            name: civ_name.clone(),
            founded_year: 0,
            collapsed_year: None,
            settlements: vec![sett_id],
            tech_level: 1,
        });
        history.settlements.push(HistoricSettlement {
            id: sett_id,
            name: sett_name,
            civ_id,
            founded_year: 0,
            destroyed_year: None,
            tile_idx: tile,
        });
        history.figures.push(HistoricalFigure {
            id: fig_id,
            name: fig_name,
            civ_id,
            birth_year: 0,
            death_year: None,
        });
        history.dynasties.push(Dynasty {
            id: dyn_id,
            name: dyn_name,
            civ_id,
            founded_year: 0,
            ended_year: None,
        });

        history.events.push(WorldEvent {
            year: 0,
            kind: HistoricalEventType::Founding,
            participants: vec![civ_id, sett_id, fig_id],
            description: String::new(),
        });
    }

    // Year-by-year simulation.
    for year in 1..=params.years {
        let active_civs: Vec<EntityId> = history
            .civilizations
            .iter()
            .filter(|c| c.collapsed_year.is_none())
            .map(|c| c.id)
            .collect();
        if active_civs.is_empty() {
            break;
        }

        // Each active civ has a chance of generating an event this year.
        for &civ_id in &active_civs {
            // ~15% chance per civ per year of something happening.
            if rng.gen_f64() > 0.15 {
                continue;
            }

            let roll = rng.roll(1, 10, 0);
            match roll {
                1 => {
                    // War: pick another active civ if any.
                    if active_civs.len() >= 2 {
                        let other = loop {
                            let idx = rng.gen_usize() % active_civs.len();
                            if active_civs[idx] != civ_id {
                                break active_civs[idx];
                            }
                        };
                        history.events.push(WorldEvent {
                            year,
                            kind: HistoricalEventType::War,
                            participants: vec![civ_id, other],
                            description: String::new(),
                        });
                    }
                }
                2 => {
                    // Discovery: tech level advances.
                    if let Some(civ) = history.civilizations.iter_mut().find(|c| c.id == civ_id) {
                        civ.tech_level = civ.tech_level.saturating_add(1).min(12);
                    }
                    history.events.push(WorldEvent {
                        year,
                        kind: HistoricalEventType::Discovery,
                        participants: vec![civ_id],
                        description: String::new(),
                    });
                }
                3 => {
                    // Catastrophe: destroy a random settlement.
                    let sett = history
                        .settlements
                        .iter_mut()
                        .find(|s| s.civ_id == civ_id && s.destroyed_year.is_none());
                    if let Some(s) = sett {
                        s.destroyed_year = Some(year);
                        let sid = s.id;
                        history.events.push(WorldEvent {
                            year,
                            kind: HistoricalEventType::Catastrophe,
                            participants: vec![civ_id, sid],
                            description: String::new(),
                        });
                    }
                }
                4 => {
                    // Golden Age: create an artifact.
                    let art_id = alloc_id();
                    let art_name = name_gen.generate(&mut rng, 4, 10);
                    history.artifacts.push(Artifact {
                        id: art_id,
                        name: art_name,
                        creator_id: civ_id,
                        year,
                    });
                    history.events.push(WorldEvent {
                        year,
                        kind: HistoricalEventType::GoldenAge,
                        participants: vec![civ_id, art_id],
                        description: String::new(),
                    });
                }
                5 => {
                    // Schism: civ splits, spawn a new civ.
                    let new_id = alloc_id();
                    let new_name = name_gen.generate(&mut rng, 4, 9);
                    let tile =
                        params.habitable_tiles[rng.gen_usize() % params.habitable_tiles.len()];
                    let sett_id = alloc_id();
                    let sett_name = name_gen.generate(&mut rng, 3, 8);
                    history.civilizations.push(Civilization {
                        id: new_id,
                        name: new_name,
                        founded_year: year,
                        collapsed_year: None,
                        settlements: vec![sett_id],
                        tech_level: history
                            .civilizations
                            .iter()
                            .find(|c| c.id == civ_id)
                            .map_or(1, |c| c.tech_level),
                    });
                    history.settlements.push(HistoricSettlement {
                        id: sett_id,
                        name: sett_name,
                        civ_id: new_id,
                        founded_year: year,
                        destroyed_year: None,
                        tile_idx: tile,
                    });
                    history.events.push(WorldEvent {
                        year,
                        kind: HistoricalEventType::Schism,
                        participants: vec![civ_id, new_id],
                        description: String::new(),
                    });
                }
                6 => {
                    // Migration: found a new settlement.
                    let sett_id = alloc_id();
                    let tile =
                        params.habitable_tiles[rng.gen_usize() % params.habitable_tiles.len()];
                    let sett_name = name_gen.generate(&mut rng, 3, 8);
                    history.settlements.push(HistoricSettlement {
                        id: sett_id,
                        name: sett_name,
                        civ_id,
                        founded_year: year,
                        destroyed_year: None,
                        tile_idx: tile,
                    });
                    if let Some(civ) = history.civilizations.iter_mut().find(|c| c.id == civ_id) {
                        civ.settlements.push(sett_id);
                    }
                    history.events.push(WorldEvent {
                        year,
                        kind: HistoricalEventType::Migration,
                        participants: vec![civ_id, sett_id],
                        description: String::new(),
                    });
                }
                7 => {
                    // Contact: two civs meet peacefully.
                    if active_civs.len() >= 2 {
                        let other = loop {
                            let idx = rng.gen_usize() % active_civs.len();
                            if active_civs[idx] != civ_id {
                                break active_civs[idx];
                            }
                        };
                        history.events.push(WorldEvent {
                            year,
                            kind: HistoricalEventType::Contact,
                            participants: vec![civ_id, other],
                            description: String::new(),
                        });
                    }
                }
                8 => {
                    // Dynasty change: new ruling figure.
                    let fig_id = alloc_id();
                    let fig_name = name_gen.generate(&mut rng, 4, 9);
                    history.figures.push(HistoricalFigure {
                        id: fig_id,
                        name: fig_name,
                        civ_id,
                        birth_year: year.saturating_sub(30),
                        death_year: None,
                    });
                    // End previous dynasty if any.
                    if let Some(dyn_) = history
                        .dynasties
                        .iter_mut()
                        .rev()
                        .find(|d| d.civ_id == civ_id && d.ended_year.is_none())
                    {
                        dyn_.ended_year = Some(year);
                    }
                    let dyn_id = alloc_id();
                    let dyn_name = name_gen.generate(&mut rng, 4, 8);
                    history.dynasties.push(Dynasty {
                        id: dyn_id,
                        name: dyn_name,
                        civ_id,
                        founded_year: year,
                        ended_year: None,
                    });
                    history.events.push(WorldEvent {
                        year,
                        kind: HistoricalEventType::DynastyChange,
                        participants: vec![civ_id, fig_id, dyn_id],
                        description: String::new(),
                    });
                }
                _ => {
                    // Founding: a new settlement in existing civ.
                    let sett_id = alloc_id();
                    let tile =
                        params.habitable_tiles[rng.gen_usize() % params.habitable_tiles.len()];
                    let sett_name = name_gen.generate(&mut rng, 3, 8);
                    history.settlements.push(HistoricSettlement {
                        id: sett_id,
                        name: sett_name,
                        civ_id,
                        founded_year: year,
                        destroyed_year: None,
                        tile_idx: tile,
                    });
                    if let Some(civ) = history.civilizations.iter_mut().find(|c| c.id == civ_id) {
                        civ.settlements.push(sett_id);
                    }
                    history.events.push(WorldEvent {
                        year,
                        kind: HistoricalEventType::Founding,
                        participants: vec![civ_id, sett_id],
                        description: String::new(),
                    });
                }
            }
        }

        // Kill off old figures (lifespan ~80 years).
        for fig in &mut history.figures {
            if fig.death_year.is_none() && year > fig.birth_year + 80 {
                fig.death_year = Some(year);
            }
        }

        // Collapse civs with no surviving settlements.
        for civ in &mut history.civilizations {
            if civ.collapsed_year.is_some() {
                continue;
            }
            let alive = history
                .settlements
                .iter()
                .any(|s| s.civ_id == civ.id && s.destroyed_year.is_none());
            if !alive {
                civ.collapsed_year = Some(year);
            }
        }
    }

    // Retroactive rationalisation: fill in narrative descriptions.
    rationalise_events(&mut history);
    history
}

/// Fill in `WorldEvent.description` based on participants and context.
/// This is the "Caves of Qud" style retroactive narrative — events were
/// generated mechanically, descriptions are synthesised afterwards.
fn rationalise_events(history: &mut History) {
    let civ_name = |id: EntityId| -> String {
        history
            .civilizations
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| format!("Civ#{}", id))
    };
    let sett_name = |id: EntityId| -> String {
        history
            .settlements
            .iter()
            .find(|s| s.id == id)
            .map(|s| s.name.clone())
            .unwrap_or_else(|| format!("Settlement#{}", id))
    };
    let fig_name = |id: EntityId| -> String {
        history
            .figures
            .iter()
            .find(|f| f.id == id)
            .map(|f| f.name.clone())
            .unwrap_or_else(|| format!("Figure#{}", id))
    };

    for event in &mut history.events {
        let p = &event.participants;
        event.description = match event.kind {
            HistoricalEventType::Founding => {
                if p.len() >= 3 {
                    format!(
                        "{} was founded by {} at {}",
                        sett_name(p[1]),
                        fig_name(p[2]),
                        civ_name(p[0])
                    )
                } else if p.len() >= 2 {
                    format!(
                        "The settlement of {} was established by {}",
                        sett_name(p[1]),
                        civ_name(p[0])
                    )
                } else {
                    "A new settlement was founded".into()
                }
            }
            HistoricalEventType::War => {
                if p.len() >= 2 {
                    format!("{} waged war against {}", civ_name(p[0]), civ_name(p[1]))
                } else {
                    "A great war broke out".into()
                }
            }
            HistoricalEventType::Discovery => {
                format!("{} made a great discovery", civ_name(p[0]))
            }
            HistoricalEventType::Catastrophe => {
                if p.len() >= 2 {
                    format!(
                        "A catastrophe struck {}, destroying {}",
                        civ_name(p[0]),
                        sett_name(p[1])
                    )
                } else {
                    "A terrible catastrophe occurred".into()
                }
            }
            HistoricalEventType::GoldenAge => {
                format!("{} entered a golden age", civ_name(p[0]))
            }
            HistoricalEventType::Schism => {
                if p.len() >= 2 {
                    format!("{} splintered from {}", civ_name(p[1]), civ_name(p[0]))
                } else {
                    "A civilisation fractured".into()
                }
            }
            HistoricalEventType::Contact => {
                if p.len() >= 2 {
                    format!(
                        "{} established contact with {}",
                        civ_name(p[0]),
                        civ_name(p[1])
                    )
                } else {
                    "First contact was made".into()
                }
            }
            HistoricalEventType::Migration => {
                if p.len() >= 2 {
                    format!(
                        "People of {} migrated to found {}",
                        civ_name(p[0]),
                        sett_name(p[1])
                    )
                } else {
                    "A great migration took place".into()
                }
            }
            HistoricalEventType::DynastyChange => {
                if p.len() >= 2 {
                    format!("{} rose to power in {}", fig_name(p[1]), civ_name(p[0]))
                } else {
                    "A new dynasty took power".into()
                }
            }
            _ => "An event occurred".into(),
        };
    }
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

    // --- World-history simulation tests ---

    #[test]
    fn history_sim_is_deterministic() {
        let params = HistoryParams::default();
        let a = simulate_history(&params, "det");
        let b = simulate_history(&params, "det");
        assert_eq!(a.events.len(), b.events.len());
        assert_eq!(a.civilizations.len(), b.civilizations.len());
        for (x, y) in a.events.iter().zip(b.events.iter()) {
            assert_eq!(x.year, y.year);
            assert_eq!(x.kind, y.kind);
        }
    }

    #[test]
    fn history_length_scales_with_years() {
        let short = simulate_history(
            &HistoryParams {
                years: 50,
                ..Default::default()
            },
            "short",
        );
        let long = simulate_history(
            &HistoryParams {
                years: 300,
                ..Default::default()
            },
            "long",
        );
        assert!(
            long.events.len() > short.events.len(),
            "long {} <= short {}",
            long.events.len(),
            short.events.len()
        );
    }

    #[test]
    fn history_has_founding_events() {
        let h = simulate_history(&HistoryParams::default(), "founding");
        let foundings = h
            .events
            .iter()
            .filter(|e| e.kind == HistoricalEventType::Founding)
            .count();
        assert!(foundings >= 1, "no founding events");
    }

    #[test]
    fn history_events_are_chronological() {
        let h = simulate_history(
            &HistoryParams {
                years: 200,
                ..Default::default()
            },
            "chrono",
        );
        for w in h.events.windows(2) {
            assert!(
                w[0].year <= w[1].year,
                "event at year {} followed by year {}",
                w[0].year,
                w[1].year
            );
        }
    }

    #[test]
    fn history_descriptions_are_filled() {
        let h = simulate_history(&HistoryParams::default(), "desc");
        for event in &h.events {
            assert!(
                !event.description.is_empty(),
                "event {:?} at year {} has empty description",
                event.kind,
                event.year
            );
        }
    }

    #[test]
    fn history_entities_have_unique_ids() {
        let h = simulate_history(&HistoryParams::default(), "ids");
        let mut ids = std::collections::HashSet::new();
        for c in &h.civilizations {
            assert!(ids.insert(c.id), "duplicate civ id {}", c.id);
        }
        for s in &h.settlements {
            assert!(ids.insert(s.id), "duplicate settlement id {}", s.id);
        }
        for f in &h.figures {
            assert!(ids.insert(f.id), "duplicate figure id {}", f.id);
        }
        for d in &h.dynasties {
            assert!(ids.insert(d.id), "duplicate dynasty id {}", d.id);
        }
        for a in &h.artifacts {
            assert!(ids.insert(a.id), "duplicate artifact id {}", a.id);
        }
    }

    #[test]
    fn no_time_paradoxes() {
        let h = simulate_history(
            &HistoryParams {
                years: 500,
                ..Default::default()
            },
            "paradox",
        );
        // Settlements cannot be destroyed before they are founded.
        for s in &h.settlements {
            if let Some(dy) = s.destroyed_year {
                assert!(
                    dy >= s.founded_year,
                    "settlement {} destroyed at {} before founding at {}",
                    s.name,
                    dy,
                    s.founded_year
                );
            }
        }
        // Figures cannot die before they are born.
        for f in &h.figures {
            if let Some(dy) = f.death_year {
                assert!(
                    dy >= f.birth_year,
                    "figure {} died at {} before birth at {}",
                    f.name,
                    dy,
                    f.birth_year
                );
            }
        }
        // Dynasties cannot end before they are founded.
        for d in &h.dynasties {
            if let Some(ey) = d.ended_year {
                assert!(
                    ey >= d.founded_year,
                    "dynasty {} ended at {} before founding at {}",
                    d.name,
                    ey,
                    d.founded_year
                );
            }
        }
    }
}
