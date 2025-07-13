/// Represents the different characters used to draw box borders.
///
/// These include corners and line segments for horizontal and vertical edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharDrawing {
    /// Represents no character (empty space).
    None,
    /// Top-left corner of a box.
    CornerTopLeft,
    /// Top-right corner of a box.
    CornerTopRight,
    /// Bottom-left corner of a box.
    CornerBottomLeft,
    /// Bottom-right corner of a box.
    CornerBottomRight,
    /// Horizontal line segment.
    Horizontal,
    /// Vertical line segment.
    Vertical,
}

impl From<CharDrawing> for char {
    /// Converts a `CharDrawing` variant into its corresponding Unicode character.
    ///
    /// # Returns
    ///
    /// * A `char` representing the box drawing element.
    fn from(value: CharDrawing) -> Self {
        match value {
            CharDrawing::None => ' ',
            CharDrawing::CornerTopLeft => '┌',
            CharDrawing::CornerTopRight => '┐',
            CharDrawing::CornerBottomLeft => '└',
            CharDrawing::CornerBottomRight => '┘',
            CharDrawing::Horizontal => '─',
            CharDrawing::Vertical => '│',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CharDrawing;

    #[test]
    fn test_char_drawing_into_char() {
        let cases = [
            (CharDrawing::None, ' '),
            (CharDrawing::CornerTopLeft, '┌'),
            (CharDrawing::CornerTopRight, '┐'),
            (CharDrawing::CornerBottomLeft, '└'),
            (CharDrawing::CornerBottomRight, '┘'),
            (CharDrawing::Horizontal, '─'),
            (CharDrawing::Vertical, '│'),
        ];

        for (input, expected) in cases {
            let c: char = input.into();
            assert_eq!(
                c, expected,
                "CharDrawing::{:?} should convert to '{}'",
                input, expected
            );
        }
    }
}
