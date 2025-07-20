/// Represents a basic color used for rendering cell foreground and background.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    /// Black color (default background).
    Black,
    /// White color (default foreground).
    White,
    /// Red color.
    Red,
    /// Blue color.
    Blue,
    /// Yellow color.
    Yellow,
    /// Green color
    Green,
}
