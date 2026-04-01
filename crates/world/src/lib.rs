#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code, unused_imports, unused)]

pub mod celestial_body;
pub mod celestial_disk;
pub mod contents;
pub mod neighborhood;
pub mod orbital_point;
pub mod star;
pub mod types;
pub mod utils;

extern crate log;

// ── Galaxy / Universe stubs ──────────────────────────────────────────
// Minimal copies of types that live in the main crate's galaxy, universe
// and generator modules. Only the fields and methods actually used by the
// world crate are included here.

pub mod galaxy_stubs {
    use crate::internal::*;
    use crate::celestial_body::types::CelestialBodySettings;
    use crate::neighborhood::StellarNeighborhood;
    use crate::star::types::StarSettings;
    use crate::types::{StarSystem, SystemSettings};
    use std::ops::*;

    // ── SpaceCoordinates ──────────────────────────────────────────────

    /// Coordinates of a point in a galactic map, in parsecs.
    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
    )]
    pub struct SpaceCoordinates {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }

    impl SpaceCoordinates {
        pub fn new(x: i64, y: i64, z: i64) -> Self {
            SpaceCoordinates { x, y, z }
        }
        pub fn abs(self, starting_point: SpaceCoordinates) -> Self {
            self - starting_point
        }
        pub fn rel(self, starting_point: SpaceCoordinates) -> Self {
            self + starting_point
        }
    }

    impl Display for SpaceCoordinates {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
        }
    }

    impl Add for SpaceCoordinates {
        type Output = Self;
        fn add(self, o: Self) -> Self {
            Self {
                x: self.x + o.x,
                y: self.y + o.y,
                z: self.z + o.z,
            }
        }
    }

    impl Sub for SpaceCoordinates {
        type Output = Self;
        fn sub(self, o: Self) -> Self {
            Self {
                x: self.x - o.x,
                y: self.y - o.y,
                z: self.z - o.z,
            }
        }
    }

    impl Mul for SpaceCoordinates {
        type Output = Self;
        fn mul(self, o: Self) -> Self {
            Self {
                x: self.x * o.x,
                y: self.y * o.y,
                z: self.z * o.z,
            }
        }
    }

    impl Div for SpaceCoordinates {
        type Output = Self;
        fn div(self, o: Self) -> Self {
            Self {
                x: self.x / o.x,
                y: self.y / o.y,
                z: self.z / o.z,
            }
        }
    }

    // ── StelliferousEra ───────────────────────────────────────────────

    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
    )]
    pub enum StelliferousEra {
        AncientStelliferous,
        EarlyStelliferous,
        #[default]
        MiddleStelliferous,
        LateStelliferous,
        EndStelliferous,
    }

    impl Display for StelliferousEra {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                StelliferousEra::AncientStelliferous => write!(f, "Ancient Stelliferous"),
                StelliferousEra::EarlyStelliferous => write!(f, "Early Stelliferous"),
                StelliferousEra::MiddleStelliferous => write!(f, "Middle Stelliferous"),
                StelliferousEra::LateStelliferous => write!(f, "Late Stelliferous"),
                StelliferousEra::EndStelliferous => write!(f, "End Stelliferous"),
            }
        }
    }

    // ── Universe ──────────────────────────────────────────────────────

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
    pub struct Universe {
        pub era: StelliferousEra,
        #[default = 13.8]
        pub age: f32,
    }

    impl Display for Universe {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "A {} billion years old Universe in the {} era",
                self.age, self.era
            )
        }
    }

    impl Universe {
        pub fn new(era: StelliferousEra, age: f32) -> Self {
            Self { era, age }
        }

        /// Simplified generate for use in tests. Returns a default universe.
        pub fn generate(settings: &GenerationSettings) -> Self {
            let mut rng =
                SeededDiceRoller::new(settings.seed.as_ref(), "uni_age");
            let age = if settings.universe.use_ours {
                13.8
            } else if let Some(a) = settings.universe.fixed_age {
                a
            } else {
                (rng.roll(1, 9960, 40) as f32) / 100.0
            };
            let era = if age < 1.0 {
                StelliferousEra::AncientStelliferous
            } else if age < 6.0 {
                StelliferousEra::EarlyStelliferous
            } else if age < 20.0 {
                StelliferousEra::MiddleStelliferous
            } else if age < 100.0 {
                StelliferousEra::LateStelliferous
            } else {
                StelliferousEra::EndStelliferous
            };
            Self { era, age }
        }
    }

    // ── GalacticNeighborhoodDensity ───────────────────────────────────

    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
    )]
    pub enum GalacticNeighborhoodDensity {
        Void(#[default = 1] u8, #[default = 4] u16),
        #[default]
        Group(#[default = 2] u8, #[default = 23] u16),
        Cluster(#[default = 1] u8, #[default = 8] u8, #[default = 209] u16),
    }

    impl Display for GalacticNeighborhoodDensity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                GalacticNeighborhoodDensity::Void(g, m) => write!(f, "Void({},{})", g, m),
                GalacticNeighborhoodDensity::Group(g, m) => write!(f, "Group({},{})", g, m),
                GalacticNeighborhoodDensity::Cluster(d, g, m) => {
                    write!(f, "Cluster({},{},{})", d, g, m)
                }
            }
        }
    }

    // ── GalacticNeighborhood ──────────────────────────────────────────

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
    pub struct GalacticNeighborhood {
        pub universe: Universe,
        pub density: GalacticNeighborhoodDensity,
    }

    impl Display for GalacticNeighborhood {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Galactic {}", self.density)
        }
    }

    impl GalacticNeighborhood {
        /// Simplified generate for use in tests.
        pub fn generate(universe: Universe, _settings: &GenerationSettings) -> Self {
            Self {
                universe,
                density: GalacticNeighborhoodDensity::default(),
            }
        }
    }

    // ── GalacticRegion ────────────────────────────────────────────────

    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
    )]
    pub enum GalacticRegion {
        Multiple,
        Core,
        Nucleus,
        Bulge,
        Bar,
        Arm,
        Disk,
        Ellipse,
        Halo,
        Aura,
        #[default]
        Void,
        GlobularCluster,
        OpenCluster,
        Association,
        Stream,
        Exile,
    }

    // ── GalacticMapDivision ───────────────────────────────────────────

    #[derive(
        Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
    )]
    pub struct GalacticMapDivision {
        #[default("default")]
        pub name: Rc<str>,
        pub region: GalacticRegion,
        pub level: u8,
        pub x: u8,
        pub y: u8,
        pub z: u8,
        pub index: SpaceCoordinates,
    }

    // ── GalacticMapDivisionLevel ──────────────────────────────────────

    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
    )]
    pub struct GalacticMapDivisionLevel {
        pub level: u8,
        pub x: u8,
        pub y: u8,
        pub z: u8,
    }

    impl GalacticMapDivisionLevel {
        pub fn new(level: u8, x: u8, y: u8, z: u8) -> Self {
            Self { level, x, y, z }
        }
        pub fn as_coord(&self) -> SpaceCoordinates {
            SpaceCoordinates::new(self.x as i64, self.y as i64, self.z as i64)
        }
    }

    // ── GalaxyCategory ────────────────────────────────────────────────

    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
    )]
    pub enum GalaxyCategory {
        Intergalactic(
            #[default = 1000] u32,
            #[default = 3000] u32,
            #[default = 1000] u32,
        ),
        Irregular(
            #[default = 3000] u32,
            #[default = 3000] u32,
            #[default = 2000] u32,
        ),
        #[default]
        Spiral(#[default = 10000] u32, #[default = 100] u32),
        Lenticular(#[default = 10000] u32, #[default = 600] u32),
        Elliptical(#[default = 10000] u32),
        Intracluster(#[default = 1] u32, #[default = 3] u32, #[default = 1] u32),
        DominantElliptical(#[default = 300000] u32),
    }

    impl Display for GalaxyCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    // ── GalaxySubCategory ─────────────────────────────────────────────

    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
    )]
    pub enum GalaxySubCategory {
        DwarfAmorphous,
        Amorphous,
        DwarfSpiral,
        FlatSpiral,
        #[default]
        BarredSpiral,
        ClassicSpiral,
        DwarfLenticular,
        CommonLenticular,
        GiantLenticular,
        DwarfElliptical,
        CommonElliptical,
        GiantElliptical,
    }

    // ── GalaxySpecialTrait ────────────────────────────────────────────

    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
    )]
    pub enum GalaxySpecialTrait {
        #[default]
        NoPeculiarity,
        ActiveNucleus, DoubleNuclei, Compact(u8), Expansive(u8),
        ExtendedHalo, MetalPoor, Dusty, GasPoor, GasRich,
        Starburst, Dead, Dormant, Satellites(u8),
        Interacting, Tail, Younger, Older,
        SubSize(u8), SuperSize(u16),
    }

    // ── GalacticHex ───────────────────────────────────────────────────

    #[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
    pub struct GalacticHex {
        pub index: SpaceCoordinates,
        pub neighborhood: StellarNeighborhood,
        pub contents: Vec<StarSystem>,
    }

    impl GalacticHex {
        pub fn new(
            index: SpaceCoordinates,
            neighborhood: StellarNeighborhood,
            contents: Vec<StarSystem>,
        ) -> Self {
            Self {
                index,
                neighborhood,
                contents,
            }
        }

        /// Simplified generate for use in tests.
        pub fn generate(
            _coord: SpaceCoordinates,
            index: SpaceCoordinates,
            galaxy: &mut Galaxy,
        ) -> Self {
            let neighborhood = StellarNeighborhood::generate(index, galaxy);
            Self {
                index,
                neighborhood,
                contents: vec![],
            }
        }
    }

    impl Display for GalacticHex {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Hex {} in {} containing {} star systems",
                self.index,
                self.neighborhood,
                self.contents.len(),
            )
        }
    }

    // ── Galaxy ────────────────────────────────────────────────────────

    #[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
    pub struct Galaxy {
        pub settings: GenerationSettings,
        pub neighborhood: GalacticNeighborhood,
        pub index: u16,
        pub name: Rc<str>,
        pub age: f32,
        pub is_dominant: bool,
        pub is_major: bool,
        pub category: GalaxyCategory,
        pub sub_category: GalaxySubCategory,
        pub special_traits: Vec<GalaxySpecialTrait>,
        pub division_levels: Vec<GalacticMapDivisionLevel>,
        pub divisions: Vec<GalacticMapDivision>,
        pub hexes: Vec<GalacticHex>,
    }

    impl Default for Galaxy {
        fn default() -> Self {
            Self {
                settings: GenerationSettings::default(),
                neighborhood: GalacticNeighborhood::default(),
                index: 0,
                name: "Milky Way".into(),
                age: 13.0,
                is_dominant: false,
                is_major: true,
                category: GalaxyCategory::default(),
                sub_category: GalaxySubCategory::default(),
                special_traits: vec![GalaxySpecialTrait::NoPeculiarity],
                division_levels: vec![],
                divisions: vec![],
                hexes: vec![],
            }
        }
    }

    impl Display for Galaxy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:04} - \"{}\" - {}", self.index, self.name, self.category)
        }
    }

    impl Galaxy {
        /// Simplified generate for use in tests.
        pub fn generate(
            neighborhood: GalacticNeighborhood,
            index: u16,
            settings: &GenerationSettings,
        ) -> Self {
            let category = GalaxyCategory::Spiral(10, 2);
            let division_levels = vec![
                GalacticMapDivisionLevel::new(0, 1, 1, 1),
                GalacticMapDivisionLevel::new(1, 10, 10, 10),
                GalacticMapDivisionLevel::new(2, 4, 4, 4),
                GalacticMapDivisionLevel::new(3, 10, 10, 10),
                GalacticMapDivisionLevel::new(4, 10, 10, 10),
                GalacticMapDivisionLevel::new(5, 10, 10, 10),
                GalacticMapDivisionLevel::new(6, 10, 10, 10),
                GalacticMapDivisionLevel::new(7, 10, 10, 10),
                GalacticMapDivisionLevel::new(8, 10, 10, 10),
                GalacticMapDivisionLevel::new(9, 10, 10, 10),
            ];
            Self {
                settings: settings.clone(),
                neighborhood,
                index,
                name: format!("Galaxy-{}", index).into(),
                age: neighborhood.universe.age - 0.5,
                is_dominant: false,
                is_major: true,
                category,
                sub_category: GalaxySubCategory::default(),
                special_traits: vec![GalaxySpecialTrait::NoPeculiarity],
                division_levels,
                divisions: vec![],
                hexes: vec![],
            }
        }

        pub fn get_galactic_start(&self) -> SpaceCoordinates {
            match self.category {
                GalaxyCategory::Intergalactic(l, w, h)
                | GalaxyCategory::Irregular(l, w, h)
                | GalaxyCategory::Intracluster(l, w, h) => {
                    let x: i64 = if l % 2 == 0 {
                        1 - (l as i64 / 2)
                    } else {
                        -(l as i64 / 2)
                    };
                    let y: i64 = if w % 2 == 0 {
                        1 - (w as i64 / 2)
                    } else {
                        -(w as i64 / 2)
                    };
                    let z: i64 = if h % 2 == 0 {
                        1 - (h as i64 / 2)
                    } else {
                        -(h as i64 / 2)
                    };
                    SpaceCoordinates::new(x, y, z)
                }
                GalaxyCategory::Spiral(r, d) | GalaxyCategory::Lenticular(r, d) => {
                    let x: i64 = 1 - (r as i64);
                    let z: i64 = if d % 2 == 0 {
                        1 - (d as i64 / 2)
                    } else {
                        -(d as i64 / 2)
                    };
                    SpaceCoordinates::new(x, x, z)
                }
                GalaxyCategory::Elliptical(r) | GalaxyCategory::DominantElliptical(r) => {
                    let x: i64 = 1 - (r as i64);
                    SpaceCoordinates::new(x, x, x)
                }
            }
        }

        pub fn get_galactic_end(&self) -> SpaceCoordinates {
            match self.category {
                GalaxyCategory::Intergalactic(l, w, h)
                | GalaxyCategory::Irregular(l, w, h)
                | GalaxyCategory::Intracluster(l, w, h) => {
                    SpaceCoordinates::new(l as i64 / 2, w as i64 / 2, h as i64 / 2)
                }
                GalaxyCategory::Spiral(r, d) | GalaxyCategory::Lenticular(r, d) => {
                    SpaceCoordinates::new(r as i64, r as i64, d as i64 / 2)
                }
                GalaxyCategory::Elliptical(r) | GalaxyCategory::DominantElliptical(r) => {
                    SpaceCoordinates::new(r as i64, r as i64, r as i64)
                }
            }
        }

        fn are_coord_valid(&self, coord: SpaceCoordinates) -> bool {
            let start = self.get_galactic_start();
            let end = self.get_galactic_end();
            coord.x >= start.x
                && coord.y >= start.y
                && coord.z >= start.z
                && coord.x <= end.x
                && coord.y <= end.y
                && coord.z <= end.z
        }

        pub fn get_hex(&mut self, coord: SpaceCoordinates) -> Result<GalacticHex, Rc<str>> {
            if !self.are_coord_valid(coord) {
                return Err("Invalid coordinates.".into());
            }
            let starting_point = self.get_galactic_start();
            let abs_coord = coord.abs(starting_point);
            let hex_size = self
                .division_levels
                .iter()
                .find(|l| l.level == 0)
                .expect("The division levels should be set")
                .as_coord();
            let index = abs_coord / hex_size;
            let possible_hex = self.hexes.iter().find(|hex| hex.index == index);

            if let Some(hex) = possible_hex {
                Ok(hex.clone())
            } else {
                // Stub: generate a default hex for this index
                let new_hex = GalacticHex {
                    index,
                    neighborhood: StellarNeighborhood::default(),
                    contents: vec![],
                };
                self.hexes.push(new_hex.clone());
                Ok(new_hex)
            }
        }

        pub fn get_divisions_for_coord(
            &mut self,
            coord: SpaceCoordinates,
        ) -> Result<Vec<GalacticMapDivision>, Rc<str>> {
            if !self.are_coord_valid(coord) {
                return Err("Invalid coordinates.".into());
            }

            let mut result = Vec::new();
            let starting_point = self.get_galactic_start();
            let abs_coord = coord.abs(starting_point);

            let mut index = abs_coord;
            for i in 0..=9u8 {
                // Calculate next index from division levels
                let size = self
                    .division_levels
                    .iter()
                    .find(|l| l.level == i)
                    .map(|l| l.as_coord())
                    .unwrap_or(SpaceCoordinates::new(1, 1, 1));
                if size.x != 0 && size.y != 0 && size.z != 0 {
                    index = index / size;
                }

                let possible_division = self
                    .divisions
                    .iter()
                    .filter(|div| div.level == i)
                    .find(|div| div.index == index);

                if let Some(division) = possible_division {
                    result.push(division.clone());
                } else {
                    let new_division = GalacticMapDivision {
                        name: format!("div-{}-{}", i, index).into(),
                        region: GalacticRegion::Void,
                        level: i,
                        x: 0,
                        y: 0,
                        z: 0,
                        index,
                    };
                    self.divisions.push(new_division.clone());
                    result.push(new_division);
                }
            }

            Ok(result)
        }
    }

    // ── UniverseSettings ──────────────────────────────────────────────

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
    pub struct UniverseSettings {
        pub fixed_era: Option<StelliferousEra>,
        pub era_before: Option<StelliferousEra>,
        pub era_after: Option<StelliferousEra>,
        pub fixed_age: Option<f32>,
        pub age_before: Option<f32>,
        pub age_after: Option<f32>,
        pub use_ours: bool,
    }

    // ── GalaxySettings ────────────────────────────────────────────────

    #[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
    pub struct GalaxySettings {
        pub fixed_neighborhood: Option<GalacticNeighborhoodDensity>,
        pub fixed_category: Option<GalaxyCategory>,
        pub fixed_sub_category: Option<GalaxySubCategory>,
        pub fixed_special_traits: Option<Vec<GalaxySpecialTrait>>,
        pub forbidden_special_traits: Option<Vec<GalaxySpecialTrait>>,
        pub fixed_age: Option<f32>,
        pub use_ours: bool,
    }

    // ── SectorSettings ────────────────────────────────────────────────

    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
    )]
    pub struct SectorSettings {
        #[default((1, 1, 1))]
        pub hex_size: (u8, u8, u8),
        #[default((10, 10, 10))]
        pub level_1_size: (u8, u8, u8),
        #[default((4, 4, 4))]
        pub level_2_size: (u8, u8, u8),
        #[default((10, 10, 10))]
        pub level_3_size: (u8, u8, u8),
        #[default((10, 10, 10))]
        pub level_4_size: (u8, u8, u8),
        #[default((10, 10, 10))]
        pub level_5_size: (u8, u8, u8),
        #[default((10, 10, 10))]
        pub level_6_size: (u8, u8, u8),
        #[default((10, 10, 10))]
        pub level_7_size: (u8, u8, u8),
        #[default((10, 10, 10))]
        pub level_8_size: (u8, u8, u8),
        #[default((10, 10, 10))]
        pub level_9_size: (u8, u8, u8),
        #[default = true]
        pub flat_map: bool,
        #[default = true]
        pub density_by_hex_instead_of_parsec: bool,
        #[default = true]
        pub max_one_system_per_hex: bool,
    }

    // ── GenerationSettings ────────────────────────────────────────────

    #[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
    pub struct GenerationSettings {
        #[default("default")]
        pub seed: Rc<str>,
        pub universe: UniverseSettings,
        pub galaxy: GalaxySettings,
        pub sector: SectorSettings,
        pub system: SystemSettings,
        pub star: StarSettings,
        pub celestial_body: CelestialBodySettings,
        #[default(false)]
        pub populate: bool,
    }
}

