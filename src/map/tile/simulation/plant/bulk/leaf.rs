use super::{Settings, TileData, TileNeighbors};

/// Detailed implementation for a leaf
#[derive(Clone, Debug)]
pub struct Leaf {
    /// The percentage of light absorbed by this leaf in photosynthesis
    absorption: f64,
}

impl Leaf {
    /// Gets the transparency of a leaf
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for this map
    pub fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return map_settings.transparency.leaf * (1.0 - self.absorption);
    }

    /// Gets the energy cost of building energy storage for a leaf
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    ///
    /// capacity: The storage capacity
    pub fn get_energy_cost_storage_energy(&self, map_settings: &Settings, capacity: f64) -> f64 {
        return map_settings.energy.storage.energy.leaf * capacity * capacity;
    }

    /// Gets the energy cost factor of running a leaf
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_factor_run(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.running.bulk.leaf;
    }

    /// Gets the base energy cost of building a new leaf
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_build_base(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.base.bulk.leaf
            + map_settings.energy.production.leaf / (1.0 - self.absorption);
    }

    /// Gets the energy gained by this leaf this round
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    ///
    /// tile: The data of the tile this plant is located on
    ///
    /// neighbors: All neighbor tiles to this tile
    pub fn get_energy_gain(
        &self,
        _map_settings: &Settings,
        tile: &TileData,
        _neighbors: &TileNeighbors,
    ) -> f64 {
        return tile.light * self.absorption;
    }
}
