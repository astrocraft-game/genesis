use crate::internal::*;
use crate::prelude::*;
#[path = "./constants.rs"]
mod constants;
use crate::contents::elements::generate_random_common_element;
use crate::contents::elements::generate_random_element;
use crate::contents::elements::generate_random_non_metal_element;
use crate::contents::elements::ChemicalComponent;
use crate::contents::elements::ALL_ELEMENTS;
use crate::contents::elements::MOST_COMMON_ELEMENTS;
use constants::*;

impl Star {
    /// Generates a new star.
    pub fn generate(
        system_gen_try: u32,
        star_index: u16,
        system_index: u16,
        system_name: Rc<str>,
        coord: SpaceCoordinates,
        population: StellarEvolution,
        hex: &GalacticHex,
        galaxy: &Galaxy,
        settings: &GenerationSettings,
    ) -> Self {
        let seed: Rc<str> = format!("{}{}", system_gen_try, &galaxy.settings.seed).into();
        let age = if settings.star.fixed_age.is_some() {
            settings.star.fixed_age.unwrap() * 1000.0
        } else {
            generate_age(
                star_index,
                system_index,
                coord,
                hex,
                &seed,
                &galaxy.neighborhood.universe,
            )
        };

        let mut mass: f64 = if settings.star.fixed_mass.is_some() {
            settings.star.fixed_mass.unwrap()
        } else {
            let generated_mass: f64 =
                generate_mass(star_index, system_index, coord, &seed, &settings.star);
            simulate_mass_loss_over_the_years(generated_mass, age)
        };

        // Main sequence estimations
        let ms_luminosity = calculate_main_sequence_luminosity(mass);
        let ms_radius = calculate_radius(mass, 0.0, 1.0, 0.0, 0.0, 0, 0, coord, galaxy);
        let ms_temperature =
            calculate_temperature_using_luminosity(ms_luminosity, ms_radius as f64) as u32;

        let mut main_lifespan = calculate_lifespan(mass, ms_luminosity);
        main_lifespan = adjust_lifespan_to_population(main_lifespan, population);
        let subgiant_lifespan = calculate_subgiant_lifespan(mass, main_lifespan);
        let giant_lifespan = calculate_giant_lifespan(mass, main_lifespan);
        let full_lifespan = main_lifespan + subgiant_lifespan + giant_lifespan;

        let age_range = get_age_range_in_star_lifecycle_dataset(
            age,
            main_lifespan,
            subgiant_lifespan,
            giant_lifespan,
        );

        let mut radius: f64;
        let mut luminosity: f32;
        let temperature: u32;
        let spectral_type: StarSpectralType;
        let luminosity_class: StarLuminosityClass;
        mass = adjust_mass_to_population(mass, population);

        if age_range > 6.0 {
            // If remnant
            mass = calculate_remnant_mass(mass, settings);

            if mass < 1.4 {
                // White dwarf
                radius = calculate_white_dwarf_radius(mass);
                let initial_luminosity = calculate_white_dwarf_initial_luminosity(mass);
                let initial_temperature =
                    calculate_temperature_using_luminosity(initial_luminosity, radius);
                temperature = calculate_white_dwarf_temperature(initial_temperature, age);
                luminosity = calculate_luminosity_using_temperature(temperature, radius);
                spectral_type =
                    generate_white_dwarf_spectral_type(star_index, system_index, coord, &seed);
                luminosity_class = StarLuminosityClass::VII;
            } else if mass < 3.2 {
                // Neutron star
                let precise_radius = calculate_precise_radius_of_neutron_star_or_black_hole(mass);
                temperature = calculate_neutron_star_temperature(age, full_lifespan);
                luminosity = calculate_luminosity_using_temperature(temperature, precise_radius);
                radius = precise_radius;
                spectral_type = StarSpectralType::XNS;
                luminosity_class = StarLuminosityClass::XNS;
            } else {
                // Black hole
                let precise_radius = calculate_precise_radius_of_neutron_star_or_black_hole(mass);
                temperature = 0;
                luminosity = 0.0;
                radius = precise_radius;
                spectral_type = StarSpectralType::XBH;
                luminosity_class = StarLuminosityClass::XBH;
            }
        } else {
            // If main sequence, subgiant or giant
            let mass_range = get_mass_range_in_star_lifecycle_dataset(mass);
            let nearest_values = get_nearest_star_lifecycle_dataset_cells(age_range, mass_range);

            // Compute interpolated values
            let interpolated_temperature = get_interpolated_temperature(
                mass,
                ms_temperature,
                nearest_values,
                age_range,
                mass_range,
            );
            let interpolated_lum_factor =
                get_interpolated_luminosity_factor(nearest_values, age_range, mass_range);
            let interpolated_luminosity =
                get_interpolated_luminosity(mass, ms_luminosity, interpolated_lum_factor);
            let interpolated_radius = get_interpolated_radius(
                mass,
                ms_radius,
                interpolated_luminosity,
                interpolated_temperature,
            );

            // Then mix main sequence and interpolated values if applicable
            radius = mix_values(ms_radius, interpolated_radius as f64, age, main_lifespan) as f64;
            luminosity = mix_values(
                ms_luminosity as f64,
                interpolated_luminosity as f64,
                age,
                main_lifespan,
            ) as f32;
            temperature = mix_values(
                ms_temperature as f64,
                interpolated_temperature as f64,
                age,
                main_lifespan,
            ) as u32;

            spectral_type = if let Some(fixed) = settings.star.fixed_spectral_type {
                fixed
            } else {
                calculate_spectral_type(temperature)
            };
            luminosity_class = if let Some(fixed) = settings.star.fixed_luminosity_class {
                fixed
            } else {
                calculate_luminosity_class(
                    luminosity,
                    spectral_type,
                    age,
                    main_lifespan,
                    subgiant_lifespan,
                )
            };
        }

        radius = adjust_radius_to_population(radius, population);
        luminosity = adjust_luminosity_to_population(luminosity, population);

        let name = get_star_name(star_index, system_name.clone(), settings);

        let mut special_traits = generate_star_peculiarities(
            system_gen_try,
            star_index,
            system_index,
            coord,
            population,
            spectral_type,
            age / 1000.0,
            galaxy,
        );
        if population == StellarEvolution::Paleodwarf
            && !special_traits.contains(&StarPeculiarity::NoMetals)
        {
            special_traits.push(StarPeculiarity::NoMetals);
        }

        // Element abundance is modulated by stellar population (a discrete
        // metallicity proxy): Paleodwarf systems are dominated by non-metals,
        // Hyperdwarf systems are metal-rich. A continuous [Fe/H] variable
        // could refine this further as future work.
        let elements_abundance: Vec<ChemicalComponent> = {
            let mut rng = SeededDiceRoller::new(
                &settings.seed,
                &format!("sys_{}_{}_{}_elem_abnd", coord, system_index, star_index),
            );
            let mut elements = Vec::new();
            let mut roll = rng.gen_u8();

            let random_element_abundance_threshold = 156;
            let non_metal_threshold = match population {
                StellarEvolution::Paleodwarf => 0,
                StellarEvolution::Subdwarf => 126,
                StellarEvolution::Dwarf => 240,
                StellarEvolution::Superdwarf => 250,
                StellarEvolution::Hyperdwarf => 255,
            };
            let common_threshold = match population {
                StellarEvolution::Paleodwarf => 0,
                StellarEvolution::Subdwarf => 56,
                StellarEvolution::Dwarf => 126,
                StellarEvolution::Superdwarf => 156,
                StellarEvolution::Hyperdwarf => 180,
            };

            while roll >= random_element_abundance_threshold {
                let specific_roll = rng.gen_u8();
                if specific_roll >= non_metal_threshold {
                    let el = generate_random_non_metal_element(&mut rng);
                    if !elements.contains(&el) {
                        elements.push(el);
                    }
                } else if specific_roll >= common_threshold {
                    let el = generate_random_element(&mut rng);
                    if !elements.contains(&el) {
                        elements.push(el);
                    }
                } else {
                    let el = generate_random_common_element(&mut rng);
                    if !elements.contains(&el) {
                        elements.push(el);
                    }
                }
                roll = rng.gen_u8();
            }
            elements
        };
        let elements_lack: Vec<ChemicalComponent> = {
            let mut rng = SeededDiceRoller::new(
                &seed,
                &format!("sys_{}_{}_{}_elem_lack", coord, system_index, star_index),
            );
            let mut elements = Vec::new();
            let mut roll = rng.gen_u8();

            let random_element_lack_threshold = 156;
            let non_metal_threshold = match population {
                StellarEvolution::Paleodwarf => 0,
                StellarEvolution::Subdwarf => 236,
                StellarEvolution::Dwarf => 200,
                StellarEvolution::Superdwarf => 156,
                StellarEvolution::Hyperdwarf => 100,
            };
            let common_threshold = match population {
                StellarEvolution::Paleodwarf => 0,
                StellarEvolution::Subdwarf => 56,
                StellarEvolution::Dwarf => 126,
                StellarEvolution::Superdwarf => 156,
                StellarEvolution::Hyperdwarf => 180,
            };

            if population != StellarEvolution::Paleodwarf {
                while roll >= random_element_lack_threshold {
                    let specific_roll = rng.gen_u8();
                    if specific_roll >= non_metal_threshold {
                        let el = generate_random_non_metal_element(&mut rng);
                        if !elements.contains(&el) {
                            elements.push(el);
                        }
                    } else if specific_roll >= common_threshold {
                        let el = generate_random_element(&mut rng);
                        if !elements.contains(&el) {
                            elements.push(el);
                        }
                    } else {
                        let el = generate_random_common_element(&mut rng);
                        if !elements.contains(&el) {
                            elements.push(el);
                        }
                    }
                    roll = rng.gen_u8()
                }
            }
            elements
        };
        let mut rng = SeededDiceRoller::new(
            &seed,
            &format!("sys_{}_{}_{}_elem_comp", coord, system_index, star_index),
        );

        elements_abundance.iter().for_each(|el| {
            let roll = rng.roll(1, 12, 0);
            if roll <= 7 {
                special_traits.push(StarPeculiarity::UnusualElementPresence((
                    *el,
                    ElementPresenceOccurrence::High,
                )));
            } else if roll <= 11 {
                special_traits.push(StarPeculiarity::UnusualElementPresence((
                    *el,
                    ElementPresenceOccurrence::VeryHigh,
                )));
            } else {
                special_traits.push(StarPeculiarity::UnusualElementPresence((
                    *el,
                    ElementPresenceOccurrence::Omnipresence,
                )));
            }
        });
        elements_lack.iter().for_each(|el| {
            let roll = rng.roll(1, 12, 0);
            if roll <= 7 {
                special_traits.push(StarPeculiarity::UnusualElementPresence((
                    *el,
                    ElementPresenceOccurrence::Low,
                )));
            } else if roll <= 11 {
                special_traits.push(StarPeculiarity::UnusualElementPresence((
                    *el,
                    ElementPresenceOccurrence::VeryLow,
                )));
            } else {
                special_traits.push(StarPeculiarity::UnusualElementPresence((
                    *el,
                    ElementPresenceOccurrence::Absence,
                )));
            }
        });

        let absolute_magnitude = crate::star::absolute_magnitude_from_luminosity(luminosity);
        let color_bv = crate::star::temperature_to_bv(temperature);
        let flare_activity = crate::star::compute_flare_activity(&spectral_type, age / 1000.0);

        Self {
            name,
            mass,
            luminosity,
            radius,
            age: age / 1000.0,
            temperature,
            population,
            spectral_type,
            luminosity_class,
            special_traits,
            orbital_point_id: star_index as u32,
            orbit: None,
            zones: vec![],
            absolute_magnitude,
            color_bv,
            flare_activity,
        }
    }
}

