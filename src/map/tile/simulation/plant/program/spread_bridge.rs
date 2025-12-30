use super::{ApplyData, Bridge, BridgeType, bridge, id_to_transfer_mode};

/// Information for the bridge part of spreading
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpreadBridge {
    /// The bridge type
    pub plant: SpreadBridgeType,
    /// The index of the new energy transfer capacity
    pub energy_capacity: usize,
    /// The index of the new energy transfer mode
    pub energy_transfer: usize,
}

impl SpreadBridge {
    /// Constructs the new bridge
    ///
    /// # Parameters
    ///
    /// data: All data required for the construction
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    pub fn apply(&self, data: &mut ApplyData, remain_count: &mut usize) -> Option<Bridge> {
        if *remain_count == 0 {
            return None;
        }
        *remain_count -= 1;

        let plant = self.plant.apply(data, remain_count);
        let energy_capacity =
            data.plant
                .program
                .program
                .apply_arithmetic(self.energy_capacity, data, remain_count);
        let energy_transfer = data
            .plant
            .program
            .program
            .apply_arithmetic(self.energy_transfer, data, remain_count)
            .round() as usize;

        return Some(Bridge::new(
            plant,
            false,
            energy_capacity,
            id_to_transfer_mode(energy_transfer).get_opposite(),
        ));
    }
}

/// Information for the plant type for bridge part of spreading
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpreadBridgeType {
    /// Produces a new log
    Log,
    /// Produces a new branch
    Branch,
}

impl SpreadBridgeType {
    /// The number of different plant types
    pub const COUNT: usize = 2;

    /// Gets a unique id for this specific bridge type smaller than COUNT
    pub fn get_id(&self) -> usize {
        return match self {
            Self::Log => 0,
            Self::Branch => 1,
        };
    }

    /// Constructs a new spread from its unique type id and the index,
    /// if less indices are used then they are ignored
    ///
    /// # Parameters
    ///
    /// id: The unique id for the operator type
    ///
    /// indices: The three indices used to get the values to operate on
    pub fn from_id(id: usize) -> Self {
        return match id % Self::COUNT {
            0 => Self::Log,
            1 => Self::Branch,
            _ => Self::Log,
        };
    }

    /// Construct the new bridge type
    ///
    /// # Parameters
    ///
    /// data: All data required for the construction
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    pub fn apply(&self, _data: &ApplyData, _remain_count: &mut usize) -> BridgeType {
        return match self {
            &Self::Log => BridgeType::Log(bridge::Log::new()),
            &Self::Branch => BridgeType::Branch(bridge::Branch::new()),
        };
    }
}
