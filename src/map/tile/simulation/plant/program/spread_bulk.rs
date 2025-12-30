use super::{ApplyData, BridgeSet, Bulk, Plant, TileProgram, bulk};

/// Information for the bulk part of spreading
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpreadBulk {
    /// The plant type
    pub plant: SpreadBulkType,
    /// The index of the maximum depth of the new program
    pub program_depth: usize,
    /// The index of the first action to apply for the program
    pub program_start: usize,
    /// The index of the extra energy of the new plant
    pub energy: usize,
    /// The index of the new energy capacity
    pub energy_capacity: usize,
    /// The index of the new energy reserve
    pub energy_reserve: usize,
}

impl SpreadBulk {
    /// Constructs the new plant
    ///
    /// # Parameters
    ///
    /// data: All data required for the construction
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    pub fn apply(&self, data: &mut ApplyData, remain_count: &mut usize) -> Option<Plant> {
        if *remain_count == 0 {
            return None;
        }
        *remain_count -= 1;

        let plant = self.plant.apply(data, remain_count);
        let program_depth = data
            .plant
            .program
            .program
            .apply_arithmetic(self.program_depth, data, remain_count)
            .round() as usize;
        let program_start = data
            .plant
            .program
            .program
            .apply_arithmetic(self.program_start, data, remain_count)
            .round() as usize;
        let energy = data
            .plant
            .program
            .program
            .apply_arithmetic(self.energy, data, remain_count);
        let energy_capacity =
            data.plant
                .program
                .program
                .apply_arithmetic(self.energy_capacity, data, remain_count);
        let energy_reserve =
            data.plant
                .program
                .program
                .apply_arithmetic(self.energy_reserve, data, remain_count);

        return Some(Plant::new(
            plant,
            BridgeSet::new(),
            TileProgram::new(
                data.plant.program.program.clone(),
                program_depth,
                program_start,
            ),
            data.plant.cum_age,
            energy,
            energy_capacity,
            energy_reserve,
        ));
    }
}

/// Information for the plant type for bulk part of spreading
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpreadBulkType {
    /// Produces a new seed
    Seed,
    /// Produces a new sugar bulb
    SugarBulb,
    /// Produces a new log
    Log,
    /// Produces a new leaf with absorption given by the arithmetic at the given
    /// index
    Leaf(usize),
}

impl SpreadBulkType {
    /// The number of different plant types
    pub const COUNT: usize = 4;

    /// Gets a unique id for this specific plant type smaller than COUNT
    pub fn get_id(&self) -> usize {
        return match self {
            Self::Seed => 0,
            Self::SugarBulb => 1,
            Self::Log => 2,
            Self::Leaf(_) => 3,
        };
    }

    /// Gets the index used in the production or if less are used then the
    /// value of the rest is 0
    pub fn get_indices(&self) -> usize {
        return match self {
            &Self::Seed => 0,
            &Self::SugarBulb => 0,
            &Self::Log => 0,
            &Self::Leaf(index) => index,
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
    pub fn from_id(id: usize, indices: usize) -> Self {
        return match id % Self::COUNT {
            0 => Self::Seed,
            1 => Self::SugarBulb,
            2 => Self::Log,
            3 => Self::Leaf(indices),
            _ => Self::Seed,
        };
    }

    /// Constructs the new plant type
    ///
    /// # Parameters
    ///
    /// data: All data required for the construction
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    pub fn apply(&self, data: &mut ApplyData, remain_count: &mut usize) -> Bulk {
        return match self {
            &Self::Seed => Bulk::Seed(bulk::Seed::new()),
            &Self::SugarBulb => Bulk::SugarBulb(bulk::SugarBulb::new()),
            &Self::Log => Bulk::Log(bulk::Log::new()),
            &Self::Leaf(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                Bulk::Leaf(bulk::Leaf::new(value))
            }
        };
    }
}
