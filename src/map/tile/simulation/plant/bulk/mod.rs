use super::Settings;

mod log;
pub use log::Log;

mod sugar_bulb;
pub use sugar_bulb::SugarBulb;

mod leaf;
pub use leaf::Leaf;

mod seed;
pub use seed::Seed;

mod ripe_seed;
pub use ripe_seed::RipeSeed;

/// The bulk of a plant tile
#[derive(Clone, Debug)]
pub enum Bulk {
    /// Skeleton of a plant, able to produce multiple bridges
    Log(Log),
    /// Storage medium for extra energy, can produce multiple bridges
    SugarBulb(SugarBulb),
    /// Converts light into energy
    Leaf(Leaf),
    /// Non-ripe seed, needs to be filled up with energy to become a ripe seed and detach from the mother plant
    Seed(Seed),
    /// A ripe seed, able to fall in the world, will become a sugar bulb when it is ready
    RipeSeed(RipeSeed),
}

impl Bulk {
    /// Gets the transparency for this plant
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for this map
    pub fn get_transparency(&self, map_settings: &Settings) -> f64 {
        return match self {
            Self::Log(plant) => plant.get_transparency(map_settings),
            Self::SugarBulb(plant) => plant.get_transparency(map_settings),
            Self::Leaf(plant) => plant.get_transparency(map_settings),
            Self::Seed(plant) => plant.get_transparency(map_settings),
            Self::RipeSeed(plant) => plant.get_transparency(map_settings),
        };
    }
}
