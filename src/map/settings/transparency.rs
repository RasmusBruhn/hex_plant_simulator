/// All transparency settings for a map
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The base transparency of any tile
    pub base: f64,
    /// The transparency of a log
    pub log: f64,
    /// The transparency of a sugar bulb
    pub sugar_bulb: f64,
    /// The baseline transparency of a leaf
    pub leaf: f64,
    /// The transparency of a seed
    pub seed: f64,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            base: 1.0,
            log: 0.0,
            sugar_bulb: 0.0,
            leaf: 1.0,
            seed: 0.0,
        };
    }

    /// Sets the base transparency and returns the updated settings
    ///
    /// # Parameters
    ///
    /// transparency: The new transparency to set
    pub fn with_base(mut self, transparency: f64) -> Self {
        self.base = transparency;

        return self;
    }

    /// Sets the transparency of a log and returns the updated settings
    ///
    /// # Parameters
    ///
    /// transparency: The new transparency to set
    pub fn with_log(mut self, transparency: f64) -> Self {
        self.log = transparency;

        return self;
    }

    /// Sets the transparency of a sugar bulb and returns the updated settings
    ///
    /// # Parameters
    ///
    /// transparency: The new transparency to set
    pub fn with_sugar_bulb(mut self, transparency: f64) -> Self {
        self.sugar_bulb = transparency;

        return self;
    }

    /// Sets the baseline transparency of a leaf and returns the updated settings
    ///
    /// # Parameters
    ///
    /// transparency: The new transparency to set
    pub fn with_leaf(mut self, transparency: f64) -> Self {
        self.leaf = transparency;

        return self;
    }

    /// Sets the transparency of a seed and returns the updated settings
    ///
    /// # Parameters
    ///
    /// transparency: The new transparency to set
    pub fn with_seed(mut self, transparency: f64) -> Self {
        self.seed = transparency;

        return self;
    }
}
