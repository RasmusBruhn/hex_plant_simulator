use super::Settings;

/// Detailed implementation for a seed
#[derive(Clone, Debug)]
pub struct Seed {}

impl Seed {
    /// Gets the transparency of a seed
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for this map
    pub fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return map_settings.transparency.seed;
    }

    /// Gets the energy cost factor of energy storage for a seed
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    /// 
    /// capacity: The storage capacity
    pub fn get_energy_cost_storage_energy(&self, map_settings: &Settings, capacity: f64) -> f64 {
        return map_settings.energy.storage.energy.seed * capacity;
    }

    /// Gets the energy cost factor of running a seed
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_run(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.running.bulk.seed;
    }

    /// Gets the energy cost of building a new seed
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_build(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.base.bulk.seed;
    }
}
