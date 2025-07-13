//! Provides a renderer for `Node` elements that targets a character-based `Frame`.
//!
//! This module defines how to visually render UI nodes (like: a text, tiles),
//! containers, and framed boxes using a terminal-compatible grid layout.
//!
//! Rendering is driven by the `Renderable` trait, which is implemented for
//! each `Node` variant.
//!
//! # Layout Model
//! - `Char`: 1x1 symbol
//! - `Text`: horizontal 1-row string
//! - `Tile`: square of size `TILE_SIZE` (e.g., 5x5)
//! - `Framed`: wraps any node in a border with padding
//! - `VerticalContainer`: stacked child nodes
//! - `HorizontalContainer`: inline child nodes
//!
//! Borders use `CharDrawing` characters for visual clarity.
use crate::char_drawing::CharDrawing;
use crate::frame::Frame;
use crate::renderable::Renderable;
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;
use carcasonne_core::model::tile::Tile;

/// The default width and height (in characters) used to render a `Tile` node.
///
/// This constant determines the grid size for all tiles. Currently fixed to `5`,
/// rendering tiles as 5x5 character matrices.
pub const TILE_SIZE: usize = 5;

/// Stateless helper for rendering `Node` elements into a `Frame`.
///
/// `NodeRenderer` encapsulates all rendering logic for node variants,
/// separating layout behavior from drawing behavior.
/// It is only used internally via the `Renderable` trait.
struct NodeRenderer;

impl NodeRenderer {
    /// Renders a single character at the specified position in the frame.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer where the character will be placed.
    /// * `point` - The coordinates where the character will be drawn.
    /// * `char` - The character to render.
    fn render_char(frame: &mut Frame, point: Point, char: &char) {
        frame.char_simple(point, *char);
    }

    /// Renders a string of characters horizontally starting at the given point.
    ///
    /// Each character is placed one position to the right of the previous.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer.
    /// * `point` - The starting position for the first character.
    /// * `str` - The string to render.
    fn render_text(frame: &mut Frame, point: Point, str: &str) {
        str.chars()
            .enumerate()
            .for_each(|(i, c)| frame.char_simple(point + Point::new(i, 0), c));
    }

    /// Renders a tile using a square grid of placeholder characters.
    ///
    /// This is a stub implementation: the tile is filled with `.` characters
    /// and does not yet reflect actual tile features.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer.
    /// * `point` - The top-left corner where the tile will be drawn.
    /// * `tile` - The tile to render
    fn render_tile(frame: &mut Frame, point: Point, _: &Tile) {
        let chars = vec![vec!['.'; TILE_SIZE]; TILE_SIZE];

        for (i, row) in chars.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                frame.char_simple(point + Point::new(i, j), *c)
            }
        }
    }

    /// Renders a framed box around a child node, using `+`, `-`, and `|` characters.
    ///
    /// Adds 1-character padding around the inner node.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer.
    /// * `point` - The top-left position of the outer frame.
    /// * `elem` - The inner node to render inside the frame.
    fn render_framed(frame: &mut Frame, point: Point, elem: &Node) {
        let inner_size = elem.size();
        let outer_size = inner_size + Size::new(2, 2);

        let x0 = point.x;
        let y0 = point.y;
        let x1 = x0 + outer_size.width - 1;
        let y1 = y0 + outer_size.height - 1;

        // Top border
        frame.char_simple(Point::new(x0, y0), CharDrawing::CornerTopLeft.into());
        for x in (x0 + 1)..x1 {
            frame.char_simple(Point::new(x, y0), CharDrawing::Horizontal.into());
        }
        frame.char_simple(Point::new(x1, y0), CharDrawing::CornerTopRight.into());

        // Middle rows
        for y in (y0 + 1)..y1 {
            frame.char_simple(Point::new(x0, y), CharDrawing::Vertical.into());
            frame.char_simple(Point::new(x1, y), CharDrawing::Vertical.into());
        }

        // Bottom border
        frame.char_simple(Point::new(x0, y1), CharDrawing::CornerBottomLeft.into());
        for x in (x0 + 1)..x1 {
            frame.char_simple(Point::new(x, y1), CharDrawing::Horizontal.into());
        }
        frame.char_simple(Point::new(x1, y1), CharDrawing::CornerBottomRight.into());

        // Render the inner element inside the border
        elem.render(frame, point + Point::new(1, 1));
    }

    /// Renders a vertical container by stacking its child nodes top-to-bottom.
    ///
    /// Each child node is placed immediately below the previous one.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer.
    /// * `point` - The top-left starting point of the container.
    /// * `elems` - A list of nodes to render vertically.
    fn vertical_container(frame: &mut Frame, point: Point, elems: &Vec<Box<Node>>) {
        let mut current_y = point.y;
        for elem in elems {
            elem.render(frame, Point::new(point.x, current_y));
            current_y += elem.size().height;
        }
    }

    /// Renders a horizontal container by laying out child nodes left-to-right.
    ///
    /// Each child node is placed immediately to the right of the previous one.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer.
    /// * `point` - The top-left starting point of the container.
    /// * `elems` - A list of nodes to render horizontally.
    fn horizontal_container(frame: &mut Frame, point: Point, elems: &Vec<Box<Node>>) {
        let mut current_x = point.x;
        for elem in elems {
            elem.render(frame, Point::new(current_x, point.y));
            current_x += elem.size().width;
        }
    }
}