/// Returns the name of the star by combining its index and the system name.
fn get_star_name(star_index: u16, name: Rc<str>, settings: &GenerationSettings) -> Rc<str> {
    if settings.star.use_ours {
        "Sun".into()
    } else {
        format!("{} {}", name, star_index + 1).into()
    }
}

/// T = T_sun * (L/L_sun)^(1/4) / (R/R_sun)^(1/2)
/// All inputs in solar units, output in Kelvin.
fn calculate_temperature_using_luminosity(luminosity: f32, radius: f64) -> f32 {
    const T_SUN: f64 = 5772.0;
    (T_SUN * (luminosity as f64).powf(0.25) / radius.sqrt()) as f32
}

/// L/L_sun = (R/R_sun)^2 * (T/T_sun)^4
/// Radius in solar radii, temperature in Kelvin, output in solar luminosities.
fn calculate_luminosity_using_temperature(temperature: u32, radius: f64) -> f32 {
    const T_SUN: f64 = 5772.0;
    (radius.powi(2) * (temperature as f64 / T_SUN).powi(4)) as f32
}

/// R/R_sun = (L/L_sun)^(1/2) / (T/T_sun)^2
/// Luminosity in solar luminosities, temperature in Kelvin, output in solar radii.
fn calculate_radius_using_luminosity_and_temperature(luminosity: f32, temperature: u32) -> f64 {
    const T_SUN: f64 = 5772.0;
    (luminosity as f64).sqrt() / (temperature as f64 / T_SUN).powi(2)
}

fn generate_mass(
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    seed: &str,
    settings: &StarSettings,
) -> f64 {
    let mut rng = SeededDiceRoller::new(
        seed,
        &format!("star_{}_{}_{}_mass", coord, system_index, star_index),
    );
    let range = rng
        .get_result(&CopyableRollToProcess::new(
            vec![
                // Brown dwarf
                CopyableWeightedResult {
                    result: (
                        BROWN_DWARF_MIN_MASS,
                        RED_DWARF_POP_HYPERDWARF_MIN_MASS - 0.001,
                    ),
                    weight: settings.brown_dwarf_gen_chance,
                },
                // Red dwarf Pop 0
                CopyableWeightedResult {
                    result: (
                        RED_DWARF_POP_HYPERDWARF_MIN_MASS,
                        RED_DWARF_POP_DWARF_MIN_MASS - 0.001,
                    ),
                    weight: settings.red_dwarf_one_gen_chance,
                },
                // Red dwarf Pop I
                CopyableWeightedResult {
                    result: (
                        RED_DWARF_POP_DWARF_MIN_MASS,
                        RED_DWARF_POP_SUBDWARF_MIN_MASS - 0.001,
                    ),
                    weight: settings.red_dwarf_two_gen_chance,
                },
                // Red dwarf Pop II
                CopyableWeightedResult {
                    result: (RED_DWARF_POP_SUBDWARF_MIN_MASS, 0.25),
                    weight: settings.red_dwarf_three_gen_chance,
                },
                CopyableWeightedResult {
                    result: (0.251, RED_DWARF_POP_PALEODWARF_MIN_MASS - 0.001),
                    weight: settings.red_dwarf_four_gen_chance,
                },
                // Red dwarf Pop III
                CopyableWeightedResult {
                    result: (
                        RED_DWARF_POP_PALEODWARF_MIN_MASS,
                        ORANGE_DWARF_MIN_MASS - 0.001,
                    ),
                    weight: settings.red_dwarf_five_gen_chance,
                },
                // Orange
                CopyableWeightedResult {
                    result: (ORANGE_DWARF_MIN_MASS, YELLOW_DWARF_MIN_MASS - 0.001),
                    weight: settings.orange_dwarf_gen_chance,
                },
                // Yellow
                CopyableWeightedResult {
                    result: (YELLOW_DWARF_MIN_MASS, WHITE_DWARF_MIN_MASS - 0.001),
                    weight: settings.yellow_dwarf_gen_chance,
                },
                // White
                CopyableWeightedResult {
                    result: (WHITE_DWARF_MIN_MASS, WHITE_GIANT_MIN_MASS - 0.001),
                    weight: settings.white_star_gen_chance,
                },
                // Giants
                CopyableWeightedResult {
                    result: (WHITE_GIANT_MIN_MASS, BLUE_GIANT_MIN_MASS - 0.001),
                    weight: settings.blue_star_one_gen_chance,
                },
                // Blue giants
                CopyableWeightedResult {
                    result: (BLUE_GIANT_MIN_MASS, 20.0),
                    weight: settings.blue_star_two_gen_chance,
                },
                CopyableWeightedResult {
                    result: (20.001, BLUE_GIANT_POP_HYPERDWARF_MAX_MASS),
                    weight: settings.blue_star_three_gen_chance,
                },
                // Pop I
                CopyableWeightedResult {
                    result: (
                        BLUE_GIANT_POP_HYPERDWARF_MAX_MASS + 0.001,
                        BLUE_GIANT_POP_DWARF_MAX_MASS,
                    ),
                    weight: settings.violet_star_one_gen_chance,
                },
                // Pop II
                CopyableWeightedResult {
                    result: (
                        BLUE_GIANT_POP_DWARF_MAX_MASS + 0.001,
                        BLUE_GIANT_POP_SUBDWARF_MAX_MASS,
                    ),
                    weight: settings.violet_star_two_gen_chance,
                },
                // Pop III
                CopyableWeightedResult {
                    result: (
                        BLUE_GIANT_POP_SUBDWARF_MAX_MASS + 0.001,
                        BLUE_GIANT_POP_PALEODWARF_MAX_MASS,
                    ),
                    weight: settings.violet_star_three_gen_chance,
                },
            ],
            RollMethod::SimpleRoll,
        ))
        .expect("Should return a range to generate a star's mass.");
    rng.gen_f64() % (range.1 - range.0) + range.0
}

