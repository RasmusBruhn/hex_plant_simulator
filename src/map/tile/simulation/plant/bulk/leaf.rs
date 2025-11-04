use super::Settings;

/// Detailed implementation for a bulk leaf
#[derive(Clone, Debug)]
pub struct Leaf {
    /// The percentage of light absorbed by this leaf in photosynthesis
    absorption: f64,
}

impl Leaf {
    /// Gets the transparency of this tile
    /// 
    /// # Parameters
    /// 
    /// map_settings: The settings for this map
    pub fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return map_settings.transparency.leaf / (1.0 - self.absorption);
    }
}
