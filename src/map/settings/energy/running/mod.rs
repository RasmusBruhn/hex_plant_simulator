pub mod bridge;

pub mod bulk;

/// The running energy cost multiplier (cost per step is build cost multiplied by this value)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The running cost multiplier for a bridge
    pub bridge: bridge::Settings,
    /// The running cost multiplier for a bulk
    pub bulk: bulk::Settings,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            bridge: bridge::Settings::new(),
            bulk: bulk::Settings::new(),
        };
    }

    /// Sets the running cost multiplier for a bridge and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub fn with_log(mut self, settings: bridge::Settings) -> Self {
        self.bridge = settings;

        return self;
    }

    /// Sets the running cost multiplier for a bulk and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub fn with_bulk(mut self, settings: bulk::Settings) -> Self {
        self.bulk = settings;

        return self;
    }
}
