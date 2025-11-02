use super::{NeighborType, Plant, Settings};

/// The state of plant growth in a tile
#[derive(Clone, Debug)]
pub enum State {
    /// There is no plant
    Nothing,
    /// A plant is currently building and the .spread value of the plant spreading, will be created next step
    Building((Plant, NeighborType)),
    /// This tile is inhabited by a plant
    Done(Plant),
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
            Self::Building((plant, _)) | Self::Done(plant) => plant.get_transparency(map_settings),
        };
    }

    /// Forwards the state to the next simulation step
    pub fn forward(&self, map_settings: &Settings) -> Self {
        return match self {
            Self::Nothing
        }
    }
}
