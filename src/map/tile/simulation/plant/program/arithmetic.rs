use super::{
    ApplyData, BridgeType, Bulk, Neighbor, NeighborDirection, State, float_to_id, id_to_float,
    id_to_neighbor_dir, neighbor_dir_to_id,
};

/// Plant action logic to calculate float operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Arithmetic {
    /// Always has the value 0.0
    Zero,
    /// Always has the value 1.0
    One,
    /// Always a constant value
    Constant(f64),
    /// Doubles the value
    Double(usize),
    /// Halfs the value
    Half(usize),
    /// Increments the value by 1
    Increment(usize),
    /// Decrements the value by 1
    Decrement(usize),
    /// Adds to values
    Add(usize, usize),
    /// Subtracts two values
    Sub(usize, usize),
    /// Multiplies two values
    Mul(usize, usize),
    /// Divides two values
    Div(usize, usize),
    /// Applies modulus operator between two values
    Mod(usize, usize),
    /// Negates a value
    Neg(usize),
    /// Finds the minimum of two values
    Min(usize, usize),
    /// Finds the minimum of a value and zero
    MinZero(usize),
    /// Finds the minimum of a value and one
    MinOne(usize),
    /// Finds the maximum of two values
    Max(usize, usize),
    /// Finds the maximum of a value and zero
    MaxZero(usize),
    /// Finds the maximum of a value and one
    MaxOne(usize),
    /// Calculates the mean of two values
    Mean(usize, usize),
    /// Gets the light value of this tile
    TileLight,
    /// Gets the gradient of the light in the specified direction
    TileLightGradient(NeighborDirection),
    /// Gets the transparency of this til
    TileTransparency,
    /// Gets the gradient of the transparency in the specified direction
    TileTransparencyGradient(NeighborDirection),
    /// A unique id for the type of the the the neighbor tile
    TilePlantType(NeighborDirection),
    /// A unique id for the bridge in the direction
    BridgeType(NeighborDirection),
    /// A unique id for the type of plant in this tile
    PlantType,
    /// Gets the age of the plant tile
    PlantAge,
    /// Gets the cumulative age of the plant
    PlantCumAge,
    /// Gets the energy capacity of the plant tile
    PlantEnergyCapacity,
    /// Gets the energy reserve of the plant tile
    PlantEnergyReserve,
    /// Gets the energy stored in the plant tile
    PlantEnergy,
    /// Gets the change in energy of the plant tile since last simulation step
    PlantEnergyChange,
    /// Gets the energy of the plant tile reserved for itself
    PlantEnergySelf,
    /// Gets the change in the self energy of the plant tile since last simulation step
    PlantEnergySelfChange,
    /// Gets the energy allocated for sharing with each neighbor ((energy - energy_self) / 6.0)
    PlantEnergyShare,
    /// Gets the change in shared energy since the last simulation step
    PlantEnergyShareChange,
}

impl Arithmetic {
    /// The number of different arithmetic operators
    pub const COUNT: usize = 37;

