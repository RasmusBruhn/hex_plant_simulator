pub mod transparency;

pub mod energy;

/// All basic settings for a map
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// All transparency settings
    pub transparency: transparency::Settings,
    /// All energy cost settings
    pub energy: energy::Settings,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            transparency: transparency::Settings::new(),
            energy: energy::Settings::new(),
        };
    }

    /// Sets the transparency of the settings and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The new transparency settings
    pub fn with_transparency(mut self, settings: transparency::Settings) -> Self {
        self.transparency = settings;

        return self;
    }

    /// Sets the energy of the settings and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The new energy settings
    pub fn with_energy(mut self, settings: energy::Settings) -> Self {
        self.energy = settings;

        return self;
    }
}
