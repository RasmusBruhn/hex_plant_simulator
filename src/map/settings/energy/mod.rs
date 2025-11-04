pub mod base;

pub mod production;

pub mod storage;

pub mod transfer;

pub mod running;

/// The energy cost
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The base cost when building
    pub base: base::Settings,
    /// The cost of production capacity
    pub production: production::Settings,
    /// The cost when building storage
    pub storage: storage::Settings,
    /// The cost when building transfer
    pub transfer: transfer::Settings,
    /// The running cost
    pub running: running::Settings,
}

impl Settings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            base: base::Settings::new(),
            production: production::Settings::new(),
            storage: storage::Settings::new(),
            transfer: transfer::Settings::new(),
            running: running::Settings::new(),
        };
    }

    /// Sets the base cost when building and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub fn with_base(mut self, settings: base::Settings) -> Self {
        self.base = settings;

        return self;
    }

    /// Sets the cost when building production and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub fn with_production(mut self, settings: production::Settings) -> Self {
        self.production = settings;

        return self;
    }

    /// Sets the cost when building storage and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub fn with_storage(mut self, settings: storage::Settings) -> Self {
        self.storage = settings;

        return self;
    }

    /// Sets the cost when building transfer and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub fn with_transfer(mut self, settings: transfer::Settings) -> Self {
        self.transfer = settings;

        return self;
    }

    /// Sets the running cost multiplier and returns the updated settings
    ///
    /// # Parameters
    ///
    /// settings: The settings to set
    pub fn with_running(mut self, settings: running::Settings) -> Self {
        self.running = settings;

        return self;
    }
}
