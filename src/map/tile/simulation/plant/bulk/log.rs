use super::Settings;

/// Detailed implementation for a bulk log
#[derive(Clone, Debug)]
pub struct Log {}

impl Log {
    /// Gets the transparency of this tile
    /// 
    /// # Parameters
    /// 
    /// map_settings: The settings for this map
    pub fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return map_settings.transparency.plant;
    }
}
