use super::{Point, Size};

/// Defines a view of the map
#[derive(Clone, Copy, Debug, PartialEq)]
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
        return self.center.x - self.size.get_w() * 0.5
            <= other.center.x - other.size.get_w() * 0.5
            && self.center.y - self.size.get_h() * 0.5
                <= other.center.y - other.size.get_h() * 0.5
            && self.center.x + self.size.get_w() * 0.5
                >= other.center.x + other.size.get_w() * 0.5
            && self.center.y + self.size.get_h() * 0.5
                >= other.center.y + other.size.get_h() * 0.5;
    }
}
