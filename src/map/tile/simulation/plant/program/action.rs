use super::{
    ApplyData, Bulk, NeighborDirection, Spread, bulk, id_to_neighbor_dir, neighbor_dir_to_id,
};

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
    /// The number of different action operators
    pub const COUNT: usize = 7;

    /// Gets a unique id for this specific action type smaller than COUNT
    pub fn get_id(&self) -> usize {
        return match self {
            Self::None => 0,
            Self::If(_, _) => 1,
            Self::IfElse(_, _, _) => 2,
            Self::Both(_, _) => 3,
            Self::Kill => 4,
            Self::Spread(_, _, _) => 5,
            Self::Grow => 6,
        };
    }

    /// Gets the three indices used in the action or if less are used then the
    /// value of the rest is 0
    pub fn get_indices(&self) -> (usize, usize, usize) {
        return match self {
            &Self::None => (0, 0, 0),
            &Self::If(index1, index2) => (index1, index2, 0),
            &Self::IfElse(index1, index2, index3) => (index1, index2, index3),
            &Self::Both(index1, index2) => (0, index1, index2),
            &Self::Kill => (0, 0, 0),
            &Self::Spread(index1, index2, dir) => (index1, index2, neighbor_dir_to_id(&dir)),
            &Self::Grow => (0, 0, 0),
        };
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
        return match id % Self::COUNT {
            0 => Self::None,
            1 => Self::If(indices.0, indices.1),
            2 => Self::IfElse(indices.0, indices.1, indices.2),
            3 => Self::Both(indices.1, indices.2),
            4 => Self::Kill,
            5 => Self::Spread(indices.0, indices.1, id_to_neighbor_dir(indices.0)),
            6 => Self::Grow,
            _ => Self::None,
        };
    }

    /// Applies the action operator
    ///
    /// # Parameters
    ///
    /// data: All data required for the apply operation
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    pub fn apply(&self, data: &mut ApplyData, remain_count: &mut usize) {
        if *remain_count == 0 {
            return;
        }
        *remain_count -= 1;

        match self {
            &Self::None => (),
            &Self::If(index1, index2) => {
                if data
                    .plant
                    .program
                    .program
                    .apply_logic(index1, data, remain_count)
                {
                    data.plant
                        .program
                        .program
                        .apply_action(index2, data, remain_count);
                }
            }
            &Self::IfElse(index1, index2, index3) => {
                if data
                    .plant
                    .program
                    .program
                    .apply_logic(index1, data, remain_count)
                {
                    data.plant
                        .program
                        .program
                        .apply_action(index2, data, remain_count);
                } else {
                    data.plant
                        .program
                        .program
                        .apply_action(index3, data, remain_count);
                }
            }
            &Self::Both(index1, index2) => {
                data.plant
                    .program
                    .program
                    .apply_action(index1, data, remain_count);

                data.plant
                    .program
                    .program
                    .apply_action(index2, data, remain_count);
            }
            &Self::Kill => data.new_plant.alive = false,
            &Self::Spread(index1, index2, dir) => match &data.new_plant.bulk {
                // only spread if the mother plant is allowed to spread and is not already spreading
                Bulk::Log(_) | Bulk::SugarBulb(_) => match &data.new_plant.spread {
                    Spread::Nothing => {
                        let mut bulk = match data.plant.program.program.apply_spread_bulk(
                            index1,
                            data,
                            remain_count,
                        ) {
                            Some(bulk) => bulk,
                            None => return,
                        };

                        let bridge = match data.plant.program.program.apply_spread_bridge(
                            index2,
                            data,
                            remain_count,
                        ) {
                            Some(bridge) => bridge,
                            None => return,
                        };

                        let energy = bulk.get_energy_cost_build(data.map_settings)
                            + bridge.get_energy_cost_build(data.map_settings);

                        // Only spread if the mother plant has enough energy
                        if energy <= data.new_plant.energy
                            && energy <= data.new_plant.energy_reserve
                        {
                            *bulk.bridges.get_mut(&dir.opposite()) = Some(bridge);
                            data.new_plant.energy -= energy;
                            data.new_plant.spread = Spread::Trying(Box::new((bulk, energy, dir)));
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            &Self::Grow => match &data.new_plant.bulk {
                Bulk::RipeSeed(_) => data.new_plant.bulk = Bulk::SugarBulb(bulk::SugarBulb::new()),
                _ => (),
            },
        };
    }
}
