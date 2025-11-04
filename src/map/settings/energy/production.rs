/// The scaling energy cost of production
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The scaling cost for a leaf
    pub leaf: f64,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self { leaf: 1.0 };
    }

    /// Sets the scaling cost for a leaf and returns the updated settings
    ///
    /// # Parameters
    ///
    /// cost: The new cost
    pub fn with_leaf(mut self, cost: f64) -> Self {
        self.leaf = cost;

        return self;
    }
}
