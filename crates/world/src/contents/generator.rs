use crate::internal::*;
use crate::prelude::*;

pub(crate) fn generate_body_from_type(
    system_traits: &Vec<SystemPeculiarity>,
    system_index: u16,
    star_id: u32,
    star_name: Rc<str>,
    star_age: f32,
    star_mass: f64,
    star_luminosity: f32,
    star_type: &StarSpectralType,
    star_class: &StarLuminosityClass,
    star_traits: &Vec<StarPeculiarity>,
    primary_star_mass: f64,
    coord: SpaceCoordinates,
    seed: &Rc<str>,
    next_id: &mut u32,
    gas_giant_arrangement: GasGiantArrangement,
    mut populated_orbit_index: u32,
    size_modifier: i32,
    body_type: TelluricBodyComposition,
    body_id: u32,
    orbit: Option<Orbit>,
    orbit_distance: f64,
    orbited_by: Vec<Orbit>,
    settings: GenerationSettings,
    is_moon: bool,
    fixed_size: Option<CelestialBodySize>,
) -> (OrbitalPoint, Vec<OrbitalPoint>) {
    if body_type == TelluricBodyComposition::Metallic {
        TelluricBodyDetails::generate_metallic_body(
            body_id,
            coord,
            system_traits,
            system_index,
            star_id,
            star_name.clone(),
            star_age,
            star_mass,
            star_type,
            star_class,
            star_luminosity,
            star_traits,
            primary_star_mass,
            gas_giant_arrangement,
            next_id,
            populated_orbit_index,
            orbit.clone(),
            orbit_distance,
            orbited_by.clone(),
            seed.clone(),
            settings.clone(),
            size_modifier,
            is_moon,
            fixed_size,
        )
    } else if body_type == TelluricBodyComposition::Rocky {
        TelluricBodyDetails::generate_rocky_body(
            body_id,
            coord,
            system_traits,
            system_index,
            star_id,
            star_name.clone(),
            star_age,
            star_mass,
            star_type,
            star_class,
            star_luminosity,
            star_traits,
            primary_star_mass,
            gas_giant_arrangement,
            next_id,
            populated_orbit_index,
            orbit.clone(),
            orbit_distance,
            orbited_by.clone(),
            seed.clone(),
            settings.clone(),
            size_modifier,
            is_moon,
            fixed_size,
        )
    } else if body_type == TelluricBodyComposition::Icy {
        IcyBodyDetails::generate_icy_body(
            body_id,
            coord,
            system_traits,
            system_index,
            star_id,
            star_name.clone(),
            star_age,
            star_mass,
            star_type,
            star_class,
            star_luminosity,
            star_traits,
            primary_star_mass,
            gas_giant_arrangement,
            next_id,
            populated_orbit_index,
            orbit,
            orbit_distance,
            orbited_by.clone(),
            seed.clone(),
            settings.clone(),
            size_modifier,
            is_moon,
            fixed_size,
        )
    } else {
        GaseousBodyDetails::generate_gas_giant(
            body_id,
            system_traits,
            system_index,
            star_id,
            star_name.clone(),
            star_age,
            star_mass,
            star_type,
            star_class,
            star_luminosity,
            star_traits,
            primary_star_mass,
            gas_giant_arrangement,
            orbit.unwrap_or_default(),
            orbit_distance,
            populated_orbit_index,
            next_id,
            coord,
            seed.clone(),
            settings.clone(),
        )
    }
}

pub(crate) fn generate_inner_body_type(
    mut rng: &mut SeededDiceRoller,
    settings: GenerationSettings,
) -> CelestialBodyComposition {
    rng.get_result(&CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult::new(
                CelestialBodyComposition::Metallic,
                if settings.celestial_body.do_not_generate_metallic {
                    0
                } else {
                    2
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Rocky,
                if settings.celestial_body.do_not_generate_rocky {
                    0
                } else {
                    6
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Icy,
                if settings.celestial_body.do_not_generate_icy {
                    0
                } else {
                    2
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Gaseous,
                if settings.celestial_body.do_not_generate_gaseous {
                    0
                } else {
                    1
                },
            ),
        ],
        RollMethod::SimpleRoll,
    ))
    .expect("A body type should have been picked.")
}

pub(crate) fn generate_outer_body_type(
    mut rng: &mut SeededDiceRoller,
    settings: GenerationSettings,
) -> CelestialBodyComposition {
    rng.get_result(&CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult::new(
                CelestialBodyComposition::Metallic,
                if settings.celestial_body.do_not_generate_metallic {
                    0
                } else {
                    1
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Rocky,
                if settings.celestial_body.do_not_generate_rocky {
                    0
                } else {
                    3
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Icy,
                if settings.celestial_body.do_not_generate_icy {
                    0
                } else {
                    6
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Gaseous,
                if settings.celestial_body.do_not_generate_gaseous {
                    0
                } else {
                    6
                },
            ),
        ],
        RollMethod::SimpleRoll,
    ))
    .expect("A body type should have been picked.")
}
