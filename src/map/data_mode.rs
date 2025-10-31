use crate::types;

/// The display mode for the background of a tile
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataModeBackground {
    /// Display the light level of the tile
    Light,
    /// Display the transparency value of the tile
    Transparency,
}

impl DataModeBackground {
    pub const COUNT: usize = 2;

    /// The id to the mode in a list of all modes
    pub fn id(&self) -> usize {
        return match self {
            Self::Light => 0,
            Self::Transparency => 1,
        };
    }

    /// Constructs a new data mode from an id
    ///
    /// # Parameters
    ///
    /// id: The id to construct from
    pub fn from_id(id: usize) -> Self {
        return match id.clamp(0, Self::COUNT - 1) {
            0 => Self::Light,
            1 => Self::Transparency,
            _ => panic!("DataModeBackground::from_id has not been updated"),
        };
    }

    /// Gets the next mode
    pub fn next(&self) -> Self {
        return Self::from_id((self.id() + 1) % Self::COUNT);
    }

    /// Gets the previous mode
    pub fn prev(&self) -> Self {
        return Self::from_id((self.id() + (Self::COUNT - 1)) % Self::COUNT);
    }

    /// Constructs a new list of the color maps for all modes
    ///
    /// # Parameters
    ///
    /// transparency: The color map for transparency mode
    ///
    /// light: The color map for light mode
    pub fn new_color_map_collection(
        light: Box<dyn types::ColorMap>,
        transparency: Box<dyn types::ColorMap>,
    ) -> [Box<dyn types::ColorMap>; Self::COUNT] {
        return [light, transparency];
    }
}
