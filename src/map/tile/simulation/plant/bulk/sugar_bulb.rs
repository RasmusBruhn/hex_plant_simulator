use super::{Settings, TileData, TileNeighbors};

/// Detailed implementation for a sugar bulb
#[derive(Clone, Debug)]
pub struct SugarBulb {}

impl SugarBulb {
    /// Gets the transparency of a sugar bulb
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for this map
    pub fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return map_settings.transparency.sugar_bulb;
    }

    /// Gets the energy cost of building energy storage for a sugar bulb
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    ///
    /// capacity: The storage capacity
    pub fn get_energy_cost_storage_energy(&self, map_settings: &Settings, capacity: f64) -> f64 {
        return map_settings.energy.storage.energy.sugar_bulb * capacity;
    }

    /// Gets the energy cost factor of running a sugar bulb
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_factor_run(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.running.bulk.sugar_bulb;
    }

    /// Gets the base energy cost of building a new sugar bulb
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_build_base(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.base.bulk.sugar_bulb;
    }

    /// Gets the energy gained by this sugar bulb this round
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
        _tile: &TileData,
        _neighbors: &TileNeighbors,
    ) -> f64 {
        return 0.0;
    }
}
