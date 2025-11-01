use super::InstanceTile;

/// All data for a single sun ray
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tile {
    /// The intensity of the sun
    intensity: f64,
}

impl Tile {
    /// Constructs a new sun tile
    ///
    /// # Parameters
    ///
    /// intensity: The intensity of the tile
    pub fn new(intensity: f64) -> Self {
        return Self { intensity };
    }

    /// Converts the sun tile to shader compatible data
    pub fn get_data(&self) -> InstanceTile {
        return InstanceTile {
            color_value: self.intensity as f32,
        };
    }
}
