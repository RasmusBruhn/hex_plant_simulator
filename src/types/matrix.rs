use std::ops::{Add, Mul, Neg, Sub};

use super::Point;

/// Defines a 2x2 matrix
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix {
    /// The values of the matrix
    pub values: [f64; 4],
}

impl Matrix {
    /// Creates a new matrix
    ///
    /// # Parameters
    ///
    /// values: The values of the matrix, first index is row, second index is column
    pub fn new(values: [f64; 4]) -> Self {
        return Self { values };
    }

    /// Transposes the matrix
    pub fn transpose(&self) -> Self {
        return Self::new([
            self.values[0],
            self.values[2],
            self.values[1],
            self.values[3],
        ]);
    }

    /// Inverts the matrix
    ///
    /// # Panics
    ///
    /// In debug mode it panics if the determinant is 0 (it is not invertible)
    pub fn inv(&self) -> Self {
        // Calculate determinant
        let d = self.det();

        // Make sure it is not invalid
        if cfg!(debug_assertions) && d == 0.0 {
            panic!("The matrix is not invertible: {:?}", self);
        }

        // Calculate inverse
        return Self::new([
            self.values[3] / d,
            -self.values[1] / d,
            -self.values[2] / d,
            self.values[0] / d,
        ]);
    }

    /// Calculates the determinant
    pub fn det(&self) -> f64 {
        return self.values[0] * self.values[3] - self.values[1] * self.values[2];
    }

    /// Calculates the two eigenvalues sorting them from largest to smallest
    pub fn eigenvalues(&self) -> [f64; 2] {
        let d = (self.values[0] + self.values[3]) * (self.values[0] + self.values[3])
            - 4.0 * self.det();

        // Make sure it is not invalid
        if cfg!(debug_assertions) && d < 0.0 {
            panic!("The matrix is singular: {:?}", self);
        }

        let sqrt_d = d.sqrt();

        return [
            0.5 * ((self.values[0] + self.values[3]) + sqrt_d),
            0.5 * ((self.values[3] + self.values[3]) - sqrt_d),
        ];
    }

    /// Gets the scaling of the matrix in the x-direction
    pub fn get_scale_x(&self) -> f64 {
        return (self.values[0] * self.values[0] + self.values[1] * self.values[1]).sqrt();
    }

    /// Gets the scaling of the matrix in the y-direction
    pub fn get_scale_y(&self) -> f64 {
        return (self.values[2] * self.values[2] + self.values[3] * self.values[3]).sqrt();
    }
}

impl Neg for &Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        return Self::Output::new([
            -self.values[0],
            -self.values[1],
            -self.values[2],
            -self.values[3],
        ]);
    }
}

impl Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        return -&self;
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        return Self::Output::new([
            self.values[0] + rhs.values[0],
            self.values[1] + rhs.values[1],
            self.values[2] + rhs.values[2],
            self.values[3] + rhs.values[3],
        ]);
    }
}

impl Add<Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        return self + &rhs;
    }
}

impl Add<&Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        return &self + rhs;
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        return &self + &rhs;
    }
}

impl Sub<&Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        return Self::Output::new([
            self.values[0] - rhs.values[0],
            self.values[1] - rhs.values[1],
            self.values[2] - rhs.values[2],
            self.values[3] - rhs.values[3],
        ]);
    }
}

impl Sub<Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Self::Output {
        return self - &rhs;
    }
}

impl Sub<&Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        return &self - rhs;
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Self::Output {
        return &self - &rhs;
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        return Self::Output::new([
            self.values[0] * rhs.values[0] + self.values[1] * rhs.values[2],
            self.values[0] * rhs.values[1] + self.values[1] * rhs.values[3],
            self.values[1] * rhs.values[0] + self.values[3] * rhs.values[2],
            self.values[1] * rhs.values[1] + self.values[3] * rhs.values[3],
        ]);
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        return self * &rhs;
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        return &self * rhs;
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        return &self * &rhs;
    }
}

impl Mul<&Point> for &Matrix {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        return Self::Output::new(
            self.values[0] * rhs.x + self.values[1] * rhs.y,
            self.values[2] * rhs.x + self.values[3] * rhs.y,
        );
    }
}

impl Mul<Point> for &Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        return self * &rhs;
    }
}

impl Mul<&Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        return &self * rhs;
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        return &self * &rhs;
    }
}

impl Mul<&f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &f64) -> Self::Output {
        return Self::Output::new([
            self.values[0] * rhs,
            self.values[1] * rhs,
            self.values[2] * rhs,
            self.values[3] * rhs,
        ]);
    }
}

impl Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        return self * &rhs;
    }
}

impl Mul<&f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &f64) -> Self::Output {
        return &self * rhs;
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        return &self * &rhs;
    }
}
