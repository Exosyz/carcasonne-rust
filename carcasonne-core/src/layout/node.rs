use crate::model::tile::Tile;

/// A node in the layout tree used for rendering.
///
/// Each `Node` represents a visual element or container. This enum enables
/// building a tree of elements that can be sized and rendered into a text-based `Frame`.
pub enum Node<'a> {
    /// Nothing to display, avoid re-render
    None,
    /// A single character.
    Char(char),
    /// A horizontal string of characters.
    Text(&'a str),
    /// A tile to render
    Tile(&'a Tile),
    /// A vertical container that stacks child nodes top-to-bottom.
    VerticalContainer(Vec<Box<Node<'a>>>),
    /// A horizontal container that lays out child nodes left-to-right.
    HorizontalContainer(Vec<Box<Node<'a>>>),
    /// A framed-drawn border around a single child node.
    Framed(Box<Node<'a>>),
}
