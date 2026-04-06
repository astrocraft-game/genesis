//! Terrain mutability ledger — tracks cumulative changes per tile.
//!
//! The factory simulation writes entries into the ledger as the player
//! mines, dumps waste, clears vegetation, or triggers erosion events.
//! The ledger enables "before/after" comparison and undo-style queries.

/// Category of terrain modification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum ChangeKind {
    /// Ore or rock removed from a geological layer.
    Mining,
    /// Material deposited (tailings, fill, waste).
    Dumping,
    /// Vegetation removed (forest clearing, harvesting).
    Deforestation,
    /// Natural or player-caused erosion event.
    Erosion,
    /// Construction placed on tile (factory building, road).
    Construction,
    /// Pollution level change.
    Pollution,
    /// Flooding (natural or dam-related).
    Flooding,
    /// Terraforming (deliberate large-scale reshaping).
    Terraforming,
}

/// A single timestamped change entry.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeEntry {
    pub kind: ChangeKind,
    /// Game tick or year when the change occurred.
    pub tick: u64,
    /// Signed magnitude: positive = added, negative = removed.
    /// Units depend on kind (metres for Mining/Dumping, kg for Pollution,
    /// fraction for Deforestation).
    pub magnitude: f32,
    /// Optional description (e.g., "Extracted 50 kt iron ore").
    pub note: String,
}

/// Per-tile change history.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TileLog {
    entries: Vec<ChangeEntry>,
}

impl TileLog {
    /// Record a new change.
    pub fn record(&mut self, kind: ChangeKind, tick: u64, magnitude: f32, note: impl Into<String>) {
        self.entries.push(ChangeEntry {
            kind,
            tick,
            magnitude,
            note: note.into(),
        });
    }

    /// All entries in chronological order.
    pub fn entries(&self) -> &[ChangeEntry] {
        &self.entries
    }

    /// Number of recorded changes.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Sum of magnitudes for a given change kind.
    pub fn total_magnitude(&self, kind: ChangeKind) -> f32 {
        self.entries
            .iter()
            .filter(|e| e.kind == kind)
            .map(|e| e.magnitude)
            .sum()
    }

    /// Entries filtered to a specific kind.
    pub fn entries_of(&self, kind: ChangeKind) -> Vec<&ChangeEntry> {
        self.entries.iter().filter(|e| e.kind == kind).collect()
    }

    /// Most recent tick with any recorded change, or `None` if empty.
    pub fn last_tick(&self) -> Option<u64> {
        self.entries.last().map(|e| e.tick)
    }
}

/// Terrain ledger for an entire grid.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TerrainLog {
    pub tiles: Vec<TileLog>,
}

impl TerrainLog {
    /// Create an empty ledger for `n` tiles.
    pub fn new(tile_count: usize) -> Self {
        Self {
            tiles: vec![TileLog::default(); tile_count],
        }
    }

    /// Record a change at a specific tile.
    pub fn record(
        &mut self,
        tile_idx: usize,
        kind: ChangeKind,
        tick: u64,
        magnitude: f32,
        note: impl Into<String>,
    ) {
        if let Some(log) = self.tiles.get_mut(tile_idx) {
            log.record(kind, tick, magnitude, note);
        }
    }

    /// Total number of entries across all tiles.
    pub fn total_entries(&self) -> usize {
        self.tiles.iter().map(|t| t.len()).sum()
    }

    /// Number of tiles that have been modified at least once.
    pub fn modified_tile_count(&self) -> usize {
        self.tiles.iter().filter(|t| !t.is_empty()).count()
    }

    /// Summary: total magnitude of a given change kind across all tiles.
    pub fn global_total(&self, kind: ChangeKind) -> f32 {
        self.tiles.iter().map(|t| t.total_magnitude(kind)).sum()
    }

    /// Collect all tile indices that have been modified.
    pub fn modified_tiles(&self) -> Vec<usize> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| !t.is_empty())
            .map(|(i, _)| i)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_ledger_is_empty() {
        let log = TerrainLog::new(100);
        assert_eq!(log.total_entries(), 0);
        assert_eq!(log.modified_tile_count(), 0);
        for tile in &log.tiles {
            assert!(tile.is_empty());
        }
    }

    #[test]
    fn record_accumulates_entries() {
        let mut log = TerrainLog::new(10);
        log.record(3, ChangeKind::Mining, 1, -50.0, "Iron ore extracted");
        log.record(3, ChangeKind::Mining, 2, -30.0, "More iron");
        log.record(5, ChangeKind::Dumping, 2, 20.0, "Tailings deposited");

        assert_eq!(log.total_entries(), 3);
        assert_eq!(log.modified_tile_count(), 2);
        assert_eq!(log.tiles[3].len(), 2);
        assert_eq!(log.tiles[5].len(), 1);
    }

    #[test]
    fn total_magnitude_per_kind() {
        let mut log = TerrainLog::new(5);
        log.record(0, ChangeKind::Mining, 1, -100.0, "");
        log.record(0, ChangeKind::Mining, 2, -50.0, "");
        log.record(0, ChangeKind::Dumping, 3, 30.0, "");

        assert_eq!(log.tiles[0].total_magnitude(ChangeKind::Mining), -150.0);
        assert_eq!(log.tiles[0].total_magnitude(ChangeKind::Dumping), 30.0);
        assert_eq!(log.tiles[0].total_magnitude(ChangeKind::Deforestation), 0.0);
    }

    #[test]
    fn global_total_across_tiles() {
        let mut log = TerrainLog::new(3);
        log.record(0, ChangeKind::Pollution, 1, 10.0, "");
        log.record(1, ChangeKind::Pollution, 1, 20.0, "");
        log.record(2, ChangeKind::Pollution, 2, 5.0, "");

        assert_eq!(log.global_total(ChangeKind::Pollution), 35.0);
        assert_eq!(log.global_total(ChangeKind::Mining), 0.0);
    }

    #[test]
    fn entries_of_filters_by_kind() {
        let mut log = TileLog::default();
        log.record(ChangeKind::Mining, 1, -10.0, "a");
        log.record(ChangeKind::Dumping, 2, 5.0, "b");
        log.record(ChangeKind::Mining, 3, -20.0, "c");

        let mining = log.entries_of(ChangeKind::Mining);
        assert_eq!(mining.len(), 2);
        assert_eq!(mining[0].magnitude, -10.0);
        assert_eq!(mining[1].magnitude, -20.0);
    }

    #[test]
    fn last_tick_tracks_most_recent() {
        let mut log = TileLog::default();
        assert!(log.last_tick().is_none());
        log.record(ChangeKind::Construction, 10, 1.0, "");
        assert_eq!(log.last_tick(), Some(10));
        log.record(ChangeKind::Construction, 25, 1.0, "");
        assert_eq!(log.last_tick(), Some(25));
    }

    #[test]
    fn modified_tiles_returns_indices() {
        let mut log = TerrainLog::new(10);
        log.record(2, ChangeKind::Mining, 1, -1.0, "");
        log.record(7, ChangeKind::Deforestation, 1, -0.5, "");

        let modified = log.modified_tiles();
        assert_eq!(modified, vec![2, 7]);
    }

    #[test]
    fn out_of_bounds_record_is_safe() {
        let mut log = TerrainLog::new(5);
        // Should not panic.
        log.record(999, ChangeKind::Mining, 1, -1.0, "");
        assert_eq!(log.total_entries(), 0);
    }
}
