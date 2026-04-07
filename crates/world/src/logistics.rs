//! Inter-base logistics — transport modes and cost calculation.
//!
//! Moving materials between bases across a planet has a cost that depends
//! on distance and transport mode. The player unlocks faster modes at
//! higher tech tiers.

/// Available transport modes, unlocked at different tech stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TransportMode {
    /// Manual ground convoy. Slow, cheap, always available.
    Hauler,
    /// Motorised truck/train. Medium speed, medium cost.
    Motorised,
    /// Aerial cargo drone. Fast, expensive, tech-gated.
    CargoDrone,
    /// Fixed pipeline for fluids only. High upfront, zero per-trip cost.
    Pipeline,
    /// Orbital shuttle. Very fast, very expensive, late-game.
    OrbitalShuttle,
}

impl TransportMode {
    /// Speed in tiles per hour.
    pub fn speed_tiles_per_hour(self) -> f32 {
        match self {
            TransportMode::Hauler => 0.5,
            TransportMode::Motorised => 2.0,
            TransportMode::CargoDrone => 8.0,
            TransportMode::Pipeline => f32::INFINITY, // instantaneous
            TransportMode::OrbitalShuttle => 20.0,
        }
    }

    /// Cost per tile-km in arbitrary currency units.
    pub fn cost_per_tile(self) -> f32 {
        match self {
            TransportMode::Hauler => 1.0,
            TransportMode::Motorised => 2.5,
            TransportMode::CargoDrone => 8.0,
            TransportMode::Pipeline => 0.0, // no per-trip cost (upfront only)
            TransportMode::OrbitalShuttle => 15.0,
        }
    }

    /// Cargo capacity in kilotonnes per trip.
    pub fn capacity_kt(self) -> f32 {
        match self {
            TransportMode::Hauler => 0.01,
            TransportMode::Motorised => 0.1,
            TransportMode::CargoDrone => 0.05,
            TransportMode::Pipeline => 1.0, // continuous flow
            TransportMode::OrbitalShuttle => 0.5,
        }
    }

    /// Whether this mode can carry solid materials (not just fluids).
    pub fn carries_solids(self) -> bool {
        self != TransportMode::Pipeline
    }

    /// Whether this mode can carry fluids.
    pub fn carries_fluids(self) -> bool {
        true // all modes can carry fluids
    }
}

/// Result of a logistics calculation between two points.
#[derive(Clone, Debug)]
pub struct LogisticsResult {
    pub mode: TransportMode,
    /// Distance in tiles (Chebyshev with longitude wrap).
    pub distance_tiles: u16,
    /// Total transport cost for one trip.
    pub trip_cost: f32,
    /// Travel time in hours for one trip.
    pub travel_hours: f32,
    /// Trips needed to move the requested amount.
    pub trips_needed: u32,
    /// Total cost for all trips.
    pub total_cost: f32,
    /// Total time for all trips (sequential).
    pub total_hours: f32,
}

/// Calculate logistics for moving material between two tiles.
///
/// - `from`, `to`: tile indices.
/// - `grid_width`, `grid_height`: grid dimensions for distance calculation.
/// - `amount_kt`: kilotonnes to transport.
/// - `mode`: transport mode to use.
pub fn calculate_logistics(
    from: usize,
    to: usize,
    grid_width: u16,
    grid_height: u16,
    amount_kt: f32,
    mode: TransportMode,
) -> LogisticsResult {
    let w = grid_width as usize;
    let h = grid_height as usize;
    let r1 = from / w;
    let c1 = from % w;
    let r2 = to / w;
    let c2 = to % w;

    // Chebyshev distance with longitude wrap.
    let dr = (r1 as i32 - r2 as i32).unsigned_abs() as u16;
    let dc_raw = (c1 as i32 - c2 as i32).unsigned_abs() as u16;
    let dc = dc_raw.min(grid_width - dc_raw);
    let distance = dr.max(dc);

    let trip_cost = distance as f32 * mode.cost_per_tile();
    let speed = mode.speed_tiles_per_hour();
    let travel_hours = if speed.is_infinite() {
        0.0
    } else {
        distance as f32 / speed
    };

    let capacity = mode.capacity_kt();
    let trips_needed = if capacity <= 0.0 {
        0
    } else {
        (amount_kt / capacity).ceil() as u32
    };

    let total_cost = trip_cost * trips_needed as f32;
    let total_hours = travel_hours * trips_needed as f32;

    LogisticsResult {
        mode,
        distance_tiles: distance,
        trip_cost,
        travel_hours,
        trips_needed,
        total_cost,
        total_hours,
    }
}

