//! Optional hydraulic and thermal erosion on a SurfaceGrid.
//!
//! Both passes mutate `layers.elevation_m` in-place. They are intended to
//! run **after** base/plate elevation + fractal noise and **before** the
//! sea-level binary search, so coastlines reflect the eroded terrain.
//!
//! This module is gated behind the `erosion` feature because the particle
//! simulation is expensive for large grids and not every caller needs it.

use crate::grid::SurfaceGrid;
use seeded_dice_roller::SeededDiceRoller;

/// Parameters controlling a single erosion pass.
#[derive(Clone, Copy, Debug)]
pub struct ErosionParams {
    /// Number of water-droplet particles to simulate.
    pub particles: u32,
    /// Maximum steps each droplet takes before evaporating.
    pub max_steps: u32,
    /// Fraction of slope converted to sediment at each step (0.0 - 1.0).
    pub erosion_rate: f32,
    /// Fraction of carried sediment deposited per step on flat terrain.
    pub deposit_rate: f32,
    /// How much of the droplet's energy is lost per step.
    pub evaporation: f32,
    /// How much momentum carries between steps (0 = random walk, 1 = inertial).
    pub inertia: f32,
    /// Talus angle threshold in degrees — slopes above this thermally relax.
    pub talus_angle_deg: f32,
    /// Number of thermal-relaxation sweeps after hydraulic erosion.
    pub thermal_iterations: u32,
}

impl Default for ErosionParams {
    fn default() -> Self {
        Self {
            particles: 20_000,
            max_steps: 30,
            erosion_rate: 0.3,
            deposit_rate: 0.2,
            evaporation: 0.02,
            inertia: 0.05,
            talus_angle_deg: 33.0,
            thermal_iterations: 4,
        }
    }
}

/// Run hydraulic erosion then thermal erosion on `grid.layers.elevation_m`.
///
/// Droplets spawn at random tile centres, slide downhill across the
/// equirectangular grid (longitude wraps, poles clamp), erode proportional
/// to the local slope, and deposit carried sediment on up-slopes or low
/// gradients. After the particle pass, a talus-angle sweep redistributes
/// mass where the land is steeper than geologically stable.
pub fn erode(grid: &mut SurfaceGrid, params: ErosionParams, seed: &str) {
    hydraulic_erosion(grid, params, seed);
    thermal_erosion(grid, params);
}

fn hydraulic_erosion(grid: &mut SurfaceGrid, params: ErosionParams, seed: &str) {
    let mut rng = SeededDiceRoller::new(seed, "hydraulic_erosion");
    let width = grid.width as i32;
    let height = grid.height as i32;

    for _ in 0..params.particles {
        // Spawn anywhere on the grid.
        let mut x = (rng.gen_u32() as i32).rem_euclid(width);
        let mut y = (rng.gen_u32() as i32).rem_euclid(height);
        let mut vx = 0.0f32;
        let mut vy = 0.0f32;
        let mut water = 1.0f32;
        let mut sediment = 0.0f32;

        for _ in 0..params.max_steps {
            let idx = (y * width + x) as usize;
            let here = grid.layers.elevation_m[idx];

            // Compute slope via 4-neighbours (with longitude wrap).
            let (lx, rx) = ((x - 1).rem_euclid(width), (x + 1).rem_euclid(width));
            let uy = (y - 1).max(0);
            let dy = (y + 1).min(height - 1);
            let left = grid.layers.elevation_m[(y * width + lx) as usize];
            let right = grid.layers.elevation_m[(y * width + rx) as usize];
            let up = grid.layers.elevation_m[(uy * width + x) as usize];
            let down = grid.layers.elevation_m[(dy * width + x) as usize];

            // Gradient: positive gx means slope rises east.
            let gx = right - left;
            let gy = down - up;

            // Accelerate the droplet by negative gradient (downhill).
            vx = vx * params.inertia - gx * (1.0 - params.inertia);
            vy = vy * params.inertia - gy * (1.0 - params.inertia);
            let speed = (vx * vx + vy * vy).sqrt().max(1e-6);
            vx /= speed;
            vy /= speed;

            let nx = x + vx.round() as i32;
            let ny = y + vy.round() as i32;
            let nx_w = nx.rem_euclid(width);
            let ny_c = ny.clamp(0, height - 1);

            let new_idx = (ny_c * width + nx_w) as usize;
            let next = grid.layers.elevation_m[new_idx];
            let dh = next - here;

            if dh > 0.0 {
                // Droplet went uphill (eddy): deposit all sediment here and stop.
                grid.layers.elevation_m[idx] += sediment.min(dh);
                break;
            }

            // Carrying capacity scales with slope * speed * water.
            let capacity = (-dh).max(0.01) * speed * water * 4.0;
            if sediment > capacity {
                // Over-saturated: deposit the excess at the current tile.
                let drop = (sediment - capacity) * params.deposit_rate;
                grid.layers.elevation_m[idx] += drop;
                sediment -= drop;
            } else {
                // Under-saturated: erode the tile we just left.
                let erode = ((capacity - sediment) * params.erosion_rate).min(-dh);
                grid.layers.elevation_m[idx] -= erode;
                sediment += erode;
            }

            water *= 1.0 - params.evaporation;
            if water < 0.01 {
                grid.layers.elevation_m[new_idx] += sediment;
                break;
            }
            x = nx_w;
            y = ny_c;
        }
    }
}

