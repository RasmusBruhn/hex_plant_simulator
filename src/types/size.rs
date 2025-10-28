use std::ops::{Add, Mul};

/// A 2D size of width and height which are both non-negative
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub const fn new(w: f64, h: f64) -> Self {
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
