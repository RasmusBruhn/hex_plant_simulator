use super::Settings;

/// Detailed implementation for a bulk sugar bulb
#[derive(Clone, Debug)]
pub struct SugarBulb {}

impl SugarBulb {
    /// Gets the transparency of this tile
    /// 
    /// # Parameters
    /// 
    /// map_settings: The settings for this map
    pub fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return map_settings.transparency.plant;
    }
}
