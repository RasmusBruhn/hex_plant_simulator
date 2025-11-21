use std::iter::once;

use super::{Settings, NeighborDirection};

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
    /// Iterates through all the bridges
    pub fn iter(&self) -> impl Iterator<Item = &Bridge> {
        return once(&self.right)
            .chain(once(&self.up_right))
            .chain(once(&self.up_left))
            .chain(once(&self.left))
            .chain(once(&self.down_left))
            .chain(once(&self.down_right))
            .filter_map(|bridge| bridge.as_ref());
    }

    /// Returns the number of connected bridges
    pub fn count(&self) -> usize {
        return self.iter().count();
    }

    /// Gets a reference to the bridge in the given direction
    ///
    /// # Parameters
    ///
    /// direction: The direction of the bridge
    pub fn get(&self, direction: &NeighborDirection) -> &Option<Bridge> {
        return match direction {
            NeighborDirection::Right => &self.right,
            NeighborDirection::UpRight => &self.up_right,
            NeighborDirection::UpLeft => &self.up_left,
            NeighborDirection::Left => &self.left,
            NeighborDirection::DownLeft => &self.down_left,
            NeighborDirection::DownRight => &self.down_right,
        };
    }

    /// Gets a mutable reference to the bridge in the given direction
    ///
    /// # Parameters
    ///
    /// direction: The direction of the bridge
    pub fn get_mut(&mut self, direction: &NeighborDirection) -> &mut Option<Bridge> {
        return match direction {
            NeighborDirection::Right => &mut self.right,
            NeighborDirection::UpRight => &mut self.up_right,
            NeighborDirection::UpLeft => &mut self.up_left,
            NeighborDirection::Left => &mut self.left,
            NeighborDirection::DownLeft => &mut self.down_left,
            NeighborDirection::DownRight => &mut self.down_right,
        };
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
        return self.bridge.get_energy_cost_build_base(map_settings)
            + self
                .bridge
                .get_energy_cost_transfer_energy(map_settings, self.energy_capacity);
    }

    /// Gets the energy cost of running a bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_run(&self, map_settings: &Settings) -> f64 {
        return self.bridge.get_energy_cost_factor_run(map_settings)
            * self.get_energy_cost_build(map_settings);
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

impl BridgeType {
    /// Gets the energy build cost of energy transfer for a bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    ///
    /// capacity: The transfer capacity
    pub fn get_energy_cost_transfer_energy(&self, map_settings: &Settings, capacity: f64) -> f64 {
        return match self {
            Self::Log(data) => data.get_energy_cost_transfer_energy(map_settings, capacity),
            Self::Branch(data) => data.get_energy_cost_transfer_energy(map_settings, capacity),
        };
    }

    /// Gets the energy cost factor of running a bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_factor_run(&self, map_settings: &Settings) -> f64 {
        return match self {
            Self::Log(data) => data.get_energy_cost_factor_run(map_settings),
            Self::Branch(data) => data.get_energy_cost_factor_run(map_settings),
        };
    }

    /// Gets the base energy cost of building a new bridge, without any transfer
    /// costs
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_build_base(&self, map_settings: &Settings) -> f64 {
        return match self {
            Self::Log(data) => data.get_energy_cost_build_base(map_settings),
            Self::Branch(data) => data.get_energy_cost_build_base(map_settings),
        };
    }
}
