use super::{Neighbor, NeighborDirection, Plant, Settings, Spread, TileNeighbors};

/// The state of plant growth in a tile
#[derive(Clone, Debug)]
pub enum State {
    /// There is no plant
    Nothing,
    /// A plant is currently building and the .spread value of the plant
    /// spreading, will be created next step, holds the plant to spread (without
    /// any resources), the energy to use for spreading and the direction the
    /// spread came from
    Building((Plant, f64, NeighborDirection)),
    /// This tile is inhabited by a plant
    Occupied(Plant),
}

impl State {
    /// Gets the transparency of the plant in this tile
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for this map
    pub fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return match self {
            Self::Nothing => 1.0,
            Self::Building((plant, _, _)) | Self::Occupied(plant) => {
                plant.get_transparency(map_settings)
            }
        };
    }

    /// Forwards the state to the next simulation step
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbors: References to all the neighbors of this tile
    pub fn forward(&self, map_settings: &Settings, neighbors: &TileNeighbors) -> Self {
        return match self {
            Self::Nothing => Self::try_spread(map_settings, neighbors),
            Self::Building(values) => Self::try_build(map_settings, values, neighbors),
            Self::Occupied(plant) => match plant.forward(map_settings, neighbors) {
                Some(plant) => Self::Occupied(plant),
                None => Self::Nothing,
            },
        };
    }

    /// See if any neighbors are trying to spread and mutates any attempt at
    /// spreading
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbors: References to all the neighbors of this tile
    fn try_spread(map_settings: &Settings, neighbors: &TileNeighbors) -> Self {
        return if let Some((plant, energy, dir)) = NeighborDirection::collection()
            .iter()
            .filter_map(|dir| {
                if let Neighbor::Tile(tile) = neighbors.get(dir) {
                    if let State::Occupied(plant) = &tile.plant {
                        if let Spread::Trying(spread) = &plant.spread {
                            if &spread.2 == dir {
                                return Some(spread.as_ref());
                            }
                        }
                    }
                }
                return None;
            })
            .min_by_key(|value| value.2.id())
        {
            Self::Building((plant.mutate(map_settings), *energy, *dir))
        } else {
            Self::Nothing
        };
    }

    /// Attempts to build the plant on this tile, fails if the mother plant is
    /// dead or there is not enough energy
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// input: The tile building input
    ///
    /// neighbors: All neighbor tiles
    fn try_build(
        map_settings: &Settings,
        input: &(Plant, f64, NeighborDirection),
        neighbors: &TileNeighbors,
    ) -> Self {
        if let Neighbor::Tile(tile) = neighbors.get(&input.2) {
            if let State::Occupied(plant) = &tile.plant {
                if plant.alive {
                    let mut new_plant = input.0.clone();

                    let cost_energy = new_plant.get_energy_cost_build(map_settings)
                        + new_plant.bridges.get(&input.2).as_ref().map_or_else(
                            || 0.0,
                            |bridge| 0.5 * bridge.get_energy_cost_build(map_settings),
                        );
                    let plant_energy = input.1 - cost_energy;
                    if plant_energy < 0.0 {
                        return Self::Nothing;
                    }
                    new_plant.energy = plant_energy.min(new_plant.energy_capacity);

                    return Self::Occupied(new_plant);
                }
            }
        }
        return Self::Nothing;
    }
}
