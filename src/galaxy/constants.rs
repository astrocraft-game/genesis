#![allow(dead_code)]
use crate::internal::*;
use crate::prelude::*;

/// The current age of the Milky Way.
pub const OUR_GALAXYS_AGE: f32 = 13.61;
/// The current age of the universe.
pub const OUR_GALAXYS_NAME: &str = "Milky Way";
/// The current age of the universe.
pub const OUR_GALAXYS_CATEGORY: GalaxyCategory = GalaxyCategory::Spiral(16203, 160);
/// The current age of the universe.
pub const OUR_GALAXYS_SUB_CATEGORY: GalaxySubCategory = GalaxySubCategory::BarredSpiral;
/// The current age of the universe.
pub const NO_SPECIAL_TRAIT: GalaxySpecialTrait = GalaxySpecialTrait::NoPeculiarity;
/// TODO List of all galaxies in our local group
pub const LOCAL_GROUP_GALAXIES: [GalaxyWithoutTraits; 38] = [
    GalaxyWithoutTraits {
        index: 0,
        name: OUR_GALAXYS_NAME,
        age: OUR_GALAXYS_AGE,
        is_dominant: false,
        is_major: true,
        category: OUR_GALAXYS_CATEGORY,
        sub_category: OUR_GALAXYS_SUB_CATEGORY,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 1,
        name: "Andromeda",
        age: -1.0,
        is_dominant: false,
        is_major: true,
        category: GalaxyCategory::Spiral(23280, 230),
        sub_category: OUR_GALAXYS_SUB_CATEGORY,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 2,
        name: "Triangulum",
        age: -1.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Spiral(9370, 94),
        sub_category: GalaxySubCategory::FlatSpiral,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 3,
        name: "Large Magellanic Cloud",
        age: 13.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(4490, 4490, 2245),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: GalaxySpecialTrait::Starburst,
        second_trait: GalaxySpecialTrait::GasRich,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 4,
        name: "Small Magellanic Cloud",
        age: 13.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(2280, 2280, 1140),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: GalaxySpecialTrait::Interacting,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    // Sagittarius Dwarf Elliptical (SagDEG) - closest to MW, being tidally disrupted
    GalaxyWithoutTraits { index: 5, name: "Sagittarius Dwarf Elliptical", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(3000), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Interacting, second_trait: GalaxySpecialTrait::Tail, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 6, name: "Ursa Minor Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(500), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 7, name: "Draco Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(500), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 8, name: "Carina Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(500), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dormant, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 9, name: "Sextans Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(1000), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 10, name: "Sculptor Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(500), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 11, name: "Fornax Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(700), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dormant, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 12, name: "Leo I", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(500), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: NO_SPECIAL_TRAIT, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 13, name: "Leo II", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(300), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 14, name: "NGC 6822", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(2330, 2330, 1165), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::Starburst, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 15, name: "NGC 185", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(1200), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dormant, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 16, name: "NGC 147", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(1100), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 17, name: "IC 10", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(1600, 1600, 800), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::Starburst, second_trait: GalaxySpecialTrait::GasRich, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 18, name: "IC 1613", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(2500, 2500, 1250), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::MetalPoor, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 19, name: "Phoenix Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(300, 300, 300), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::Dormant, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 20, name: "Cetus Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(300), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 21, name: "Tucana Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(300), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 22, name: "Pegasus Dwarf Irregular", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(1000, 1000, 500), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::GasRich, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 23, name: "Aquarius Dwarf", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(500, 500, 250), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::MetalPoor, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 24, name: "Wolf-Lundmark-Melotte", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(2600, 2600, 1300), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::MetalPoor, second_trait: GalaxySpecialTrait::GasRich, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 25, name: "Sagittarius Dwarf Irregular", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(500, 500, 250), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::GasRich, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 26, name: "Leo A", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(500, 500, 250), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::MetalPoor, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 27, name: "Pisces Dwarf", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(350, 350, 175), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: NO_SPECIAL_TRAIT, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 28, name: "Antlia Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(300), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 29, name: "NGC 3109", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(4600, 4600, 2300), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::GasRich, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 30, name: "Sextans A", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(1400, 1400, 700), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::MetalPoor, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 31, name: "Sextans B", age: 12.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(1100, 1100, 550), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::GasRich, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 32, name: "Leo T", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Irregular(80, 80, 80), sub_category: GalaxySubCategory::DwarfAmorphous, first_trait: GalaxySpecialTrait::GasRich, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 33, name: "Andromeda II", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(400), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 34, name: "Andromeda III", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(300), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 35, name: "Andromeda I", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(350), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 36, name: "Cassiopeia Dwarf", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(300), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dormant, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
    GalaxyWithoutTraits { index: 37, name: "Pegasus Dwarf Spheroidal", age: 13.0, is_dominant: false, is_major: false, category: GalaxyCategory::Elliptical(250), sub_category: GalaxySubCategory::DwarfElliptical, first_trait: GalaxySpecialTrait::Dead, second_trait: NO_SPECIAL_TRAIT, third_trait: NO_SPECIAL_TRAIT },
];

/// Data used to calculate a universe's age.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GalaxyWithoutTraits<'a> {
    /// The numeric identifier of this galaxy in its neighborhood.
    pub index: u16,
    /// The name of this galaxy.
    pub name: &'a str,
    /// The age of this galaxy in billions of years.
    pub age: f32,
    /// Is this galaxy a dominant one in its cluster?
    pub is_dominant: bool,
    /// Is this galaxy a major one in its neighborhood?
    pub is_major: bool,
    /// In what category this galaxy belongs to.
    pub category: GalaxyCategory,
    /// In what sub-category this galaxy belongs to.
    pub sub_category: GalaxySubCategory,
    /// The first peculiarity this galaxy has, if any.
    pub first_trait: GalaxySpecialTrait,
    /// The second peculiarity this galaxy has, if any.
    pub second_trait: GalaxySpecialTrait,
    /// The third peculiarity this galaxy has, if any.
    pub third_trait: GalaxySpecialTrait,
}
/// Pairs traits that are incompatible with one another.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub struct OppositeTraits(pub Vec<GalaxySpecialTrait>, pub Vec<GalaxySpecialTrait>);

/// Generates a list of pairs of incompatible traits.
pub fn get_opposite_traits() -> Vec<OppositeTraits> {
    vec![
        OppositeTraits(
            vec![GalaxySpecialTrait::Younger],
            vec![GalaxySpecialTrait::Older],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::Dusty],
            vec![GalaxySpecialTrait::MetalPoor],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::GasPoor],
            vec![GalaxySpecialTrait::GasRich],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::Compact(0)],
            vec![GalaxySpecialTrait::Expansive(0)],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::SubSize(0)],
            vec![GalaxySpecialTrait::SuperSize(0)],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::Starburst],
            vec![GalaxySpecialTrait::Dead, GalaxySpecialTrait::Dormant],
        ),
    ]
}
