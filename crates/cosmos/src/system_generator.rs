use crate::internal::*;
use crate::prelude::*;
use crate::types::StarSystem;
use std::collections::HashSet;

#[path = "system_constants.rs"]
mod constants;
use crate::contents::generator::generate_stars_systems;
use crate::contents::zones::generate_star_zones;
use constants::*;

// StarSystem generation (free function since type is in system crate)
/// Generates a brand new star system at the given coordinates
pub fn generate_star_system(
    system_index: u16,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    sub_sector: &GalacticMapDivision,
    galaxy: &mut Galaxy,
) -> StarSystem {
    let mut center_id: u32 = 0;
    let mut main_star_id: u32 = 0;
    let mut all_objects: Vec<OrbitalPoint> = Vec::new();
    let mut special_traits: Vec<SystemPeculiarity> = Vec::new();

    let name = get_system_name(system_index, coord, galaxy);

    let mut accept_system = false;
    let mut i = 0;
    while !accept_system {
        all_objects = Vec::new();
        special_traits = generate_system_peculiarities(i, system_index, coord, galaxy);

        let number_of_stars = generate_number_of_stars_in_system(i, system_index, coord, galaxy);
        let mut stars = generate_stars(
            i,
            number_of_stars,
            system_index,
            name.clone(),
            coord,
            hex,
            sub_sector,
            galaxy,
        );

        if stars.len() > 1 {
            let result = generate_binary_relations(
                i,
                &mut stars,
                &mut all_objects,
                system_index,
                coord,
                galaxy,
            );
            center_id = result.0;
            main_star_id = result.1;

            let mut calculated_ids = HashSet::new();
            for id in all_objects.iter().map(|op| op.id).collect::<Vec<u32>>() {
                calculate_distance_from_system_center(id, &mut all_objects, &mut calculated_ids);
            }
        } else {
            let center =
                OrbitalPoint::new(0, None, AstronomicalObject::Star(stars.remove(0)), vec![]);
            center_id = 0;
            main_star_id = 0;
            all_objects.push(center);
        }

        // Star orbit eccentricity, inclination, period, and spin are
        // computed inside `make_binary_pair` via Kepler's third law and
        // `stellar_rotation_period`. Single stars have `orbit = None`.
        update_existing_orbits(&mut all_objects);
        generate_star_zones(&mut all_objects);
        generate_stars_systems(
            i,
            &mut all_objects,
            &special_traits,
            system_index,
            coord,
            galaxy,
        );
        update_existing_orbits(&mut all_objects);

        accept_system = if galaxy.settings.system.only_interesting {
            let mut is_interesting = false;

            is_interesting = all_objects
                .iter()
                .find(|o| {
                    if let AstronomicalObject::TelluricBody(body) = o.object.clone() {
                        if let CelestialBodyDetails::Telluric(details) = body.details {
                            details.world_type == CelestialBodyWorldType::Terrestrial
                                || details.world_type == CelestialBodyWorldType::Ocean
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .is_some();

            is_interesting
        } else {
            true
        };
        i += 1;
        if i > 5000 {
            panic!("There should be at least one interesting system in every 5000 tries!");
        }
    }
    StarSystem::new(name, center_id, main_star_id, all_objects, special_traits)
}

/// Temporary name generation
fn get_system_name(system_index: u16, coord: SpaceCoordinates, galaxy: &Galaxy) -> Rc<str> {
    let settings = &galaxy.settings;
    if settings.star.use_ours {
        "Sol".into()
    } else {
        let mut rng = SeededDiceRoller::new(
            &galaxy.settings.seed,
            &format!("sys_{}_{}_ste_evo", coord, system_index),
        );
        let random_names = get_random_names();
        (random_names[rng.gen_usize() % random_names.len()]).into()
    }
}

fn generate_number_of_stars_in_system(
    system_gen_try: u32,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> u16 {
    let mut rng = SeededDiceRoller::new(
        &format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("sys_{}_{}_ste_evo", coord, system_index),
    );
    let mut count: u16 = rng
        .get_result(&CopyableRollToProcess::new(
            vec![
                CopyableWeightedResult::new(1, 400),
                CopyableWeightedResult::new(2, 280),
                CopyableWeightedResult::new(3, 120),
                CopyableWeightedResult::new(4, 32),
                CopyableWeightedResult::new(5, 20),
                CopyableWeightedResult::new(6, 12),
                CopyableWeightedResult::new(7, 4),
                CopyableWeightedResult::new(8, 2),
                CopyableWeightedResult::new(9, 1),
            ],
            RollMethod::SimpleRoll,
        ))
        .unwrap();
    if let Some(min) = galaxy.settings.star.min_stars {
        count = count.max(min as u16);
    }
    if let Some(max) = galaxy.settings.star.max_stars {
        count = count.min(max as u16);
    }
    count
}

fn generate_stars(
    system_gen_try: u32,
    number_of_stars: u16,
    system_index: u16,
    system_name: Rc<str>,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    sub_sector: &GalacticMapDivision,
    galaxy: &mut Galaxy,
) -> Vec<Star> {
    let mut stars = Vec::new();
    for star_index in 0..number_of_stars {
        let evolution = if let Some(pop) = galaxy.settings.star.fixed_population {
            pop
        } else {
            generate_stellar_evolution(
                system_gen_try,
                star_index,
                system_index,
                coord,
                hex,
                sub_sector,
                galaxy,
            )
        };

        stars.push(Star::generate(
            system_gen_try,
            star_index,
            system_index,
            system_name.clone(),
            coord,
            evolution,
            hex,
            galaxy,
            &galaxy.settings,
        ));
    }
    stars
}

/// Generates the Population of stars in a system.
fn generate_stellar_evolution(
    system_gen_try: u32,
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    sub_sector: &GalacticMapDivision,
    galaxy: &mut Galaxy,
) -> StellarEvolution {
    let mut subsector_rng = SeededDiceRoller::new(
        &format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("sys_{}_ste_evo", sub_sector.index),
    );
    let mut hex_rng = SeededDiceRoller::new(
        &format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("sys_{}_ste_evo", hex.index),
    );
    let mut rng = SeededDiceRoller::new(
        &format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("sys_{}_{}_ste_evo", coord, system_index),
    );
    let mut coord_rng = SeededDiceRoller::new(
        &format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("sys_{}_ste_evo", star_index),
    );

    let mut modifier = 0;
    match galaxy.neighborhood.universe.era {
        StelliferousEra::AncientStelliferous => modifier -= 10,
        StelliferousEra::EarlyStelliferous => modifier -= 5,
        StelliferousEra::LateStelliferous => modifier += 2,
        StelliferousEra::EndStelliferous => modifier += 5,
        _ => (),
    }
    modifier += if galaxy.is_dominant {
        2
    } else if !galaxy.is_major {
        -2
    } else {
        0
    };
    if let GalaxyCategory::Intergalactic(_, _, _) = galaxy.category {
        modifier -= 10
    }
    match galaxy.sub_category {
        GalaxySubCategory::DwarfAmorphous
        | GalaxySubCategory::DwarfSpiral
        | GalaxySubCategory::DwarfElliptical
        | GalaxySubCategory::DwarfLenticular => modifier -= 2,
        GalaxySubCategory::GiantLenticular | GalaxySubCategory::GiantElliptical => modifier += 1,
        _ => (),
    }
    galaxy.special_traits.iter().for_each(|t| match t {
        GalaxySpecialTrait::MetalPoor => modifier -= 5,
        GalaxySpecialTrait::Younger => modifier -= 2,
        GalaxySpecialTrait::SubSize(_) => modifier -= 1,
        GalaxySpecialTrait::Dusty | GalaxySpecialTrait::SuperSize(_) => modifier += 1,
        GalaxySpecialTrait::Starburst => modifier += 2,
        _ => (),
    });
    let divisions = galaxy
        .get_divisions_for_coord(coord)
        .expect("Should have returned divisions.");
    let mut regions = Vec::new();
    divisions.iter().for_each(|div| {
        if regions.iter().find(|r| **r == div.region).is_none() {
            regions.push(div.region);
        }
    });
    regions.iter().for_each(|region| match region {
        GalacticRegion::Nucleus => modifier += 2,
        GalacticRegion::Core | GalacticRegion::Bar | GalacticRegion::Arm => modifier += 1,
        GalacticRegion::Disk => modifier -= 1,
        GalacticRegion::Ellipse => modifier -= 2,
        GalacticRegion::Halo | GalacticRegion::Void | GalacticRegion::Stream => modifier -= 5,
        GalacticRegion::Aura => modifier -= 10,
        _ => (),
    });

    let roll = subsector_rng.roll(1, 4, -1)
        + hex_rng.roll(1, 3, -1)
        + coord_rng.roll(1, 3, -1)
        + rng.roll(1, 4, -1)
        + modifier;

    if roll < -10 {
        StellarEvolution::Paleodwarf
    } else if roll < 3 {
        StellarEvolution::Subdwarf
    } else if roll < 10 {
        StellarEvolution::Dwarf
    } else if roll < 20 {
        StellarEvolution::Superdwarf
    } else {
        StellarEvolution::Hyperdwarf
    }
}

/// For a given list of stars, generates binary pairs and makes them dance together. Returns the id of the system's
/// center point of gravity (res.0), the id of the system's main star (res.1) and the last id used for an object (res.2).
fn generate_binary_relations(
    system_gen_try: u32,
    stars_left: &mut Vec<Star>,
    all_objects: &mut Vec<OrbitalPoint>,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> (u32, u32, u32) {
    let mut rng = SeededDiceRoller::new(
        &format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("sys_{}_{}_bin_rel", coord, system_index),
    );
    let mut center_id = 0;
    let mut last_id = 0;
    let main_star_id = last_id;
    let number_of_stars = stars_left.len();

    // Extract the biggest star of the bunch
    let biggest_mass_in_vec = stars_left
        .iter()
        .map(|star| star.mass)
        .max_by(|a, b| {
            a.partial_cmp(b)
                .expect("There should be at least two stars to compare.")
        })
        .expect("There should be at least one star with some mass.");
    let star_index = stars_left
        .iter()
        .position(|star| star.mass == biggest_mass_in_vec)
        .expect("I should be able to find the index of a star.");
    let most_massive = stars_left.remove(star_index);

    // Use it as our first primary member in pairs
    let mut most_massive_mass = most_massive.mass;
    let mut most_massive_radius = most_massive.radius;
    let mut most_massive_point = OrbitalPoint::new(
        last_id,
        None,
        AstronomicalObject::Star(most_massive),
        vec![],
    );

    let mut first_turn = true;
    let mut previous_actual_distance = 0.005;
    // Then make binary pairs as long as you have stars
    while !stars_left.is_empty() {
        // If at least two stars left, with a random chance of 1 in 4 or more
        if stars_left.len() > 1
            && (!rng.gen_u8().is_multiple_of(7)
                || (!first_turn
                    && number_of_stars.is_multiple_of(2)
                    && !rng.gen_u8().is_multiple_of(5)))
        {
            // Make a pair and have it dance with the biggest mass/last pair.
            last_id += 1;
            let first_of_pair = stars_left.remove(0);
            let first_of_pair_mass = first_of_pair.mass;
            let first_of_pair_radius = first_of_pair.radius;
            let first_of_pair_point = OrbitalPoint::new(
                last_id,
                None,
                AstronomicalObject::Star(first_of_pair),
                vec![],
            );
            last_id += 1;
            let second_of_pair = stars_left.remove(0);
            let second_of_pair_mass = second_of_pair.mass;
            let second_of_pair_radius = second_of_pair.radius;
            let second_of_pair_point = OrbitalPoint::new(
                last_id,
                None,
                AstronomicalObject::Star(second_of_pair),
                vec![],
            );

            // Generate our new pair
            let result = make_binary_pair(
                last_id,
                first_of_pair_mass,
                first_of_pair_radius,
                first_of_pair_point,
                second_of_pair_mass,
                second_of_pair_radius,
                second_of_pair_point,
                calculate_stars_minimum_distance(first_of_pair_radius, second_of_pair_radius),
                star_index,
                system_index,
                coord,
                galaxy,
                all_objects,
            );
            last_id = result.0;
            let less_massive_point = result.1;
            let less_massive_mass = result.2;
            let less_massive_radius = result.3;

            // Use the newly generated pair as the less massive member to generate the next pair
            let result = make_binary_pair(
                last_id,
                most_massive_mass,
                most_massive_radius,
                most_massive_point,
                less_massive_mass,
                less_massive_radius,
                less_massive_point,
                previous_actual_distance,
                star_index,
                system_index,
                coord,
                galaxy,
                all_objects,
            );
            last_id = result.0;

            // Then update what values to use in the next turn
            most_massive_point = result.1;
            most_massive_mass = result.2;
            most_massive_radius = result.3;
            previous_actual_distance = result.4;
            center_id = most_massive_point.id;
        } else {
            // Take a single star and have it dance with the biggest mass/last pair.
            last_id += 1;
            let less_massive = stars_left.remove(0);
            let less_massive_mass = less_massive.mass;
            let less_massive_radius = less_massive.radius;
            let less_massive_point = OrbitalPoint::new(
                last_id,
                None,
                AstronomicalObject::Star(less_massive),
                vec![],
            );

            let result = make_binary_pair(
                last_id,
                most_massive_mass,
                most_massive_radius,
                most_massive_point,
                less_massive_mass,
                less_massive_radius,
                less_massive_point,
                previous_actual_distance,
                star_index,
                system_index,
                coord,
                galaxy,
                all_objects,
            );
            last_id = result.0;

            // Then update what values to use in the next turn
            most_massive_point = result.1;
            most_massive_mass = result.2;
            most_massive_radius = result.3;
            previous_actual_distance = result.4;
            center_id = most_massive_point.id;
        }
        first_turn = false;
    }
    // Finally, add the center point to the system's list of objects
    all_objects.push(most_massive_point);

    (center_id, main_star_id, last_id)
}

/// Returns the last id used ([u32], res.0), the [OrbitalPoint] at the center of the newly made binary pair (res.1), the mass ([f32], res.2)
/// and radius ([f32], res.3) of that pair, and the actual distance between the two elements ([f64], res.4) to use in further calculations.
fn make_binary_pair(
    mut last_id: u32,
    mut most_massive_mass: f64,
    mut most_massive_radius: f64,
    mut most_massive_point: OrbitalPoint,
    mut less_massive_mass: f64,
    mut less_massive_radius: f64,
    mut less_massive_point: OrbitalPoint,
    min_distance_between_bodies: f64,
    star_index: usize,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
    all_objects: &mut Vec<OrbitalPoint>,
) -> (u32, OrbitalPoint, f64, f64, f64) {
    // Switch stars if necessary so that most_massive_point is the most massive.
    if less_massive_mass > most_massive_mass {
        let temp_mass = less_massive_mass;
        let temp_radius = less_massive_radius;
        let temp_point = less_massive_point;
        less_massive_mass = most_massive_mass;
        less_massive_radius = most_massive_radius;
        less_massive_point = most_massive_point;
        most_massive_mass = temp_mass;
        most_massive_radius = temp_radius;
        most_massive_point = temp_point;
    }

    last_id += 1;
    let result = find_center_of_binary_pair(
        &mut most_massive_point,
        most_massive_mass,
        most_massive_radius,
        &mut less_massive_point,
        less_massive_mass,
        less_massive_radius,
        min_distance_between_bodies,
        last_id,
        star_index as u16,
        system_index,
        coord,
        galaxy,
    );
    all_objects.push(most_massive_point);
    all_objects.push(less_massive_point);

    (last_id, result.0, result.1, result.2, result.3)
}

/// Organizes two elements (either stars or binary pairs) into a binary system, updates them, and returns said binary system's barycentre
/// (an [OrbitalPoint], res.0), their mass ([f32], res.1) and radius ([f32], res.2), and the actual distance between the two elements
/// ([f64], res.3) to use in further calculations.
fn find_center_of_binary_pair(
    most_massive_point: &mut OrbitalPoint,
    most_massive_mass: f64,
    most_massive_radius: f64,
    less_massive_point: &mut OrbitalPoint,
    less_massive_mass: f64,
    less_massive_radius: f64,
    min_distance: f64,
    next_id: u32,
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> (OrbitalPoint, f64, f64, f64) {
    let mut center = OrbitalPoint::new(next_id, None, AstronomicalObject::Void, vec![]);

    let actual_distance =
        generate_distance_between_stars(star_index, system_index, min_distance, 0, coord, galaxy);
    let barycentre_distance_from_most_massive =
        calculate_barycentre(actual_distance, most_massive_mass, less_massive_mass);

    let (star_ecc, star_incl, star_period) = generate_binary_star_orbit_params(
        actual_distance,
        most_massive_mass,
        less_massive_mass,
        star_index,
        system_index,
        coord,
        galaxy,
    );

    // Derive each star's spin rate from spectral type and age.
    // Short-period binaries are tidally synchronised to the orbit.
    let synchronised = star_period < 20.0;
    let most_massive_rotation =
        stellar_rotation_for_point(most_massive_point, star_period, synchronised);
    let less_massive_rotation =
        stellar_rotation_for_point(less_massive_point, star_period, synchronised);

    let most_massive_min = barycentre_distance_from_most_massive * (1.0 - star_ecc as f64);
    let most_massive_max = barycentre_distance_from_most_massive * (1.0 + star_ecc as f64);
    let most_massive_orbit = Orbit::new(
        next_id,
        Some(most_massive_point.id),
        ZoneType::ForbiddenZone,
        barycentre_distance_from_most_massive,
        most_massive_min,
        most_massive_max,
        barycentre_distance_from_most_massive,
        star_ecc,
        star_incl,
        0.0,
        star_period,
        most_massive_rotation,
        f32::INFINITY,
    );
    let less_dist = actual_distance - barycentre_distance_from_most_massive;
    let less_min = less_dist * (1.0 - star_ecc as f64);
    let less_max = less_dist * (1.0 + star_ecc as f64);
    let less_massive_orbit = Orbit::new(
        next_id,
        Some(less_massive_point.id),
        ZoneType::ForbiddenZone,
        less_dist,
        less_min,
        less_max,
        less_dist,
        star_ecc,
        star_incl,
        0.0,
        star_period,
        less_massive_rotation,
        f32::INFINITY,
    );

    center.orbits.push(most_massive_orbit.clone());
    center.orbits.push(less_massive_orbit.clone());

    most_massive_point.set_own_orbit(most_massive_orbit);
    less_massive_point.set_own_orbit(less_massive_orbit);

    let most_massive_distance_and_radius =
        most_massive_radius + barycentre_distance_from_most_massive;
    let less_massive_distance_and_radius =
        less_massive_radius + actual_distance - barycentre_distance_from_most_massive;
    let radius = if most_massive_distance_and_radius > less_massive_distance_and_radius {
        most_massive_distance_and_radius
    } else {
        less_massive_distance_and_radius
    };

    (
        center,
        most_massive_mass + less_massive_mass,
        radius,
        most_massive_distance_and_radius + less_massive_distance_and_radius,
    )
}

/// Calculates the minimum distance there can be between two stars.
/// The radius are in solar radii, but the return of the function is in AU.
fn calculate_stars_minimum_distance(radius_first_star: f64, radius_second_star: f64) -> f64 {
    ConversionUtils::solar_radii_to_astronomical_units(radius_first_star)
        + ConversionUtils::solar_radii_to_astronomical_units(radius_second_star)
}

fn calculate_distance_from_system_center(
    orbital_point_id: u32,
    all_objects: &mut [OrbitalPoint],
    calculated_ids: &mut HashSet<u32>,
) {
    // Check if the distance for this orbital point has already been calculated
    if !calculated_ids.insert(orbital_point_id) {
        return;
    }

    // Find the orbital point in the list of all objects
    let orbital_point_index = all_objects
        .iter()
        .position(|op| op.id == orbital_point_id)
        .expect("OrbitalPoint not found");

    let (orbit_option, orbital_point) = {
        let orbital_point = &all_objects[orbital_point_index];
        (orbital_point.own_orbit.clone(), orbital_point.clone())
    };

    // Calculate the distance for the primary orbit if it exists
    if let Some(orbit) = orbit_option {
        calculate_distance_from_system_center(orbit.primary_body_id, all_objects, calculated_ids);

        let primary_orbit_distance = get_primary_orbit_distance(&orbit, all_objects);

        // Update the distance from the system center
        if let Some(own_orbit) = &mut all_objects[orbital_point_index].own_orbit {
            own_orbit.average_distance_from_system_center += primary_orbit_distance;
        }
    }

    // Calculate the distances for any orbits around this orbital point
    for orbit in orbital_point.orbits.clone() {
        calculate_distance_from_system_center(orbit.primary_body_id, all_objects, calculated_ids);

        let primary_orbit_distance = get_primary_orbit_distance(&orbit, all_objects);

        if let Some(orbit) = all_objects[orbital_point_index]
            .orbits
            .iter_mut()
            .find(|o| o.id.unwrap_or(u32::MAX) == orbit.id.unwrap_or(u32::MAX))
        {
            orbit.average_distance_from_system_center += primary_orbit_distance;
        }
    }
}

fn get_primary_orbit_distance(orbit: &Orbit, all_objects: &[OrbitalPoint]) -> f64 {
    let primary_orbit_option = all_objects
        .iter()
        .find(|op| op.id == orbit.primary_body_id)
        .expect("Primary Orbit not found")
        .own_orbit
        .as_ref();

    match primary_orbit_option {
        Some(primary_orbit) => primary_orbit.average_distance_from_system_center,
        None => 0.0,
    }
}

fn update_existing_orbits(all_objects: &mut Vec<OrbitalPoint>) {
    all_objects
        .iter_mut()
        .for_each(|o| o.update_object_own_orbit());
}

/// Spin period of a star in days. Late-type stars (F-M) spin down via
/// magnetic braking following the Skumanich law (P ∝ sqrt(t)); early-type
/// and substellar objects retain their young-star rotation rates. Anchor
/// values are chosen so that the Sun (G-type, 4.6 Gyr) comes out at ≈27 days.
fn stellar_rotation_period(spectral_type: &StarSpectralType, age_myr: f32) -> f32 {
    use StarSpectralType::*;
    let (anchor_period_days, has_magnetic_braking): (f32, bool) = match spectral_type {
        O(_) | WR(_) => (3.0, false),
        B(_) => (5.0, false),
        A(_) => (2.0, false),
        F(_) => (7.0, true),
        G(_) => (12.6, true),
        K(_) => (20.0, true),
        M(_) => (30.0, true),
        L(_) | T(_) | Y(_) => (5.0, false),
        _ => (20.0, false),
    };
    if has_magnetic_braking {
        // Anchor at 1000 Myr; period scales with sqrt(age).
        anchor_period_days * (age_myr.max(1.0) / 1000.0).sqrt()
    } else {
        anchor_period_days
    }
}

fn stellar_rotation_for_point(
    point: &OrbitalPoint,
    orbital_period_days: f32,
    synchronised: bool,
) -> f32 {
    if synchronised {
        return orbital_period_days;
    }
    if let AstronomicalObject::Star(star) = &point.object {
        stellar_rotation_period(&star.spectral_type, star.age)
    } else {
        0.0
    }
}

/// Generate eccentricity, inclination, and orbital period for a binary star pair.
#[allow(clippy::too_many_arguments)]
fn generate_binary_star_orbit_params(
    actual_distance: f64,
    mass1: f64,
    mass2: f64,
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> (f32, f32, f32) {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("star_{}_{}_{}_bin_orb", coord, system_index, star_index),
    );
    // Binary eccentricity depends on orbital period
    // Short period (<10 days equiv ~0.05 AU): tidally circularized
    // Medium period: 0-0.6
    // Long period: thermal distribution f(e)=2e, up to 0.95
    let ecc = if actual_distance < 0.05 {
        rng.roll(1, 5, 0) as f32 * 0.01
    } else if actual_distance < 1.0 {
        let roll = rng.roll(2, 6, -2);
        (roll as f32 * 0.05).clamp(0.0, 0.6)
    } else {
        // Thermal distribution approximation: sample uniform in e^2
        let u = rng.gen_f64().clamp(0.01, 0.99);
        (u.sqrt() as f32).min(0.95)
    };

    // Inclination: roughly isotropic (uniform in cos(i))
    let cos_i = rng.gen_f64() * 2.0 - 1.0;
    let incl = (cos_i.acos().to_degrees()) as f32;

    // Orbital period via Kepler's third law
    let period_years = (actual_distance.powi(3) / (mass1 + mass2)).sqrt();
    let period_days = (period_years * 365.256) as f32;

    (ecc, incl, period_days)
}

fn generate_distance_between_stars(
    star_index: u16,
    system_index: u16,
    min_distance: f64,
    modifier: i32,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> f64 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("star_{}_{}_{}_mass", coord, system_index, star_index),
    );
    let min_distance_multiplied = if min_distance < 0.5 {
        min_distance * 6000.0
    } else if min_distance < 2.5 {
        min_distance * 600.0
    } else if min_distance < 10.0 {
        min_distance * 60.0
    } else if min_distance < 25.0 {
        min_distance * 10.0
    } else {
        min_distance * 2.0
    };
    let range = rng
        .get_result(&CopyableRollToProcess::new(
            vec![
                // Very close
                CopyableWeightedResult {
                    result: (
                        if min_distance < 15.0 {
                            min_distance
                        } else {
                            min_distance_multiplied
                        },
                        min_distance_multiplied + 0.48,
                    ),
                    weight: 3,
                },
                // Close
                CopyableWeightedResult {
                    result: (
                        min_distance_multiplied + 0.48,
                        min_distance_multiplied + 6.0,
                    ),
                    weight: 3,
                },
                // Moderate
                CopyableWeightedResult {
                    result: (
                        min_distance_multiplied + 6.0,
                        min_distance_multiplied + 72.0,
                    ),
                    weight: 3,
                },
                // Wide
                CopyableWeightedResult {
                    result: (
                        min_distance_multiplied + 72.0,
                        min_distance_multiplied + 120.0,
                    ),
                    weight: 2,
                },
                // Distant
                CopyableWeightedResult {
                    result: (
                        min_distance_multiplied + 120.0,
                        min_distance_multiplied + 600.0,
                    ),
                    weight: 3,
                },
            ],
            RollMethod::PreparedRoll(PreparedRoll::new(3, 6, modifier)),
        ))
        .expect("Should return a range to generate a the distance between two stars.");

    rng.gen_f64() % (range.1 - range.0) + range.0
}

/// Finds where the barycentre between two stars or centers of mass is.
fn calculate_barycentre(distance_between: f64, heaviest_mass: f64, lowest_mass: f64) -> f64 {
    distance_between * (lowest_mass / (heaviest_mass + lowest_mass))
}

fn generate_system_peculiarities(
    system_gen_try: u32,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> Vec<SystemPeculiarity> {
    let mut rng = SeededDiceRoller::new(
        &format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("sys_{}_{}_pec", coord, system_index),
    );
    let mut traits = Vec::new();

    // CarbonRich: ~3% chance
    if rng.roll(1, 100, 0) <= 3 {
        traits.push(SystemPeculiarity::CarbonRich);
    }

    // Cataclysm: ~7% chance
    if rng.roll(1, 100, 0) <= 7 {
        let severity = match rng.roll(1, 10, 0) {
            1..=5 => CataclysmSeverity::Minor,
            6..=8 => CataclysmSeverity::Major,
            9 => CataclysmSeverity::Extreme,
            _ => CataclysmSeverity::Ultimate,
        };
        traits.push(SystemPeculiarity::Cataclysm(severity));
    }

    // UnusualDebrisDensity: ~10% chance
    if rng.roll(1, 100, 0) <= 10 {
        let density = match rng.roll(1, 4, 0) {
            1 => DebrisDensity::MuchLower,
            2 => DebrisDensity::Lower,
            3 => DebrisDensity::Higher,
            _ => DebrisDensity::MuchHigher,
        };
        traits.push(SystemPeculiarity::UnusualDebrisDensity(density));
    }

    // Nebulae: ~5% chance
    if rng.roll(1, 100, 0) <= 5 {
        let size = match rng.roll(1, 10, 0) {
            1..=4 => NebulaeApparentSize::Tiny,
            5..=7 => NebulaeApparentSize::Small,
            8..=9 => NebulaeApparentSize::Large,
            _ => NebulaeApparentSize::Dominant,
        };
        traits.push(SystemPeculiarity::Nebulae(size));
    }

    traits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::galaxy::map::division_level::GalacticMapDivisionLevel;

    fn make_test_galaxy(seed: &str) -> Galaxy {
        let mut galaxy = Galaxy::default();
        galaxy.settings.seed = seed.into();
        galaxy.division_levels =
            GalacticMapDivisionLevel::generate_division_levels(&galaxy.settings);
        galaxy
    }

    #[test]
    fn stellar_rotation_solar_analog() {
        // Sun: G-type, 4600 Myr → ~27 days (calibrated anchor).
        let p = stellar_rotation_period(&StarSpectralType::G(2), 4600.0);
        assert!(p > 24.0 && p < 30.0, "Sun rotation out of range: {}", p);
    }

    #[test]
    fn stellar_rotation_young_vs_old() {
        // Skumanich: older late-type stars spin slower than young ones.
        let young = stellar_rotation_period(&StarSpectralType::G(2), 100.0);
        let old = stellar_rotation_period(&StarSpectralType::G(2), 10_000.0);
        assert!(old > young, "old ({}) should exceed young ({})", old, young);
    }

    #[test]
    fn stellar_rotation_no_braking_for_early_types() {
        // O/B/A stars do not spin down via magnetic braking.
        let young = stellar_rotation_period(&StarSpectralType::B(5), 10.0);
        let old = stellar_rotation_period(&StarSpectralType::B(5), 1000.0);
        assert!((young - old).abs() < f32::EPSILON);
    }

    #[test]
    fn stellar_rotation_m_dwarfs_spin_slow_when_old() {
        let old_m = stellar_rotation_period(&StarSpectralType::M(4), 10_000.0);
        assert!(
            old_m > 50.0,
            "old M dwarf should spin slowly, got {}",
            old_m
        );
    }

    #[test]
    fn generate_system_produces_objects() {
        let coord = SpaceCoordinates::new(0, 0, 0);
        let hex = GalacticHex::default();
        let div = GalacticMapDivision::default();
        let mut galaxy = make_test_galaxy("test_system_gen");

        let system = generate_star_system(0, coord, &hex, &div, &mut galaxy);
        assert!(
            !system.all_objects.is_empty(),
            "System should have at least one object"
        );
        assert!(!system.name.is_empty(), "System should have a name");
    }

    #[test]
    fn generate_multiple_systems_deterministic() {
        let coord = SpaceCoordinates::new(1, 2, 3);
        let hex = GalacticHex::default();
        let div = GalacticMapDivision::default();

        let mut g1 = make_test_galaxy("determinism_test");
        let s1 = generate_star_system(0, coord, &hex, &div, &mut g1);

        let mut g2 = make_test_galaxy("determinism_test");
        let s2 = generate_star_system(0, coord, &hex, &div, &mut g2);

        assert_eq!(
            s1.all_objects.len(),
            s2.all_objects.len(),
            "Same seed should produce same system"
        );
        assert_eq!(s1.name, s2.name);
    }

    #[test]
    fn system_has_main_star() {
        let coord = SpaceCoordinates::new(5, 5, 5);
        let hex = GalacticHex::default();
        let div = GalacticMapDivision::default();
        let mut galaxy = make_test_galaxy("star_test");

        let system = generate_star_system(0, coord, &hex, &div, &mut galaxy);
        let main_star = system
            .all_objects
            .iter()
            .find(|o| o.id == system.main_star_id);
        assert!(main_star.is_some(), "System should have a main star");
    }
}
