use super::{NeighborType, Plant, Settings, TileNeighbors};

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
        todo!()
    }
}
