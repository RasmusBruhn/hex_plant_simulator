/// The running energy cost multiplier for a bridge (cost per step is build cost multiplied by this value)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The running cost multiplier for a log
    pub log: f64,
    /// The running cost multiplier for a branch
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

    /// Sets the running cost multiplier for a log and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_log(mut self, cost: f64) -> Self {
        self.log = cost;

        return self;
    }

    /// Sets the running cost mulitplier for a branch and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_branch(mut self, cost: f64) -> Self {
        self.branch = cost;

        return self;
    }
}
