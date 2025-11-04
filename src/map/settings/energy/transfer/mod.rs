pub mod energy;

/// The scaling energy cost of building transfer
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The scaling cost for energy transfer
    pub energy: energy::Settings,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            energy: energy::Settings::new(),
        };
    }

    /// Sets the scaling cost for energy transfer and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub fn with_energy(mut self, settings: energy::Settings) -> Self {
        self.energy = settings;

        return self;
    }
}
