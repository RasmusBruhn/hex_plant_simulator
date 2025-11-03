use super::{NeighborType, Plant};

/// Describes the state of spreading a plant tile
#[derive(Clone, Debug)]
pub enum Spread {
    /// The plant is not attempting to spread
    Nothing,
    /// The plant has announced its intensions of spreading, holds the direction, the non-mutated offspring and the energy used
    Trying(Box<(NeighborType, Plant, f64)>),
    /// The plant it waiting to see if the spread was successful, holds the direction, the non-mutated offspring and the energy used
    Waiting(Box<(NeighborType, Plant, f64)>),
}
