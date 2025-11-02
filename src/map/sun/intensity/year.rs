use crate::constants;

use super::Intensity;

/// The intensity for a location on a real world planet
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntensityYearPlanet {
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

impl IntensityYearPlanet {
    /// Constructs a new intensity object
    ///
    /// # Parameters
    ///
    /// tilt: The tilt of the planet in radians, 0 is when the equator is in the orbital plane
    ///
    /// latitude: The latitude of the target location on the planet in radians, 0 is the equator
    ///
    /// year: The length of a year in usits of iteration steps
    ///
    /// intensity: The maximum intensity when the sun is right overhead
    pub fn new(tilt: f64, latitude: f64, year: f64, intensity: f64) -> Self {
        return Self {
            size: 1,
            tilt,
            latitude,
            year,
            intensity,
        };
    }
}

impl Intensity for IntensityYearPlanet {
    fn get_intensity(&self, _tile: usize, t: usize) -> (f64, f64) {
        let time_year = ((t as f64 / self.year) % 1.0) * 2.0 * constants::MATH_PI;
        let x = time_year.cos() * self.tilt.tan();
        let max_intensity = (1.0 + x * x).sqrt();
        return (
            self.intensity * self.latitude.cos() / max_intensity,
            self.intensity * self.latitude.sin() * x / max_intensity,
        );
    }

    fn get_size(&self) -> usize {
        return self.size;
    }

    fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}
