use crate::constants;

use super::Intensity;

/// The intensity for a location on a real world planet
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntensityDayPlanet {
    /// The size of the map
    pub size: usize,
    /// The length of a day in units of iteration steps
    pub day: f64,
}

impl IntensityDayPlanet {
    /// Constructs a new intensity object
    ///
    /// # Parameters
    ///
    /// day: The length of a day in units of iteration steps
    pub fn new(day: f64) -> Self {
        return Self { size: 1, day };
    }
}

impl Intensity for IntensityDayPlanet {
    fn get_intensity(&self, tile: usize, t: usize) -> (f64, f64) {
        let time_day = ((t as f64 / self.day + 1.0 - tile as f64 / self.size as f64) % 1.0)
            * 2.0
            * constants::MATH_PI;
        return (time_day.cos(), 1.0);
    }

    fn get_size(&self) -> usize {
        return self.size;
    }

    fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}