fn thermal_erosion(grid: &mut SurfaceGrid, params: ErosionParams) {
    // Convert talus angle to a per-neighbour height threshold. One tile
    // spans roughly (2πR / width) metres at the equator; we use a proxy
    // tile size of 50km which matches Fast-resolution Earth.
    let tile_size_m = 50_000.0f32;
    let max_delta_m = params.talus_angle_deg.to_radians().tan() * tile_size_m;

    let width = grid.width as i32;
    let height = grid.height as i32;
    let n = (width * height) as usize;
    let mut delta = vec![0.0f32; n];

    for _ in 0..params.thermal_iterations {
        delta.iter_mut().for_each(|d| *d = 0.0);
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                let here = grid.layers.elevation_m[idx];
                // 4-neighbours with longitude wrap, latitude clamp.
                let neighbours = [
                    ((x - 1).rem_euclid(width), y),
                    ((x + 1).rem_euclid(width), y),
                    (x, (y - 1).max(0)),
                    (x, (y + 1).min(height - 1)),
                ];
                for (nx, ny) in neighbours {
                    let nidx = (ny * width + nx) as usize;
                    if nidx == idx {
                        continue;
                    }
                    let diff = here - grid.layers.elevation_m[nidx];
                    if diff > max_delta_m {
                        // Move half the excess down to the neighbour.
                        let transfer = (diff - max_delta_m) * 0.5;
                        delta[idx] -= transfer;
                        delta[nidx] += transfer;
                    }
                }
            }
        }
        for (elev, &d) in grid.layers.elevation_m.iter_mut().zip(delta.iter()) {
            *elev += d;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geology::generate_geology;
    use crate::grid::GridResolution;
    use crate::types::{OrbitContext, PlanetSimulationInput, StarContext};

    fn earth_grid_pre_sealevel() -> SurfaceGrid {
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
        // generate_geology gives us the post-noise elevation map already.
        generate_geology(&input, 71.0, GridResolution::Fast, "erosion")
    }

    #[test]
    fn erosion_is_deterministic() {
        let mut a = earth_grid_pre_sealevel();
        let mut b = earth_grid_pre_sealevel();
        let params = ErosionParams {
            particles: 500,
            ..Default::default()
        };
        erode(&mut a, params, "det");
        erode(&mut b, params, "det");
        assert_eq!(a.layers.elevation_m, b.layers.elevation_m);
    }

    #[test]
    fn erosion_preserves_mass_within_tolerance() {
        // Particle hydraulic erosion is not strictly mass-conserving, but
        // the *mean* elevation should drift only slightly on a spherical
        // grid because sediment is redistributed locally.
        let mut g = earth_grid_pre_sealevel();
        let mean_before: f32 =
            g.layers.elevation_m.iter().sum::<f32>() / g.layers.elevation_m.len() as f32;
        let params = ErosionParams {
            particles: 1000,
            ..Default::default()
        };
        erode(&mut g, params, "mass");
        let mean_after: f32 =
            g.layers.elevation_m.iter().sum::<f32>() / g.layers.elevation_m.len() as f32;
        assert!(
            (mean_before - mean_after).abs() < 2000.0,
            "mean drift {} too large",
            (mean_before - mean_after).abs()
        );
    }

    #[test]
    fn thermal_erosion_reduces_steep_slopes() {
        let mut g = earth_grid_pre_sealevel();
        let width = g.width as i32;
        let height = g.height as i32;

        let max_slope = |grid: &SurfaceGrid| -> f32 {
            let mut m = 0.0f32;
            for y in 0..height {
                for x in 0..width {
                    let idx = (y * width + x) as usize;
                    let here = grid.layers.elevation_m[idx];
                    let nx = (x + 1).rem_euclid(width);
                    let ny = (y + 1).min(height - 1);
                    let r = grid.layers.elevation_m[(y * width + nx) as usize];
                    let d = grid.layers.elevation_m[(ny * width + x) as usize];
                    m = m.max((here - r).abs()).max((here - d).abs());
                }
            }
            m
        };
        let before = max_slope(&g);
        let params = ErosionParams {
            particles: 0, // skip hydraulic, isolate thermal
            thermal_iterations: 20,
            talus_angle_deg: 33.0,
            ..Default::default()
        };
        erode(&mut g, params, "thermal");
        let after = max_slope(&g);
        assert!(
            after <= before + 1e-3,
            "thermal erosion should not steepen slopes (before={}, after={})",
            before,
            after
        );
    }

    #[test]
    fn hydraulic_erosion_lowers_peaks() {
        // Particle erosion transports sediment from high slopes to low
        // gradients, so the elevation maximum should drop and the minimum
        // should rise as valleys fill and peaks erode.
        let mut g = earth_grid_pre_sealevel();
        let max_before = g
            .layers
            .elevation_m
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
        let min_before = g
            .layers
            .elevation_m
            .iter()
            .copied()
            .fold(f32::INFINITY, f32::min);
        let params = ErosionParams {
            particles: 20_000,
            thermal_iterations: 0,
            ..Default::default()
        };
        erode(&mut g, params, "peaks");
        let max_after = g
            .layers
            .elevation_m
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
        let min_after = g
            .layers
            .elevation_m
            .iter()
            .copied()
            .fold(f32::INFINITY, f32::min);
        assert!(
            max_after < max_before,
            "peaks should be eroded: {} -> {}",
            max_before,
            max_after
        );
        assert!(
            min_after > min_before,
            "basins should be filled: {} -> {}",
            min_before,
            min_after
        );
    }

    #[test]
    fn zero_particles_skips_hydraulic_pass() {
        let mut g = earth_grid_pre_sealevel();
        let before = g.layers.elevation_m.clone();
        let params = ErosionParams {
            particles: 0,
            thermal_iterations: 0,
            ..Default::default()
        };
        erode(&mut g, params, "noop");
        assert_eq!(g.layers.elevation_m, before);
    }
}
