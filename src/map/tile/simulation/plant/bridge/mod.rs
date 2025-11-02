mod log;
pub use log::Log;

mod branch;
pub use branch::Branch;

/// All bridges for a single plant tile
#[derive(Clone, Debug)]
pub struct BridgeSet {
    /// The bridge connecting to the tile to the right
    pub right: Option<Bridge>,
    /// The bridge connecting to the tile to the up-right
    pub up_right: Option<Bridge>,
    /// The bridge connecting to the tile to the up-left
    pub up_left: Option<Bridge>,
    /// The bridge connecting to the tile to the left
    pub left: Option<Bridge>,
    /// The bridge connecting to the tile to the down-left
    pub down_left: Option<Bridge>,
    /// The bridge connecting to the tile to the down-right
    pub down_right: Option<Bridge>,
}

impl BridgeSet {
    /// Returns the number of connected bridges
    pub fn count(&self) -> usize {
        return self.right.is_some() as usize
            + self.up_right.is_some() as usize
            + self.up_left.is_some() as usize
            + self.left.is_some() as usize
            + self.down_left.is_some() as usize
            + self.down_right.is_some() as usize;
    }
}

/// A bridge connecting two plant tiles
#[derive(Clone, Debug)]
pub struct Bridge {
    /// The type of bridge
    pub bridge: BridgeType,
    /// The maximum amount of energy able to transfer per step
    pub energy_capacity: f64,
}

/// The type of bridge
#[derive(Clone, Debug)]
pub enum BridgeType {
    /// Able to transfer large amounts of energy but expensive
    Log(Log),
    /// Able to only transfer small amounts of energy but cheap
    Branch(Branch),
}
