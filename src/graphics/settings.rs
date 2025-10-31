use super::InstanceType;
use crate::{map, types};

/// All non-gpu settings for rendering
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// The screen clear color
    pub color_clear: types::Color,
    /// The color maps for all the instance types
    pub color_maps: [Vec<Box<dyn types::ColorMap>>; InstanceType::COUNT],
    /// The display mode for the background
    pub mode_background: map::DataModeBackground,
}

impl Settings {
    /// Sets the clear color of the settings and returns it
    ///
    /// # Parameters
    ///
    /// color: The color to set
    pub fn with_color_clear(mut self, color: types::Color) -> Self {
        self.color_clear = color;

        return self;
    }

    /// Sets the background display mode of the settings and returns it
    ///
    /// # Parameters
    ///
    /// mode: The mode to set
    pub fn with_mode_background(mut self, mode: map::DataModeBackground) -> Self {
        self.mode_background = mode;

        return self;
    }

    /// Sets one of the color maps of the settings and returns it
    ///
    /// # Parameters
    ///
    /// color_map: The color map to set
    ///
    /// instance: The instance type to set the color map for
    pub fn with_color_map(
        mut self,
        color_map: Vec<types::ColorMap>,
        instance: &InstanceType,
    ) -> Self {
        self.color_maps[instance.id()] = color_map;

        return self;
    }
}
