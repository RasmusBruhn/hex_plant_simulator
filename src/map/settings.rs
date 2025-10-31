/// All basic settings for a map
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The base transparency of any tile
    pub transparency: f64,
    /// The speed of the sun (increment per simulation step)
    pub sun_speed: f64,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            transparency: 1.0,
            sun_speed: 0.0,
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

    /// Sets the speed of the sun in the settings and returns the updated settings
    ///
    /// # Parameters
    ///
    /// speed: The new speed to set
    pub fn with_sun_speed(mut self, speed: f64) -> Self {
        self.sun_speed = speed;

        return self;
    }
}
