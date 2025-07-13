use crate::char_drawing::CharDrawing;
use crate::color::Color;
use crate::renderable::Renderable;
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

/// A single text-based cell in the frame, containing a character and its associated colors.
#[derive(Clone)]
pub struct Cell {
    /// The character displayed in the cell.
    pub symbol: char,
    /// The background color of the cell.
    pub background_color: Color,
    /// The foreground (text) color of the cell.
    pub foreground_color: Color,
}

/// A 2D buffer of `Cell`, used for rendering a text-based user interface.
///
/// A `Frame` represents a rectangular grid of characters and colors,
/// similar to a terminal screen or a custom canvas. It is used to build
/// text UIs where layout and content are manually controlled.
///
/// The size is fixed at creation, and the cell content can be modified
/// using various helper methods.
pub struct Frame {
    /// The dimensions of the frame (width × height).
    pub size: Size,
    /// A 2D vector of cells, indexed as `cells[y][x]`.
    pub cells: Vec<Vec<Cell>>,
}

impl Frame {
    /// Creates a new `Frame` with the specified size.
    ///
    /// The frame is initialized with empty cells (spaces) having black background and white foreground colors.
    ///
    /// # Parameters
    ///
    /// * `size` - The size of the frame as a `Size` struct.
    ///
    /// # Returns
    ///
    /// A new `Frame` instance with all cells initialized.
    pub fn new(size: Size) -> Self {
        Self {
            size,
            cells: vec![
                vec![
                    Cell {
                        symbol: CharDrawing::None.into(),
                        background_color: Color::Black,
                        foreground_color: Color::White,
                    };
                    size.width
                ];
                size.height
            ],
        }
    }

    /// Sets a specific `Cell` at the given position in the frame.
    ///
    /// # Panics
    ///
    /// Panics if the given `point` is out of the frame's bounds.
    ///
    /// # Parameters
    ///
    /// * `point` - The position (x, y) where the cell should be set.
    /// * `cell` - The `Cell` to set at the given position.
    fn set_cell(&mut self, point: Point, cell: Cell) {
        assert!(
            point.y < self.size.height && point.x < self.size.width,
            "Point out of bounds"
        );

        self.cells[point.y][point.x] = cell;
    }

    /// Sets a character cell at the specified position with given foreground and background colors.
    ///
    /// # Parameters
    ///
    /// * `point` - The position where the character will be drawn.
    /// * `c` - The character to draw.
    /// * `foreground_color` - The color of the character.
    /// * `background_color` - The background color behind the character.
    pub fn char(
        &mut self,
        point: Point,
        c: char,
        foreground_color: Color,
        background_color: Color,
    ) {
        self.set_cell(
            point,
            Cell {
                symbol: c,
                background_color: background_color.clone(),
                foreground_color: foreground_color.clone(),
            },
        );
    }

    /// A simplified version of `char` that draws a character with white foreground and black background.
    ///
    /// # Parameters
    ///
    /// * `point` - The position where the character will be drawn.
    /// * `c` - The character to draw.
    pub fn char_simple(&mut self, point: Point, c: char) {
        self.char(point, c, Color::White, Color::Black);
    }
}

impl From<Node<'_>> for Frame {
    /// Converts a `Node` into a rendered `Frame`.
    ///
    /// This creates a frame of the node's size and renders the node starting at position (0, 0).
    ///
    /// # Parameters
    ///
    /// * `value` - The `Node` to convert.
    ///
    /// # Returns
    ///
    /// A `Frame` containing the rendered node.
    fn from(value: Node) -> Self {
        let mut frame = Frame::new(value.size());
        value.render(&mut frame, Point::zero());
        frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use carcasonne_core::layout::node::Node;
    use carcasonne_core::layout::point::Point;

    #[test]
    fn frame_new_initializes_correctly() {
        let size = Size::new(4, 3);
        let frame = Frame::new(size);

        assert_eq!(frame.size.width, 4);
        assert_eq!(frame.size.height, 3);

        for row in &frame.cells {
            for cell in row {
                assert_eq!(cell.symbol, CharDrawing::None.into());
                assert_eq!(cell.background_color, Color::Black);
                assert_eq!(cell.foreground_color, Color::White);
            }
        }
    }

    #[test]
    fn frame_char_sets_correct_cell() {
        let size = Size::new(3, 3);
        let mut frame = Frame::new(size);
        let point = Point::new(1, 1);
        let c = 'X';

        frame.char(point, c, Color::Red, Color::Blue);

        let cell = &frame.cells[1][1];
        assert_eq!(cell.symbol, c);
        assert_eq!(cell.foreground_color, Color::Red);
        assert_eq!(cell.background_color, Color::Blue);
    }

    #[test]
    fn frame_char_simple_defaults_colors() {
        let size = Size::new(2, 2);
        let mut frame = Frame::new(size);
        let point = Point::new(0, 0);
        let c = 'A';

        frame.char_simple(point, c);

        let cell = &frame.cells[0][0];
        assert_eq!(cell.symbol, c);
        assert_eq!(cell.foreground_color, Color::White);
        assert_eq!(cell.background_color, Color::Black);
    }

    #[test]
    #[should_panic(expected = "Point out of bounds")]
    fn set_cell_panics_on_out_of_bounds() {
        let size = Size::new(2, 2);
        let mut frame = Frame::new(size);

        let out_of_bounds_point = Point::new(2, 0);
        frame.char_simple(out_of_bounds_point, 'Z');
    }

    #[test]
    fn from_node_creates_frame_and_renders() {
        let node = Node::Char('Q');
        let frame: Frame = node.into();

        assert_eq!(frame.size.width, 1);
        assert_eq!(frame.size.height, 1);
        assert_eq!(frame.cells[0][0].symbol, 'Q');
    }
}
