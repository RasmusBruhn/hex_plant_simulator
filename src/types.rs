use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 2D point
#[derive(Clone, Copy, Debug)]
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
    pub fn norm_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }

    /// Calculates the norm of the point
    pub fn norm(&self) -> f64 {
        return self.norm_squared().sqrt();
    }

    /// Calculates the dot product between two points
    pub fn dot(&self, rhs: &Point) -> f64 {
        return self.x * rhs.x + self.y * rhs.y;
    }

    /// Calculates the cross product between two points
    pub fn cross(&self, rhs: &Point) -> f64 {
        return self.x * rhs.y - self.y * rhs.x;
    }

    /// Converts it to a size
    pub fn to_size(&self) -> Size {
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

/// A 2D size of width and height which are both non-negative
#[derive(Clone, Copy, Debug)]
pub struct Size {
    /// The width
    w: f64,
    /// The height
    h: f64,
}

impl Size {
    /// Creates a new size, if any of width or height are negative their signs are flipped
    ///
    /// # Parameters
    ///
    /// w: The width
    ///
    /// h: The height
    pub fn new(w: f64, h: f64) -> Self {
        return Self {
            w: w.abs(),
            h: h.abs(),
        };
    }

    /// Retrieves the width
    pub fn get_w(&self) -> f64 {
        return self.w;
    }

    /// Retrieves the height
    pub fn get_h(&self) -> f64 {
        return self.h;
    }

    /// Sets the width
    pub fn set_w(&mut self, w: f64) {
        self.w = w.abs();
    }

    /// Sets the height
    pub fn set_h(&mut self, h: f64) {
        self.h = h.abs();
    }
}

impl Mul<&f64> for &Size {
    type Output = Size;

    fn mul(self, rhs: &f64) -> Self::Output {
        let rhs = rhs.abs();
        let w = self.w * rhs;
        let h = self.h * rhs;
        return Self::Output { w, h };
    }
}

impl Mul<f64> for &Size {
    type Output = Size;

    fn mul(self, rhs: f64) -> Self::Output {
        return self * &rhs;
    }
}

impl Mul<&f64> for Size {
    type Output = Size;

    fn mul(self, rhs: &f64) -> Self::Output {
        return &self * rhs;
    }
}

impl Mul<f64> for Size {
    type Output = Size;

    fn mul(self, rhs: f64) -> Self::Output {
        return &self * &rhs;
    }
}

impl Add<&Size> for &Size {
    type Output = Size;

    fn add(self, rhs: &Size) -> Self::Output {
        let w = self.w + rhs.w;
        let h = self.h + rhs.h;

        return Self::Output { w, h };
    }
}

impl Add<Size> for &Size {
    type Output = Size;

    fn add(self, rhs: Size) -> Self::Output {
        return self + &rhs;
    }
}

impl Add<&Size> for Size {
    type Output = Size;

    fn add(self, rhs: &Size) -> Self::Output {
        return &self + rhs;
    }
}

impl Add<Size> for Size {
    type Output = Size;

    fn add(self, rhs: Size) -> Self::Output {
        return &self + &rhs;
    }
}

/// A 2D size of width and height which are both non-negative integers
#[derive(Clone, Copy, Debug)]
pub struct ISize {
    /// The width
    pub w: usize,
    /// The height
    pub h: usize,
}

impl ISize {
    /// Retrieves the total size (w * h)
    pub fn size(&self) -> usize {
        return self.w * self.h;
    }
}

impl Mul<usize> for &ISize {
    type Output = ISize;

    fn mul(self, rhs: usize) -> Self::Output {
        let w = self.w * rhs;
        let h = self.h * rhs;
        return Self::Output { w, h };
    }
}

impl Add<&ISize> for &ISize {
    type Output = ISize;

    fn add(self, rhs: &ISize) -> Self::Output {
        let w = self.w + rhs.w;
        let h = self.h + rhs.h;

        return Self::Output { w, h };
    }
}

/// A 2D index
#[derive(Clone, Copy, Debug)]
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

/// Defines a view of the map
#[derive(Clone, Copy, Debug)]
pub struct View {
    /// The center of the rectangle
    center: Point,
    /// The size of the rectangle
    size: Size,
}

impl View {
    /// Creates a new view
    ///
    /// # Parameters
    ///
    /// center: The center of the rectangle
    ///
    /// size: The size of the rectangle
    pub fn new(center: Point, size: Size) -> Self {
        return Self { center, size };
    }

    /// Retrieves the center
    pub fn get_center(&self) -> &Point {
        return &self.center;
    }

    /// Retrieves the size
    pub fn get_size(&self) -> &Size {
        return &self.size;
    }

    /// Checks if this view contains another view
    ///
    /// # Parameters
    ///
    /// other: The other view to check if is contained
    pub fn contains(&self, other: &View) -> bool {
        return self.center.x - self.size.w * 0.5 <= other.center.x - other.size.w * 0.5
            && self.center.y - self.size.h * 0.5 <= other.center.y - other.size.h * 0.5
            && self.center.x + self.size.w * 0.5 >= other.center.x + other.size.w * 0.5
            && self.center.y + self.size.h * 0.5 >= other.center.y + other.size.h * 0.5;
    }
}

/// Defines a 2x2 matrix
#[derive(Clone, Copy, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformTransform2D {
    /// The transform as a 4x4 matrix
    pub transform: [[f32; 4]; 4],
}

/// Describes a single RGBA color
#[derive(Copy, Clone, Debug)]
pub struct Color {
    /// The red component
    r: f64,
    /// The green component
    g: f64,
    /// The blue component
    b: f64,
    /// The alpha component
    a: f64,
}

impl Color {
    /// Constructs a new color from RGBA values
    ///
    /// # Parameters
    ///
    /// r: The red component
    ///
    /// g: The green component
    ///
    /// b: The blue component
    ///
    /// a: The alpha component
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        return Self { r, g, b, a };
    }

    /// Retrieves the red component of the color
    pub const fn get_r(&self) -> f64 {
        return self.r;
    }

    /// Retrieves the green component of the color
    pub const fn get_g(&self) -> f64 {
        return self.g;
    }

    /// Retrieves the blue component of the color
    pub const fn get_b(&self) -> f64 {
        return self.b;
    }

    /// Retrieves the alpha component of the color
    pub const fn get_a(&self) -> f64 {
        return self.a;
    }

    /// Constructs the shader compatible version of a color
    pub const fn get_data(&self) -> [f32; 4] {
        return [self.r as f32, self.g as f32, self.b as f32, self.a as f32];
    }

    pub const fn get_wgpu(&self) -> wgpu::Color {
        return wgpu::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        };
    }
}
