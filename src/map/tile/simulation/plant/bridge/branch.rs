use super::Settings;

/// Detailed implementation for a bridge branch
#[derive(Clone, Debug)]
pub struct Branch {}

impl Branch {
    /// Gets the energy build cost of energy transfer for a branch bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    /// 
    /// capacity: The transfer capacity
    pub fn get_energy_cost_transfer_energy(&self, map_settings: &Settings, capacity: f64) -> f64 {
        return map_settings.energy.transfer.energy.branch * capacity * capacity;
    }

    /// Gets the energy cost factor of running a branch bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_factor_run(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.running.bridge.branch;
    }

    /// Gets the energy base cost of building a new branch bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_build_base(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.base.bridge.branch;
    }
}
