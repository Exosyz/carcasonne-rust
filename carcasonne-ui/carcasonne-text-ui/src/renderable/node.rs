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

use crate::frame::Frame;
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::point::Point;

/// Stateless helper for rendering `Node` elements into a `Frame`.
///
/// `NodeRenderer` encapsulates all rendering logic for node variants,
/// separating layout behavior from drawing behavior.
/// It is only used internally via the `Renderable` trait.
struct NodeRenderer;

impl NodeRenderer {
    /// Renders a framed box around a child node, using `+`, `-`, and `|` characters.
    ///
    /// Adds 1-character padding around the inner node.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer.
    /// * `point` - The top-left position of the outer frame.
    /// * `elem` - The inner node to render inside the frame.
    fn framed(frame: &mut Frame, point: Point, elem: Node) {
        /*
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
         */
    }
}
