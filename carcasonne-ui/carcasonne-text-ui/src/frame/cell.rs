use crate::color::ColorWrapper;
use carcasonne_core::layout::node::NodeTag;
use crossterm::queue;
use crossterm::style::{Attribute, Color, SetAttribute, SetBackgroundColor, SetForegroundColor};

/// Represents a single character cell in the frame buffer,
/// including its symbol and optional display attributes (tags).
#[derive(Clone)]
pub struct Cell {
    /// The character symbol displayed in this cell.
    pub symbol: char,

    /// A list of styling tags applied to this cell (e.g., color, bold).
    pub tags: Vec<CellTag>,
}

impl Cell {
    /// Creates a new cell with the given character symbol and no tags.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The character to display in the cell.
    pub fn new(symbol: char) -> Self {
        Self {
            symbol,
            tags: vec![],
        }
    }

    /// Creates a new cell with the given character symbol and styling tags.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The character to display in the cell.
    /// * `tags` - A vector of styling tags to apply.
    pub fn with_tags(symbol: char, tags: Vec<CellTag>) -> Self {
        Self { symbol, tags }
    }

    /// Applies all styling tags of the cell to the given output stream.
    ///
    /// This method uses `crossterm` commands to queue terminal style changes.
    ///
    /// # Panics
    ///
    /// Panics if applying any style command fails.
    ///
    /// # Arguments
    ///
    /// * `out` - The output stream implementing `std::io::Write` where style commands are sent.
    pub fn apply_tags(&self, out: &mut impl std::io::Write) {
        self.tags.iter().for_each(|tag| {
            match tag {
                CellTag::Underline => queue!(out, SetAttribute(Attribute::Underlined)),
                CellTag::Bold => queue!(out, SetAttribute(Attribute::Bold)),
                CellTag::Foreground(color) => queue!(out, SetForegroundColor(*color)),
                CellTag::Background(color) => queue!(out, SetBackgroundColor(*color)),
            }
            .expect("Fail to apply style command")
        });
    }
}

/// Represents different kinds of styling tags that can be applied to a `Cell`.
#[derive(Clone, PartialEq)]
pub enum CellTag {
    /// Foreground (text) color.
    Foreground(Color),
    /// Background color.
    Background(Color),
    /// Underline attribute.
    Underline,
    /// Bold attribute.
    Bold,
}

impl From<&NodeTag> for CellTag {
    /// Converts a high-level `NodeTag` into a corresponding `CellTag`
    /// suitable for terminal rendering.
    ///
    /// Colors are converted via `ColorWrapper` to `crossterm` `Color`.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the source `NodeTag`.
    ///
    /// # Returns
    ///
    /// A `CellTag` with equivalent styling information.
    fn from(value: &NodeTag) -> Self {
        match value {
            NodeTag::Underline => CellTag::Underline,
            NodeTag::Bold => CellTag::Bold,
            NodeTag::Foreground(color) => CellTag::Foreground(ColorWrapper(color.clone()).into()),
            NodeTag::Background(color) => CellTag::Background(ColorWrapper(color.clone()).into()),
        }
    }
}