// ── Prelude ──────────────────────────────────────────────────────────

pub mod prelude {
    pub use crate::celestial_body::gaseous::types::*;
    pub use crate::celestial_body::gaseous::GaseousBodyDetails;
    pub use crate::celestial_body::icy::types::*;
    pub use crate::celestial_body::icy::IcyBodyDetails;
    pub use crate::celestial_body::telluric::types::*;
    pub use crate::celestial_body::telluric::TelluricBodyDetails;
    pub use crate::celestial_body::traits::types::*;
    pub use crate::celestial_body::traits::*;
    pub use crate::celestial_body::types::*;
    pub use crate::celestial_body::world::types::*;
    pub use crate::celestial_body::world::WorldGenerator;
    pub use crate::celestial_body::CelestialBody;
    pub use crate::celestial_disk::belt::types::*;
    pub use crate::celestial_disk::belt::CelestialBeltDetails;
    pub use crate::celestial_disk::ring::types::*;
    pub use crate::celestial_disk::ring::CelestialRingDetails;
    pub use crate::celestial_disk::types::*;
    pub use crate::celestial_disk::CelestialDisk;
    pub use crate::contents::elements::*;
    pub use crate::contents::types::*;
    pub use crate::galaxy_stubs::*;
    pub use crate::neighborhood::types::*;
    pub use crate::neighborhood::StellarNeighborhood;
    pub use crate::orbital_point::types::*;
    pub use crate::orbital_point::OrbitalPoint;
    pub use crate::star::types::*;
    pub use crate::star::Star;
    pub use crate::types::*;
}

pub mod internal {
    pub use crate::celestial_body::moon::*;
    pub use crate::utils::conversion::ConversionUtils;
    pub use crate::utils::harmonics::OrbitalHarmonicsUtils;
    pub use crate::utils::math::MathUtils;
    pub use crate::utils::string::StringUtils;
    pub use log::*;
    pub use ordered_float::OrderedFloat;
    pub use seeded_dice_roller::*;
    pub use serde::{Deserialize, Serialize};
    pub use smart_default::SmartDefault;
    pub use std::fmt::Display;
    pub use std::mem::discriminant;
    pub use std::rc::Rc;
    pub use strum::IntoEnumIterator;
    pub use strum_macros::EnumIter;
}