fn adjust_mass_to_population(mass: f64, population: StellarEvolution) -> f64 {
    match population {
        StellarEvolution::Hyperdwarf => {
            if mass > BLUE_GIANT_POP_HYPERDWARF_MAX_MASS {
                BLUE_GIANT_POP_HYPERDWARF_MAX_MASS
            } else {
                mass
            }
        }
        StellarEvolution::Superdwarf => {
            if mass > BLUE_GIANT_POP_SUPERDWARF_MAX_MASS {
                BLUE_GIANT_POP_SUPERDWARF_MAX_MASS
            } else {
                mass
            }
        }
        StellarEvolution::Subdwarf => {
            if mass > BLUE_GIANT_POP_SUBDWARF_MAX_MASS {
                BLUE_GIANT_POP_SUBDWARF_MAX_MASS
            } else {
                mass
            }
        }
        StellarEvolution::Paleodwarf => {
            if mass > BLUE_GIANT_POP_PALEODWARF_MAX_MASS {
                BLUE_GIANT_POP_PALEODWARF_MAX_MASS
            } else {
                mass
            }
        }
        _ => mass,
    }
}

fn adjust_lifespan_to_population(lifespan: f32, population: StellarEvolution) -> f32 {
    match population {
        StellarEvolution::Hyperdwarf => lifespan * 0.5,
        StellarEvolution::Superdwarf => lifespan * 2.0,
        StellarEvolution::Subdwarf => lifespan * 0.5,
        StellarEvolution::Paleodwarf => lifespan * 0.1,
        _ => lifespan,
    }
}

fn adjust_radius_to_population(radius: f64, population: StellarEvolution) -> f64 {
    match population {
        StellarEvolution::Hyperdwarf => radius * 1.5,
        StellarEvolution::Superdwarf => radius * 1.25,
        StellarEvolution::Subdwarf => radius * 0.75,
        StellarEvolution::Paleodwarf => radius * 0.5,
        _ => radius,
    }
}

fn adjust_luminosity_to_population(luminosity: f32, population: StellarEvolution) -> f32 {
    match population {
        StellarEvolution::Hyperdwarf => luminosity * 0.5,
        StellarEvolution::Superdwarf => luminosity * 0.75,
        // Subdwarfs (Population II) are metal-poor, less opaque, and thus less
        // luminous than main sequence stars of the same mass (~0.4-0.6x).
        StellarEvolution::Subdwarf => luminosity * 0.5,
        StellarEvolution::Paleodwarf => luminosity * 0.3,
        _ => luminosity,
    }
}

/// Reduces mass towards 150 solar masses if higher, as a star that is bigger than that blows off its mass as solar wind until it gets to 150.
fn simulate_mass_loss_over_the_years(mass: f64, age: f32) -> f64 {
    if mass > 150.0 {
        150.0_f64.max(mass - age as f64)
    } else {
        mass
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_star_peculiarities(
    system_gen_try: u32,
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    population: StellarEvolution,
    spectral_type: StarSpectralType,
    age_gyr: f32,
    galaxy: &Galaxy,
) -> Vec<StarPeculiarity> {
    let mut rng = SeededDiceRoller::new(
        &format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("star_{}_{}_{}_pec", coord, system_index, star_index),
    );
    let mut traits = Vec::new();

    let is_hot = matches!(
        spectral_type,
        StarSpectralType::O(_) | StarSpectralType::B(_) | StarSpectralType::WR(_)
    );

    // ChaoticOrbits: ~3%
    if rng.roll(1, 100, 0) <= 3 {
        traits.push(StarPeculiarity::ChaoticOrbits);
    }

    // ExcessiveRadiation: 5% for hot stars, 1% otherwise
    let rad_chance = if is_hot { 5 } else { 1 };
    if rng.roll(1, 100, 0) <= rad_chance {
        traits.push(StarPeculiarity::ExcessiveRadiation);
    }

    // RotationAnomaly: ~5%
    if rng.roll(1, 100, 0) <= 5 {
        let speed = match rng.roll(1, 4, 0) {
            1 => RotationAnomalySpeed::MuchSlower,
            2 => RotationAnomalySpeed::Slower,
            3 => RotationAnomalySpeed::Faster,
            _ => RotationAnomalySpeed::MuchFaster,
        };
        traits.push(StarPeculiarity::RotationAnomaly(speed));
    }

    // UnusualMetallicity: ~8%
    if rng.roll(1, 100, 0) <= 8 {
        let met = match population {
            StellarEvolution::Paleodwarf | StellarEvolution::Subdwarf => match rng.roll(1, 4, 0) {
                1 => StarMetallicityDifference::MuchLower,
                2 => StarMetallicityDifference::Lower,
                3 => StarMetallicityDifference::Higher,
                _ => StarMetallicityDifference::MuchHigher,
            },
            _ => match rng.roll(1, 4, 0) {
                1 => StarMetallicityDifference::Lower,
                2 => StarMetallicityDifference::MuchLower,
                3 => StarMetallicityDifference::Higher,
                _ => StarMetallicityDifference::MuchHigher,
            },
        };
        traits.push(StarPeculiarity::UnusualMetallicity(met));
    }

    // PowerfulStellarWinds: 5% for hot, 1% otherwise
    let wind_chance = if is_hot { 5 } else { 1 };
    if rng.roll(1, 100, 0) <= wind_chance {
        traits.push(StarPeculiarity::PowerfulStellarWinds);
    }

    // StrongMagneticField: ~4%
    if rng.roll(1, 100, 0) <= 4 {
        traits.push(StarPeculiarity::StrongMagneticField);
    }

    // VariableStar: ~6%
    if rng.roll(1, 100, 0) <= 6 {
        let interval = match rng.roll(1, 7, 0) {
            1 => VariableStarInterval::Minutes,
            2 => VariableStarInterval::Hours,
            3 => VariableStarInterval::Days,
            4 => VariableStarInterval::Months,
            5 => VariableStarInterval::Years,
            6 => VariableStarInterval::Decades,
            _ => VariableStarInterval::Centuries,
        };
        traits.push(StarPeculiarity::VariableStar(interval));
    }

    // CircumstellarDisk: ~10% for young systems (< 1 Gyr)
    if age_gyr < 1.0 && rng.roll(1, 100, 0) <= 10 {
        traits.push(StarPeculiarity::CircumstellarDisk);
    }

    // Limit to max 3 peculiarities
    traits.truncate(3);
    traits
}

fn calculate_radius(
    mass: f64,
    age: f32,
    main_lifespan: f32,
    subgiant_lifespan: f32,
    giant_lifespan: f32,
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> f64 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("star_{}_{}_{}_radius", coord, system_index, star_index),
    );
    let mut radius = mass.powf(0.8);

    let rand_multiplier = rng.roll(1, 4666, 999) as f64 / 10000.0;
    if age < main_lifespan {
        // Do nothing
    } else if age < main_lifespan + subgiant_lifespan {
        // Subgiant
        radius = radius * rand_multiplier * 1.5;
    } else if age < main_lifespan + subgiant_lifespan + giant_lifespan {
        // Giant
        radius = radius * rand_multiplier * 3.0;
    } else {
        // Remnant
        if mass < 8.0 {
            // White dwarf
            radius /= 60.0;
        } else if mass < 50.0 {
            // Neutron star
            radius = 0.001_f64.max((mass / (mass - 6.0) + mass) / 20000.0);
        } else {
            // Black hole
            radius = mass / 33333.33333;
        }
    }
    (radius * 1000.0).round() / 1000.0
}

fn calculate_main_sequence_luminosity(mass: f64) -> f32 {
    (if mass <= 0.27 {
        0.0002 + f64::powf(mass, 3.0)
    } else if mass <= 0.45 {
        0.8 * f64::powf(mass, 3.0)
    } else if mass <= 0.6 {
        0.66 * f64::powf(mass, 3.0)
    } else if mass <= 0.8 {
        0.56 * f64::powf(mass, 3.0)
    } else if mass <= 0.9 {
        f64::powf(mass, 3.0) - 0.25
    } else if mass <= 1.0 {
        mass - 0.36
    } else if mass <= 1.05 {
        mass - 0.18
    } else if mass <= 1.1 {
        mass
    } else if mass <= 1.2 {
        f64::powf(mass, 3.0)
    } else if mass <= 1.4 {
        f64::powf(mass, 3.9)
    } else if mass <= 2.0 {
        f64::powf(mass, 4.0)
    } else if mass <= 55.0 {
        1.4 * f64::powf(mass, 3.5)
    } else {
        32000.0 * mass
    }) as f32
}

/// In millions of years.
fn generate_age(
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    seed: &str,
    universe: &Universe,
) -> f32 {
    let mut rng = SeededDiceRoller::new(
        seed,
        &format!("star_{}_{}_{}_age", coord, system_index, star_index),
    );
    let mut age = if let StellarNeighborhoodAge::Ancient(years)
    | StellarNeighborhoodAge::Old(years)
    | StellarNeighborhoodAge::Young(years) = hex.neighborhood.age
    {
        years as f32
    } else if universe.era == StelliferousEra::AncientStelliferous
        || universe.era == StelliferousEra::EarlyStelliferous
    {
        ((universe.age * 1000.0) - 300.0)
            .min(((universe.age) * 1000.0) - rng.roll(1, 9000, 0) as f32)
    } else {
        rng.roll(1, 9000, 999) as f32
    };
    age = if age >= universe.age * 1000.0 - 40.0 {
        universe.age * 1000.0 - 40.0
    } else if age < 1.0 {
        1.0
    } else {
        age
    };
    age
}

/// In millions of years.
fn calculate_lifespan(mass: f64, luminosity: f32) -> f32 {
    f32::powi(10.0, 4) * mass as f32 / luminosity
}

fn calculate_subgiant_lifespan(mass: f64, main_lifespan: f32) -> f32 {
    if mass > RED_DWARF_POP_PALEODWARF_MIN_MASS {
        main_lifespan * 0.15
    } else {
        0.0
    }
}

fn calculate_giant_lifespan(mass: f64, main_lifespan: f32) -> f32 {
    if mass > RED_DWARF_POP_PALEODWARF_MIN_MASS {
        main_lifespan * 0.0917
    } else {
        0.0
    }
}

fn get_interpolated_radius(
    mass: f64,
    ms_radius: f64,
    interpolated_luminosity: f32,
    interpolated_temperature: u32,
) -> f64 {
    if mass < 0.4 {
        ms_radius
    } else {
        calculate_radius_using_luminosity_and_temperature(
            interpolated_luminosity,
            interpolated_temperature,
        )
    }
}

fn get_interpolated_luminosity(mass: f64, ms_luminosity: f32, interpolated_lum_factor: f32) -> f32 {
    if mass < 0.4 {
        ms_luminosity
    } else {
        f32::powf(10.0, interpolated_lum_factor)
    }
}

fn get_interpolated_luminosity_factor(
    nearest_values: [TemperatureAndLuminosity; 4],
    age_range: f32,
    mass_range: f32,
) -> f32 {
    interpolate_f32(
        nearest_values[0].1,
        nearest_values[1].1,
        nearest_values[2].1,
        nearest_values[3].1,
        age_range,
        mass_range,
    )
}

fn get_interpolated_temperature(
    mass: f64,
    ms_temperature: u32,
    nearest_values: [TemperatureAndLuminosity; 4],
    age_range: f32,
    mass_range: f32,
) -> u32 {
    if mass < 0.4 {
        ms_temperature
    } else {
        interpolate_f32(
            nearest_values[0].0,
            nearest_values[1].0,
            nearest_values[2].0,
            nearest_values[3].0,
            age_range,
            mass_range,
        ) as u32
    }
}

fn calculate_spectral_type(temperature: u32) -> StarSpectralType {
    // Find the two temperatures in the dataset that the given temperature is between
    let (lower_temp, lower_class) = TEMPERATURE_TO_SPECTRAL_TYPE_DATASET
        .iter()
        .find(|&(t, _)| *t <= temperature)
        .unwrap();
    let (upper_temp, upper_class) = TEMPERATURE_TO_SPECTRAL_TYPE_DATASET
        .iter()
        .rev()
        .find(|&(t, _)| *t > temperature)
        .unwrap();

    // Interpolate the class value between the two nearest temperatures
    let class_as_int: u32 = (*lower_class as f32
        + (temperature as f32 - *lower_temp as f32) * (*upper_class as f32 - *lower_class as f32)
            / (*upper_temp as f32 - *lower_temp as f32)) as u32;

    // Convert the class value to the spectral type

    match class_as_int / 10 {
        0 => StarSpectralType::WR((class_as_int % 10) as u8),
        1 => StarSpectralType::O((class_as_int % 10) as u8),
        2 => StarSpectralType::B((class_as_int % 10) as u8),
        3 => StarSpectralType::A((class_as_int % 10) as u8),
        4 => StarSpectralType::F((class_as_int % 10) as u8),
        5 => StarSpectralType::G((class_as_int % 10) as u8),
        6 => StarSpectralType::K((class_as_int % 10) as u8),
        7 => StarSpectralType::M((class_as_int % 10) as u8),
        8 => StarSpectralType::L((class_as_int % 10) as u8),
        9 => StarSpectralType::T((class_as_int % 10) as u8),
        _ => StarSpectralType::Y((class_as_int % 10) as u8),
    }
}

fn generate_white_dwarf_spectral_type(
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    seed: &str,
) -> StarSpectralType {
    let mut rng = SeededDiceRoller::new(
        seed,
        &format!("star_{}_{}_{}_wd_st", coord, system_index, star_index),
    );
    rng.get_result(&CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult {
                result: StarSpectralType::DA,
                weight: 688,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DB,
                weight: 150,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DC,
                weight: 90,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DX,
                weight: 50,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DQ,
                weight: 15,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DZ,
                weight: 6,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DO,
                weight: 1,
            },
        ],
        RollMethod::SimpleRoll,
    ))
    .expect("Should return a white dwarf spectral type.")
}

