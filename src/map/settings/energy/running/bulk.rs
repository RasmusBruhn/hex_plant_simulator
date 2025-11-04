/// The running energy cost multiplier for a bulk (cost per step is build cost multiplied by this value)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The running cost multiplier for a log
    pub log: f64,
    /// The running cost multiplier for a sugar bulb
    pub sugar_bulb: f64,
    /// The running cost multiplier for a leaf
    pub leaf: f64,
    /// The running cost multiplier for a seed
    pub seed: f64,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            log: 0.0,
            sugar_bulb: 0.0,
            leaf: 0.0,
            seed: 0.0,
        };
    }

    /// Sets the running cost multiplier for a log and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_log(mut self, cost: f64) -> Self {
        self.log = cost;

        return self;
    }

    /// Sets the running cost multiplier for a sugar bulb and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_sugar_bulb(mut self, cost: f64) -> Self {
        self.sugar_bulb = cost;

        return self;
    }

    /// Sets the running cost multiplier for a leaf and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_leaf(mut self, cost: f64) -> Self {
        self.leaf = cost;

        return self;
    }

    /// Sets the running cost multiplier for a seed and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_seed(mut self, cost: f64) -> Self {
        self.seed = cost;

        return self;
    }
}
