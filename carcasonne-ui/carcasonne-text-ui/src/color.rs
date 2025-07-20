use carcasonne_core::color::Color;

/// Wrapper types around `Color` to provide conversion implementations.
pub struct ColorWrapper(pub Color);

impl From<ColorWrapper> for Color {
    fn from(value: ColorWrapper) -> Self {
        value.0.clone()
    }
}

impl From<ColorWrapper> for crossterm::style::Color {
    /// Converts a `ColorWrapper` into a corresponding `crossterm::style::Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasonne_core::color::Color;
    /// use carcasonne_text_ui::color::ColorWrapper;
    ///
    /// let my_color = Color::Red;
    /// let term_color: crossterm::style::Color = ColorWrapper(my_color).into();
    /// assert_eq!(term_color, crossterm::style::Color::Red);
    /// ```
    fn from(value: ColorWrapper) -> Self {
        match value.into() {
            Color::Black => Self::Black,
            Color::White => Self::White,
            Color::Red => Self::Red,
            Color::Blue => Self::Blue,
            Color::Yellow => Self::Yellow,
            Color::Green => Self::Green,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use carcasonne_core::color::Color;

    macro_rules! test_color_conversion {
        ($($variant:ident => $fn_name:ident),*) => {
            $(
                #[test]
                fn $fn_name() {
                    let c = Color::$variant;
                    let ct_color: crossterm::style::Color = ColorWrapper(c).into();
                    assert_eq!(ct_color, crossterm::style::Color::$variant);
                }
            )*
        };
    }

    test_color_conversion! {
        Black => test_black,
        White => test_white,
        Red => test_red,
        Blue => test_blue,
        Yellow => test_yellow,
        Green => test_green
    }
}
