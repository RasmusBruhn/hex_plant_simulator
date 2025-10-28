use std::ops::{Add, Mul};

/// A 2D size of width and height which are both non-negative integers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
