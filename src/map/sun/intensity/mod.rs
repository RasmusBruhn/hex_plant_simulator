// The angle of the sun above the horizon (alpha) for a planet is given by:
//
// cos(alpha) = (cos(beta) * cos(psi) + cos(phi) * tan(theta) * sin(beta)) / sqrt(1 + cos^2(phi) * tan^2(theta))
//
// theta: Tilt of the planet (0: equator is in orbital plane)
// beta: Latitude of location on planet
// phi: Time of year (0: summer for positive beta)
// psi: Time of day (0: noon)

use std::fmt::Debug;

mod year;
pub use year::IntensityYearPlanet;

mod day;
pub use day::IntensityDayPlanet;

mod full;
pub use full::IntensityYearDay;

/// Defines the global intensity of the sun for all tiles as a function of time
pub trait Intensity: Clone + Debug {
    /// Gets the intesity for a single tile at a specific iteration step, returns the primary and secondary intensities
    ///
    /// # Parameters
    ///
    /// tile: The index of the tile
    ///
    /// t: The time step of the simulation
    fn get_intensity(&self, tile: usize, t: usize) -> (f64, f64);

    /// Returns the map size
    fn get_size(&self) -> usize;

    /// Sets the size of the map
    /// 
    /// # Parameters
    /// 
    /// size: The size of the map
    fn set_size(&mut self, size: usize);

    /// Gets an iterator over all tiles of the intensity
    ///
    /// # Parameters
    ///
    /// t: The time step of the simulation
    fn iter(&self, t: usize) -> impl Iterator<Item = (f64, f64)> {
        return (0..self.get_size()).map(move |tile| self.get_intensity(tile, t));
    }
}
