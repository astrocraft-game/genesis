pub use crate::types::{DeltaType, Hydrography, LakeDistribution, LakeFormationType, LiquidType};
use crate::types::{OrbitContext, PlanetSimulationInput};

/// Generate a simple hydrography profile with Hadley-cell zonal rainfall.
///
/// The model is deliberately coarse — three latitude bands
/// (equatorial/mid-latitude/polar) with rainfall redistributed by axial tilt.
/// Low-tilt worlds concentrate rain in the equatorial belt (ITCZ); high-tilt
/// worlds smear precipitation across latitudes via strong seasonal migration.
///
/// Returns `None` for bodies with no hydrosphere or no atmosphere.
pub fn generate_hydrography(
    context: &PlanetSimulationInput,
    atmospheric_pressure: f32,
    hydrosphere_pct: f32,
) -> Option<Hydrography> {
    if hydrosphere_pct <= 0.0 || atmospheric_pressure < 0.05 {
        return None;
    }

    // Global mean precipitation scales with available surface water, a
    // moisture-carrying atmosphere, and temperature (Clausius-Clapeyron
    // roughly doubles water vapour per 10 K above 273 K).
    let temp_k = context.blackbody_temp_k as f32;
    let water_factor = (hydrosphere_pct / 100.0).clamp(0.05, 1.0);
    let pressure_factor = atmospheric_pressure.clamp(0.1, 5.0).sqrt();
    let temp_factor = ((temp_k - 250.0) / 35.0).clamp(0.2, 3.0);
    let mean_precipitation_mm =
        (1000.0 * water_factor * pressure_factor * temp_factor).clamp(50.0, 4000.0);

    let (cells, zonal) = zonal_distribution(&context.orbit, mean_precipitation_mm);

    // River count scales with land area available and water throughput.
    let land_fraction = (100.0 - hydrosphere_pct).max(0.0) / 100.0;
    let major_river_count =
        ((mean_precipitation_mm / 100.0) * land_fraction * 12.0).clamp(0.0, 80.0) as u32;
    let longest_river_km = (mean_precipitation_mm * land_fraction * 3.5).clamp(0.0, 7000.0);

    Some(Hydrography {
        major_river_count,
        longest_river_km,
        mean_precipitation_mm,
        zonal_precipitation_mm: zonal,
        hadley_cells_per_hemisphere: cells,
        dominant_delta_type: if major_river_count > 20 {
            DeltaType::Arcuate
        } else {
            DeltaType::None
        },
    })
}

/// Redistributes the global mean precipitation across three zonal bands and
/// returns the atmospheric-cell count per hemisphere. All values in mm/yr.
fn zonal_distribution(orbit: &OrbitContext, mean_mm: f32) -> (u8, [f32; 3]) {
    let tilt = orbit.axial_tilt_deg.abs();

    // Cell count per hemisphere:
    //   tilt < 8°   → 1 (single merged cell, strong equator-to-pole transport)
    //   tilt 8-40°  → 3 (Hadley + Ferrel + Polar, Earth-like)
    //   tilt 40-54° → 2 (Ferrel cell collapses)
    //   tilt > 54°  → 1 (chaotic seasonal reversal — treated as single cell)
    let cells: u8 = if tilt < 8.0 {
        1
    } else if tilt < 40.0 {
        3
    } else if tilt < 54.0 {
        2
    } else {
        1
    };

    // Base weights (equatorial, mid-lat, polar). Low tilt → strongly peaked
    // at equator; Earth-like → ITCZ + subtropical desert + mid-lat rain belt;
    // high tilt → smeared by seasonal migration.
    let weights = match cells {
        1 if tilt < 8.0 => [2.2, 0.7, 0.3], // low-tilt: equator-peaked
        3 => [1.8, 0.8, 0.5],               // Earth-like
        2 => [1.4, 1.0, 0.8],               // collapsed Ferrel
        _ => [1.1, 1.2, 1.1],               // chaotic high-tilt
    };
    // Normalize so area-weighted mean matches mean_mm. Zonal areas (fraction
    // of hemisphere by latitude): equatorial 0-30°=0.50, mid 30-60°=0.37,
    // polar 60-90°=0.13.
    let zonal_area = [0.50, 0.37, 0.13];
    let weighted_sum: f32 = weights
        .iter()
        .zip(zonal_area.iter())
        .map(|(w, a)| w * a)
        .sum();
    let norm = if weighted_sum > 0.0 {
        mean_mm / weighted_sum
    } else {
        mean_mm
    };
    let zonal = [weights[0] * norm, weights[1] * norm, weights[2] * norm];
    (cells, zonal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StarContext;

    fn earth_like_context() -> PlanetSimulationInput {
        PlanetSimulationInput {
            blackbody_temp_k: 288,
            star: StarContext {
                age_gyr: 4.6,
                ..Default::default()
            },
            orbit: OrbitContext {
                axial_tilt_deg: 23.4,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn earth_like_produces_hydrography() {
        let h = generate_hydrography(&earth_like_context(), 1.0, 71.0).unwrap();
        assert_eq!(h.hadley_cells_per_hemisphere, 3);
        assert!(h.mean_precipitation_mm > 500.0 && h.mean_precipitation_mm < 2500.0);
    }

    #[test]
    fn low_tilt_concentrates_rain_at_equator() {
        let mut ctx = earth_like_context();
        ctx.orbit.axial_tilt_deg = 2.0;
        let h = generate_hydrography(&ctx, 1.0, 71.0).unwrap();
        assert_eq!(h.hadley_cells_per_hemisphere, 1);
        let [eq, mid, polar] = h.zonal_precipitation_mm;
        assert!(
            eq > mid && mid > polar,
            "expected equator>mid>polar, got {:?}",
            h.zonal_precipitation_mm
        );
        assert!(eq > 2.5 * polar, "equator {} not ≫ polar {}", eq, polar);
    }

    #[test]
    fn earth_tilt_has_subtropical_dry_bands() {
        let h = generate_hydrography(&earth_like_context(), 1.0, 71.0).unwrap();
        let [eq, mid, _polar] = h.zonal_precipitation_mm;
        // Mid-latitudes are drier than equatorial — the subtropical high.
        assert!(
            eq > mid,
            "expected equator > mid-lat, got {:?}",
            h.zonal_precipitation_mm
        );
    }

    #[test]
    fn high_tilt_smears_rainfall() {
        let mut ctx = earth_like_context();
        ctx.orbit.axial_tilt_deg = 70.0;
        let h = generate_hydrography(&ctx, 1.0, 71.0).unwrap();
        let [eq, _mid, polar] = h.zonal_precipitation_mm;
        // High-tilt worlds smear rainfall; equator-to-polar ratio is low.
        let ratio = eq / polar;
        assert!(
            ratio < 1.5,
            "expected smeared rainfall, got ratio {}",
            ratio
        );
    }

    #[test]
    fn no_water_means_no_hydrography() {
        assert!(generate_hydrography(&earth_like_context(), 1.0, 0.0).is_none());
    }

    #[test]
    fn no_atmosphere_means_no_hydrography() {
        assert!(generate_hydrography(&earth_like_context(), 0.0, 50.0).is_none());
    }

    #[test]
    fn hot_ocean_world_has_more_rain() {
        let mut ctx = earth_like_context();
        ctx.blackbody_temp_k = 320;
        let hot = generate_hydrography(&ctx, 1.0, 95.0).unwrap();
        let earth = generate_hydrography(&earth_like_context(), 1.0, 71.0).unwrap();
        assert!(hot.mean_precipitation_mm > earth.mean_precipitation_mm);
    }
}
