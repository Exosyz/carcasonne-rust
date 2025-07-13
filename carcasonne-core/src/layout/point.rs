use std::ops::Add;

/// A 2D point with non-negative integer coordinates.
///
/// This struct is commonly used for positioning elements in a grid or layout.
/// Both `x` and `y` are `usize`, making it suitable for indexing 2D arrays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    /// The horizontal coordinate.
    pub x: usize,
    /// The vertical coordinate.
    pub y: usize,
}

impl Point {
    /// Creates a new `Point` with the given coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasonne_core::layout::point::Point;
    ///
    /// let p = Point::new(3, 5);
    /// assert_eq!(p.x, 3);
    /// assert_eq!(p.y, 5);
    /// ```
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Returns the origin point `(0, 0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasonne_core::layout::point::Point;
    ///
    /// let origin = Point::zero();
    /// assert_eq!(origin, Point::new(0, 0));
    /// ```
    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    /// Adds two `Point`s by summing their `x` and `y` coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasonne_core::layout::point::Point;
    ///
    /// let a = Point::new(1, 2);
    /// let b = Point::new(3, 4);
    /// let c = a + b;
    ///
    /// assert_eq!(c, Point::new(4, 6));
    /// ```
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        assert_eq!(Point::new(1, 2) + Point::new(3, 4), Point::new(4, 6));
    }

    #[test]
    fn test_point_zero() {
        assert_eq!(Point::zero(), Point::new(0, 0));
    }
}
