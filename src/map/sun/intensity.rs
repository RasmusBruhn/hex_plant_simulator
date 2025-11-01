use std::fmt::Debug;

use crate::constants;

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

    /// Gets an iterator over all tiles of the intensity
    ///
    /// # Parameters
    ///
    /// t: The time step of the simulation
    fn iter(&self, t: usize) -> impl Iterator<Item = (f64, f64)> {
        return (0..self.get_size()).map(move |tile| self.get_intensity(tile, t));
    }
}

/// The intensity for a location on a real world planet
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntensityPlanet {
    /// The size of the map
    pub size: usize,
    /// The tilt of the planet in radians, 0 is when the equator is in the orbital plane
    pub tilt: f64,
    /// The latitude of the target location on the planet in radians, 0 is the equator
    pub latitude: f64,
    /// The length of a year in usits of iteration steps
    pub year: f64,
    /// The maximum intensity when the sun is right overhead
    pub intensity: f64,
}

impl IntensityPlanet {
    /// Constructs a new intensity object
    ///
    /// # Parameters
    ///
    /// size: The size of the map
    ///
    /// tilt: The tilt of the planet in radians, 0 is when the equator is in the orbital plane
    ///
    /// latitude: The latitude of the target location on the planet in radians, 0 is the equator
    ///
    /// year: The length of a year in usits of iteration steps
    ///
    /// intensity: The maximum intensity when the sun is right overhead
    pub fn new(size: usize, tilt: f64, latitude: f64, year: f64, intensity: f64) -> Self {
        return Self {
            size,
            tilt,
            latitude,
            year,
            intensity,
        };
    }
}
//sqrt(1 + cos^2(phi) * tan^2(theta))
impl Intensity for IntensityPlanet {
    fn get_intensity(&self, tile: usize, t: usize) -> (f64, f64) {
        let time_year = ((t as f64 / self.year) % 1.0) * 2.0 * constants::MATH_PI;
        let time_year_cos = time_year.cos();
        let max_intensity = (1.0 + time_year_cos * time_year_cos);
    }
}
