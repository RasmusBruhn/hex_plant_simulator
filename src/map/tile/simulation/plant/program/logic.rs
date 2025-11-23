use super::{ApplyData, Arithmetic, NeighborDirection, Plant, TileData, TileNeighbors};

/// Plant action logic to calculate boolean operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Logic {
    /// Always false
    False,
    /// Always true
    True,
    /// Applies and operator between two logic operations
    And(usize, usize),
    /// Applies or operator between two logic operators
    Or(usize, usize),
    /// Applies xor operator between two logic operators
    Xor(usize, usize),
    /// Applies not operator on a logic operator
    Not(usize),
    /// Checks if two arithmetic operators are equal
    Equal(usize, usize),
    /// Checks if two arithmetic operators are qual when rounded to integers
    EqualRound(usize, usize),
    /// Checks if two arithmetic operators are not equal
    NotEqual(usize, usize),
    /// Checks if two arithmetic operators are not equal when rounded to integers
    NotEqualRound(usize, usize),
    /// Checks if one arithmetic operator is greater than another arithmetic operator
    Greater(usize, usize),
    /// Checks if one arithmetic operator is greater than another arithmetic operator when rounded to integers
    GreaterRound(usize, usize),
    /// Checks if one arithmetic operator is greater than or equal another arithmetic operator
    GreaterOrEqual(usize, usize),
    /// Checks if one arithmetic operator is greater than or equal another arithmetic operator when rounded to integers
    GreaterOrEqualRound(usize, usize),
    /// Checks if one arithmetic operator is less than another arithmetic operator
    Less(usize, usize),
    /// Checks if one arithmetic operator is less than another arithmetic operator when rounded to integers
    LessRound(usize, usize),
    /// Checks if one arithmetic operator is less than or equal another arithmetic operator
    LessOrEqual(usize, usize),
    /// Checks if one arithmetic operator is less than or equal another arithmetic operator when rounded to integers
    LessOrEqualRound(usize, usize),
    /// Checks if a arithmetic operator is positive
    IsPositive(usize),
    /// Checks if a arithmetic operator is positive when rounded to an integer
    IsPositiveRound(usize),
    /// Checks if a arithmetic operator is positive or zero
    IsNotNegative(usize),
    /// Checks if a arithmetic operator is positive or zero when rounded to an integer
    IsNotNegativeRound(usize),
    /// Checks if a arithmetic operator is zero
    IsZero(usize),
    /// Checks if a arithmetic operator is zero when rounded to an integer
    IsZeroRound(usize),
    /// Checks if a arithmetic operator is negative or zero
    IsNotPositive(usize),
    /// Checks if a arithmetic operator is negative or zero when rounded to an integer
    IsNotPositiveRound(usize),
    /// Checks if a arithmetic operator is negative
    IsNegative(usize),
    /// Checks if a arithmetic operator is negative when rounded to an integer
    IsNegativeRound(usize),
    /// True if the tile in the speicifed direction is available for spreading
    TileFree(NeighborDirection),
}

impl Logic {
    /// The number of different logic operators
    pub const COUNT: usize = 29;

    /// Gets a unique id for this specific logic operator type smaller than
    /// COUNT
    pub fn get_id(&self) -> usize {
        return match self {
            Self::False => 0,
            Self::True => 1,
            Self::And(_, _) => 2,
            Self::Or(_, _) => 3,
            Self::Xor(_, _) => 4,
            Self::Not(_) => 5,
            Self::Equal(_, _) => 6,
            Self::EqualRound(_, _) => 7,
            Self::NotEqual(_, _) => 8,
            Self::NotEqualRound(_, _) => 9,
            Self::Greater(_, _) => 10,
            Self::GreaterRound(_, _) => 11,
            Self::GreaterOrEqual(_, _) => 12,
            Self::GreaterOrEqualRound(_, _) => 13,
            Self::Less(_, _) => 14,
            Self::LessRound(_, _) => 15,
            Self::LessOrEqual(_, _) => 16,
            Self::LessOrEqualRound(_, _) => 17,
            Self::IsPositive(_) => 18,
            Self::IsPositiveRound(_) => 19,
            Self::IsNotNegative(_) => 20,
            Self::IsNotNegativeRound(_) => 21,
            Self::IsZero(_) => 22,
            Self::IsZeroRound(_) => 23,
            Self::IsNotPositive(_) => 24,
            Self::IsNotPositiveRound(_) => 25,
            Self::IsNegative(_) => 26,
            Self::IsNegativeRound(_) => 27,
            Self::TileFree(_) => 28,
        };
    }

