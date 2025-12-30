use std::{mem::transmute, rc::Rc};

use super::{
    Bridge, BridgeSet, BridgeType, Bulk, Neighbor, NeighborDirection, Plant, Spread, State,
    TileData, TileNeighbors, TransferMode, bridge, bulk,
};
use crate::map;

mod arithmetic;
use arithmetic::Arithmetic;

mod logic;
use logic::Logic;

mod action;
use action::Action;

mod spread_bulk;
use spread_bulk::SpreadBulk;

mod spread_bridge;
use spread_bridge::SpreadBridge;

/// Specific program used for a single plant tile
#[derive(Clone, Debug)]
pub struct TileProgram {
    /// The general program information
    program: Rc<Program>,
    /// The maximum number of operations to run
    max_depth: usize,
    /// The id of the first action to apply
    start_id: usize,
}

impl TileProgram {
    /// Constructs a new tile program
    ///
    /// # Parameters
    ///
    /// program: The program to copy
    ///
    /// max_depth: The maximum depth to run the program for
    ///
    /// start_id: The id of the first action to start the program
    fn new(program: Rc<Program>, max_depth: usize, start_id: usize) -> Self {
        return Self {
            program,
            max_depth,
            start_id,
        };
    }

    /// Applies the program
    ///
    /// # Parameters
    ///
    /// plant: The original plant
    ///
    /// tile: The original tile of the plant
    ///
    /// neighbors: The original neighbors of the plant
    ///
    /// map_settings: The settings for the map
    ///
    /// new_plant: The new plant to replace the original one
    pub fn apply<'a>(
        &self,
        plant: &'a Plant,
        tile: &'a TileData,
        neighbors: &'a TileNeighbors<'a>,
        map_settings: &'a map::settings::Settings,
        new_plant: &'a mut Plant,
    ) {
        let mut apply_data = ApplyData {
            plant,
            tile,
            neighbors,
            map_settings,
            new_plant,
        };
        let mut remain_count = self.max_depth;
        self.program.apply_action(
            self.start_id % self.program.action.len(),
            &mut apply_data,
            &mut remain_count,
        );
    }
}

/// A general program used for an entire plant
#[derive(Clone, Debug)]
struct Program {
    /// All line of arithmetic code
    arithmetic: Vec<Arithmetic>,
    /// All lines of logic code
    logic: Vec<Logic>,
    /// All lones of action code
    action: Vec<Action>,
    /// All lines of bulk spread code
    spread_bulk: Vec<SpreadBulk>,
    /// All lines of bridge spread code
    spread_bridge: Vec<SpreadBridge>,
}

impl Program {
    /// Applies the arithmetic operator specified or 0.0 if out of bounds
    ///
    /// # Parameters
    ///
    /// index: The index of the operator to apply
    ///
    /// data: All data required for the operation
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    fn apply_arithmetic(&self, index: usize, data: &ApplyData, remain_count: &mut usize) -> f64 {
        return if index < self.arithmetic.len() {
            self.arithmetic[index].apply(data, remain_count)
        } else {
            0.0
        };
    }

    /// Applies the logic operator specified or false if out of bounds
    ///
    /// # Parameters
    ///
    /// index: The index of the operator to apply
    ///
    /// data: All data required for the operation
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    fn apply_logic(&self, index: usize, data: &ApplyData, remain_count: &mut usize) -> bool {
        return if index < self.logic.len() {
            self.logic[index].apply(data, remain_count)
        } else {
            false
        };
    }

    /// Applies the action operator specified
    ///
    /// # Parameters
    ///
    /// index: The index of the operator to apply
    ///
    /// data: All data required for the operation
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    fn apply_action(&self, index: usize, data: &mut ApplyData, remain_count: &mut usize) {
        if index < self.action.len() {
            self.action[index].apply(data, remain_count)
        };
    }

    /// Applies the bulk spread operator specified or None if out of bounds
    ///
    /// # Parameters
    ///
    /// index: The index of the operator to apply
    ///
    /// data: All data required for the operation
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    fn apply_spread_bulk(
        &self,
        index: usize,
        data: &mut ApplyData,
        remain_count: &mut usize,
    ) -> Option<Plant> {
        return if index < self.spread_bulk.len() {
            self.spread_bulk[index].apply(data, remain_count)
        } else {
            None
        };
    }

    /// Applies the bridge spread operator specified or None if out of bounds
    ///
    /// # Parameters
    ///
    /// index: The index of the operator to apply
    ///
    /// data: All data required for the operation
    ///
    /// remaining count: The remaining number of operators to evaluate before
    /// returning default values
    fn apply_spread_bridge(
        &self,
        index: usize,
        data: &mut ApplyData,
        remain_count: &mut usize,
    ) -> Option<Bridge> {
        return if index < self.spread_bridge.len() {
            self.spread_bridge[index].apply(data, remain_count)
        } else {
            None
        };
    }
}

/// All data required to apply an operator
#[derive(Debug)]
struct ApplyData<'a> {
    /// The plant this operator is applying for
    plant: &'a Plant,
    /// The data of the tile for this plant
    tile: &'a TileData,
    /// All neighbor tiles for this plant
    neighbors: &'a TileNeighbors<'a>,
    /// The settings for the map
    map_settings: &'a map::settings::Settings,
    /// The the current plant for the next simulation step
    new_plant: &'a mut Plant,
}

/// Converts a neighbor direction to a unique id
///
/// # Parameters
///
/// dir: The direction to convert
fn neighbor_dir_to_id(dir: &NeighborDirection) -> usize {
    return match dir {
        NeighborDirection::Right => 0,
        NeighborDirection::UpRight => 1,
        NeighborDirection::UpLeft => 2,
        NeighborDirection::Left => 3,
        NeighborDirection::DownLeft => 4,
        NeighborDirection::DownRight => 5,
    };
}

/// Converts a unique id to a neighbor direction
///
/// # Parameters
///
/// id: The id to convert
fn id_to_neighbor_dir(id: usize) -> NeighborDirection {
    match id % 6 {
        0 => NeighborDirection::Right,
        1 => NeighborDirection::UpRight,
        2 => NeighborDirection::UpLeft,
        3 => NeighborDirection::Left,
        4 => NeighborDirection::DownLeft,
        _ => NeighborDirection::DownRight,
    }
}

/// Converts a float value to a usize (bitwise identical)
///
/// # Parameters
///
/// value: The value to convert
fn float_to_id(value: f64) -> usize {
    return unsafe { transmute(value) };
}

/// Converts a usize value to a float (bitwise identical)
///
/// # Parameters
///
/// value: The value to convert
fn id_to_float(value: usize) -> f64 {
    return unsafe { transmute(value) };
}

/// Converts a transfer mode to a unique id
///
/// # Parameters
///
/// mode: The mode to convert
fn transfer_mode_to_id(mode: &TransferMode) -> usize {
    return match mode {
        TransferMode::Open => 0,
        TransferMode::In => 1,
        TransferMode::Out => 2,
        TransferMode::Closed => 3,
    };
}

/// Converts a unique id to a transfer mode
///
/// # Parameters
///
/// id: The id to convert
fn id_to_transfer_mode(id: usize) -> TransferMode {
    match id % 6 {
        0 => TransferMode::Open,
        1 => TransferMode::In,
        2 => TransferMode::Out,
        _ => TransferMode::Closed,
    }
}
