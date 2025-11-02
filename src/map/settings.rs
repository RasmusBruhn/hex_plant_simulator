/// All basic settings for a map
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The base transparency of any tile
    pub transparency: f64,
    /// The transparency of a non-leaf plant
    pub transparency_plant: f64,
    /// The baseline transparency of a leaf
    pub transparency_leaf: f64,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            transparency: 1.0,
            transparency_plant: 0.0,
            transparency_leaf: 1.0,
        };
    }

    /// Sets the transparency of the settings and returns the updated settings
    ///
    /// # Parameters
    ///
    /// transparency: The new transparency to set
    pub fn with_transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;

        return self;
    }

    /// Sets the transparency_plant of the settings and returns the updated settings
    ///
    /// # Parameters
    ///
    /// transparency: The new transparency to set
    pub fn with_transparency_plant(mut self, transparency: f64) -> Self {
        self.transparency_plant = transparency;

        return self;
    }

    /// Sets the transparency_leaf of the settings and returns the updated settings
    ///
    /// # Parameters
    ///
    /// transparency: The new transparency to set
    pub fn with_transparency_leaf(mut self, transparency: f64) -> Self {
        self.transparency_leaf = transparency;

        return self;
    }
}