    /// Gets a unique id for this specific arithmetic operator type smaller than
    /// COUNT
    pub fn get_id(&self) -> usize {
        return match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Constant(_) => 2,
            Self::Double(_) => 3,
            Self::Half(_) => 4,
            Self::Increment(_) => 5,
            Self::Decrement(_) => 6,
            Self::Add(_, _) => 7,
            Self::Sub(_, _) => 8,
            Self::Mul(_, _) => 9,
            Self::Div(_, _) => 10,
            Self::Mod(_, _) => 11,
            Self::Neg(_) => 12,
            Self::Min(_, _) => 13,
            Self::MinZero(_) => 14,
            Self::MinOne(_) => 15,
            Self::Max(_, _) => 16,
            Self::MaxZero(_) => 17,
            Self::MaxOne(_) => 18,
            Self::Mean(_, _) => 19,
            Self::TileLight => 20,
            Self::TileLightGradient(_) => 21,
            Self::TileTransparency => 22,
            Self::TileTransparencyGradient(_) => 23,
            Self::TilePlantType(_) => 24,
            Self::BridgeType(_) => 25,
            Self::PlantType => 26,
            Self::PlantAge => 27,
            Self::PlantCumAge => 28,
            Self::PlantEnergyCapacity => 29,
            Self::PlantEnergyReserve => 30,
            Self::PlantEnergy => 31,
            Self::PlantEnergyChange => 32,
            Self::PlantEnergySelf => 33,
            Self::PlantEnergySelfChange => 34,
            Self::PlantEnergyShare => 35,
            Self::PlantEnergyShareChange => 36,
        };
    }

    /// Gets the two indices used in the arithmetic operator or if only one or
    /// zero is used then the second (and first) value is 0
    pub fn get_indices(&self) -> (usize, usize) {
        return match self {
            &Self::Zero => (0, 0),
            &Self::One => (0, 0),
            &Self::Constant(value) => (float_to_id(value), 0),
            &Self::Double(index) => (index, 0),
            &Self::Half(index) => (index, 0),
            &Self::Increment(index) => (index, 0),
            &Self::Decrement(index) => (index, 0),
            &Self::Add(index1, index2) => (index1, index2),
            &Self::Sub(index1, index2) => (index1, index2),
            &Self::Mul(index1, index2) => (index1, index2),
            &Self::Div(index1, index2) => (index1, index2),
            &Self::Mod(index1, index2) => (index1, index2),
            &Self::Neg(index) => (index, 0),
            &Self::Min(index1, index2) => (index1, index2),
            &Self::MinZero(index) => (index, 0),
            &Self::MinOne(index) => (index, 0),
            &Self::Max(index1, index2) => (index1, index2),
            &Self::MaxZero(index) => (index, 0),
            &Self::MaxOne(index) => (index, 0),
            &Self::Mean(index1, index2) => (index1, index2),
            &Self::TileLight => (0, 0),
            &Self::TileLightGradient(dir) => (neighbor_dir_to_id(&dir), 0),
            &Self::TileTransparency => (0, 0),
            &Self::TileTransparencyGradient(dir) => (neighbor_dir_to_id(&dir), 0),
            &Self::TilePlantType(dir) => (neighbor_dir_to_id(&dir), 0),
            &Self::BridgeType(dir) => (neighbor_dir_to_id(&dir), 0),
            &Self::PlantType => (0, 0),
            &Self::PlantAge => (0, 0),
            &Self::PlantCumAge => (0, 0),
            &Self::PlantEnergyCapacity => (0, 0),
            &Self::PlantEnergyReserve => (0, 0),
            &Self::PlantEnergy => (0, 0),
            &Self::PlantEnergyChange => (0, 0),
            &Self::PlantEnergySelf => (0, 0),
            &Self::PlantEnergySelfChange => (0, 0),
            &Self::PlantEnergyShare => (0, 0),
            &Self::PlantEnergyShareChange => (0, 0),
        };
    }

    /// Constructs a new arithmetic operator from its unique type id and the two
    /// indices, if less than two indices are used then they are ignored
    ///
    /// # Parameters
    ///
    /// id: The unique id for the operator type
    ///
    /// indices: The two indices used to get the values to operate on
    pub fn from_id(id: usize, indices: (usize, usize)) -> Self {
        return match id % Self::COUNT {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Constant(id_to_float(indices.0)),
            3 => Self::Double(indices.0),
            4 => Self::Half(indices.0),
            5 => Self::Increment(indices.0),
            6 => Self::Decrement(indices.0),
            7 => Self::Add(indices.0, indices.1),
            8 => Self::Sub(indices.0, indices.1),
            9 => Self::Mul(indices.0, indices.1),
            10 => Self::Div(indices.0, indices.1),
            11 => Self::Mod(indices.0, indices.1),
            12 => Self::Neg(indices.0),
            13 => Self::Min(indices.0, indices.1),
            14 => Self::MinZero(indices.0),
            15 => Self::MinOne(indices.0),
            16 => Self::Max(indices.0, indices.1),
            17 => Self::MaxZero(indices.0),
            18 => Self::MaxOne(indices.0),
            19 => Self::Mean(indices.0, indices.1),
            20 => Self::TileLight,
            21 => Self::TileLightGradient(id_to_neighbor_dir(indices.0)),
            22 => Self::TileTransparency,
            23 => Self::TileTransparencyGradient(id_to_neighbor_dir(indices.0)),
            24 => Self::TilePlantType(id_to_neighbor_dir(indices.0)),
            25 => Self::BridgeType(id_to_neighbor_dir(indices.0)),
            26 => Self::PlantType,
            27 => Self::PlantAge,
            28 => Self::PlantCumAge,
            29 => Self::PlantEnergyCapacity,
            30 => Self::PlantEnergyReserve,
            31 => Self::PlantEnergy,
            32 => Self::PlantEnergyChange,
            33 => Self::PlantEnergySelf,
            34 => Self::PlantEnergySelfChange,
            35 => Self::PlantEnergyShare,
            36 => Self::PlantEnergyShareChange,
            _ => Self::Zero,
        };
    }

    /// Applies the arithmetic operator
    ///
    /// # Parameters
    ///
    /// data: All data required for the apply operation
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    pub fn apply(&self, data: &ApplyData, remain_count: &mut usize) -> f64 {
        if *remain_count == 0 {
            return 0.0;
        }
        *remain_count -= 1;

        return match self {
            &Self::Zero => 0.0,
            &Self::One => 1.0,
            &Self::Constant(value) => value,
            &Self::Double(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                value * 2.0
            }
            &Self::Half(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                value * 0.5
            }
            &Self::Increment(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                value + 1.0
            }
            &Self::Decrement(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                value - 1.0
            }
            &Self::Add(index1, index2) => {
                let value1 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index1, data, remain_count);
                let value2 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index2, data, remain_count);

                value1 + value2
            }
            &Self::Sub(index1, index2) => {
                let value1 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index1, data, remain_count);
                let value2 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index2, data, remain_count);

                value1 - value2
            }
            &Self::Mul(index1, index2) => {
                let value1 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index1, data, remain_count);
                let value2 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index2, data, remain_count);

                value1 * value2
            }
            &Self::Div(index1, index2) => {
                let value1 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index1, data, remain_count);
                let value2 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index2, data, remain_count);

                if value2 == 0.0 { 0.0 } else { value1 / value2 }
            }
            &Self::Mod(index1, index2) => {
                let value1 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index1, data, remain_count);
                let value2 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index2, data, remain_count);

                if value2 == 0.0 { 0.0 } else { value1 % value2 }
            }
            &Self::Neg(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                -value
            }
            &Self::Min(index1, index2) => {
                let value1 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index1, data, remain_count);
                let value2 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index2, data, remain_count);

                value1.min(value2)
            }
            &Self::MinZero(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                value.min(0.0)
            }
            &Self::MinOne(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                value.min(1.0)
            }
            &Self::Max(index1, index2) => {
                let value1 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index1, data, remain_count);
                let value2 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index2, data, remain_count);

                value1.max(value2)
            }
            &Self::MaxZero(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                value.max(0.0)
            }
            &Self::MaxOne(index) => {
                let value = data
                    .plant
                    .program
                    .program
                    .apply_arithmetic(index, data, remain_count);

                value.max(1.0)
            }
            &Self::Mean(index1, index2) => {
                let value1 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index1, data, remain_count);
                let value2 =
                    data.plant
                        .program
                        .program
                        .apply_arithmetic(index2, data, remain_count);

                (value1 + value2) * 0.5
            }
            &Self::TileLight => data.tile.light,
            &Self::TileLightGradient(dir) => {
                let neighbor = match data.neighbors.get(&dir) {
                    Neighbor::Empty => 0.0,
                    Neighbor::Tile(tile) => tile.data.light,
                    Neighbor::SunTile(tile) => tile.intensity,
                };

                neighbor - data.tile.light
            }
            &Self::TileTransparency => data.tile.transparency,
            &Self::TileTransparencyGradient(dir) => {
                let neighbor = match data.neighbors.get(&dir) {
                    Neighbor::Empty => 0.0,
                    Neighbor::Tile(tile) => tile.data.transparency,
                    Neighbor::SunTile(_) => 1.0,
                };

                neighbor - data.tile.transparency
            }
            &Self::TilePlantType(dir) => match data.neighbors.get(&dir) {
                Neighbor::Tile(tile) => match &tile.plant {
                    State::Occupied(plant) => match &plant.bulk {
                        Bulk::Seed(_) => 1.0,
                        Bulk::RipeSeed(_) => 2.0,
                        Bulk::SugarBulb(_) => 3.0,
                        Bulk::Log(_) => 4.0,
                        Bulk::Leaf(_) => 5.0,
                    },
                    _ => 0.0,
                },
                _ => 0.0,
            },
            &Self::BridgeType(dir) => match data.plant.bridges.get(&dir) {
                Some(bridge) => match &bridge.bridge {
                    BridgeType::Log(_) => 1.0,
                    BridgeType::Branch(_) => 2.0,
                },
                None => 0.0,
            },
            &Self::PlantType => match &data.plant.bulk {
                Bulk::Seed(_) => 1.0,
                Bulk::RipeSeed(_) => 2.0,
                Bulk::SugarBulb(_) => 3.0,
                Bulk::Log(_) => 4.0,
                Bulk::Leaf(_) => 5.0,
            },
            &Self::PlantAge => data.plant.age as f64,
            &Self::PlantCumAge => data.plant.cum_age as f64,
            &Self::PlantEnergyCapacity => data.plant.energy_capacity,
            &Self::PlantEnergyReserve => data.plant.energy_reserve,
            &Self::PlantEnergy => data.plant.energy,
            &Self::PlantEnergyChange => data.new_plant.energy - data.plant.energy,
            &Self::PlantEnergySelf => data.plant.energy.min(data.plant.energy_reserve),
            &Self::PlantEnergySelfChange => {
                data.new_plant.energy.min(data.plant.energy_reserve)
                    - data.plant.energy.min(data.plant.energy_reserve)
            }
            &Self::PlantEnergyShare => {
                (data.plant.energy - data.plant.energy_reserve).max(0.0) / 6.0
            }
            &Self::PlantEnergyShareChange => {
                (data.new_plant.energy - data.plant.energy_reserve).max(0.0) / 6.0
                    - (data.plant.energy - data.plant.energy_reserve).max(0.0) / 6.0
            }
        };
    }
}
