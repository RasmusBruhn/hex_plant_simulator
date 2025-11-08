use super::{NeighborDirection, Plant};

/// Describes the state of spreading a plant tile
#[derive(Clone, Debug)]
pub enum Spread {
    /// The plant is not attempting to spread
    Nothing,
    /// The plant has announced its intensions of spreading, holds the direction, the non-mutated offspring and the energy used
    Trying(Box<(Plant, f64, NeighborDirection)>),
    /// The plant it waiting to see if the spread was successful, holds the direction and the energy used
    Waiting(Box<(f64, NeighborDirection)>),
}
