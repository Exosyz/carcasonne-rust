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
use crate::renderable::tile_renderer::TileRenderer;
use crate::renderable::Renderable;
use carcasonne_core::color::Color;
use carcasonne_core::layout::node::NodeTag::{Bold, Foreground, Underline};
use carcasonne_core::layout::node::{Node, NodeTag};
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;
use carcasonne_core::model::rotation::Rotation;
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
    fn char(frame: &mut Frame, point: Point, char: char) {
        frame.char_simple(point, char);
    }

    /// Renders a character on the given frame at a specific point, applying
    /// style metadata from the provided tags.
    ///
    /// This function extracts visual information (such as foreground and background
    /// colors) from the given `tags` and applies it when rendering the character
    /// onto the `frame`.
    ///
    /// # Arguments
    ///
    /// * `frame` - A mutable reference to the rendering `Frame`.
    /// * `point` - The position on the frame where the character will be drawn.
    /// * `char` - The character to render.
    /// * `tags` - A collection of `NodeTag` metadata used to influence visual style.
    fn rich_char(frame: &mut Frame, point: Point, char: char, tags: Vec<NodeTag>) {
        frame.char(point, char, &tags);
    }

    /// Renders a string of characters horizontally starting at the given point.
    ///
    /// Each character is placed one position to the right of the previous.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer.
    /// * `point` - The starting position for the first character.
    /// * `str` - The string to render.
    fn text(frame: &mut Frame, point: Point, str: &str) {
        str.chars()
            .enumerate()
            .for_each(|(i, c)| frame.char_simple(point + Point::new(i, 0), c));
    }

    /// Renders a string on the given frame starting at a specific point,
    /// applying style metadata from the provided tags to each character.
    ///
    /// This function iterates over each character in the string and renders it
    /// horizontally on the frame, starting at the given `point`. Style information
    /// such as color can be derived from the `tags`.
    ///
    /// # Arguments
    ///
    /// * `frame` - A mutable reference to the rendering [`Frame`] where the text will be drawn.
    /// * `point` - The top-left starting position for the string.
    /// * `str` - The string to be rendered.
    /// * `tags` - A list of [`NodeTag`] used to determine visual properties such as
    ///   foreground and background color.
    ///
    /// # Notes
    ///
    /// * Characters are placed horizontally with no line wrapping.
    /// * You can extend this for rich text layouts by supporting bold, italic.
    fn rich_text(frame: &mut Frame, point: Point, str: &str, tags: Vec<NodeTag>) {
        str.chars()
            .enumerate()
            .for_each(|(i, c)| frame.char(point + Point::new(i, 0), c, &tags));
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
    fn tile(frame: &mut Frame, point: Point, tile: &Tile, rotation: &Rotation) {
        let chars = TileRenderer::tile(TILE_SIZE, tile, rotation);

        chars.iter().enumerate().for_each(|(j, row)| {
            row.iter()
                .enumerate()
                .for_each(|(i, c)| frame.char_simple(point + Point::new(i, j), *c))
        });
    }

    /// Renders a framed box around a child node, using `+`, `-`, and `|` characters.
    ///
    /// Adds 1-character padding around the inner node.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer.
    /// * `point` - The top-left position of the outer frame.
    /// * `elem` - The inner node to render inside the frame.
    fn framed(frame: &mut Frame, point: Point, elem: Node) {
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
    fn vertical_container(frame: &mut Frame, point: Point, elems: Vec<Node>) {
        let mut current_y = point.y;
        for elem in elems {
            let size = elem.size();
            elem.render(frame, Point::new(point.x, current_y));
            current_y += size.height;
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
    fn horizontal_container(frame: &mut Frame, point: Point, elems: Vec<Node>) {
        let mut current_x = point.x;
        for elem in elems {
            let size = elem.size();
            elem.render(frame, Point::new(current_x, point.y));
            current_x += size.width;
        }
    }

    /// Renders a vertical menu at a specified position on the frame, highlighting the selected item.
    ///
    /// Each menu item is displayed with a radio marker indicating whether it is selected or not.
    /// The selected item is underlined.
    ///
    /// # Arguments
    ///
    /// * `frame` - A mutable reference to the [`Frame`] on which the menu will be drawn.
    /// * `point` - The top-left starting position where the menu will be placed.
    /// * `elems` - A list of strings representing each menu entry.
    /// * `selected_index` - The index of the currently selected menu item, which will be underlined.
    ///
    /// # Notes
    ///
    /// - This function uses custom `CharDrawing` symbols to indicate selection.
    /// - The menu layout is built using a horizontal container for each line,
    ///   and all lines are stacked vertically.
    fn menu(frame: &mut Frame, point: Point, elems: Vec<&str>, selected_index: usize) {
        let built_menu = elems
            .iter()
            .enumerate()
            .map(|(i, s)| {
                Node::HorizontalContainer(vec![
                    Node::Char(if i == selected_index {
                        CharDrawing::SelectedRadio.into()
                    } else {
                        CharDrawing::Radio.into()
                    }),
                    Node::Char(CharDrawing::None.into()),
                    if i == selected_index {
                        Node::RichText(s, vec![Underline])
                    } else {
                        Node::Text(s)
                    },
                ])
            })
            .collect();

        Self::vertical_container(frame, point, built_menu)
    }

    /// Renders an input field with a visible cursor at the specified position.
    ///
    /// This function sets the cursor location on the frame and then displays the provided string
    /// at the given point.
    ///
    /// # Arguments
    ///
    /// * `frame` - A mutable reference to the [`Frame`] for rendering.
    /// * `point` - The top-left origin of the input field.
    /// * `str` - The current value of the input field to be displayed.
    /// * `position` - The position of the cursor relative to the `point`.
    ///
    /// # Notes
    ///
    /// - The cursor position is calculated as `point + position`.
    /// - This function does not handle editing input, only rendering the current state.
    fn input(frame: &mut Frame, point: Point, str: &str, position: Point) {
        frame.set_cursor(Some(point + position));
        Self::text(frame, point, str);
    }

    /// Renders an error message with a red foreground and bold style at a given position.
    ///
    /// Used to inform the user of validation or system errors in the UI.
    ///
    /// # Arguments
    ///
    /// * `frame` - A mutable reference to the [`Frame`] for rendering.
    /// * `point` - The position where the error message should appear.
    /// * `str` - The error message text to display.
    ///
    /// # Styling
    ///
    /// The message will be bold and red using the [`Bold`] and [`NodeTag::Foreground(Color::Red)`] tags.
    fn error(frame: &mut Frame, point: Point, str: &str) {
        Self::rich_text(frame, point, str, vec![Bold, Foreground(Color::Red)]);
    }

    fn multiline_text(frame: &mut Frame, point: Point, str: &str) {
        Self::vertical_container(frame, point, str.lines().map(Node::Text).collect())
    }

    fn multiline_rich_text(frame: &mut Frame, point: Point, str: &str, tags: Vec<NodeTag>) {
        Self::vertical_container(
            frame,
            point,
            str.lines()
                .map(|s| Node::RichText(s, tags.clone()))
                .collect(),
        )
    }
}

impl<'a> Renderable for Node<'a> {
    /// Renders a `Node` into a `Frame` starting from the specified top-left `Point`.
    ///
    /// Each node type determines how its contents are laid out and drawn.
    /// This function delegates the actual rendering to the internal `NodeRenderer`.
    fn render(self, frame: &mut Frame, point: Point) {
        match self {
            Node::None => {}
            Node::Char(char) => NodeRenderer::char(frame, point, char),
            Node::RichChar(char, tags) => NodeRenderer::rich_char(frame, point, char, tags),
            Node::Text(str) => NodeRenderer::text(frame, point, str),
            Node::RichText(str, tags) => NodeRenderer::rich_text(frame, point, str, tags),
            Node::Tile(tile, rotation) => NodeRenderer::tile(frame, point, tile, rotation),
            Node::VerticalContainer(elems) => NodeRenderer::vertical_container(frame, point, elems),
            Node::HorizontalContainer(elems) => {
                NodeRenderer::horizontal_container(frame, point, elems)
            }
            Node::Framed(elem) => NodeRenderer::framed(frame, point, *elem),
            Node::Menu(elems, selected_index) => {
                NodeRenderer::menu(frame, point, elems, selected_index)
            }
            Node::Input(str, position) => NodeRenderer::input(frame, point, str, position),
            Node::Error(str) => NodeRenderer::error(frame, point, str),
            Node::MultiLineText(str) => NodeRenderer::multiline_text(frame, point, str),
            Node::MultiLineRichText(str, tags) => {
                NodeRenderer::multiline_rich_text(frame, point, str, tags)
            }
        }
    }

    /// Returns the space required to render the node and its children.
    ///
    /// This method is used for layout computation prior to rendering. It returns a `Size`
    /// that represents the width and height of the node’s bounding box.
    fn size(&self) -> Size {
        match self {
            Node::None => Size::new(0, 0),
            Node::Char(_) | Node::RichChar(_, _) => Size::new(1, 1),
            Node::Text(str) | Node::RichText(str, _) | Node::Input(str, _) | Node::Error(str) => {
                Size::new(str.len(), 1)
            }
            Node::Tile(_, _) => Size::new(TILE_SIZE, TILE_SIZE),
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
            Node::Menu(elems, _) => Size::new(
                elems.iter().map(|s| s.len()).max().unwrap_or(0) + 2,
                elems.len(),
            ),
            Node::MultiLineText(str) | Node::MultiLineRichText(str, _) => {
                let max_line_length = str.lines().map(|l| l.len()).max().unwrap_or(0);
                Size::new(max_line_length, str.lines().count())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::cell::CellTag;
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
        tile_id: String::new(),
        tile_features: Vec::new(),
        tile_extension: None,
    };
    fn tile_node() -> Node<'static> {
        Node::Tile(&TILE_INSTANCE, &Rotation::R0)
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
    fn test_size_none() {
        let n = none_node();
        assert_eq!(n.size(), Size::new(0, 0));
    }

    #[test]
    fn test_size_vertical_container() {
        let v = Node::VerticalContainer(vec![text_node("Hi"), char_node('X'), text_node("World")]);
        // width = max(2,1,5) = 5, height = 1+1+1 = 3
        assert_eq!(v.size(), Size::new(5, 3));
    }

    #[test]
    fn test_size_horizontal_container() {
        let h =
            Node::HorizontalContainer(vec![text_node("Hi"), char_node('X'), text_node("World")]);
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
        let v = Node::VerticalContainer(vec![text_node("A"), text_node("BC")]);
        v.render(&mut frame, Point::new(0, 0));
        assert_eq!(frame.cells[0][0].symbol, 'A');
        assert_eq!(frame.cells[1][0].symbol, 'B');
        assert_eq!(frame.cells[1][1].symbol, 'C');
    }

    #[test]
    fn test_render_horizontal_container() {
        let mut frame = Frame::new(Size::new(10, 3));
        let h = Node::HorizontalContainer(vec![text_node("A"), text_node("BC")]);
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

    #[test]
    fn test_render_rich_text_with_style() {
        let mut frame = Frame::new(Size::new(10, 2));
        let n = Node::RichText("Hi", vec![Underline, Foreground(Color::Green)]);
        n.render(&mut frame, Point::new(0, 0));

        assert_eq!(frame.cells[0][0].symbol, 'H');
        assert!(frame.cells[0][0].tags.contains(&CellTag::Underline));
        assert!(
            frame.cells[0][0]
                .tags
                .contains(&CellTag::Foreground(crossterm::style::Color::Green))
        );
    }

    #[test]
    fn test_render_rich_char() {
        let mut frame = Frame::new(Size::new(3, 3));
        let n = Node::RichChar('X', vec![Bold, Foreground(Color::Blue)]);
        n.render(&mut frame, Point::new(1, 1));

        assert_eq!(frame.cells[1][1].symbol, 'X');
        assert!(frame.cells[1][1].tags.contains(&CellTag::Bold));
        assert!(
            frame.cells[1][1]
                .tags
                .contains(&CellTag::Foreground(crossterm::style::Color::Blue))
        );
    }

    #[test]
    fn test_render_input_field() {
        let mut frame = Frame::new(Size::new(10, 2));
        let input = Node::Input("Hello", Point::new(2, 0));
        input.render(&mut frame, Point::new(1, 1));

        // Text starts at (1,1)
        assert_eq!(frame.cells[1][1].symbol, 'H');
        assert_eq!(frame.cursor, Some(Point::new(3, 1))); // 1 + 2 = 3
    }

    #[test]
    fn test_render_error_message() {
        let mut frame = Frame::new(Size::new(20, 1));
        let err = Node::Error("Oops");
        err.render(&mut frame, Point::new(0, 0));

        assert_eq!(frame.cells[0][0].symbol, 'O');
        assert!(frame.cells[0][0].tags.contains(&CellTag::Bold));
        assert!(
            frame.cells[0][0]
                .tags
                .contains(&CellTag::Foreground(crossterm::style::Color::Red))
        );
    }

    #[test]
    fn test_render_tile_stub() {
        let mut frame = Frame::new(Size::new(5, 5));
        let tile = tile_node();
        tile.render(&mut frame, Point::new(0, 0));

        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                assert_eq!(frame.cells[y][x].symbol, '.');
            }
        }
    }

    #[test]
    fn test_render_menu_with_selection() {
        let mut frame = Frame::new(Size::new(20, 5));
        let menu = Node::Menu(vec!["Play", "Options", "Quit"], 1);
        menu.render(&mut frame, Point::new(0, 0));

        assert_eq!(frame.cells[0][0].symbol, CharDrawing::Radio.into());
        assert_eq!(frame.cells[1][0].symbol, CharDrawing::SelectedRadio.into());
        assert_eq!(frame.cells[1][2].symbol, 'O');
        assert!(frame.cells[1][2].tags.contains(&CellTag::Underline));
    }

    #[test]
    fn test_render_menu_empty() {
        let mut frame = Frame::new(Size::new(10, 1));
        let menu = Node::Menu(vec![], 0);
        menu.render(&mut frame, Point::new(0, 0));

        // Should not panic and frame remains untouched
        assert_eq!(frame.cells[0][0].symbol, ' ');
    }
}
