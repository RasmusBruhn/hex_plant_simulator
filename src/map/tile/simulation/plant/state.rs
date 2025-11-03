use std::iter::once;

use super::{Neighbor, NeighborType, Plant, Settings, Spread, TileNeighbors};

/// The state of plant growth in a tile
#[derive(Clone, Debug)]
pub enum State {
    /// There is no plant
    Nothing,
    /// A plant is currently building and the .spread value of the plant spreading, will be created next step
    Building((Plant, NeighborType)),
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
            Self::Building((plant, _)) | Self::Occupied(plant) => {
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
            Self::Building((plant, _)) => Self::Occupied(plant.clone()),
            Self::Occupied(plant) => match plant.forward(map_settings, neighbors) {
                Some(plant) => Self::Occupied(plant),
                None => Self::Nothing,
            },
        };
    }

    /// See if any neighbors are trying to spread
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbors: References to all the neighbors of this tile
    fn try_spread(map_settings: &Settings, neighbors: &TileNeighbors) -> Self {
        // Find the neighbor trying to spread with the highest priority
        return if let Some((plant, neighbor_type)) =
            once(Self::neighbor_spread(&neighbors.right, NeighborType::Left))
                .chain(once(Self::neighbor_spread(
                    &neighbors.up_right,
                    NeighborType::DownLeft,
                )))
                .chain(once(Self::neighbor_spread(
                    &neighbors.up_left,
                    NeighborType::DownRight,
                )))
                .chain(once(Self::neighbor_spread(
                    &neighbors.left,
                    NeighborType::Right,
                )))
                .chain(once(Self::neighbor_spread(
                    &neighbors.down_left,
                    NeighborType::UpRight,
                )))
                .chain(once(Self::neighbor_spread(
                    &neighbors.down_right,
                    NeighborType::UpLeft,
                )))
                .filter_map(|value| value)
                .min_by_key(|(_, value)| value.id())
        {
            Self::Building((plant.mutate(map_settings), neighbor_type))
        } else {
            Self::Nothing
        };
    }

    /// Returns the neighbor tile if the neighbor is attempting to spread to the center tile
    ///
    /// # Parameters
    ///
    /// neighbor: The neighbor to check
    ///
    /// spread_direction: The direction it must spread to spread to the center tile
    fn neighbor_spread<'a>(
        neighbor: &Neighbor<'a>,
        spread_direction: NeighborType,
    ) -> Option<(&'a Plant, NeighborType)> {
        return if let Neighbor::Tile(neighbor_tile) = neighbor {
            if let State::Occupied(neighbor_plant) = &neighbor_tile.plant {
                if let Spread::Trying(neighbor_spread) = &neighbor_plant.spread {
                    if &neighbor_spread.0 == &spread_direction {
                        Some((&neighbor_spread.1, spread_direction))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
    }
}