/// Compare all transport modes for a route and return sorted by total cost.
pub fn compare_modes(
    from: usize,
    to: usize,
    grid_width: u16,
    grid_height: u16,
    amount_kt: f32,
    is_fluid: bool,
) -> Vec<LogisticsResult> {
    let modes = if is_fluid {
        vec![
            TransportMode::Hauler,
            TransportMode::Motorised,
            TransportMode::CargoDrone,
            TransportMode::Pipeline,
            TransportMode::OrbitalShuttle,
        ]
    } else {
        vec![
            TransportMode::Hauler,
            TransportMode::Motorised,
            TransportMode::CargoDrone,
            TransportMode::OrbitalShuttle,
        ]
    };

    let mut results: Vec<LogisticsResult> = modes
        .into_iter()
        .map(|mode| calculate_logistics(from, to, grid_width, grid_height, amount_kt, mode))
        .collect();

    results.sort_by(|a, b| {
        a.total_cost
            .partial_cmp(&b.total_cost)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hauler_is_cheapest_per_tile() {
        assert!(TransportMode::Hauler.cost_per_tile() < TransportMode::CargoDrone.cost_per_tile());
    }

    #[test]
    fn drone_is_faster_than_hauler() {
        assert!(
            TransportMode::CargoDrone.speed_tiles_per_hour()
                > TransportMode::Hauler.speed_tiles_per_hour()
        );
    }

    #[test]
    fn pipeline_only_for_fluids() {
        assert!(!TransportMode::Pipeline.carries_solids());
        assert!(TransportMode::Pipeline.carries_fluids());
    }

    #[test]
    fn cost_scales_with_distance() {
        let near = calculate_logistics(0, 5, 72, 36, 1.0, TransportMode::Hauler);
        let far = calculate_logistics(0, 35, 72, 36, 1.0, TransportMode::Hauler);
        assert!(far.total_cost > near.total_cost);
    }

    #[test]
    fn more_cargo_needs_more_trips() {
        let small = calculate_logistics(0, 10, 72, 36, 0.005, TransportMode::Hauler);
        let large = calculate_logistics(0, 10, 72, 36, 0.1, TransportMode::Hauler);
        assert!(large.trips_needed > small.trips_needed);
    }

    #[test]
    fn pipeline_has_zero_trip_cost() {
        let r = calculate_logistics(0, 100, 72, 36, 10.0, TransportMode::Pipeline);
        assert_eq!(r.trip_cost, 0.0);
        assert_eq!(r.total_cost, 0.0);
        assert_eq!(r.travel_hours, 0.0);
    }

    #[test]
    fn longitude_wrap_shortens_distance() {
        // Tile 0 (col 0) to tile 71 (col 71) should wrap to distance 1.
        let r = calculate_logistics(0, 71, 72, 36, 1.0, TransportMode::Hauler);
        assert_eq!(r.distance_tiles, 1);
    }

    #[test]
    fn compare_modes_returns_sorted_by_cost() {
        let results = compare_modes(0, 50, 72, 36, 1.0, false);
        for w in results.windows(2) {
            assert!(w[0].total_cost <= w[1].total_cost);
        }
    }

    #[test]
    fn compare_modes_includes_pipeline_for_fluids() {
        let results = compare_modes(0, 50, 72, 36, 1.0, true);
        assert!(results.iter().any(|r| r.mode == TransportMode::Pipeline));
    }

    #[test]
    fn compare_modes_excludes_pipeline_for_solids() {
        let results = compare_modes(0, 50, 72, 36, 1.0, false);
        assert!(!results.iter().any(|r| r.mode == TransportMode::Pipeline));
    }
}
