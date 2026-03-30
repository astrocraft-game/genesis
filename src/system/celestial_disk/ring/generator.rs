use crate::internal::*;
use crate::prelude::*;

impl CelestialRingDetails {
    /// Generate ring details for a planet based on its properties.
    pub fn generate(
        seed: &str,
        coord: SpaceCoordinates,
        system_index: u16,
        body_id: u32,
        body_mass_earth: f64,
        body_radius_earth: f64,
        body_density: f32,
        blackbody_temp: u32,
    ) -> Option<Self> {
        let mut rng = SeededDiceRoller::new(
            seed,
            &format!("sys_{}_{}_bdy{}_ring", coord, system_index, body_id),
        );

        // Only gas giants and large icy bodies get rings
        if body_mass_earth < 10.0 {
            return None;
        }

        // Chance of rings: ~60% for gas giants, ~20% for ice giants
        let ring_chance = if body_mass_earth > 50.0 { 60 } else { 20 };
        if rng.roll(1, 100, 0) > ring_chance as i64 {
            return None;
        }

        let composition = if blackbody_temp < 150 {
            match rng.roll(1, 4, 0) {
                1..=3 => CelestialRingComposition::Ice,
                _ => CelestialRingComposition::Dust,
            }
        } else if blackbody_temp < 400 {
            match rng.roll(1, 3, 0) {
                1 => CelestialRingComposition::Rock,
                2 => CelestialRingComposition::Dust,
                _ => CelestialRingComposition::Ice,
            }
        } else {
            match rng.roll(1, 3, 0) {
                1 => CelestialRingComposition::Rock,
                2 => CelestialRingComposition::Metal,
                _ => CelestialRingComposition::Dust,
            }
        };

        let level = match rng.roll(1, 10, 0) {
            1..=3 => CelestialRingLevel::Unnoticeable,
            4..=6 => CelestialRingLevel::Noticeable,
            7..=9 => CelestialRingLevel::Visible,
            _ => CelestialRingLevel::Spectacular,
        };

        Some(Self { level, composition })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gas_giant_can_have_rings() {
        let mut has_ring = false;
        for i in 0..50 {
            let ring = CelestialRingDetails::generate(
                &format!("seed{}", i),
                SpaceCoordinates::new(0, 0, 0),
                0, 0, 300.0, 11.0, 1.3, 120,
            );
            if ring.is_some() {
                has_ring = true;
                break;
            }
        }
        assert!(has_ring, "At least one gas giant should have rings in 50 tries");
    }

    #[test]
    fn small_body_no_rings() {
        let ring = CelestialRingDetails::generate(
            "seed", SpaceCoordinates::new(0, 0, 0), 0, 0, 1.0, 1.0, 5.5, 288,
        );
        assert!(ring.is_none());
    }
}
