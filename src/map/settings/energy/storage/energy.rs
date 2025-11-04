/// The energy cost of building energy storage
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The storage cost for a log
    pub log: f64,
    /// The storage cost for a sugar bulb
    pub sugar_bulb: f64,
    /// The storage cost for a leaf
    pub leaf: f64,
    /// The storage cost for a seed
    pub seed: f64,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            log: 1.0,
            sugar_bulb: 1.0,
            leaf: 1.0,
            seed: 1.0,
        };
    }

    /// Sets the storage cost for a log and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_log(mut self, cost: f64) -> Self {
        self.log = cost;

        return self;
    }

    /// Sets the storage cost for a sugar bulb and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_sugar_bulb(mut self, cost: f64) -> Self {
        self.sugar_bulb = cost;

        return self;
    }

    /// Sets the storage cost for a leaf and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_leaf(mut self, cost: f64) -> Self {
        self.leaf = cost;

        return self;
    }

    /// Sets the storage cost for a seed and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_seed(mut self, cost: f64) -> Self {
        self.seed = cost;

        return self;
    }
}
