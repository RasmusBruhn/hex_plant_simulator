use super::{Neighbor, Settings, Tile, TileNeighbors};

impl Tile {
    /// Calculates the next state of the tile
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbohrs: References to all the neighbohrs of this til
    pub fn forward(&self, map_settings: &Settings, neighbors: &TileNeighbors) -> Self {
        // Set the new transparency
        let transparency = map_settings.transparency;

        // Set the new light level
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
        let light = 0.5 * (light_right + light_left);

        return Self {
            transparency,
            light,
        };
    }
}
