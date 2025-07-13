use std::iter::Sum;
use std::ops::Add;

/// A 2D size with width and height, both represented as `usize`.
///
/// This struct is typically used to represent dimensions of a rectangular area,
/// such as screen regions, layouts, or tiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    /// The width component of the size.
    pub width: usize,

    /// The height component of the size.
    pub height: usize,
}

impl Size {
    /// Creates a new `Size` with the given width and height.
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasonne_core::layout::size::Size;
    /// let s = Size::new(10, 5);
    /// assert_eq!(s.width, 10);
    /// assert_eq!(s.height, 5);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Add for Size {
    type Output = Size;
    /// Adds two `Size` values by summing their width and height independently.
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasonne_core::layout::size::Size;
    /// let a = Size::new(3, 4);
    /// let b = Size::new(5, 6);
    /// let c = a + b;
    /// assert_eq!(c, Size::new(8, 10));
    /// ```
    fn add(self, other: Size) -> Size {
        Size {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl Sum for Size {
    /// Sums an iterator of `Size` values by performing pairwise addition.
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasonne_core::layout::size::Size;
    /// let sizes = vec![
    ///     Size::new(1, 2),
    ///     Size::new(3, 4),
    ///     Size::new(5, 6),
    /// ];
    /// let total: Size = sizes.into_iter().sum();
    /// assert_eq!(total, Size::new(9, 12));
    /// ```
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Size::new(0, 0), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::Size;

    #[test]
    fn creates_size_correctly() {
        let size = Size::new(10, 5);
        assert_eq!(size.width, 10);
        assert_eq!(size.height, 5);
    }

    #[test]
    fn adds_two_sizes() {
        let a = Size::new(3, 4);
        let b = Size::new(5, 6);
        let c = a + b;
        assert_eq!(c, Size::new(8, 10));
    }

    #[test]
    fn sum_iterator_of_sizes() {
        let sizes = vec![Size::new(1, 2), Size::new(3, 4), Size::new(5, 6)];
        let total: Size = sizes.into_iter().sum();
        assert_eq!(total, Size::new(9, 12));
    }

    #[test]
    fn zero_sum_returns_zero_size() {
        let sizes: Vec<Size> = vec![];
        let total: Size = sizes.into_iter().sum();
        assert_eq!(total, Size::new(0, 0));
    }

    #[test]
    fn equality_and_copy_derive_work() {
        let a = Size::new(7, 8);
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn debug_output_is_correct() {
        let s = Size::new(2, 3);
        let dbg_str = format!("{:?}", s);
        assert!(dbg_str.contains("Size"));
        assert!(dbg_str.contains("2"));
        assert!(dbg_str.contains("3"));
    }
}
