//! Resource scanning and discovery — fog-of-war for planetary data.
//!
//! The player starts with all tiles `Unknown`. Scanning progressively
//! reveals data: surface scan shows biome/elevation/surface resources,
//! deep scan reveals underground strata/ore nodes/caves, and full mapping
//! unlocks fluid reservoirs and precise quantities.
//!
//! The game engine calls `scan_tile` / `scan_region` to advance a tile's
//! state, then queries `is_visible` to decide what to show the player.

/// How thoroughly a tile has been surveyed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScanState {
    /// No data — tile is hidden.
    #[default]
    Unknown,
    /// Orbital/surface scan: biome, elevation, temperature, surface resources.
    SurfaceScan,
    /// Deep geological scan: underground strata, ore nodes, caves.
    DeepScan,
    /// Fully mapped: fluid reservoirs, precise quantities, everything visible.
    FullyMapped,
}

impl ScanState {
    /// Whether surface-level data (biome, elevation) is revealed.
    pub fn surface_visible(self) -> bool {
        self >= ScanState::SurfaceScan
    }

    /// Whether underground data (strata, ore, caves) is revealed.
    pub fn underground_visible(self) -> bool {
        self >= ScanState::DeepScan
    }

    /// Whether all data (fluids, exact quantities) is revealed.
    pub fn fully_mapped(self) -> bool {
        self == ScanState::FullyMapped
    }
}

/// Per-tile scan state for an entire grid.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScanMap {
    pub states: Vec<ScanState>,
}

impl ScanMap {
    /// Create a fresh scan map with all tiles Unknown.
    pub fn new(tile_count: usize) -> Self {
        Self {
            states: vec![ScanState::Unknown; tile_count],
        }
    }

    /// Advance a single tile to the given scan level (never downgrades).
    pub fn scan_tile(&mut self, tile_idx: usize, level: ScanState) {
        if let Some(state) = self.states.get_mut(tile_idx) {
            if level > *state {
                *state = level;
            }
        }
    }

    /// Advance all tiles in a rectangular region (col_min..col_max,
    /// row_min..row_max) to the given level. Wraps longitude.
    pub fn scan_region(
        &mut self,
        width: u16,
        col_min: u16,
        col_max: u16,
        row_min: u16,
        row_max: u16,
        level: ScanState,
    ) {
        let w = width as usize;
        let h = self.states.len() / w;
        for r in row_min as usize..=(row_max as usize).min(h.saturating_sub(1)) {
            let mut c = col_min;
            loop {
                let idx = r * w + (c % width) as usize;
                self.scan_tile(idx, level);
                if c == col_max {
                    break;
                }
                c = (c + 1) % width;
            }
        }
    }

    /// How many tiles are at each scan level.
    pub fn counts(&self) -> (usize, usize, usize, usize) {
        let mut unknown = 0;
        let mut surface = 0;
        let mut deep = 0;
        let mut full = 0;
        for &s in &self.states {
            match s {
                ScanState::Unknown => unknown += 1,
                ScanState::SurfaceScan => surface += 1,
                ScanState::DeepScan => deep += 1,
                ScanState::FullyMapped => full += 1,
            }
        }
        (unknown, surface, deep, full)
    }

    /// Fraction of tiles that have been scanned at least to surface level.
    pub fn explored_fraction(&self) -> f32 {
        let revealed = self.states.iter().filter(|s| s.surface_visible()).count();
        revealed as f32 / self.states.len().max(1) as f32
    }

    /// Tile indices that are still Unknown.
    pub fn unexplored_tiles(&self) -> Vec<usize> {
        self.states
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == ScanState::Unknown)
            .map(|(i, _)| i)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_map_is_all_unknown() {
        let sm = ScanMap::new(100);
        let (unk, surf, deep, full) = sm.counts();
        assert_eq!(unk, 100);
        assert_eq!(surf, 0);
        assert_eq!(deep, 0);
        assert_eq!(full, 0);
        assert_eq!(sm.explored_fraction(), 0.0);
    }

    #[test]
    fn scan_tile_advances_state() {
        let mut sm = ScanMap::new(10);
        sm.scan_tile(3, ScanState::SurfaceScan);
        assert_eq!(sm.states[3], ScanState::SurfaceScan);
        assert!(sm.states[3].surface_visible());
        assert!(!sm.states[3].underground_visible());
    }

    #[test]
    fn scan_never_downgrades() {
        let mut sm = ScanMap::new(10);
        sm.scan_tile(5, ScanState::DeepScan);
        sm.scan_tile(5, ScanState::SurfaceScan); // should not downgrade
        assert_eq!(sm.states[5], ScanState::DeepScan);
    }

    #[test]
    fn scan_region_advances_rectangle() {
        let mut sm = ScanMap::new(72 * 36); // Fast grid
        sm.scan_region(72, 10, 15, 5, 8, ScanState::SurfaceScan);
        // Tiles in the region should be scanned.
        let idx = 5 * 72 + 12;
        assert_eq!(sm.states[idx], ScanState::SurfaceScan);
        // Tile outside region should be Unknown.
        let outside = 0; // row 0, col 0
        assert_eq!(sm.states[outside], ScanState::Unknown);
    }

    #[test]
    fn explored_fraction_correct() {
        let mut sm = ScanMap::new(100);
        for i in 0..25 {
            sm.scan_tile(i, ScanState::SurfaceScan);
        }
        assert!((sm.explored_fraction() - 0.25).abs() < 0.01);
    }

    #[test]
    fn unexplored_tiles_shrinks() {
        let mut sm = ScanMap::new(10);
        assert_eq!(sm.unexplored_tiles().len(), 10);
        sm.scan_tile(3, ScanState::FullyMapped);
        sm.scan_tile(7, ScanState::DeepScan);
        assert_eq!(sm.unexplored_tiles().len(), 8);
    }

    #[test]
    fn fully_mapped_reveals_everything() {
        let state = ScanState::FullyMapped;
        assert!(state.surface_visible());
        assert!(state.underground_visible());
        assert!(state.fully_mapped());
    }

    #[test]
    fn surface_scan_hides_underground() {
        let state = ScanState::SurfaceScan;
        assert!(state.surface_visible());
        assert!(!state.underground_visible());
        assert!(!state.fully_mapped());
    }

    #[test]
    fn out_of_bounds_scan_is_safe() {
        let mut sm = ScanMap::new(5);
        sm.scan_tile(999, ScanState::FullyMapped);
        assert_eq!(sm.counts().3, 0); // no tile changed
    }
}
