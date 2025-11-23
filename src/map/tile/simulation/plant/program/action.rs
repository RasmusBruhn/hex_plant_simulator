use super::{ApplyData, Arithmetic, NeighborDirection, Plant, TileData, TileNeighbors};

/// Plant action logic to handle spreading and internal production management
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// Do nothing
    None,
    /// If .0 is true then runs action .1
    If(usize, usize),
    /// If .0 is true then runs action .1 otherwise runs action .2
    IfElse(usize, usize, usize),
    /// Runs action .0 and then action .1
    Both(usize, usize),
    /// Kills the plant
    Kill,
    /// Attempts to spread the plant defined by bulk of .0 and bridge of .1 to
    /// the tile in the direction of .2
    Spread(usize, usize, NeighborDirection),
    /// Only applicable if plant type is a grounded RipeSeed, starts the initial
    /// growing process
    Grow,
}

impl Action {
    /// The number of different logic operators
    pub const COUNT: usize = 1;

    /// Gets a unique id for this specific action type smaller than COUNT
    pub fn get_id(&self) -> usize {
        todo!()
    }

    /// Gets the three indices used in the action or if less are used then the
    /// value of the rest is 0
    pub fn get_indices(&self) -> (usize, usize) {
        todo!()
    }

    /// Constructs a new action from its unique type id and the three indices,
    /// if less than three indices are used then they are ignored
    ///
    /// # Parameters
    ///
    /// id: The unique id for the operator type
    ///
    /// indices: The three indices used to get the values to operate on
    pub fn from_id(id: usize, indices: (usize, usize, usize)) -> Self {
        todo!()
    }

    /// Applies the action operator
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
