// Log: #52361e
// Leaf: #1b6623
// SugarBulb: #93b5ae
// Seed: #f2bb07
// RipeSeed: #b30c1a

// Log: #52361e
// Branch: #78583c
use super::{Neighbor, NeighborDirection, Settings, TileData, TileNeighbors};

mod state;
pub use state::State;

mod spread;
use spread::Spread;

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
    fn get_bulk_energy_cost_build(&self, map_settings: &Settings) -> f64 {
        return self.bulk.get_energy_cost_build_base(map_settings)
            + self
                .bulk
                .get_energy_cost_storage_energy(map_settings, self.energy_capacity);
    }

    /// Gets the energy cost of running the bulk of this plant
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    fn get_bulk_energy_cost_run(&self, map_settings: &Settings) -> f64 {
        return self.get_bulk_energy_cost_build(map_settings)
            * self.bulk.get_energy_cost_factor_run(map_settings);
    }

    /// Gets the energy cost of building this plant
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    fn get_energy_cost_build(&self, map_settings: &Settings) -> f64 {
        return self.get_bulk_energy_cost_build(map_settings)
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
        return self.get_bulk_energy_cost_run(map_settings)
            + self
                .bridges
                .iter()
                .map(|bridge| 0.5 * bridge.get_energy_cost_run(map_settings))
                .sum::<f64>();
    }

    /// Gets the energy gained by this plant this round
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    ///
    /// tile: The data of the tile this plant is located on
    ///
    /// neighbors: All neighbor tiles to this tile
    fn get_energy_gain(
        &self,
        map_settings: &Settings,
        tile: &TileData,
        neighbors: &TileNeighbors,
    ) -> f64 {
        return self.bulk.get_energy_gain(map_settings, tile, neighbors);
    }

    /// Forwards the state of this plant to the next simulation step
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// tile: The tile data for the tile of this plant
    ///
    /// neighbors: References to all the neighbors of this tile
    fn forward(
        &self,
        map_settings: &Settings,
        tile: &TileData,
        neighbors: &TileNeighbors,
    ) -> Option<Self> {
        // Kill it if it was assigned to die
        if !self.alive {
            return None;
        }

        // Setup initial bridges
        let mut bridges = self.bridges.clone();

        // Remove unused bridges
        Self::remove_bridges(&mut bridges, neighbors);

        // Handle ongoing spreading
        let (spread, energy) = match &self.spread {
            Spread::Nothing => (Spread::Nothing, self.energy),
            Spread::Trying(value) => (Spread::Waiting(Box::new((value.1, value.2))), self.energy),
            Spread::Waiting(value) => (
                Spread::Nothing,
                Self::spread_resolve(&mut bridges, &value.1, value.0, self.energy, neighbors),
            ),
        };

        // Gain and spend energy
        let cost_energy = self.get_energy_cost_run(map_settings);
        let gain_energy = self.get_energy_gain(map_settings, tile, neighbors);

        // Transfer energy

        // Get total energy and make sure it has enough to survive

        todo!()
    }

    /// Removes any bridge connected to a tile which is not occupied with an alive plant
    ///
    /// # Parameters
    ///
    /// bridges: The bridges to modify
    ///
    /// neighbors: All of the neighboring tiles
    fn remove_bridges(bridges: &mut BridgeSet, neighbors: &TileNeighbors) {
        NeighborDirection::collection().iter().for_each(|dir| {
            if let Neighbor::Tile(tile) = neighbors.get(dir) {
                if let State::Occupied(plant) = &tile.plant {
                    if plant.alive {
                        return;
                    }
                }
            }

            *bridges.get_mut(dir) = None;
        });
    }

    /// Resolves a spread action after waiting, returning the new energy of this
    /// plant and sets the new bridge if it is spreading
    ///
    /// # Parameters
    ///
    /// bridges: The bridges for the plant after removing dead connections
    ///
    /// direction: The direction to spread in
    ///
    /// energy: The energy used to spread
    ///
    /// self_energy: The enrgy of the plant
    ///
    /// neighbors: All neighbors of this tile
    fn spread_resolve(
        bridges: &mut BridgeSet,
        direction: &NeighborDirection,
        energy: f64,
        self_energy: f64,
        neighbors: &TileNeighbors,
    ) -> f64 {
        if let Neighbor::Tile(tile) = neighbors.get(direction) {
            if let State::Building((plant, _, build_dir)) = &tile.plant {
                if build_dir == direction {
                    if let Some(bridge) = plant.bridges.get(&direction.opposite()).as_ref() {
                        *bridges.get_mut(direction) = Some(bridge.get_opposite());
                        return self_energy;
                    }
                }
            }
        }
        return self_energy + energy;
    }

    /// Returns a mutated version of itself
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    fn mutate(&self, _map_settings: &Settings) -> Self {
        return self.clone();
    }
}
