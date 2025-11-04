/// The base energy cost of building a bridge
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The base cost for a log
    pub log: f64,
    /// The base cost for a branch
    pub branch: f64,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            log: 0.0,
            branch: 0.0,
        };
    }

    /// Sets the base cost for a log and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_log(mut self, cost: f64) -> Self {
        self.log = cost;

        return self;
    }

    /// Sets the base cost for a branch and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_branch(mut self, cost: f64) -> Self {
        self.branch = cost;

        return self;
    }
}
