use super::Settings;

/// Detailed implementation for a bridge log
#[derive(Clone, Debug)]
pub struct Log {}

impl Log {
    /// Gets the energy build cost of energy transfer for a log bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    /// 
    /// capacity: The transfer capacity
    pub fn get_energy_cost_transfer_energy(&self, map_settings: &Settings, capacity: f64) -> f64 {
        return map_settings.energy.transfer.energy.log * capacity;
    }

    /// Gets the energy cost factor of running a log bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_factor_run(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.running.bridge.log;
    }

    /// Gets the energy base cost of building a new log bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_build_base(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.base.bridge.log;
    }
}
