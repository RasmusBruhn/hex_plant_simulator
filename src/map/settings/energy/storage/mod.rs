pub mod energy;

/// The energy cost when building storage
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The cost when building energy storage
    pub energy: energy::Settings,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            energy: energy::Settings::new(),
        };
    }

    /// Sets the cost when building energy storage and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub fn with_energy(mut self, settings: energy::Settings) -> Self {
        self.energy = settings;

        return self;
    }
}
