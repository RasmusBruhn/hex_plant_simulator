// Log: #52361e
// Leaf: #1b6623
// SugarBulb: #93b5ae
// Seed: #f2bb07
// RipeSeed: #b30c1a

// Log: #52361e
// Branch: #78583c
use super::{Settings, TileNeighbors};

mod state;
pub use state::State;

mod spread;
use spread::Spread;

mod neighbor;
use neighbor::NeighborType;

mod bridge;
use bridge::BridgeSet;

mod bulk;
use bulk::Bulk;

/// A single plant tile
#[derive(Clone, Debug)]
pub struct Plant {
    /// The bulk of the plant
    bulk: Bulk,
    /// All bridges connecting to this tile
    bridges: BridgeSet,
    /// If the plant is currently alive
    alive: bool,
    /// The energy in this plant tile
    energy: f64,
    /// The maximum amount of energy allowed
    energy_capacity: f64,
    /// If there is less than this amount of energy then no energy may leave
    /// this tile, if there are more energy then each neighbor can take up to
    /// 1/N of any extra energy where N is the number of bridges connected
    energy_reserve: f64,
    /// Set if it attempts to spread to a neighboring tile, the tile it will
    /// spread to and the energy allocated for creating the new plant
    spread: Spread,
}

impl Plant {
    /// Gets the transparency of this plant
    /// 
    /// # Parameters
    /// 
    /// map_settings: The settings for this map
    fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return self.bulk.get_transparency(map_settings);
    }

    /// Forwards the state of this plant to the next simulation step
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbors: References to all the neighbors of this tile
    fn forward(&self, map_settings: &Settings, neighbors: &TileNeighbors) -> Option<Self> {
        todo!()
    }
}
