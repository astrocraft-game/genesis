use crate::internal::*;
use crate::prelude::*;

pub mod generator;
pub mod types;

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct StellarNeighborhood {
    pub age: StellarNeighborhoodAge,
}

impl StellarNeighborhood {
    pub fn new(age: StellarNeighborhoodAge) -> Self {
        Self { age }
    }
}

impl Display for StellarNeighborhood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} stellar neighborhood", self.age)
    }
}
