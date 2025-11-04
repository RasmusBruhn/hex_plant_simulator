use super::Settings;

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
    /// If true then this plant is the mother plant
    pub exiting: bool,
    /// The maximum amount of energy able to transfer per step
    pub energy_capacity: f64,
    /// The transfer mode for energy
    pub energy_transfer: TransferMode,
}

impl Bridge {
    /// Gets the other end of the bridge
    pub fn get_opposite(&self) -> Self {
        return Self {
            bridge: self.bridge.clone(),
            exiting: !self.exiting,
            energy_capacity: self.energy_capacity,
            energy_transfer: self.energy_transfer.get_opposite(),
        };
    }

    /// Gets the energy cost of building a new bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_build(&self, map_settings: &Settings) -> f64 {
        let (base, transfer_energy) = match &self.bridge {
            BridgeType::Log(data) => (
                data.get_energy_cost_build(map_settings),
                data.get_energy_cost_transfer_energy(map_settings),
            ),
            BridgeType::Branch(data) => (
                data.get_energy_cost_build(map_settings),
                data.get_energy_cost_transfer_energy(map_settings),
            ),
        };

        return base + transfer_energy * self.energy_capacity;
    }

    /// Gets the energy cost of running a bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_run(&self, map_settings: &Settings) -> f64 {
        let factor = match &self.bridge {
            BridgeType::Log(data) => data.get_energy_cost_run(map_settings),
            BridgeType::Branch(data) => data.get_energy_cost_run(map_settings),
        };

        return factor * self.get_energy_cost_build(map_settings);
    }
}

/// Which direction transfer is allowed
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransferMode {
    /// Allows tranfer out of the plant
    Out,
    /// Allows transfer into the plant
    In,
    /// Allows all transfer directions
    Open,
    /// Disallows all transfer directions
    Closed,
}

impl TransferMode {
    /// Gets the transfer mode for the other end of the bridge
    pub fn get_opposite(&self) -> Self {
        return match self {
            Self::Out => Self::In,
            Self::In => Self::Out,
            Self::Open => Self::Open,
            Self::Closed => Self::Closed,
        };
    }

    /// True if this plant can receive through this bridge
    pub fn can_receive(&self) -> bool {
        return match self {
            Self::In | Self::Open => true,
            Self::Out | Self::Closed => false,
        };
    }

    /// True if this plant can transmit through this bridge
    pub fn can_transmit(&self) -> bool {
        return match self {
            Self::Out | Self::Open => true,
            Self::In | Self::Closed => false,
        };
    }
}

/// The type of bridge
#[derive(Clone, Debug)]
pub enum BridgeType {
    /// Able to transfer large amounts of energy but expensive
    Log(Log),
    /// Able to only transfer small amounts of energy but cheap
    Branch(Branch),
}
