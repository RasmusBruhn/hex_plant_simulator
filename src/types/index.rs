use std::ops::Add;

use super::Point;

/// A 2D index
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Index {
    /// The x-index
    pub x: isize,
    /// The y-index
    pub y: isize,
}

impl Index {
    /// Creates a new index
    ///
    /// # Parameters
    ///
    /// x: The x-index
    ///
    /// y: The y-index
    pub fn new(x: isize, y: isize) -> Self {
        return Self { x, y };
    }

    /// Constructs a new index from a point by flooring the coordinates
    pub fn from_point(point: &Point) -> Self {
        return Self {
            x: point.x.floor() as isize,
            y: point.y.floor() as isize,
        };
    }
}

impl Add<&Index> for &Index {
    type Output = Index;

    fn add(self, rhs: &Index) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

impl Add<Index> for &Index {
    type Output = Index;

    fn add(self, rhs: Index) -> Self::Output {
        return self + &rhs;
    }
}

impl Add<&Index> for Index {
    type Output = Index;

    fn add(self, rhs: &Index) -> Self::Output {
        return &self + rhs;
    }
}

impl Add<Index> for Index {
    type Output = Index;

    fn add(self, rhs: Index) -> Self::Output {
        return &self + &rhs;
    }
}
