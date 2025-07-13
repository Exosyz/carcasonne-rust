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
}

impl From<&Color> for crossterm::style::Color {
    /// Converts a reference to a custom `Color` enum into a `crossterm` terminal color.
    ///
    /// # Parameters
    ///
    /// - `value`: A reference to the `Color` enum variant.
    ///
    /// # Returns
    ///
    /// * Corresponding `crossterm::style::Color` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasonne_text_ui::color::Color;
    ///
    /// let my_color = Color::Red;
    /// let term_color: crossterm::style::Color = (&my_color).into();
    /// assert_eq!(term_color, crossterm::style::Color::Red);
    /// ```
    fn from(value: &Color) -> Self {
        match value {
            Color::Black => Self::Black,
            Color::White => Self::White,
            Color::Red => Self::Red,
            Color::Blue => Self::Blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_color_conversion {
    ($($variant:ident => $fn_name:ident),*) => {
        $(
            #[test]
            fn $fn_name() {
                let c = Color::$variant;
                let ct_color: crossterm::style::Color = (&c).into();
                assert_eq!(ct_color, crossterm::style::Color::$variant);
            }
        )*
    };
}

    test_color_conversion! {
        Black => test_black,
        White => test_white,
        Red => test_red,
        Blue => test_blue
    }
}
