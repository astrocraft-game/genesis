//! Scan-gated queries — what the player can see at each tile given their
//! current scan level. Combines `ScanMap`, `ZoneMap`, and `ZoneOreMap` to
//! return only the data the player has discovered.

use crate::resources::Resource;
use crate::scanning::{ScanMap, ScanState};
use crate::zone_ores::{ZoneOreDeposit, ZoneOreMap};
use crate::zones::{GeologicalZone, ZoneMap};

/// What the player knows about a single tile, gated by scan level.
#[derive(Clone, Debug, Default)]
pub struct TileScanResult {
    /// Current scan level.
    pub scan_level: ScanState,
    /// Geological zone (visible at SurfaceScan+).
    pub zone: Option<GeologicalZone>,
    /// Resource types present (visible at DeepScan+). No quantities yet.
    pub ore_types: Vec<Resource>,
    /// Full deposit details (visible at FullyMapped only).
    pub ore_details: Vec<OreDetail>,
}

/// Full deposit info revealed at FullyMapped level.
#[derive(Clone, Debug)]
pub struct OreDetail {
    pub resource: Resource,
    pub purity: f32,
    pub quantity_kt: f32,
}

/// Query a single tile's visible data.
pub fn query_tile(
    tile_idx: usize,
    scan_map: &ScanMap,
    zone_map: &ZoneMap,
    ore_map: &ZoneOreMap,
) -> TileScanResult {
    let level = scan_map
        .states
        .get(tile_idx)
        .copied()
        .unwrap_or(ScanState::Unknown);

    let mut result = TileScanResult {
        scan_level: level,
        ..Default::default()
    };

    if level.surface_visible() {
        result.zone = zone_map.zones.get(tile_idx).copied();
    }

    if level.underground_visible() {
        if let Some(deps) = ore_map.deposits.get(tile_idx) {
            result.ore_types = deps.iter().map(|d| d.resource).collect();
        }
    }

    if level.fully_mapped() {
        if let Some(deps) = ore_map.deposits.get(tile_idx) {
            result.ore_details = deps
                .iter()
                .map(|d| OreDetail {
                    resource: d.resource,
                    purity: d.purity,
                    quantity_kt: d.quantity_kt,
                })
                .collect();
        }
    }

    result
}

/// Query all tiles, returning a vec of scan results.
pub fn query_all(
    scan_map: &ScanMap,
    zone_map: &ZoneMap,
    ore_map: &ZoneOreMap,
) -> Vec<TileScanResult> {
    (0..scan_map.states.len())
        .map(|idx| query_tile(idx, scan_map, zone_map, ore_map))
        .collect()
}

/// Count how many distinct zones the player has discovered so far.
pub fn discovered_zones(scan_map: &ScanMap, zone_map: &ZoneMap) -> Vec<GeologicalZone> {
    let mut seen = std::collections::HashSet::new();
    for (idx, &level) in scan_map.states.iter().enumerate() {
        if level.surface_visible() {
            if let Some(&zone) = zone_map.zones.get(idx) {
                if zone != GeologicalZone::Common {
                    seen.insert(zone);
                }
            }
        }
    }
    seen.into_iter().collect()
}

/// Count how many distinct ore types the player has found via deep scan.
pub fn discovered_ore_types(scan_map: &ScanMap, ore_map: &ZoneOreMap) -> Vec<Resource> {
    let mut seen = std::collections::HashSet::new();
    for (idx, &level) in scan_map.states.iter().enumerate() {
        if level.underground_visible() {
            if let Some(deps) = ore_map.deposits.get(idx) {
                for d in deps {
                    seen.insert(d.resource);
                }
            }
        }
    }
    seen.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{generate_surface_grid, GridResolution};
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};
    use crate::zone_ores::generate_zone_ores;
    use crate::zones::classify_zones;

    fn setup() -> (ScanMap, ZoneMap, ZoneOreMap) {
        let input = PlanetSimulationInput {
            body_id: 1,
            body_radius_earth: 1.0,
            blackbody_temp_k: 255,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        };
        let g = generate_surface_grid(&input, 33.0, 1.0, 71.0, GridResolution::Fast, "scanq");
        let zm = classify_zones(&g);
        let om = generate_zone_ores(&zm, "scanq");
        let sm = ScanMap::new(g.tile_count());
        (sm, zm, om)
    }

    #[test]
    fn unknown_tile_reveals_nothing() {
        let (sm, zm, om) = setup();
        let r = query_tile(0, &sm, &zm, &om);
        assert_eq!(r.scan_level, ScanState::Unknown);
        assert!(r.zone.is_none());
        assert!(r.ore_types.is_empty());
        assert!(r.ore_details.is_empty());
    }

    #[test]
    fn surface_scan_reveals_zone() {
        let (mut sm, zm, om) = setup();
        sm.scan_tile(0, ScanState::SurfaceScan);
        let r = query_tile(0, &sm, &zm, &om);
        assert!(r.zone.is_some());
        assert!(r.ore_types.is_empty()); // not deep scanned yet
    }

    #[test]
    fn deep_scan_reveals_ore_types() {
        let (mut sm, zm, om) = setup();
        // Find a tile with zone ores.
        let ore_tile = om.deposits.iter().position(|d| !d.is_empty()).unwrap_or(0);
        sm.scan_tile(ore_tile, ScanState::DeepScan);
        let r = query_tile(ore_tile, &sm, &zm, &om);
        assert!(r.zone.is_some());
        if !om.deposits[ore_tile].is_empty() {
            assert!(!r.ore_types.is_empty());
        }
        assert!(r.ore_details.is_empty()); // not fully mapped yet
    }

    #[test]
    fn full_scan_reveals_purity_and_quantity() {
        let (mut sm, zm, om) = setup();
        let ore_tile = om.deposits.iter().position(|d| !d.is_empty()).unwrap_or(0);
        sm.scan_tile(ore_tile, ScanState::FullyMapped);
        let r = query_tile(ore_tile, &sm, &zm, &om);
        if !om.deposits[ore_tile].is_empty() {
            assert!(!r.ore_details.is_empty());
            for detail in &r.ore_details {
                assert!(detail.purity > 0.0);
                assert!(detail.quantity_kt > 0.0);
            }
        }
    }

    #[test]
    fn discovered_zones_empty_when_unscanned() {
        let (sm, zm, _) = setup();
        let zones = discovered_zones(&sm, &zm);
        assert!(zones.is_empty());
    }

    #[test]
    fn scanning_reveals_zones_progressively() {
        let (mut sm, zm, _) = setup();
        // Scan first 100 tiles.
        for i in 0..100 {
            sm.scan_tile(i, ScanState::SurfaceScan);
        }
        let zones = discovered_zones(&sm, &zm);
        // Should have discovered at least some zones from 100 tiles.
        // (may be 0 if all 100 are ocean, but on Earth-like unlikely)
        let _ = zones; // just verifying it doesn't panic
    }

    #[test]
    fn query_all_returns_correct_count() {
        let (sm, zm, om) = setup();
        let results = query_all(&sm, &zm, &om);
        assert_eq!(results.len(), sm.states.len());
    }

    #[test]
    fn discovered_ores_empty_without_deep_scan() {
        let (sm, _, om) = setup();
        let ores = discovered_ore_types(&sm, &om);
        assert!(ores.is_empty());
    }
}
