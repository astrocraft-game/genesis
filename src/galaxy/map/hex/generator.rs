use crate::internal::*;
use crate::prelude::*;

impl GalacticHex {
    /// Generates the [GalacticHex] at the given coordinates.
    pub fn generate(coord: SpaceCoordinates, index: SpaceCoordinates, galaxy: &mut Galaxy) -> Self {
        debug!(
            "generating new hex (seed: {}, coord: {})",
            galaxy.settings.seed, coord
        );
        let contents = Vec::new();
        let neighborhood = StellarNeighborhood { age: system::neighborhood::types::StellarNeighborhoodAge::Mature };
        let mut generated = Self {
            index,
            neighborhood,
            contents,
        };

        let number_of_systems_to_generate = get_number_of_systems_to_generate(galaxy, index, coord);
        for i in 0..number_of_systems_to_generate {
            // Convert main-crate types to system-crate stub types at the boundary
            let sys_coord = system::galaxy_stubs::SpaceCoordinates::new(coord.x, coord.y, coord.z);
            let sys_hex = to_sys_hex(&generated);
            let sys_div = to_sys_division(&galaxy.get_division_at_level(coord, 1).expect("Should return a subsector."));
            let mut sys_galaxy = to_sys_galaxy(galaxy);

            generated.contents.push(system::generator::generate_star_system(
                i,
                sys_coord,
                &sys_hex,
                &sys_div,
                &mut sys_galaxy,
            ));
        }

        debug!("generated: {}", generated);
        generated
    }
}

/// Convert main crate Galaxy → system crate stub Galaxy
fn to_sys_galaxy(galaxy: &Galaxy) -> system::galaxy_stubs::Galaxy {
    let mut sys_gal = system::galaxy_stubs::Galaxy::default();
    sys_gal.settings.seed = galaxy.settings.seed.clone();
    sys_gal.settings.system = galaxy.settings.system.clone();
    sys_gal.settings.star = galaxy.settings.star.clone();
    sys_gal.settings.celestial_body = galaxy.settings.celestial_body.clone();
    sys_gal.age = galaxy.age;
    sys_gal.is_dominant = galaxy.is_dominant;
    sys_gal.is_major = galaxy.is_major;
    // Map category
    sys_gal.category = match galaxy.category {
        GalaxyCategory::Intergalactic(a, b, c) => system::galaxy_stubs::GalaxyCategory::Intergalactic(a, b, c),
        GalaxyCategory::Irregular(a, b, c) => system::galaxy_stubs::GalaxyCategory::Irregular(a, b, c),
        GalaxyCategory::Spiral(a, b) => system::galaxy_stubs::GalaxyCategory::Spiral(a, b),
        GalaxyCategory::Lenticular(a, b) => system::galaxy_stubs::GalaxyCategory::Lenticular(a, b),
        GalaxyCategory::Elliptical(a) => system::galaxy_stubs::GalaxyCategory::Elliptical(a),
        GalaxyCategory::Intracluster(a, b, c) => system::galaxy_stubs::GalaxyCategory::Intracluster(a, b, c),
        GalaxyCategory::DominantElliptical(a) => system::galaxy_stubs::GalaxyCategory::DominantElliptical(a),
    };
    sys_gal.sub_category = match galaxy.sub_category {
        GalaxySubCategory::DwarfAmorphous => system::galaxy_stubs::GalaxySubCategory::DwarfAmorphous,
        GalaxySubCategory::DwarfSpiral => system::galaxy_stubs::GalaxySubCategory::DwarfSpiral,
        GalaxySubCategory::DwarfLenticular => system::galaxy_stubs::GalaxySubCategory::DwarfLenticular,
        GalaxySubCategory::DwarfElliptical => system::galaxy_stubs::GalaxySubCategory::DwarfElliptical,
        GalaxySubCategory::FlatSpiral => system::galaxy_stubs::GalaxySubCategory::FlatSpiral,
        GalaxySubCategory::BarredSpiral => system::galaxy_stubs::GalaxySubCategory::BarredSpiral,
        GalaxySubCategory::ClassicSpiral => system::galaxy_stubs::GalaxySubCategory::ClassicSpiral,
        GalaxySubCategory::CommonLenticular => system::galaxy_stubs::GalaxySubCategory::CommonLenticular,
        GalaxySubCategory::GiantLenticular => system::galaxy_stubs::GalaxySubCategory::GiantLenticular,
        GalaxySubCategory::CommonElliptical => system::galaxy_stubs::GalaxySubCategory::CommonElliptical,
        GalaxySubCategory::GiantElliptical => system::galaxy_stubs::GalaxySubCategory::GiantElliptical,
        _ => system::galaxy_stubs::GalaxySubCategory::DwarfAmorphous,
    };
    sys_gal.neighborhood.universe.age = galaxy.neighborhood.universe.age;
    sys_gal.neighborhood.universe.era = match galaxy.neighborhood.universe.era {
        StelliferousEra::AncientStelliferous => system::galaxy_stubs::StelliferousEra::AncientStelliferous,
        StelliferousEra::EarlyStelliferous => system::galaxy_stubs::StelliferousEra::EarlyStelliferous,
        StelliferousEra::MiddleStelliferous => system::galaxy_stubs::StelliferousEra::MiddleStelliferous,
        StelliferousEra::LateStelliferous => system::galaxy_stubs::StelliferousEra::LateStelliferous,
        StelliferousEra::EndStelliferous => system::galaxy_stubs::StelliferousEra::EndStelliferous,
    };
    sys_gal
}

