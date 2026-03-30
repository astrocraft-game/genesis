use crate::internal::*;
use crate::prelude::*;

/// Generate comprehensive planetary detail from base world parameters.
pub fn generate_planetary_detail(
    atmospheric_pressure: f32,
    atmospheric_composition: &[(f32, ChemicalComponent)],
    blackbody_temperature: u32,
    gravity: f32,
    radius: f64,
    hydrosphere: f32,
    ice_over_water: f32,
    ice_over_land: f32,
    volcanism: f32,
    tectonic_activity: f32,
    magnetic_field: MagneticFieldStrength,
    body_type: TelluricBodyComposition,
    world_type: CelestialBodyWorldType,
    life_level: LifeLevel,
    axial_tilt: f32,
    eccentricity: f32,
    rotation_days: f32,
    is_tidally_locked: bool,
    tidal_heating: u32,
    seed: &str,
    coord: SpaceCoordinates,
    system_index: u16,
    star_id: u32,
    orbital_point_id: u32,
) -> PlanetaryDetail {
    let mut rng = SeededDiceRoller::new(
        seed,
        &format!("sys_{}_{}_str_{}_bdy{}_detail", coord, system_index, star_id, orbital_point_id),
    );

    let g_ms2 = gravity * 9.81;
    let has_atmosphere = atmospheric_pressure > 0.01;
    let has_liquid = hydrosphere > 0.5;
    let land_fraction = (100.0 - hydrosphere).max(0.0) / 100.0;

    // 1. Atmospheric layers
    let atmospheric_layers = if has_atmosphere {
        let mean_molecular_mass = if atmospheric_composition.iter().any(|(f, c)| *f > 0.5 && *c == ChemicalComponent::CarbonDioxide) {
            0.044
        } else if atmospheric_composition.iter().any(|(f, c)| *f > 0.5 && *c == ChemicalComponent::Nitrogen) {
            0.028
        } else { 0.029 };
        let scale_height = 8.314 * blackbody_temperature as f64 / (mean_molecular_mass * g_ms2 as f64 * 1000.0);
        let scale_height_km = (scale_height / 1000.0) as f32;
        let tropopause_km = scale_height_km * 2.0 * (atmospheric_pressure / 1.0).sqrt().min(8.0);
        let has_o3_or_haze = atmospheric_composition.iter().any(|(_, c)| *c == ChemicalComponent::Oxygen)
            || life_level.as_u8() >= LifeLevel::PlantLike.as_u8();
        let exobase_km = tropopause_km * 10.0;
        Some(AtmosphericLayers { scale_height_km, tropopause_km, has_stratosphere: has_o3_or_haze, exobase_km })
    } else { None };

    // 2. Breathability & Toxicity
    let breathability = match atmospheric_pressure {
        p if p < 0.001 => AtmosphereBreathability::Vacuum,
        p if p < 0.1 => AtmosphereBreathability::Trace,
        p if p < 0.43 => AtmosphereBreathability::VeryThin,
        p if p < 0.71 => AtmosphereBreathability::ThinBreathable,
        p if p < 1.5 => AtmosphereBreathability::Standard,
        p if p < 2.5 => AtmosphereBreathability::Dense,
        p if p < 10.0 => AtmosphereBreathability::VeryDense,
        _ => AtmosphereBreathability::Superdense,
    };

    let has_oxygen = atmospheric_composition.iter().any(|(f, c)| *c == ChemicalComponent::Oxygen && *f > 0.1);
    let has_co2_high = atmospheric_composition.iter().any(|(f, c)| *c == ChemicalComponent::CarbonDioxide && *f > 0.05);
    let has_h2s = atmospheric_composition.iter().any(|(_, c)| *c == ChemicalComponent::HydrogenSulfide);
    let has_so2 = atmospheric_composition.iter().any(|(_, c)| *c == ChemicalComponent::SulfurDioxide);
    let has_hcl = atmospheric_composition.iter().any(|(_, c)| *c == ChemicalComponent::Chlorine);

    let toxicity = if !has_atmosphere { AtmosphereToxicity::Benign }
    else if has_hcl { AtmosphereToxicity::Insidious }
    else if has_so2 && atmospheric_pressure > 1.0 { AtmosphereToxicity::Corrosive }
    else if has_h2s { AtmosphereToxicity::HighlyToxic }
    else if has_co2_high { AtmosphereToxicity::MildlyToxic }
    else if !has_oxygen && atmospheric_pressure > 0.1 { AtmosphereToxicity::Suffocating }
    else if has_oxygen && atmospheric_pressure > 0.4 && atmospheric_pressure < 2.0 { AtmosphereToxicity::Benign }
    else { AtmosphereToxicity::Marginal };

    // 3. Cloud decks
    let mut cloud_decks = Vec::new();
    if has_atmosphere {
        let scale_h = atmospheric_layers.as_ref().map_or(8.5, |l| l.scale_height_km);
        // Water clouds
        if has_liquid && blackbody_temperature > 200 && blackbody_temperature < 400 {
            cloud_decks.push(CloudDeck {
                composition: if blackbody_temperature > 273 { CloudComposition::Water } else { CloudComposition::WaterIce },
                base_altitude_km: scale_h * 0.5,
                top_altitude_km: scale_h * 1.5,
                optical_depth: (hydrosphere / 30.0).clamp(1.0, 50.0),
                coverage_fraction: (hydrosphere / 150.0 + 0.2).clamp(0.1, 1.0),
            });
        }
        // CO2/H2SO4 clouds for Venus-like
        if has_co2_high && atmospheric_pressure > 10.0 {
            cloud_decks.push(CloudDeck {
                composition: CloudComposition::SulfuricAcid,
                base_altitude_km: scale_h * 3.0,
                top_altitude_km: scale_h * 5.0,
                optical_depth: 30.0,
                coverage_fraction: 1.0,
            });
        }
        // Ammonia clouds
        if atmospheric_composition.iter().any(|(_, c)| *c == ChemicalComponent::Ammonia) && blackbody_temperature < 200 {
            cloud_decks.push(CloudDeck {
                composition: CloudComposition::Ammonia,
                base_altitude_km: scale_h * 1.0,
                top_altitude_km: scale_h * 2.0,
                optical_depth: 5.0,
                coverage_fraction: 0.6,
            });
        }
        // Methane clouds
        if atmospheric_composition.iter().any(|(_, c)| *c == ChemicalComponent::Methane) && blackbody_temperature < 150 {
            cloud_decks.push(CloudDeck {
                composition: CloudComposition::Methane,
                base_altitude_km: scale_h * 0.3,
                top_altitude_km: scale_h * 1.0,
                optical_depth: 3.0,
                coverage_fraction: 0.4,
            });
        }
    }

    // 4. Greenhouse effect
    let greenhouse = if has_atmosphere {
        let equilibrium_temp = (278.0 * 1.0_f32.powf(0.25)) / 1.0; // simplified, using blackbody as proxy
        let co2_fraction: f32 = atmospheric_composition.iter()
            .filter(|(_, c)| *c == ChemicalComponent::CarbonDioxide).map(|(f, _)| f).sum();
        let co2_pp = co2_fraction * atmospheric_pressure;
        let delta = if co2_pp > 0.0004 { (10.0 * (co2_pp / 0.0004).ln()).max(0.0) } else { 0.0 };
        let is_runaway = blackbody_temperature as f32 + delta > 500.0 && hydrosphere < 5.0;
        let albedo = if cloud_decks.iter().any(|c| c.coverage_fraction > 0.8) { 0.7 }
            else if hydrosphere > 50.0 { 0.3 }
            else if ice_over_land > 50.0 { 0.6 }
            else { 0.25 };
        Some(GreenhouseEffect {
            equilibrium_temp_k: blackbody_temperature as f32,
            surface_temp_k: blackbody_temperature as f32 + delta,
            greenhouse_delta_k: delta,
            bond_albedo: albedo,
            is_runaway,
        })
    } else { None };

    // 5. Sky appearance
    let sky = if atmospheric_pressure > 0.001 {
        let has_dust = body_type == TelluricBodyComposition::Rocky && atmospheric_pressure < 0.05;
        let has_tholin = atmospheric_composition.iter().any(|(_, c)| *c == ChemicalComponent::Methane)
            && atmospheric_composition.iter().any(|(_, c)| *c == ChemicalComponent::Nitrogen);
        let daytime_color = if atmospheric_pressure < 0.001 { SkyColor::Black }
            else if has_tholin { SkyColor::Orange }
            else if has_dust { SkyColor::Butterscotch }
            else if has_co2_high && atmospheric_pressure > 10.0 { SkyColor::Amber }
            else if has_oxygen { SkyColor::Blue }
            else if atmospheric_pressure < 0.2 { SkyColor::PaleBlue }
            else { SkyColor::White };
        let sunset_color = if has_dust { SkyColor::Blue } // Mars has blue sunsets
            else if daytime_color == SkyColor::Blue { SkyColor::Red }
            else { SkyColor::Yellow };
        Some(SkyAppearance { daytime_color, sunset_color, daytime_stars_visible: atmospheric_pressure < 0.05 })
    } else { Some(SkyAppearance { daytime_color: SkyColor::Black, sunset_color: SkyColor::Black, daytime_stars_visible: true }) };

    // 6. Wind profile
    let wind = if has_atmosphere {
        let omega = if rotation_days.abs() > 0.01 { 2.0 * std::f32::consts::PI / (rotation_days * 86400.0) } else { 0.0 };
        let is_slow = rotation_days > 10.0;
        let base_wind = (atmospheric_pressure * 5.0 + gravity * 3.0).sqrt() * 5.0;
        let max_wind = base_wind * (2.0 + if is_slow { 3.0 } else { 1.0 });
        let superrotation = is_slow && atmospheric_pressure > 0.5;
        Some(WindProfile {
            mean_surface_wind_ms: base_wind.min(200.0),
            max_wind_ms: max_wind.min(600.0),
            superrotation,
        })
    } else { None };

    // 7. Hydrography (rivers)
    let hydrography = if has_liquid && land_fraction > 0.05 && atmospheric_pressure > 0.01 {
        let precip = (hydrosphere * 15.0 * atmospheric_pressure.sqrt()).clamp(0.0, 3000.0);
        let land_area_km2 = 4.0 * std::f64::consts::PI * (radius * 6371.0).powi(2) * land_fraction as f64;
        let basin_area = 600_000.0_f64;
        let river_count = (land_area_km2 / basin_area).max(1.0) as u32;
        let longest = (1.4 * (land_area_km2 / river_count as f64).powf(0.57)) as f32;
        let delta = if precip > 1000.0 { DeltaType::BirdFoot }
            else if precip > 500.0 { DeltaType::Arcuate }
            else { DeltaType::Cuspate };
        Some(Hydrography { major_river_count: river_count, longest_river_km: longest.min(20000.0), mean_precipitation_mm: precip, dominant_delta_type: delta })
    } else { None };

    // 8. Lake distribution
    let lakes = if has_liquid && land_fraction > 0.02 {
        let is_glaciated = ice_over_land > 10.0;
        let lake_density = if is_glaciated { 0.3 } else { 0.01 };
        let land_area_km2 = 4.0 * std::f64::consts::PI * (radius * 6371.0).powi(2) * land_fraction as f64;
        let count = (land_area_km2 * lake_density as f64 / 10000.0).max(1.0) as u32;
        let largest = (land_area_km2 * 0.005).min(500000.0) as f32;
        let dom_type = if is_glaciated { LakeFormationType::Glacial }
            else if tectonic_activity > 30.0 { LakeFormationType::Tectonic }
            else if volcanism > 30.0 { LakeFormationType::Volcanic }
            else { LakeFormationType::Fluvial };
        let liquid = match world_type {
            CelestialBodyWorldType::Ammonia => LiquidType::Ammonia,
            _ if blackbody_temperature < 150 => LiquidType::MethaneEthane,
            _ => LiquidType::Water,
        };
        Some(LakeDistribution { lake_count: count, dominant_type: dom_type, largest_lake_km2: largest, liquid_type: liquid })
    } else { None };

    // 9. Glaciation state
    let glaciation = {
        let ice_fraction = (ice_over_water * hydrosphere / 100.0 + ice_over_land * (100.0 - hydrosphere) / 100.0) / 100.0;
        let snowball = ice_fraction > 0.9;
        let cap_loc = if is_tidally_locked { IceCapLocation::DarkSide }
            else if snowball { IceCapLocation::Global }
            else if axial_tilt > 40.0 { IceCapLocation::Equatorial }
            else if ice_fraction > 0.01 { IceCapLocation::Polar }
            else { IceCapLocation::None };
        if ice_fraction > 0.01 || snowball {
            Some(GlaciationState { ice_coverage_fraction: ice_fraction, in_glacial_period: ice_fraction > 0.2, snowball_state: snowball, ice_cap_location: cap_loc })
        } else { None }
    };

    // 10. Ocean chemistry
    let ocean_chemistry = if hydrosphere > 5.0 {
        let liquid = match world_type {
            CelestialBodyWorldType::Ammonia => LiquidType::Ammonia,
            _ if blackbody_temperature < 150 => LiquidType::MethaneEthane,
            _ => LiquidType::Water,
        };
        let salinity = rng.roll(1, 60, 10) as f32;
        let ph = if liquid == LiquidType::Water { 6.5 + rng.roll(1, 30, 0) as f32 / 10.0 } else { 0.0 };
        let anoxic = !has_oxygen || life_level.as_u8() < LifeLevel::PlantLike.as_u8();
        let iron = if anoxic && liquid == LiquidType::Water { OceanIronContent::High } else { OceanIronContent::Negligible };
        let vents = volcanism > 10.0 && hydrosphere > 20.0;
        Some(OceanChemistry { liquid_type: liquid, salinity_g_per_kg: salinity, ph, anoxic, iron_content: iron, hydrothermal_vents: vents })
    } else { None };

    // 11. Volcanic profile
    let volcanic_profile = if volcanism > 1.0 {
        let has_tectonics = tectonic_activity > 10.0;
        let dom_type = if tidal_heating > 10 { VolcanoType::Fissure }
            else if body_type == TelluricBodyComposition::Icy { VolcanoType::Cryovolcano }
            else if has_tectonics { VolcanoType::Stratovolcano }
            else { VolcanoType::Shield };
        let count = (volcanism * 15.0 + if has_tectonics { 500.0 } else { 50.0 }) as u32;
        let tallest = if has_tectonics { volcanism * 0.2 } else { volcanism * 0.5 / gravity.max(0.1) };
        let super_v = volcanism > 50.0 && rng.roll(1, 4, 0) == 1;
        let flood = volcanism > 40.0 && rng.roll(1, 6, 0) <= 2;
        Some(VolcanicProfile { active_count: count, dominant_type: dom_type, flood_basalt_history: flood, tallest_volcano_km: tallest.min(25.0), supervolcano_present: super_v })
    } else { None };

    // 12. Mineral diversity
    let mineral_diversity = {
        let (stage, count) = if life_level.as_u8() >= LifeLevel::PlantLike.as_u8() && has_oxygen {
            (MineralEvolutionStage::Biogenic, 4000 + rng.roll(1, 2000, 0) as u32)
        } else if has_oxygen {
            (MineralEvolutionStage::Oxidized, 3000 + rng.roll(1, 1500, 0) as u32)
        } else if tectonic_activity > 10.0 {
            (MineralEvolutionStage::TectonicallyActive, 1200 + rng.roll(1, 500, 0) as u32)
        } else if hydrosphere > 5.0 {
            (MineralEvolutionStage::Hydrated, 800 + rng.roll(1, 400, 0) as u32)
        } else if volcanism > 5.0 {
            (MineralEvolutionStage::Differentiated, 300 + rng.roll(1, 200, 0) as u32)
        } else {
            (MineralEvolutionStage::Primordial, 40 + rng.roll(1, 30, 0) as u32)
        };
        Some(MineralDiversity { mineral_count: count, evolution_stage: stage })
    };

    // 13. Surface material
    let surface_material = {
        let primary = if volcanism > 60.0 { SurfaceMaterialType::SandDunes }
            else if body_type == TelluricBodyComposition::Icy { SurfaceMaterialType::IceCrust }
            else if !has_atmosphere { SurfaceMaterialType::Regolith }
            else if has_oxygen && life_level.as_u8() >= LifeLevel::PlantLike.as_u8() { SurfaceMaterialType::Soil }
            else if blackbody_temperature > 200 && atmospheric_pressure < 0.05 { SurfaceMaterialType::IronOxideFines }
            else if hydrosphere < 5.0 && atmospheric_pressure < 0.1 { SurfaceMaterialType::BarrenRock }
            else { SurfaceMaterialType::BarrenRock };
        let depth = if !has_atmosphere { rng.roll(1, 15, 2) as f32 }
            else { rng.roll(1, 20, 5) as f32 };
        let perchlorate = !has_atmosphere && body_type == TelluricBodyComposition::Rocky && rng.roll(1, 3, 0) == 1;
        let oxidized = has_atmosphere || primary == SurfaceMaterialType::IronOxideFines;
        Some(SurfaceMaterial { primary_type: primary, depth_m: depth, perchlorates: perchlorate, oxidized })
    };

    // 14. Radiation environment
    let radiation = {
        let has_mag = magnetic_field != MagneticFieldStrength::None;
        let mag_factor = if has_mag { 0.1 } else { 1.0 };
        let atmo_factor = if atmospheric_pressure > 0.5 { 0.1 } else if atmospheric_pressure > 0.01 { 0.5 } else { 1.0 };
        let base_dose = 400.0; // mSv/yr unshielded at 1 AU
        let dose = base_dose * mag_factor * atmo_factor;
        let uv = if has_atmosphere && atmospheric_pressure > 0.1 {
            if has_oxygen { 8.0 } else { 25.0 }
        } else { 50.0 };
        let hazard = if dose < 5.0 { RadiationHazard::Negligible }
            else if dose < 50.0 { RadiationHazard::Low }
            else if dose < 500.0 { RadiationHazard::Moderate }
            else if dose < 10000.0 { RadiationHazard::High }
            else { RadiationHazard::Extreme };
        Some(RadiationEnvironment { surface_dose_msv_yr: dose, uv_index_peak: uv, radiation_hazard: hazard })
    };

    // 15. Seismic profile
    let seismic = {
        let source = if tidal_heating > 15 { SeismicitySource::TidalExtreme }
            else if tidal_heating > 3 { SeismicitySource::TidalOnly }
            else if tectonic_activity > 50.0 { SeismicitySource::TectonicExtreme }
            else if tectonic_activity > 10.0 { SeismicitySource::TectonicModerate }
            else if volcanism > 5.0 { SeismicitySource::Residual }
            else { SeismicitySource::None };
        let max_mag = match source {
            SeismicitySource::None => 0.0,
            SeismicitySource::Residual => 3.0 + rng.roll(1, 20, 0) as f32 / 10.0,
            SeismicitySource::TidalOnly => 3.0 + rng.roll(1, 20, 0) as f32 / 10.0,
            SeismicitySource::TectonicModerate => 6.0 + rng.roll(1, 20, 0) as f32 / 10.0,
            SeismicitySource::TectonicExtreme => 8.0 + rng.roll(1, 15, 0) as f32 / 10.0,
            SeismicitySource::TidalExtreme => 5.0 + rng.roll(1, 30, 0) as f32 / 10.0,
        };
        let quakes_m4 = match source {
            SeismicitySource::None => 0,
            SeismicitySource::Residual => rng.roll(1, 100, 0) as u32,
            SeismicitySource::TidalOnly => rng.roll(1, 500, 50) as u32,
            SeismicitySource::TectonicModerate => rng.roll(1, 10000, 5000) as u32,
            SeismicitySource::TectonicExtreme => rng.roll(1, 20000, 10000) as u32,
            SeismicitySource::TidalExtreme => rng.roll(1, 50000, 10000) as u32,
        };
        Some(SeismicProfile { max_magnitude: max_mag, quakes_per_year_m4: quakes_m4, seismicity_source: source })
    };

    // 16. Dust storms
    let dust_storms = if atmospheric_pressure > 0.001 && atmospheric_pressure < 0.1 && land_fraction > 0.3 {
        let global = atmospheric_pressure < 0.02 && land_fraction > 0.5;
        let interval = if global { 2.0 + rng.roll(1, 6, 0) as f32 } else { 0.0 };
        let peak = (15.0 + rng.roll(1, 30, 0) as f32).min(50.0);
        Some(DustStormProfile { global_storms_possible: global, global_storm_interval_years: interval, peak_wind_ms: peak, dust_devils_active: true })
    } else { None };

    // 17. Lightning
    let lightning = {
        let has_water_clouds = cloud_decks.iter().any(|c| c.composition == CloudComposition::Water);
        let has_volc = volcanism > 20.0;
        let has_dust = dust_storms.is_some();
        if has_water_clouds || has_volc || has_dust {
            let mechanism = if has_water_clouds { LightningMechanism::WaterCloud }
                else if has_volc { LightningMechanism::VolcanicPlume }
                else { LightningMechanism::DustTriboelectric };
            let rate = match mechanism {
                LightningMechanism::WaterCloud => (hydrosphere / 70.0).clamp(0.1, 5.0),
                LightningMechanism::VolcanicPlume => 0.1,
                LightningMechanism::DustTriboelectric => 0.01,
                _ => 0.0,
            };
            Some(LightningProfile { present: true, flash_rate_relative: rate, mechanism })
        } else { None }
    };

    PlanetaryDetail {
        atmospheric_layers,
        breathability,
        toxicity,
        cloud_decks,
        greenhouse,
        sky,
        wind,
        hydrography,
        lakes,
        glaciation,
        ocean_chemistry,
        volcanic_profile,
        mineral_diversity,
        surface_material,
        radiation,
        seismic,
        dust_storms,
        lightning,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn earth_like_detail() -> PlanetaryDetail {
        let comp = vec![
            (0.78, ChemicalComponent::Nitrogen),
            (0.21, ChemicalComponent::Oxygen),
            (0.01, ChemicalComponent::Argon),
        ];
        generate_planetary_detail(
            1.0, &comp, 288, 1.0, 1.0, 71.0, 5.0, 10.0,
            30.0, 35.0, MagneticFieldStrength::Moderate,
            TelluricBodyComposition::Rocky, CelestialBodyWorldType::Terrestrial,
            LifeLevel::Sentient, 23.4, 0.017, 1.0, false, 0,
            "test", SpaceCoordinates::new(0, 0, 0), 0, 0, 0,
        )
    }

    #[test]
    fn earth_like_has_blue_sky() {
        let d = earth_like_detail();
        assert_eq!(d.sky.unwrap().daytime_color, SkyColor::Blue);
    }

    #[test]
    fn earth_like_is_breathable() {
        let d = earth_like_detail();
        assert_eq!(d.breathability, AtmosphereBreathability::Standard);
        assert_eq!(d.toxicity, AtmosphereToxicity::Benign);
    }

    #[test]
    fn earth_like_has_rivers_and_lakes() {
        let d = earth_like_detail();
        assert!(d.hydrography.is_some());
        assert!(d.hydrography.unwrap().major_river_count > 0);
        assert!(d.lakes.is_some());
    }

    #[test]
    fn earth_like_has_water_clouds() {
        let d = earth_like_detail();
        assert!(!d.cloud_decks.is_empty());
        assert_eq!(d.cloud_decks[0].composition, CloudComposition::Water);
    }

    #[test]
    fn earth_like_has_lightning() {
        let d = earth_like_detail();
        assert!(d.lightning.is_some());
        assert_eq!(d.lightning.unwrap().mechanism, LightningMechanism::WaterCloud);
    }

    #[test]
    fn earth_like_mineral_diversity() {
        let d = earth_like_detail();
        let m = d.mineral_diversity.unwrap();
        assert!(m.mineral_count > 4000, "Earth-like should have >4000 minerals, got {}", m.mineral_count);
        assert_eq!(m.evolution_stage, MineralEvolutionStage::Biogenic);
    }

    #[test]
    fn earth_like_radiation_low() {
        let d = earth_like_detail();
        let r = d.radiation.unwrap();
        assert!(matches!(r.radiation_hazard, RadiationHazard::Negligible | RadiationHazard::Low));
    }

    #[test]
    fn airless_body_has_black_sky() {
        let d = generate_planetary_detail(
            0.0, &[], 200, 0.16, 0.27, 0.0, 0.0, 0.0,
            0.0, 0.0, MagneticFieldStrength::None,
            TelluricBodyComposition::Rocky, CelestialBodyWorldType::Rock,
            LifeLevel::None, 1.5, 0.05, 27.3, false, 0,
            "test", SpaceCoordinates::new(0, 0, 0), 0, 0, 1,
        );
        let sky = d.sky.unwrap();
        assert_eq!(sky.daytime_color, SkyColor::Black);
        assert!(sky.daytime_stars_visible);
        assert!(d.cloud_decks.is_empty());
        assert_eq!(d.breathability, AtmosphereBreathability::Vacuum);
    }

    #[test]
    fn mars_like_has_dust_storms() {
        let comp = vec![(0.95, ChemicalComponent::CarbonDioxide), (0.03, ChemicalComponent::Nitrogen)];
        let d = generate_planetary_detail(
            0.006, &comp, 210, 0.38, 0.53, 0.0, 0.0, 0.0,
            5.0, 2.0, MagneticFieldStrength::None,
            TelluricBodyComposition::Rocky, CelestialBodyWorldType::Rock,
            LifeLevel::None, 25.2, 0.093, 1.03, false, 0,
            "test", SpaceCoordinates::new(0, 0, 0), 0, 0, 2,
        );
        assert!(d.dust_storms.is_some());
        assert_eq!(d.sky.unwrap().daytime_color, SkyColor::Butterscotch);
    }

    #[test]
    fn venus_like_is_corrosive() {
        let comp = vec![(0.965, ChemicalComponent::CarbonDioxide), (0.035, ChemicalComponent::Nitrogen)];
        let d = generate_planetary_detail(
            92.0, &comp, 737, 0.91, 0.95, 0.0, 0.0, 0.0,
            30.0, 20.0, MagneticFieldStrength::None,
            TelluricBodyComposition::Rocky, CelestialBodyWorldType::Greenhouse,
            LifeLevel::None, 177.0, 0.007, 243.0, false, 0,
            "test", SpaceCoordinates::new(0, 0, 0), 0, 0, 3,
        );
        assert!(matches!(d.toxicity, AtmosphereToxicity::MildlyToxic | AtmosphereToxicity::Suffocating));
        assert_eq!(d.breathability, AtmosphereBreathability::Superdense);
    }
}
