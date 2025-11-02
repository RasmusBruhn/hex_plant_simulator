use crate::map::sun::Intensity;

/// A sun intensity multiplying the intensity for a year with the relative
/// intensity of a day to have a yearly and a daily cycle
#[derive(Clone, Debug)]
pub struct IntensityYearDay<Y: Intensity, D: Intensity> {
    /// The yearly intensity cycle
    year: Y,
    /// The daily intensity cycle
    day: D,
}

impl<Y: Intensity, D: Intensity> Intensity for IntensityYearDay<Y, D> {
    fn get_intensity(&self, tile: usize, t: usize) -> (f64, f64) {
        let year = self.year.get_intensity(tile, t);
        let day = self.day.get_intensity(tile, t);
        return (year.0 * day.0, year.1 * day.1);
    }

    fn iter(&self, t: usize) -> impl Iterator<Item = (f64, f64)> {
        return self
            .year
            .iter(t)
            .zip(self.day.iter(t))
            .map(|(year, day)| return (year.0 * day.0, year.1 * day.1));
    }

    fn get_size(&self) -> usize {
        return self.year.get_size();
    }

    fn set_size(&mut self, size: usize) {
        self.year.set_size(size);
        self.day.set_size(size);
    }
}
