use super::{Intensity, Tile};

/// Describes the current state of the sun
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State<S: Intensity> {
    /// The intensity variation
    pub intensity: S,
}

impl<S: Intensity> State<S> {
    /// Constructs a new sun state
    ///
    /// # Parameters
    ///
    /// intensity: The intensity variations
    pub fn new(intensity: S) -> Self {
        return Self { intensity };
    }

    /// Constructs all the sun intensity tiles for the current time of the simulation
    ///
    /// # Parameters
    ///
    /// t: The simulation step of the tile
    pub fn get_tiles(&self, t: usize) -> Vec<Tile> {
        return self
            .intensity
            .iter(t)
            .map(|intensity| {
                return Tile::new(intensity.0 + intensity.1);
            })
            .collect();
    }
}
