use crate::internal::*;
use crate::prelude::*;

impl CelestialBeltDetails {
    /// Generate belt details based on zone type and system properties.
    pub fn generate(
        seed: &str,
        coord: SpaceCoordinates,
        system_index: u16,
        belt_id: u32,
        zone: ZoneType,
    ) -> Self {
        let mut rng = SeededDiceRoller::new(
            seed,
            &format!("sys_{}_{}_belt{}", coord, system_index, belt_id),
        );

        let composition = match zone {
            ZoneType::InnerZone | ZoneType::InnerLimit => match rng.roll(1, 6, 0) {
                1..=2 => CelestialBeltType::Dust,
                3..=4 => CelestialBeltType::Meteoroid,
                5 => CelestialBeltType::Ore,
                _ => CelestialBeltType::Ash,
            },
            ZoneType::BioZone => match rng.roll(1, 6, 0) {
                1..=2 => CelestialBeltType::Asteroid,
                3 => CelestialBeltType::Debris,
                4 => CelestialBeltType::Meteoroid,
                5 => CelestialBeltType::Ore,
                _ => CelestialBeltType::Dust,
            },
            ZoneType::OuterZone => match rng.roll(1, 6, 0) {
                1..=2 => CelestialBeltType::Frost,
                3 => CelestialBeltType::Comet,
                4 => CelestialBeltType::Asteroid,
                5 => CelestialBeltType::Debris,
                _ => CelestialBeltType::Dust,
            },
            _ => CelestialBeltType::Debris,
        };

        Self { composition }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn belt_generation_deterministic() {
        let b1 = CelestialBeltDetails::generate(
            "seed42", SpaceCoordinates::new(0, 0, 0), 0, 0, ZoneType::BioZone,
        );
        let b2 = CelestialBeltDetails::generate(
            "seed42", SpaceCoordinates::new(0, 0, 0), 0, 0, ZoneType::BioZone,
        );
        assert_eq!(b1.composition, b2.composition);
    }

    #[test]
    fn outer_zone_favors_icy() {
        let mut frost_count = 0;
        for i in 0..50 {
            let belt = CelestialBeltDetails::generate(
                &format!("s{}", i), SpaceCoordinates::new(0, 0, 0), 0, 0, ZoneType::OuterZone,
            );
            if belt.composition == CelestialBeltType::Frost || belt.composition == CelestialBeltType::Comet {
                frost_count += 1;
            }
        }
        assert!(frost_count > 10, "Outer zone should favor icy belts, got {}/50", frost_count);
    }
}