impl<'a> Renderable for Node<'a> {
    /// Renders a `Node` into a `Frame` starting from the specified top-left `Point`.
    ///
    /// Each node type determines how its contents are laid out and drawn.
    /// This function delegates the actual rendering to the internal `NodeRenderer`.
    fn render(&self, frame: &mut Frame, point: Point) {
        match self {
            Node::None => {}
            Node::Char(char) => NodeRenderer::render_char(frame, point, char),
            Node::Text(str) => NodeRenderer::render_text(frame, point, str),
            Node::Tile(tile) => NodeRenderer::render_tile(frame, point, tile),
            Node::VerticalContainer(elems) => NodeRenderer::vertical_container(frame, point, elems),
            Node::HorizontalContainer(elems) => {
                NodeRenderer::horizontal_container(frame, point, elems)
            }
            Node::Framed(elem) => NodeRenderer::render_framed(frame, point, elem),
        }
    }

    /// Returns the space required to render the node and its children.
    ///
    /// This method is used for layout computation prior to rendering. It returns a `Size`
    /// that represents the width and height of the node’s bounding box.
    fn size(&self) -> Size {
        match self {
            Node::None => Size::new(0, 0),
            Node::Char(_) => Size::new(1, 1),
            Node::Text(str) => Size::new(str.len(), 1),
            Node::Tile(_) => Size::new(TILE_SIZE, TILE_SIZE),
            Node::VerticalContainer(elems) => elems
                .iter()
                .map(|e| e.size())
                .fold(Size::new(0, 0), |acc, s| {
                    Size::new(acc.width.max(s.width), acc.height + s.height)
                }),

            Node::HorizontalContainer(elems) => elems
                .iter()
                .map(|e| e.size())
                .fold(Size::new(0, 0), |acc, s| {
                    Size::new(acc.width + s.width, acc.height.max(s.height))
                }),
            Node::Framed(elem) => elem.size() + Size::new(2, 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use carcasonne_core::layout::point::Point;
    use carcasonne_core::layout::size::Size;
    use carcasonne_core::model::tile::Tile;

    // Helper Node constructors for tests
    fn char_node(c: char) -> Node<'static> {
        Node::Char(c)
    }
    fn text_node(s: &'static str) -> Node<'static> {
        Node::Text(s)
    }
    static TILE_INSTANCE: Tile = Tile {
        tile_features: Vec::new(),
        tile_extension: None,
    };
    fn tile_node() -> Node<'static> {
        Node::Tile(&TILE_INSTANCE)
    }
    fn none_node() -> Node<'static> {
        Node::None
    }

    #[test]
    fn test_size_char() {
        let n = char_node('X');
        assert_eq!(n.size(), Size::new(1, 1));
    }

    #[test]
    fn test_size_text() {
        let n = text_node("Hello");
        assert_eq!(n.size(), Size::new(5, 1));
    }

    #[test]
    fn test_size_tile() {
        let n = tile_node();
        assert_eq!(n.size(), Size::new(TILE_SIZE, TILE_SIZE));
    }

    #[test]
    fn test_size_none() {
        let n = none_node();
        assert_eq!(n.size(), Size::new(0, 0));
    }

    #[test]
    fn test_size_vertical_container() {
        let v = Node::VerticalContainer(vec![
            Box::new(text_node("Hi")),
            Box::new(char_node('X')),
            Box::new(text_node("World")),
        ]);
        // width = max(2,1,5) = 5, height = 1+1+1 = 3
        assert_eq!(v.size(), Size::new(5, 3));
    }

    #[test]
    fn test_size_horizontal_container() {
        let h = Node::HorizontalContainer(vec![
            Box::new(text_node("Hi")),
            Box::new(char_node('X')),
            Box::new(text_node("World")),
        ]);
        // width = 2 + 1 + 5 = 8, height = max(1,1,1) = 1
        assert_eq!(h.size(), Size::new(8, 1));
    }

    #[test]
    fn test_size_framed() {
        let inner = text_node("Hi");
        let framed = Node::Framed(Box::new(inner));
        // inner size = (2,1) + (2,2) padding = (4,3)
        assert_eq!(framed.size(), Size::new(4, 3));
    }

    #[test]
    fn test_render_char() {
        let mut frame = Frame::new(Size::new(3, 3));
        let n = char_node('Z');
        n.render(&mut frame, Point::new(1, 1));
        assert_eq!(frame.cells[1][1].symbol, 'Z');
    }

    #[test]
    fn test_render_text() {
        let mut frame = Frame::new(Size::new(10, 2));
        let n = text_node("ABC");
        n.render(&mut frame, Point::new(2, 1));
        assert_eq!(frame.cells[1][2].symbol, 'A');
        assert_eq!(frame.cells[1][3].symbol, 'B');
        assert_eq!(frame.cells[1][4].symbol, 'C');
    }

    #[test]
    fn test_render_vertical_container() {
        let mut frame = Frame::new(Size::new(10, 5));
        let v = Node::VerticalContainer(vec![Box::new(text_node("A")), Box::new(text_node("BC"))]);
        v.render(&mut frame, Point::new(0, 0));
        assert_eq!(frame.cells[0][0].symbol, 'A');
        assert_eq!(frame.cells[1][0].symbol, 'B');
        assert_eq!(frame.cells[1][1].symbol, 'C');
    }

    #[test]
    fn test_render_horizontal_container() {
        let mut frame = Frame::new(Size::new(10, 3));
        let h =
            Node::HorizontalContainer(vec![Box::new(text_node("A")), Box::new(text_node("BC"))]);
        h.render(&mut frame, Point::new(0, 0));
        assert_eq!(frame.cells[0][0].symbol, 'A');
        assert_eq!(frame.cells[0][1].symbol, 'B');
        assert_eq!(frame.cells[0][2].symbol, 'C');
    }

    #[test]
    fn test_render_framed() {
        let mut frame = Frame::new(Size::new(10, 5));
        let inner = text_node("Hi");
        let framed = Node::Framed(Box::new(inner));
        framed.render(&mut frame, Point::new(1, 1));

        // Check corners (assuming CharDrawing uses + - | as in the example)
        assert_eq!(frame.cells[1][1].symbol, CharDrawing::CornerTopLeft.into());
        assert_eq!(frame.cells[1][4].symbol, CharDrawing::CornerTopRight.into());
        assert_eq!(
            frame.cells[3][1].symbol,
            CharDrawing::CornerBottomLeft.into()
        );
        assert_eq!(
            frame.cells[3][4].symbol,
            CharDrawing::CornerBottomRight.into()
        );

        // Check inner text position (offset by +1,+1 inside frame)
        assert_eq!(frame.cells[2][2].symbol, 'H');
        assert_eq!(frame.cells[2][3].symbol, 'i');
    }
}
