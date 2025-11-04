use super::Settings;

/// Detailed implementation for a bridge branch
#[derive(Clone, Debug)]
pub struct Branch {}

impl Branch {
    /// Gets the energy cost factor of energy transfer for a branch bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_transfer_energy(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.transfer.energy.branch;
    }

    /// Gets the energy cost factor of running a branch bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_run(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.running.bridge.branch;
    }

    /// Gets the energy cost of building a new branch bridge
    ///
    /// # Parameters
    ///
    /// map_settings: The general map settings
    pub fn get_energy_cost_build(&self, map_settings: &Settings) -> f64 {
        return map_settings.energy.base.bridge.branch;
    }
}
