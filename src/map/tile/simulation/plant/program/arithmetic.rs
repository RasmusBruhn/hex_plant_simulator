use super::{ApplyData, NeighborDirection, Plant, TileData, TileNeighbors};

/// Plant action logic to calculate float operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Arithmetic {
    /// Always has the value 0.0
    Zero,
    /// Always has the value 1.0
    One,
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
    pub const COUNT: usize = 33;

    /// Gets a unique id for this specific arithmetic operator type smaller than
    /// COUNT
    pub fn get_id(&self) -> usize {
        return match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Double(_) => 2,
            Self::Half(_) => 3,
            Self::Increment(_) => 4,
            Self::Decrement(_) => 5,
            Self::Add(_, _) => 6,
            Self::Sub(_, _) => 7,
            Self::Mul(_, _) => 8,
            Self::Div(_, _) => 9,
            Self::Mod(_, _) => 10,
            Self::Neg(_) => 11,
            Self::Min(_, _) => 12,
            Self::MinZero(_) => 13,
            Self::MinOne(_) => 14,
            Self::Max(_, _) => 15,
            Self::MaxZero(_) => 16,
            Self::MaxOne(_) => 17,
            Self::Mean(_, _) => 18,
            Self::TileLight => 19,
            Self::TileLightGradient(_) => 20,
            Self::TileTransparency => 21,
            Self::TileTransparencyGradient(_) => 22,
            Self::PlantAge => 23,
            Self::PlantCumAge => 24,
            Self::PlantEnergyCapacity => 25,
            Self::PlantEnergyReserve => 26,
            Self::PlantEnergy => 27,
            Self::PlantEnergyChange => 28,
            Self::PlantEnergySelf => 29,
            Self::PlantEnergySelfChange => 30,
            Self::PlantEnergyShare => 31,
            Self::PlantEnergyShareChange => 32,
        };
    }

    /// Gets the two indices used in the arithmetic operator or if only one or
    /// zero is used then the second (and first) value is 0
    pub fn get_indices(&self) -> (usize, usize) {
        return match self {
            &Self::Zero => (0, 0),
            &Self::One => (0, 0),
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
            &Self::TileLightGradient(dir) => (
                match dir {
                    NeighborDirection::Right => 0,
                    NeighborDirection::UpRight => 1,
                    NeighborDirection::UpLeft => 2,
                    NeighborDirection::Left => 3,
                    NeighborDirection::DownLeft => 4,
                    NeighborDirection::DownRight => 5,
                },
                0,
            ),
            &Self::TileTransparency => (0, 0),
            &Self::TileTransparencyGradient(dir) => (
                match dir {
                    NeighborDirection::Right => 0,
                    NeighborDirection::UpRight => 1,
                    NeighborDirection::UpLeft => 2,
                    NeighborDirection::Left => 3,
                    NeighborDirection::DownLeft => 4,
                    NeighborDirection::DownRight => 5,
                },
                0,
            ),
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
        return match id {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Double(indices.0),
            3 => Self::Half(indices.0),
            4 => Self::Increment(indices.0),
            5 => Self::Decrement(indices.0),
            6 => Self::Add(indices.0, indices.1),
            7 => Self::Sub(indices.0, indices.1),
            8 => Self::Mul(indices.0, indices.1),
            9 => Self::Div(indices.0, indices.1),
            10 => Self::Mod(indices.0, indices.1),
            11 => Self::Neg(indices.0),
            12 => Self::Min(indices.0, indices.1),
            13 => Self::MinZero(indices.0),
            14 => Self::MinOne(indices.0),
            15 => Self::Max(indices.0, indices.1),
            16 => Self::MaxZero(indices.0),
            17 => Self::MaxOne(indices.0),
            18 => Self::Mean(indices.0, indices.1),
            19 => Self::TileLight,
            20 => Self::TileLightGradient(match indices.0 {
                0 => NeighborDirection::Right,
                1 => NeighborDirection::UpRight,
                2 => NeighborDirection::UpLeft,
                3 => NeighborDirection::Left,
                4 => NeighborDirection::DownLeft,
                _ => NeighborDirection::DownRight,
            }),
            21 => Self::TileTransparency,
            22 => Self::TileTransparencyGradient(match indices.0 {
                0 => NeighborDirection::Right,
                1 => NeighborDirection::UpRight,
                2 => NeighborDirection::UpLeft,
                3 => NeighborDirection::Left,
                4 => NeighborDirection::DownLeft,
                _ => NeighborDirection::DownRight,
            }),
            23 => Self::PlantAge,
            24 => Self::PlantCumAge,
            25 => Self::PlantEnergyCapacity,
            26 => Self::PlantEnergyReserve,
            27 => Self::PlantEnergy,
            28 => Self::PlantEnergyChange,
            29 => Self::PlantEnergySelf,
            30 => Self::PlantEnergySelfChange,
            31 => Self::PlantEnergyShare,
            32 => Self::PlantEnergyShareChange,
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
    pub fn apply(&self, data: &ApplyData, remain_count: &mut usize) -> bool {
        todo!()
    }
}