    /// Gets the two indices used in the logic operator or if only one or zero
    /// is used then the second (and first) value is 0
    pub fn get_indices(&self) -> (usize, usize) {
        return match self {
            &Self::False => (0, 0),
            &Self::True => (0, 0),
            &Self::And(index1, index2) => (index1, index2),
            &Self::Or(index1, index2) => (index1, index2),
            &Self::Xor(index1, index2) => (index1, index2),
            &Self::Not(index) => (index, 0),
            &Self::Equal(index1, index2) => (index1, index2),
            &Self::EqualRound(index1, index2) => (index1, index2),
            &Self::NotEqual(index1, index2) => (index1, index2),
            &Self::NotEqualRound(index1, index2) => (index1, index2),
            &Self::Greater(index1, index2) => (index1, index2),
            &Self::GreaterRound(index1, index2) => (index1, index2),
            &Self::GreaterOrEqual(index1, index2) => (index1, index2),
            &Self::GreaterOrEqualRound(index1, index2) => (index1, index2),
            &Self::Less(index1, index2) => (index1, index2),
            &Self::LessRound(index1, index2) => (index1, index2),
            &Self::LessOrEqual(index1, index2) => (index1, index2),
            &Self::LessOrEqualRound(index1, index2) => (index1, index2),
            &Self::IsPositive(index) => (index, 0),
            &Self::IsPositiveRound(index) => (index, 0),
            &Self::IsNotNegative(index) => (index, 0),
            &Self::IsNotNegativeRound(index) => (index, 0),
            &Self::IsZero(index) => (index, 0),
            &Self::IsZeroRound(index) => (index, 0),
            &Self::IsNotPositive(index) => (index, 0),
            &Self::IsNotPositiveRound(index) => (index, 0),
            &Self::IsNegative(index) => (index, 0),
            &Self::IsNegativeRound(index) => (index, 0),
            &Self::TileFree(dir) => (
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
        };
    }

    /// Constructs a new logic operator from its unique type id and the two
    /// indices, if less than two indices are used then they are ignored
    ///
    /// # Parameters
    ///
    /// id: The unique id for the operator type
    ///
    /// indices: The two indices used to get the values to operate on
    pub fn from_id(id: usize, indices: (usize, usize)) -> Self {
        return match id {
            0 => Self::False,
            1 => Self::True,
            2 => Self::And(indices.0, indices.1),
            3 => Self::Or(indices.0, indices.1),
            4 => Self::Xor(indices.0, indices.1),
            5 => Self::Not(indices.0),
            6 => Self::Equal(indices.0, indices.1),
            7 => Self::EqualRound(indices.0, indices.1),
            8 => Self::NotEqual(indices.0, indices.1),
            9 => Self::NotEqualRound(indices.0, indices.1),
            10 => Self::Greater(indices.0, indices.1),
            11 => Self::GreaterRound(indices.0, indices.1),
            12 => Self::GreaterOrEqual(indices.0, indices.1),
            13 => Self::GreaterOrEqualRound(indices.0, indices.1),
            14 => Self::Less(indices.0, indices.1),
            15 => Self::LessRound(indices.0, indices.1),
            16 => Self::LessOrEqual(indices.0, indices.1),
            17 => Self::LessOrEqualRound(indices.0, indices.1),
            18 => Self::IsPositive(indices.0),
            19 => Self::IsPositiveRound(indices.0),
            20 => Self::IsNotNegative(indices.0),
            21 => Self::IsNotNegativeRound(indices.0),
            22 => Self::IsZero(indices.0),
            23 => Self::IsZeroRound(indices.0),
            24 => Self::IsNotPositive(indices.0),
            25 => Self::IsNotPositiveRound(indices.0),
            26 => Self::IsNegative(indices.0),
            27 => Self::IsNegativeRound(indices.0),
            28 => Self::TileFree(match indices.0 {
                0 => NeighborDirection::Right,
                1 => NeighborDirection::UpRight,
                2 => NeighborDirection::UpLeft,
                3 => NeighborDirection::Left,
                4 => NeighborDirection::DownLeft,
                _ => NeighborDirection::DownRight,
            }),
            _ => Self::False,
        };
    }

    /// Applies the logic operator
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