fn calculate_luminosity_class(
    luminosity: f32,
    spectral_type: StarSpectralType,
    age: f32,
    main_lifespan: f32,
    subgiant_lifespan: f32,
) -> StarLuminosityClass {
    match spectral_type {
        StarSpectralType::L(_) | StarSpectralType::T(_) | StarSpectralType::Y(_) => {
            return StarLuminosityClass::Y
        }
        StarSpectralType::DA
        | StarSpectralType::DB
        | StarSpectralType::DC
        | StarSpectralType::DO
        | StarSpectralType::DZ
        | StarSpectralType::DQ
        | StarSpectralType::DX => {
            return StarLuminosityClass::VII;
        }
        StarSpectralType::XNS => {
            return StarLuminosityClass::XNS;
        }
        StarSpectralType::XBH => {
            return StarLuminosityClass::XBH;
        }
        _ => (),
    }
    if age <= main_lifespan {
        StarLuminosityClass::V
    } else if age <= subgiant_lifespan {
        StarLuminosityClass::IV
    } else {
        if luminosity <= 100.0 {
            StarLuminosityClass::III
        } else if luminosity <= 1000.0 {
            StarLuminosityClass::II
        } else if luminosity <= 31333.3 {
            StarLuminosityClass::Ib
        } else if luminosity <= 75000.0 {
            StarLuminosityClass::Ia
        } else {
            StarLuminosityClass::O
        }
    }
}

fn calculate_remnant_mass(mass: f64, _settings: &GenerationSettings) -> f64 {
    // Empirical linear fits to the initial-final mass relation (Cummings
    // et al. 2018). The 0.318 constant is a coefficient, not 1/π.
    #[allow(clippy::approx_constant)]
    if mass < 2.7 {
        0.096 * mass + 0.429
    } else {
        0.137 * mass + 0.318
    }
}

fn calculate_white_dwarf_temperature(initial_temperature: f32, age: f32) -> u32 {
    (initial_temperature * f32::powf(age / 1000.0, -1.3 / 4.0)) as u32
}

fn calculate_white_dwarf_initial_luminosity(mass: f64) -> f32 {
    (10.0_f64.powf(-2.15) * mass.powf(3.95)) as f32
}

