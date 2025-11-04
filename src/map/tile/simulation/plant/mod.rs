// Log: #52361e
// Leaf: #1b6623
// SugarBulb: #93b5ae
// Seed: #f2bb07
// RipeSeed: #b30c1a

// Log: #52361e
// Branch: #78583c
use super::{Neighbor, Settings, Tile, TileNeighbors};

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

    /// Gets the energy cost of building the bulk of this plant
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    fn get_energy_cost_build_bulk(&self, map_settings: &Settings) -> f64 {
        return self.bulk.get_energy_cost_build(map_settings)
            + self
                .bulk
                .get_energy_cost_storage_energy(map_settings, self.energy_capacity);
    }

    /// Gets the energy cost of building this plant
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    fn get_energy_cost_build(&self, map_settings: &Settings) -> f64 {
        return self.get_energy_cost_build_bulk(map_settings)
            + self
                .bridges
                .iter()
                .map(|bridge| 0.5 * bridge.get_energy_cost_build(map_settings))
                .sum::<f64>();
    }

    /// Gets the energy cost of running this plant
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    fn get_energy_cost_run(&self, map_settings: &Settings) -> f64 {
        return self.get_energy_cost_build_bulk(map_settings)
            * self.bulk.get_energy_cost_run(map_settings)
            + self
                .bridges
                .iter()
                .map(|bridge| 0.5 * bridge.get_energy_cost_run(map_settings))
                .sum::<f64>();
    }

    /// Forwards the state of this plant to the next simulation step
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbors: References to all the neighbors of this tile
    fn forward(&self, map_settings: &Settings, neighbors: &TileNeighbors) -> Option<Self> {
        // Kill it if it was assigned to die
        if !self.alive {
            return None;
        }

        // Remove unused bridges

        // Handle ongoing spreading
        let (spread, energy) = match self.spread {
            Spread::Nothing => (Spread::Nothing, 0.0),
            Spread::Trying(value) => (Spread::Waiting(value.clone()), 0.0),
            Spread::Waiting(value) => (
                Spread::Nothing,
                self.spread_resolve(bridges, &value.0, value.2, neighbors),
            ),
        };

        // Gain and spend energy

        todo!()
    }

    /// Resolves a spread action after waiting, returning the new bridges and
    /// the energy of this plant
    ///
    /// # Parameters
    ///
    /// bridges: The bridges for the plant after removing dead connections
    ///
    /// spread_direction: The direction to spread in
    ///
    /// spread_energy: The energy used to spread
    ///
    /// neighbors: All neighbors of this tile
    fn spread_resolve(
        &self,
        mut bridges: BridgeSet,
        spread_direction: &NeighborType,
        spread_energy: f64,
        neighbors: &TileNeighbors,
    ) -> (BridgeSet, f64) {
        let neighbor = match spread_direction {
            NeighborType::Right => &neighbors.right,
            NeighborType::UpRight => &neighbors.up_right,
            NeighborType::UpLeft => &neighbors.up_left,
            NeighborType::Left => &neighbors.left,
            NeighborType::DownLeft => &neighbors.down_left,
            NeighborType::DownRight => &neighbors.down_right,
        };

        if let Neighbor::Tile(tile) = neighbor {
            if let State::Building((plant, direction)) = &tile.plant {
                if direction == spread_direction {
                    match &spread_direction {
                        NeighborType::Right => {
                            bridges.right = plant.bridges.left.map(|bridge| bridge.get_opposite())
                        }
                        NeighborType::UpRight => bridges.le,
                        NeighborType::UpLeft => &neighbors.up_left,
                        NeighborType::Left => &neighbors.left,
                        NeighborType::DownLeft => &neighbors.down_left,
                        NeighborType::DownRight => &neighbors.down_right,
                    };
                    return ();
                }
            }
        }
    }

    /// Returns a mutated version of itself
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    fn mutate(&self, map_settings: &Settings) -> Self {
        return self.clone();
    }
}
