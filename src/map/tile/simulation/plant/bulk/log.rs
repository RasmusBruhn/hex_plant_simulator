use super::Settings;

/// Detailed implementation for a log
#[derive(Clone, Debug)]
pub struct Log {}

impl Log {
    /// Gets the transparency of a log
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for this map
    pub fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return map_settings.transparency.log;
    }

    /// Gets the energy cost factor of energy storage for a log
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    /// 
    /// capacity: The storage capacity
    pub fn get_energy_cost_storage_energy(&self, map_settings: &Settings, capacity: f64) -> f64 {
        return map_settings.energy.storage.energy.log * capacity;
    }

    /// Gets the energy cost factor of running a log
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_run(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.running.bulk.log;
    }

    /// Gets the energy cost of building a new log
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_build(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.base.bulk.log;
    }
}
