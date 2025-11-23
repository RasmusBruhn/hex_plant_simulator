use super::{NeighborDirection, Plant, TileData, TileNeighbors};

mod arithmetic;
pub use arithmetic::Arithmetic;

mod logic;
pub use logic::Logic;

mod action;
pub use action::Action;

mod spread_bulk;
pub use spread_bulk::SpreadBulk;

mod spread_bridge;
pub use spread_bridge::SpreadBridge;

/// All data required to apply an operator
#[derive(Clone, Copy, Debug)]
pub struct ApplyData<'a> {
    /// The plant this operator is applying for
    pub plant: &'a Plant,
    /// The data of the til for this plant
    pub tile: &'a TileData,
    /// All neighbor tiles for this plant
    pub neighbors: &'a TileNeighbors<'a>,
    /// The energy of the plant in the new simulation step
    pub new_energy: f64,
}
