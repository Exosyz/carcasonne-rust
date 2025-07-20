use crate::color::Color;
use crate::layout::point::Point;
use crate::model::tile::Tile;

/// Represents an element in the layout tree used for rendering.
///
/// A `Node` can be a primitive visual element (character, text, tile),
/// a container (vertical or horizontal),
/// or a decorated element such as framed or styled text.
/// The layout tree enables recursive composition and rendering of UI components
/// into a text-based frame.
///
/// The lifetime `a` allows nodes to borrow string slices or tile references
/// for efficient rendering.
pub enum Node<'a> {
    /// Represents absence of content; no rendering performed.
    None,

    /// A single character element.
    Char(char),

    /// A single character with styling tags.
    RichChar(char, Vec<NodeTag>),

    /// A horizontal string of characters.
    Text(&'a str),

    /// A horizontal string with styling tags applied.
    RichText(&'a str, Vec<NodeTag>),

    /// A reference to a tile to render.
    Tile(&'a Tile),

    /// A vertical container stacking child nodes top-to-bottom.
    VerticalContainer(Vec<Node<'a>>),

    /// A horizontal container laying out child nodes left-to-right.
    HorizontalContainer(Vec<Node<'a>>),

    /// A single child node drawn with a border frame around it.
    Framed(Box<Node<'a>>),

    /// A menu list with options (string slices) and a selected index.
    Menu(Vec<&'a str>, usize),

    /// An input field displaying a string and a cursor position.
    Input(&'a str, Point),

    /// An error message to display.
    Error(&'a str),
}

/// Tags used to modify the appearance of `Node` elements.
///
/// Tags represent text styles or color attributes that affect rendering.
/// These are applied to `RichChar` and `RichText` nodes.
pub enum NodeTag {
    /// Underline the text.
    Underline,

    /// Bold the text.
    Bold,

    /// Set the foreground color.
    Foreground(Color),

    /// Set the background color.
    Background(Color),
}
