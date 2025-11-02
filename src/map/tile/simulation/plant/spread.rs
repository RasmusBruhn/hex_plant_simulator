use super::NeighborType;

/// Describes the state of spreading a plant tile
#[derive(Clone, Debug)]
pub enum Spread {
    /// The plant is not attempting to spread
    Nothing,
    /// The plant has announced its intensions of spreading
    Trying((NeighborType, f64)),
    /// The plant it waiting to see if the spread was successful
    Waiting((NeighborType, f64)),
}