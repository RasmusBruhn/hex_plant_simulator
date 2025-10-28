use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Size;

/// A 2D point
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// The x-coordinate
    pub x: f64,
    /// The y-coordinate
    pub y: f64,
}

impl Point {
    /// Creates a new point
    ///
    /// # Parameters
    ///
    /// x: The x-coordinate
    ///
    /// y: The y-coordinate
    pub const fn new(x: f64, y: f64) -> Self {
        return Self { x, y };
    }

    /// Calculates the norm squared of the point
    pub const fn norm_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }

    /// Calculates the norm of the point
    pub fn norm(&self) -> f64 {
        return self.norm_squared().sqrt();
    }

    /// Calculates the dot product between two points
    pub const fn dot(&self, rhs: &Point) -> f64 {
        return self.x * rhs.x + self.y * rhs.y;
    }

    /// Calculates the cross product between two points
    pub const fn cross(&self, rhs: &Point) -> f64 {
        return self.x * rhs.y - self.y * rhs.x;
    }

    /// Converts it to a size
    pub const fn to_size(&self) -> Size {
        return Size::new(self.x, self.y);
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        return Self::Output::new(-self.x, -self.y);
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        return -&self;
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Self::Output { x, y };
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        return self + &rhs;
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        return &self + rhs;
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        return &self + &rhs;
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        return Self::Output { x, y };
    }
}

impl Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        return self - &rhs;
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        return &self - rhs;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        return &self - &rhs;
    }
}

impl Mul<&f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: &f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;

        return Self::Output { x, y };
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        return self * &rhs;
    }
}

impl Mul<&f64> for Point {
    type Output = Point;

    fn mul(self, rhs: &f64) -> Self::Output {
        return &self * rhs;
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        return &self * &rhs;
    }
}

impl Div<&f64> for &Point {
    type Output = Point;

    fn div(self, rhs: &f64) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;

        return Self::Output { x, y };
    }
}

impl Div<f64> for &Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        return self / &rhs;
    }
}

impl Div<&f64> for Point {
    type Output = Point;

    fn div(self, rhs: &f64) -> Self::Output {
        return &self / rhs;
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        return &self / &rhs;
    }
}
