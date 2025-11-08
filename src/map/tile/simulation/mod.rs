use super::{Neighbor, Settings, Tile, TileNeighbors, NeighborDirection};

pub mod plant;

impl Tile {
    /// Calculates the next state of the tile
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbors: References to all the neighbors of this til
    pub fn forward(&self, map_settings: &Settings, neighbors: &TileNeighbors) -> Self {
        return Self {
            plant: self.plant.forward(map_settings, neighbors),
            transparency: self.forward_transparency(map_settings, neighbors),
            light: self.forward_light(map_settings, neighbors),
        };
    }

    /// Calculates the next transparency of the tile
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbors: References to all the neighbors of this til
    fn forward_transparency(&self, map_settings: &Settings, _neighbors: &TileNeighbors) -> f64 {
        return map_settings.transparency.base * self.plant.get_transparency(map_settings);
    }

    /// Calculates the next light level of the tile
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbors: References to all the neighbors of this til
    fn forward_light(&self, _map_settings: &Settings, neighbors: &TileNeighbors) -> f64 {
        let light_right = match neighbors.up_right {
            Neighbor::Empty => 0.0,
            Neighbor::Tile(tile) => tile.light * tile.transparency,
            Neighbor::SunTile(tile) => tile.intensity,
        };
        let light_left = match neighbors.up_left {
            Neighbor::Empty => 0.0,
            Neighbor::Tile(tile) => tile.light * tile.transparency,
            Neighbor::SunTile(tile) => tile.intensity,
        };
        return 0.5 * (light_right + light_left);
    }
}