/// Neutron star surface temperature as a function of age using a two-phase
/// cooling model: modified URCA (neutrino-dominated) up to ~10^5 yr, then
/// photon cooling after that.
///
/// Reference: Yakovlev & Pethick (2004), "Neutron Star Cooling".
/// Approximate observed temperatures for young neutron stars:
///   Crab (~10^3 yr): ~1.5e6 K
///   Vela (~10^4 yr): ~7e5 K
///   Geminga (~3e5 yr): ~3e5 K
///
/// `age` and `full_lifespan` are both in millions of years (Myr).
fn calculate_neutron_star_temperature(age: f32, full_lifespan: f32) -> u32 {
    // Years elapsed since the supernova that formed the neutron star.
    let nova_age_myr = (age - full_lifespan).max(0.0);
    let t_years = (nova_age_myr * 1.0e6).max(1.0);

    // Anchor: surface T ≈ 2e6 K at t = 10 yr.
    // Modified URCA: T_s ∝ t^(-1/6) for t < 1e5 yr.
    // Photon cooling: T_s ∝ t^(-1/2) for t ≥ 1e5 yr, stitched continuously.
    let t_surface = if t_years < 1.0e5 {
        2.0e6 * (t_years / 10.0).powf(-1.0 / 6.0)
    } else {
        let t_transition = 2.0e6 * (1.0e5_f32 / 10.0).powf(-1.0 / 6.0);
        t_transition * (t_years / 1.0e5).powf(-0.5)
    };

    t_surface.max(1_000.0) as u32
}

#[cfg(test)]
mod neutron_star_tests {
    use super::*;

    /// Helper: compute surface temperature given t_years since supernova.
    fn temp_at_years(t_years: f32) -> u32 {
        // Use age=full_lifespan + t_years_in_myr so nova_age_myr = t_years_in_myr.
        let full_lifespan = 100.0f32;
        let age = full_lifespan + t_years / 1.0e6;
        calculate_neutron_star_temperature(age, full_lifespan)
    }

    #[test]
    fn young_neutron_star_is_megakelvin() {
        // At 10 yr: ~2 MK
        let t = temp_at_years(10.0);
        assert!(t > 1_500_000 && t < 2_500_000, "got {}", t);
    }

    #[test]
    fn crab_era_matches_observation() {
        // Crab (~1000 yr): observed ~1.5 MK, model expects ~0.9-1 MK
        let t = temp_at_years(1_000.0);
        assert!(t > 700_000 && t < 1_400_000, "got {}", t);
    }

    #[test]
    fn vela_era_matches_observation() {
        // Vela (~10000 yr): observed ~0.7 MK, model ~0.63 MK
        let t = temp_at_years(10_000.0);
        assert!(t > 400_000 && t < 900_000, "got {}", t);
    }

    #[test]
    fn photon_cooling_kicks_in() {
        // At 1e8 yr (long past transition): should be tens of thousands K
        let t = temp_at_years(1.0e8);
        assert!(t > 5_000 && t < 30_000, "got {}", t);
    }

    #[test]
    fn temperature_monotonically_decreases() {
        let ages = [
            10.0,
            100.0,
            1000.0,
            10_000.0,
            100_000.0,
            1_000_000.0,
            10_000_000.0,
        ];
        let mut prev = u32::MAX;
        for &t in &ages {
            let cur = temp_at_years(t);
            assert!(cur < prev, "non-monotonic at t={}: {} >= {}", t, cur, prev);
            prev = cur;
        }
    }

    #[test]
    fn floors_at_1000k() {
        // Extremely old neutron star — temperature should floor.
        let t = temp_at_years(1.0e15);
        assert!(t >= 1_000);
    }
}

fn calculate_precise_radius_of_neutron_star_or_black_hole(mass: f64) -> f64 {
    let g: f64 = 6.674e-11;
    let c: f64 = 299_792_458.0;
    let sun_in_km = 696_340.0;
    2.0 * g * mass / c * 2.0 / sun_in_km
}

/// White dwarf luminosity via Mestel's cooling law.
fn calculate_white_dwarf_luminosity(mass_solar: f64, cooling_age_gyr: f64) -> f32 {
    if cooling_age_gyr <= 0.001 {
        return 0.01;
    }
    let l = 10.0_f64.powf(-2.15) * mass_solar.powf(3.95) * cooling_age_gyr.powf(-1.4);
    l.clamp(1e-6, 1.0) as f32
}

fn calculate_white_dwarf_radius(mass: f64) -> f64 {
    0.0084 * mass.powf(-1.0 / 3.0)
}

/// * 2.0 because my dataset has half-way points
fn get_age_range_in_star_lifecycle_dataset(
    age: f32,
    main_lifespan: f32,
    subgiant_lifespan: f32,
    giant_lifespan: f32,
) -> f32 {
    let to_subgiant_lifespan = main_lifespan + subgiant_lifespan;
    let to_giant_lifespan = to_subgiant_lifespan + giant_lifespan;
    if age >= 0.0 && age <= main_lifespan {
        (age / main_lifespan) * 2.0
    } else if age > main_lifespan && age <= to_subgiant_lifespan {
        2.0 + ((age - main_lifespan) / to_subgiant_lifespan) * 2.0
    } else if age > to_subgiant_lifespan && age <= to_giant_lifespan {
        4.0 + ((age - to_subgiant_lifespan) / to_giant_lifespan) * 2.0
    } else {
        7.0
    }
}

fn get_mass_range_in_star_lifecycle_dataset(mass: f64) -> f32 {
    (if mass < 0.4 {
        0.0
    } else if (0.4..=0.5).contains(&mass) {
        mass / 0.5
    } else if mass > 0.5 && mass <= 1.0 {
        1.0 + ((mass - 0.5) / (1.0 - 0.5))
    } else if mass > 1.0 && mass <= 2.0 {
        2.0 + ((mass - 1.0) / (2.0 - 1.0))
    } else if mass > 2.0 && mass <= 5.0 {
        3.0 + ((mass - 2.0) / (5.0 - 2.0))
    } else if mass > 5.0 && mass <= 15.0 {
        4.0 + ((mass - 5.0) / (15.0 - 5.0))
    } else if mass > 15.0 && mass <= 60.0 {
        5.0 + ((mass - 15.0) / (60.0 - 15.0))
    } else if mass > 60.0 && mass <= 500.0 {
        6.0 + ((mass - 60.0) / (500.0 - 60.0))
    } else {
        8.0
    }) as f32
}

fn get_nearest_star_lifecycle_dataset_cells(
    age_range: f32,
    mass_range: f32,
) -> [TemperatureAndLuminosity; 4] {
    if !(0.0..=6.0).contains(&age_range) || !(0.0..=7.0).contains(&mass_range) {
        panic!(
            "{}",
            format!(
                "age_range ({}) or mass_range ({}) is out of bounds",
                age_range, mass_range
            )
        );
    }

    let x = age_range as usize;
    let x1 = if age_range.fract() != 0.0 { x + 1 } else { x };
    let y = mass_range as usize;
    let y1 = if mass_range.fract() != 0.0 { y + 1 } else { y };

    let a = STAR_LIFECYCLE_DATASET[y][x];
    let b = STAR_LIFECYCLE_DATASET[y][x1];
    let c = STAR_LIFECYCLE_DATASET[y1][x];
    let d = STAR_LIFECYCLE_DATASET[y1][x1];

    [a, b, c, d]
}

fn interpolate_f32(x0_y0: f32, x1_y0: f32, x0_y1: f32, x1_y1: f32, x: f32, y: f32) -> f32 {
    let xf = x.fract();
    let yf = y.fract();
    let i1 = x0_y0 * (1.0 - yf) + x0_y1 * yf;
    let i2 = x1_y0 * (1.0 - yf) + x1_y1 * yf;
    i1 * (1.0 - xf) + i2 * xf
}

