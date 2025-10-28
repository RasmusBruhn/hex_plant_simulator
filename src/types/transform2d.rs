use std::ops::Mul;

use super::{Matrix, Point};

/// A 2D transform which acts on Point types, including rotation, scaling and translation.
///
/// The operation is y = r * (x - c) where
///
/// y: The output point
///
/// x: The input point
///
/// c: The center point
///
/// r: The 2x2 center_transform matrix
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform2D {
    /// The transform to apply relative to the center
    pub transform: Matrix,
    /// The center of the coordinate system
    pub center: Point,
}

impl Transform2D {
    /// Creates the identity operation
    pub fn identity() -> Self {
        let transform = Matrix::new([1.0, 0.0, 0.0, 1.0]);
        let center = Point::new(0.0, 0.0);

        return Self { transform, center };
    }

    /// Rotate around origo
    ///
    /// # Parameters
    ///
    /// angle: The angle to rotate
    pub fn rotation(angle: f64) -> Self {
        let transform = Matrix::new([angle.cos(), -angle.sin(), angle.sin(), angle.cos()]);
        let center = Point::new(0.0, 0.0);

        return Self { transform, center };
    }

    /// Applies the transformation at a defined location
    ///
    /// # Parameters
    ///
    /// rotation_center: The center of the rotation
    pub fn transform_at(&self, rotation_center: &Point) -> Self {
        let transform = self.transform;
        let center = self.center + rotation_center - self.transform * rotation_center;

        return Self { transform, center };
    }

    /// Scale at origo
    ///
    /// # Parameters
    ///
    /// scale: The ratio to scale x and y with
    pub fn scale(scale: &Point) -> Self {
        let transform = Matrix::new([scale.x, 0.0, 0.0, scale.y]);
        let center = Point::new(0.0, 0.0);

        return Self { transform, center };
    }

    /// Translates a point
    ///
    /// # Parameters
    ///
    /// offset: The amount to translate
    pub fn translate(offset: &Point) -> Self {
        let transform = Matrix::new([1.0, 0.0, 0.0, 1.0]);
        let center = *offset;

        return Self { transform, center };
    }

    /// Retrieves the inverse transform
    pub fn inv(&self) -> Self {
        let transform = self.transform.inv();
        let center = -transform * self.center;

        return Self { transform, center };
    }

    /// Retrieves the offset
    pub fn get_center(&self) -> &Point {
        return &self.center;
    }

    /// Retrieves the center transform
    pub fn get_center_transform(&self) -> &Matrix {
        return &self.transform;
    }

    /// Gets the scaling in the x-direction
    pub fn get_scaling_x(&self) -> f64 {
        return self.transform.get_scale_x();
    }

    /// Gets the scaling in the y-direction
    pub fn get_scaling_y(&self) -> f64 {
        return self.transform.get_scale_y();
    }

    /// Retrieves the data for the gpu
    pub fn get_data(&self) -> UniformTransform2D {
        return UniformTransform2D {
            transform: [
                [
                    self.transform.values[0] as f32,
                    self.transform.values[2] as f32,
                    0.0,
                    0.0,
                ],
                [
                    self.transform.values[1] as f32,
                    self.transform.values[3] as f32,
                    0.0,
                    0.0,
                ],
                [0.0, 0.0, 1.0, 0.0],
                [self.center.x as f32, self.center.y as f32, 0.0, 1.0],
            ],
        };
    }
}

impl Mul<&Transform2D> for &Transform2D {
    type Output = Transform2D;

    fn mul(self, rhs: &Transform2D) -> Self::Output {
        let transform = self.transform * rhs.transform;
        let center = self.transform * rhs.center + self.center;

        return Self::Output { transform, center };
    }
}

impl Mul<Transform2D> for &Transform2D {
    type Output = Transform2D;

    fn mul(self, rhs: Transform2D) -> Self::Output {
        return self * &rhs;
    }
}

impl Mul<&Transform2D> for Transform2D {
    type Output = Transform2D;

    fn mul(self, rhs: &Transform2D) -> Self::Output {
        return &self * rhs;
    }
}

impl Mul<Transform2D> for Transform2D {
    type Output = Transform2D;

    fn mul(self, rhs: Transform2D) -> Self::Output {
        return &self * &rhs;
    }
}

impl Mul<&Point> for &Transform2D {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        return self.transform * rhs + self.center;
    }
}

impl Mul<Point> for &Transform2D {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        return self * &rhs;
    }
}

impl Mul<&Point> for Transform2D {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        return &self * rhs;
    }
}

impl Mul<Point> for Transform2D {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        return &self * &rhs;
    }
}

/// A representation of the Transform2D class able to be shared with wgsl
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformTransform2D {
    /// The transform as a 4x4 matrix
    pub transform: [[f32; 4]; 4],
}