fn to_sys_hex(hex: &GalacticHex) -> system::galaxy_stubs::GalacticHex {
    system::galaxy_stubs::GalacticHex {
        index: system::galaxy_stubs::SpaceCoordinates::new(hex.index.x, hex.index.y, hex.index.z),
        neighborhood: system::neighborhood::StellarNeighborhood { age: system::neighborhood::types::StellarNeighborhoodAge::Mature },
        contents: vec![],
    }
}

fn to_sys_division(div: &GalacticMapDivision) -> system::galaxy_stubs::GalacticMapDivision {
    let mut d = system::galaxy_stubs::GalacticMapDivision::default();
    d.level = div.level;
    d.x = div.x;
    d.y = div.y;
    d.z = div.z;
    d
}

/// Calculates how many systems should be generated using the expected stellar distribution of the hex.
fn get_number_of_systems_to_generate(
    galaxy: &mut Galaxy,
    index: SpaceCoordinates,
    coord: SpaceCoordinates,
) -> u16 {
    let mut rng = SeededDiceRoller::new(&galaxy.settings.seed, &format!("hex_{}_nbr_sys", index));
    let mut number_of_systems_to_generate = 0;
    let success_on;
    let to_roll: PreparedRoll;

    let turns = if galaxy.settings.sector.density_by_hex_instead_of_parsec {
        1
    } else {
        let hex_size = galaxy.settings.sector.hex_size;
        hex_size.0 * hex_size.1 * hex_size.2
    };

    let region = galaxy
        .get_division_at_level(coord, 1)
        .expect("Should return a subsector.")
        .region;
    match region {
        GalacticRegion::Void => { to_roll = PreparedRoll::new(1, 50, 0); success_on = 1; }
        GalacticRegion::Aura => { to_roll = PreparedRoll::new(1, 20, 0); success_on = 1; }
        GalacticRegion::Halo | GalacticRegion::Exile => { to_roll = PreparedRoll::new(1, 10, 0); success_on = 1; }
        GalacticRegion::Stream | GalacticRegion::Association => { to_roll = PreparedRoll::new(1, 5, 0); success_on = 1; }
        GalacticRegion::Ellipse | GalacticRegion::Disk | GalacticRegion::Multiple => { to_roll = PreparedRoll::new(1, 2, 0); success_on = 1; }
        GalacticRegion::Arm | GalacticRegion::OpenCluster => { to_roll = PreparedRoll::new(1, 4, 0); success_on = 3; }
        GalacticRegion::Bar => { to_roll = PreparedRoll::new(1, 20, 0); success_on = 19; }
        GalacticRegion::Bulge | GalacticRegion::GlobularCluster => { to_roll = PreparedRoll::new(1, 100, 0); success_on = 99; }
        GalacticRegion::Core | GalacticRegion::Nucleus => { to_roll = PreparedRoll::new(1, 500, 0); success_on = 499; }
    };

    for _ in 0..turns {
        let roll = rng.roll_prepared(&to_roll);
        if roll <= success_on { number_of_systems_to_generate += roll; }
    }

    rng = SeededDiceRoller::new(&galaxy.settings.seed, &format!("hex_{}_nbr_brwn", index));
    let mut number_of_brown_dwarfs = 0;
    for _ in 0..turns {
        let roll = rng.roll_prepared(&to_roll);
        if roll <= success_on { number_of_brown_dwarfs += roll; }
    }
    number_of_systems_to_generate += number_of_brown_dwarfs / 5;

    if galaxy.settings.sector.max_one_system_per_hex && number_of_systems_to_generate > 1 {
        number_of_systems_to_generate = 1;
    }

    number_of_systems_to_generate as u16
}