/// My "main sequence only" calculation is almost good, but it's useless for subgiants and giants, and my calculation that interpolates from
/// a dataset of temperatures and luminosity for mass/age steps wasn't a bad idea per se, but I don't have enough data to make it effective.
/// However, I need to go forward, so I'll mix the two for main sequence and use the dataset for giants.
/// If you're reading this, have knowledge in the field and an idea of how to improve my star generation sequence in a more realistic way,
/// feel free to reach out to me and contribute!
fn mix_values(a: f64, b: f64, age: f32, main_lifespan: f32) -> f64 {
    let result;
    let pond_a = 0.3 + age as f64 / main_lifespan as f64;
    if pond_a >= 1.0 {
        result = b;
    } else {
        let pond_b = 1.0 - pond_a;
        result = a * pond_a + b * pond_b;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_main_sequence_values_approaching_reality() {
        let mut n = 0;
        let mut rad_ms_sum = 0.0;
        let mut lum_ms_sum = 0.0;
        let mut temp_ms_sum = 0.0;

        let coord = SpaceCoordinates {
            ..Default::default()
        };
        let galaxy = &Galaxy {
            ..Default::default()
        };

        for star in get_test_stars().iter() {
            if is_star_a_main_sequence_dwarf(star) {
                let mass = star.mass;
                let ms_luminosity = calculate_main_sequence_luminosity(mass);
                let ms_radius = calculate_radius(mass, 0.0, 1.0, 0.0, 0.0, 0, 0, coord, galaxy);
                let ms_temperature =
                    calculate_temperature_using_luminosity(ms_luminosity, ms_radius) as u32;
                let main_lifespan = calculate_lifespan(mass, ms_luminosity);
                let subgiant_lifespan = calculate_subgiant_lifespan(mass, main_lifespan);
                let spectral_type = calculate_spectral_type(ms_temperature);

                n += 1;
                rad_ms_sum += MathUtils::get_difference_percentage(ms_radius, star.radius);
                lum_ms_sum += MathUtils::get_difference_percentage(
                    ms_luminosity as f64,
                    star.luminosity as f64,
                );
                temp_ms_sum += MathUtils::get_difference_percentage(
                    ms_temperature as f64,
                    star.temperature as f64,
                );

                print_real_to_generated_star_comparison(
                    star,
                    mass,
                    ms_radius,
                    ms_luminosity,
                    ms_temperature,
                    spectral_type,
                    calculate_luminosity_class(
                        ms_luminosity,
                        spectral_type,
                        star.age,
                        main_lifespan,
                        subgiant_lifespan,
                    ),
                    star.age,
                );
            }
        }

        rad_ms_sum /= n as f64;
        lum_ms_sum /= n as f64;
        temp_ms_sum /= n as f64;

        // The results shouldn't have a variance higher than 10% in general
        print_real_to_generated_stars_comparison_results(rad_ms_sum, lum_ms_sum, temp_ms_sum);

        assert!((-0.2..=0.2).contains(&rad_ms_sum));
        assert!((-0.2..=0.2).contains(&lum_ms_sum));
        assert!((-0.2..=0.2).contains(&temp_ms_sum));
    }

    #[test]
    fn generate_interpolated_values_approaching_reality() {
        let mut n = 0;
        let mut rad_sum = 0.0;
        let mut lum_sum = 0.0;
        let mut temp_sum = 0.0;

        let coord = SpaceCoordinates {
            ..Default::default()
        };
        let galaxy = &Galaxy {
            ..Default::default()
        };

        for star in get_test_stars().iter() {
            if is_star_main_sequence_or_giant(star) {
                let age = star.age * 1000.0;
                let mass = star.mass;

                // Main sequence estimations
                let ms_luminosity = calculate_main_sequence_luminosity(mass);
                let ms_radius = calculate_radius(mass, 0.0, 1.0, 0.0, 0.0, 0, 0, coord, galaxy);
                let ms_temperature =
                    calculate_temperature_using_luminosity(ms_luminosity, ms_radius) as u32;

                let main_lifespan = calculate_lifespan(mass, ms_luminosity);
                let age_range = get_age_range_in_star_lifecycle_dataset(
                    age,
                    main_lifespan,
                    calculate_subgiant_lifespan(mass, main_lifespan),
                    calculate_giant_lifespan(mass, main_lifespan),
                );
                let mass_range = get_mass_range_in_star_lifecycle_dataset(mass);
                if age_range < 7.0 && mass_range < 6.0 {
                    let nearest_values =
                        get_nearest_star_lifecycle_dataset_cells(age_range, mass_range);

                    // Compute interpolated values
                    let interpolated_temperature = get_interpolated_temperature(
                        mass,
                        ms_temperature,
                        nearest_values,
                        age_range,
                        mass_range,
                    );
                    let interpolated_lum_factor =
                        get_interpolated_luminosity_factor(nearest_values, age_range, mass_range);
                    let interpolated_luminosity =
                        get_interpolated_luminosity(mass, ms_luminosity, interpolated_lum_factor);
                    let interpolated_radius = get_interpolated_radius(
                        mass,
                        ms_radius,
                        interpolated_luminosity,
                        interpolated_temperature,
                    );
                    let main_lifespan = calculate_lifespan(mass, ms_luminosity);
                    let subgiant_lifespan = calculate_subgiant_lifespan(mass, main_lifespan);
                    let spectral_type = calculate_spectral_type(interpolated_temperature);

                    n += 1;
                    rad_sum +=
                        MathUtils::get_difference_percentage(interpolated_radius, star.radius);
                    lum_sum += MathUtils::get_difference_percentage(
                        interpolated_luminosity as f64,
                        star.luminosity as f64,
                    );
                    temp_sum += MathUtils::get_difference_percentage(
                        interpolated_temperature as f64,
                        star.temperature as f64,
                    );

                    print_real_to_generated_star_comparison(
                        star,
                        mass,
                        interpolated_radius,
                        interpolated_luminosity,
                        interpolated_temperature,
                        calculate_spectral_type(interpolated_temperature),
                        calculate_luminosity_class(
                            interpolated_luminosity,
                            spectral_type,
                            star.age,
                            main_lifespan,
                            subgiant_lifespan,
                        ),
                        age * 1000.0,
                    );
                }
            }
        }

        rad_sum /= n as f64;
        lum_sum /= n as f64;
        temp_sum /= n as f64;

        // The results shouldn't have a variance higher than 10% in general
        print_real_to_generated_stars_comparison_results(rad_sum, lum_sum, temp_sum);

        assert!((-0.2..=0.2).contains(&rad_sum));
        assert!((-0.2..=0.2).contains(&lum_sum));
        assert!((-0.2..=0.2).contains(&temp_sum));
    }

    #[test]
    fn calculate_values_approaching_reality() {
        let mut n = 0;
        let mut rad_calc_sum = 0.0;
        let mut lum_calc_sum = 0.0;
        let mut temp_calc_sum = 0.0;

        for star in get_test_stars().iter() {
            if is_star_main_sequence_or_giant(star) {
                let calc_radius = calculate_radius_using_luminosity_and_temperature(
                    star.luminosity,
                    star.temperature,
                );
                let calc_luminosity =
                    calculate_luminosity_using_temperature(star.temperature, star.radius);
                let calc_temperature =
                    calculate_temperature_using_luminosity(star.luminosity, star.radius);
                let main_lifespan = calculate_lifespan(star.mass, calc_luminosity);
                let subgiant_lifespan = calculate_subgiant_lifespan(star.mass, main_lifespan);
                let spectral_type = calculate_spectral_type(calc_temperature as u32);

                n += 1;
                rad_calc_sum += MathUtils::get_difference_percentage(calc_radius, star.radius);
                lum_calc_sum += MathUtils::get_difference_percentage(
                    calc_luminosity as f64,
                    star.luminosity as f64,
                );
                temp_calc_sum += MathUtils::get_difference_percentage(
                    calc_temperature as f64,
                    star.temperature as f64,
                );

                print_real_to_generated_star_comparison(
                    star,
                    star.mass,
                    calc_radius,
                    calc_luminosity,
                    calc_temperature as u32,
                    calculate_spectral_type(calc_temperature as u32),
                    calculate_luminosity_class(
                        calc_luminosity,
                        spectral_type,
                        star.age,
                        main_lifespan,
                        subgiant_lifespan,
                    ),
                    star.age,
                );
            }
        }

        rad_calc_sum /= n as f64;
        lum_calc_sum /= n as f64;
        temp_calc_sum /= n as f64;

        // The results shouldn't have a variance higher than 10% in general
        print_real_to_generated_stars_comparison_results(rad_calc_sum, lum_calc_sum, temp_calc_sum);

        assert!((-0.2..=0.2).contains(&rad_calc_sum));
        assert!((-0.2..=0.2).contains(&lum_calc_sum));
        assert!((-0.2..=0.2).contains(&temp_calc_sum));
    }

    #[test]
    fn generate_stars_looking_like_actual_stars() {
        let mut n = 0;
        let mut rad_sum = 0.0;
        let mut lum_sum = 0.0;
        let mut temp_sum = 0.0;

        let coord = SpaceCoordinates {
            ..Default::default()
        };
        let hex = &GalacticHex {
            ..Default::default()
        };
        let galaxy = &Galaxy {
            ..Default::default()
        };

        for star in get_test_stars().iter() {
            if is_star_main_sequence_or_giant(star) {
                let settings = GenerationSettings {
                    star: StarSettings {
                        fixed_age: Some(star.age),
                        fixed_mass: Some(star.mass),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let mut generated_star = Star::generate(
                    0,
                    0,
                    0,
                    "Test".into(),
                    coord,
                    StellarEvolution::Dwarf,
                    hex,
                    galaxy,
                    &settings,
                );
                generated_star.name = star.name.clone();

                print_real_to_generated_star_comparison(
                    star,
                    generated_star.mass,
                    generated_star.radius,
                    generated_star.luminosity,
                    generated_star.temperature,
                    generated_star.spectral_type,
                    generated_star.luminosity_class,
                    generated_star.age,
                );

                n += 1;
                rad_sum += MathUtils::get_difference_percentage(generated_star.radius, star.radius);
                lum_sum += MathUtils::get_difference_percentage(
                    generated_star.luminosity as f64,
                    star.luminosity as f64,
                );
                temp_sum += MathUtils::get_difference_percentage(
                    generated_star.temperature as f64,
                    star.temperature as f64,
                );
            }
        }

        rad_sum /= n as f64;
        lum_sum /= n as f64;
        temp_sum /= n as f64;

        // The results shouldn't have a variance higher than 10% in general
        print_real_to_generated_stars_comparison_results(rad_sum, lum_sum, temp_sum);

        assert!((-0.2..=0.2).contains(&rad_sum));
        assert!((-0.2..=0.2).contains(&lum_sum));
        assert!((-0.2..=0.2).contains(&temp_sum));
    }

    #[test]
    fn assert_that_generation_returns_proper_type_for_standard_mass() {
        let expected_values = vec![
            (0.1, 3100, 0.0012),
            (0.15, 3200, 0.0036),
            (0.2, 3200, 0.0079),
            (0.25, 3300, 0.015),
            (0.3, 3300, 0.024),
            (0.35, 3400, 0.37),
            (0.4, 3500, 0.054),
            (0.45, 3600, 0.07),
            (0.5, 3800, 0.09),
            (0.55, 4000, 0.11),
            (0.6, 4200, 0.13),
            (0.65, 4400, 0.15),
            (0.7, 4600, 0.19),
            (0.75, 4900, 0.23),
            (0.8, 5200, 0.28),
            (0.85, 5400, 0.36),
            (0.9, 5500, 0.45),
            (0.95, 5700, 0.56),
            (1.0, 5800, 0.68),
            (1.05, 5900, 0.87),
            (1.1, 6000, 1.1),
            (1.15, 6100, 1.4),
            (1.2, 6300, 1.7),
            (1.25, 6400, 2.1),
            (1.3, 6500, 2.5),
            (1.35, 6600, 3.1),
            (1.4, 6700, 3.7),
            (1.45, 6900, 4.3),
            (1.5, 7000, 5.1),
            (1.6, 7300, 6.7),
            (1.7, 7500, 8.6),
            (1.8, 7800, 11.0),
            (1.9, 8000, 13.0),
            (2.0, 8200, 16.0),
        ];
        let mut generated = vec![];
        for expected in expected_values.iter() {
            let settings = GenerationSettings {
                seed: Rc::from(expected.0.to_string()),
                universe: UniverseSettings {
                    use_ours: true,
                    ..Default::default()
                },
                galaxy: GalaxySettings {
                    use_ours: true,
                    ..Default::default()
                },
                star: StarSettings {
                    fixed_mass: Some(expected.0),
                    fixed_age: Some(0.00001f32),
                    ..Default::default()
                },
                ..Default::default()
            };
            let neighborhood =
                GalacticNeighborhood::generate(Universe::generate(&settings), &settings);
            let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);
            let coord = SpaceCoordinates::new(0, 0, 0);
            let hex = galaxy
                .get_hex(coord.rel(galaxy.get_galactic_start()))
                .expect("Should have generated a hex.");

            let generated_star = Star::generate(
                0,
                0,
                0,
                "Test".into(),
                coord,
                StellarEvolution::Dwarf,
                &hex,
                &galaxy,
                &settings,
            );

            generated.push(generated_star);
        }

        for i in 0..expected_values.len() {
            assert!(
                expected_values[i].1 - 1000 <= generated[i].temperature
                    && generated[i].temperature <= expected_values[i].1 + 1000
            );
        }
    }

    #[test]
    fn calculate_proper_star_age() {
        for i in 0..1000 {
            let mut rng = SeededDiceRoller::new(&format!("{}", i), "test_age");
            let settings = &GenerationSettings {
                seed: Rc::from(i.to_string()),
                galaxy: GalaxySettings {
                    ..Default::default()
                },
                ..Default::default()
            };
            let neighborhood =
                GalacticNeighborhood::generate(Universe::generate(settings), settings);
            let mut galaxy = Galaxy::generate(neighborhood, (i as u16) % 5, settings);
            let gal_end = galaxy.get_galactic_end();
            let x = rng.gen_u32() as i64 % gal_end.x;
            let y = rng.gen_u32() as i64 % gal_end.y;
            let z = rng.gen_u32() as i64 % gal_end.z;
            let coord = SpaceCoordinates::new(x, y, z);

            let age = generate_age(
                i as u16,
                i as u16 + 1,
                coord,
                &GalacticHex::generate(coord, coord, &mut galaxy),
                &galaxy.settings.seed,
                &galaxy.neighborhood.universe,
            ) / 1000.0;
            assert!(age > 0.0 && age < galaxy.neighborhood.universe.age);
        }
    }

    #[test]
    fn calculate_proper_spectral_type() {
        assert_eq!(calculate_spectral_type(380000), StarSpectralType::WR(2));
        assert_eq!(calculate_spectral_type(170000), StarSpectralType::WR(3));
        assert_eq!(calculate_spectral_type(117000), StarSpectralType::WR(4));
        assert_eq!(calculate_spectral_type(54000), StarSpectralType::O(2));
        assert_eq!(calculate_spectral_type(45000), StarSpectralType::O(3));
        assert_eq!(calculate_spectral_type(43300), StarSpectralType::O(4));
        assert_eq!(calculate_spectral_type(40600), StarSpectralType::O(5));
        assert_eq!(calculate_spectral_type(39500), StarSpectralType::O(6));
        assert_eq!(calculate_spectral_type(37100), StarSpectralType::O(7));
        assert_eq!(calculate_spectral_type(35100), StarSpectralType::O(8));
        assert_eq!(calculate_spectral_type(33300), StarSpectralType::O(9));
        assert_eq!(calculate_spectral_type(29200), StarSpectralType::B(0));
        assert_eq!(calculate_spectral_type(23000), StarSpectralType::B(1));
        assert_eq!(calculate_spectral_type(21000), StarSpectralType::B(2));
        assert_eq!(calculate_spectral_type(17600), StarSpectralType::B(3));
        assert_eq!(calculate_spectral_type(15200), StarSpectralType::B(5));
        assert_eq!(calculate_spectral_type(14300), StarSpectralType::B(6));
        assert_eq!(calculate_spectral_type(13500), StarSpectralType::B(7));
        assert_eq!(calculate_spectral_type(12300), StarSpectralType::B(8));
        assert_eq!(calculate_spectral_type(11400), StarSpectralType::B(9));
        assert_eq!(calculate_spectral_type(9600), StarSpectralType::A(0));
        assert_eq!(calculate_spectral_type(9330), StarSpectralType::A(1));
        assert_eq!(calculate_spectral_type(9040), StarSpectralType::A(2));
        assert_eq!(calculate_spectral_type(8750), StarSpectralType::A(3));
        assert_eq!(calculate_spectral_type(8480), StarSpectralType::A(4));
        assert_eq!(calculate_spectral_type(8310), StarSpectralType::A(5));
        assert_eq!(calculate_spectral_type(7920), StarSpectralType::A(7));
        assert_eq!(calculate_spectral_type(7350), StarSpectralType::F(0));
        assert_eq!(calculate_spectral_type(7200), StarSpectralType::F(1));
        assert_eq!(calculate_spectral_type(7050), StarSpectralType::F(2));
        assert_eq!(calculate_spectral_type(6850), StarSpectralType::F(3));
        assert_eq!(calculate_spectral_type(6700), StarSpectralType::F(5));
        assert_eq!(calculate_spectral_type(6550), StarSpectralType::F(6));
        assert_eq!(calculate_spectral_type(6400), StarSpectralType::F(7));
        assert_eq!(calculate_spectral_type(6300), StarSpectralType::F(8));
        assert_eq!(calculate_spectral_type(6050), StarSpectralType::G(0));
        assert_eq!(calculate_spectral_type(5930), StarSpectralType::G(1));
        assert_eq!(calculate_spectral_type(5800), StarSpectralType::G(2));
        assert_eq!(calculate_spectral_type(5660), StarSpectralType::G(5));
        assert_eq!(calculate_spectral_type(5440), StarSpectralType::G(8));
        assert_eq!(calculate_spectral_type(5240), StarSpectralType::K(0));
        assert_eq!(calculate_spectral_type(5110), StarSpectralType::K(1));
        assert_eq!(calculate_spectral_type(4960), StarSpectralType::K(2));
        assert_eq!(calculate_spectral_type(4800), StarSpectralType::K(3));
        assert_eq!(calculate_spectral_type(4600), StarSpectralType::K(4));
        assert_eq!(calculate_spectral_type(4400), StarSpectralType::K(5));
        assert_eq!(calculate_spectral_type(4000), StarSpectralType::K(7));
        assert_eq!(calculate_spectral_type(3750), StarSpectralType::M(0));
        assert_eq!(calculate_spectral_type(3700), StarSpectralType::M(1));
        assert_eq!(calculate_spectral_type(3600), StarSpectralType::M(2));
        assert_eq!(calculate_spectral_type(3500), StarSpectralType::M(3));
        assert_eq!(calculate_spectral_type(3400), StarSpectralType::M(4));
        assert_eq!(calculate_spectral_type(3200), StarSpectralType::M(5));
        assert_eq!(calculate_spectral_type(3100), StarSpectralType::M(6));
        assert_eq!(calculate_spectral_type(2900), StarSpectralType::M(7));
        assert_eq!(calculate_spectral_type(2700), StarSpectralType::M(8));
        assert_eq!(calculate_spectral_type(2600), StarSpectralType::L(0));
        assert_eq!(calculate_spectral_type(2200), StarSpectralType::L(3));
        assert_eq!(calculate_spectral_type(1500), StarSpectralType::L(8));
        assert_eq!(calculate_spectral_type(1400), StarSpectralType::T(2));
        assert_eq!(calculate_spectral_type(1000), StarSpectralType::T(6));
        assert_eq!(calculate_spectral_type(800), StarSpectralType::T(8));
        assert_eq!(calculate_spectral_type(370), StarSpectralType::Y(0));
        assert_eq!(calculate_spectral_type(350), StarSpectralType::Y(1));
        assert_eq!(calculate_spectral_type(320), StarSpectralType::Y(2));
        assert_eq!(calculate_spectral_type(250), StarSpectralType::Y(4));
    }

    #[test]
    fn interpolate_temperature_properly() {
        let mut x0_y0 = 5000.0;
        let mut x1_y0 = 6000.0;
        let mut x0_y1 = 5500.0;
        let mut x1_y1 = 6500.0;
        let mut x = 2.5;
        let mut y = 1.5;
        let mut result = interpolate_f32(x0_y0, x1_y0, x0_y1, x1_y1, x, y);
        let mut expected = 5750.0;
        assert!((result - expected).abs() < 0.001);

        x0_y0 = 0.0;
        x1_y0 = 1000.0;
        x0_y1 = 0.0;
        x1_y1 = 1000.0;
        x = 2.5;
        y = 1.5;
        result = interpolate_f32(x0_y0, x1_y0, x0_y1, x1_y1, x, y);
        expected = 500.0;
        assert!((result - expected).abs() < 0.001);

        x0_y0 = 0.0;
        x1_y0 = 1000.0;
        x0_y1 = 500.0;
        x1_y1 = 1500.0;
        x = 2.5;
        y = 1.5;
        result = interpolate_f32(x0_y0, x1_y0, x0_y1, x1_y1, x, y);
        expected = 750.0;
        assert!((result - expected).abs() < 0.001);

        x0_y0 = 0.0;
        x1_y0 = 1000.0;
        x0_y1 = 500.0;
        x1_y1 = 1500.0;
        x = 1.75;
        y = 1.5;
        result = interpolate_f32(x0_y0, x1_y0, x0_y1, x1_y1, x, y);
        expected = 1000.0;
        assert!((result - expected).abs() < 0.001);
    }

    fn print_real_to_generated_stars_comparison_results(rad_sum: f64, lum_sum: f64, temp_sum: f64) {
        println!(
        "\nVariance from generated values to real ones - radius: {}%, luminosity: {}%, temperature: {}%\n",
        format!("{}{}", if rad_sum > 0.0 {"+"} else {""}, rad_sum * 100.0),
        format!("{}{}", if lum_sum > 0.0 {"+"} else {""}, lum_sum * 100.0),
        format!("{}{}", if temp_sum > 0.0 {"+"} else {""}, temp_sum * 100.0),
    );
    }

    fn print_real_to_generated_star_comparison(
        star: &Star,
        mass: f64,
        radius: f64,
        luminosity: f32,
        temperature: u32,
        spectral_type: StarSpectralType,
        luminosity_class: StarLuminosityClass,
        age: f32,
    ) {
        println!(
            "     Real {} - mass: {}, rad: {}, lum: {}, temp: {}K, type: {} {}, age: {}",
            star.name,
            star.mass,
            star.radius,
            star.luminosity,
            star.temperature,
            star.spectral_type,
            star.luminosity_class,
            star.age
        );
        println!(
            "Generated {} - mass: {}, rad: {} ({}), lum: {} ({}), temp: {}K ({}), type: {} {}, age: {}\n",
            star.name,
            mass,
            radius,
            StringUtils::get_difference_percentage_str(radius, star.radius),
            luminosity,
            StringUtils::get_difference_percentage_str(luminosity as f64, star.luminosity as f64),
            temperature,
            StringUtils::get_difference_percentage_str(temperature as f64, star.temperature as f64),
            spectral_type,
            luminosity_class,
            age
        );
    }

    #[test]
    fn magnitude_from_luminosity_sun() {
        let mag = crate::star::absolute_magnitude_from_luminosity(1.0);
        assert!(
            (mag - 4.83).abs() < 0.01,
            "Sun abs mag should be ~4.83, got {}",
            mag
        );
    }

    #[test]
    fn bv_color_ordering() {
        // Hot stars should have lower B-V than cool stars
        let bv_hot = crate::star::temperature_to_bv(10000);
        let bv_sun = crate::star::temperature_to_bv(5772);
        let bv_cool = crate::star::temperature_to_bv(3500);
        assert!(
            bv_hot < bv_sun && bv_sun < bv_cool,
            "B-V should increase with decreasing temperature: hot={}, sun={}, cool={}",
            bv_hot,
            bv_sun,
            bv_cool
        );
    }

    #[test]
    fn bv_to_rgb_produces_valid_colors() {
        let (r, g, b) = crate::star::bv_to_rgb(0.63);
        assert!(r > 200, "Sun should be warm-colored, r={}", r);
        assert!(g > 150, "Sun should have green component, g={}", g);
        assert!(b > 100, "Sun should have blue component, b={}", b);
    }

    #[test]
    fn magnitude_luminosity_roundtrip() {
        let lum_original = 100.0_f32;
        let mag = crate::star::absolute_magnitude_from_luminosity(lum_original);
        let lum_back = crate::star::luminosity_from_absolute_magnitude(mag);
        assert!(
            (lum_back - lum_original).abs() / lum_original < 0.01,
            "roundtrip: {} -> mag {} -> {}",
            lum_original,
            mag,
            lum_back
        );
    }

    #[test]
    fn stefan_boltzmann_solar_units_exact_for_sun() {
        // Sun: L=1, R=1 -> T should be 5772K
        let temp = calculate_temperature_using_luminosity(1.0, 1.0);
        assert!(
            (temp - 5772.0).abs() < 1.0,
            "Sun temperature should be ~5772K, got {}",
            temp
        );
        // Roundtrip: T=5772, R=1 -> L should be 1.0
        let lum = calculate_luminosity_using_temperature(5772, 1.0);
        assert!(
            (lum - 1.0).abs() < 0.01,
            "Sun luminosity should be ~1.0, got {}",
            lum
        );
        // Roundtrip: L=1, T=5772 -> R should be 1.0
        let rad = calculate_radius_using_luminosity_and_temperature(1.0, 5772);
        assert!(
            (rad - 1.0).abs() < 0.01,
            "Sun radius should be ~1.0, got {}",
            rad
        );
    }

    /// Returns true if the star is currently in the main sequence phase of its life.
    fn is_star_a_main_sequence_dwarf(star: &Star) -> bool {
        star.luminosity_class == StarLuminosityClass::V
            && (discriminant(&star.spectral_type) == discriminant(&StarSpectralType::WR(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::O(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::B(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::A(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::F(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::G(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::K(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::M(0)))
    }

    /// Returns true if the star is currently in the main sequence, subgiant or giant phase of its life.
    fn is_star_main_sequence_or_giant(star: &Star) -> bool {
        (star.luminosity_class == StarLuminosityClass::O
            || star.luminosity_class == StarLuminosityClass::Ia
            || star.luminosity_class == StarLuminosityClass::Ib
            || star.luminosity_class == StarLuminosityClass::II
            || star.luminosity_class == StarLuminosityClass::III
            || star.luminosity_class == StarLuminosityClass::IV
            || star.luminosity_class == StarLuminosityClass::V
            || star.luminosity_class == StarLuminosityClass::IV)
            && (discriminant(&star.spectral_type) == discriminant(&StarSpectralType::WR(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::O(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::B(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::A(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::F(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::G(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::K(0))
                || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::M(0)))
    }
}
