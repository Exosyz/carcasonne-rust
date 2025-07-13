use crate::frame::Frame;
use carcasonne_core::layout::node::Node;
use carcasonne_core::renderer::Renderer;
use crossterm::style::{Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    cursor, execute, queue,
    terminal::{Clear, ClearType},
};
use std::io::Write;

/// A renderer that outputs the game view as text to the terminal.
///
/// Uses `crossterm` for terminal control and styling.
/// Enables raw mode on creation and disables it on a drop.
#[derive(Default, Debug)]
pub struct TextRenderer<W: Write> {
    out: W,
}

impl<W: Write> TextRenderer<W> {
    /// Creates a new `TextRenderer` and enables raw mode.
    pub fn new(out: W) -> Self {
        enable_raw_mode().ok();
        Self { out }
    }
}

impl<W: Write> Drop for TextRenderer<W> {
    /// Disables raw mode on a drop to restore the terminal state.
    fn drop(&mut self) {
        disable_raw_mode().ok();
    }
}

impl<W: Write> Renderer for TextRenderer<W> {
    /// Renders the given root `Node` by converting it into a `Frame`,
    /// then printing each cell's symbol to the terminal with appropriate colors.
    ///
    /// Clears the terminal before rendering and resets colors after each cell.
    ///
    /// # Parameters
    ///
    /// * `node` - The root node of the UI tree to render.
    fn render(&mut self, node: Node) {
        execute!(self.out, Clear(ClearType::All), cursor::MoveTo(0, 0),).ok();

        let frame: Frame = node.into();

        for i in 0..frame.size.height {
            for j in 0..frame.size.width {
                let cell = &frame.cells[i][j];
                queue!(
                    self.out,
                    SetForegroundColor((&cell.foreground_color).into()),
                    //SetBackgroundColor((&cell.background_color).into()),
                    Print(cell.symbol),
                    ResetColor
                )
                .unwrap();
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use carcasonne_core::layout::node::Node;
    use std::io::Cursor;

    #[test]
    fn test_render_char_node() {
        let mut buffer = Cursor::new(vec![]);
        {
            let mut renderer = TextRenderer::new(&mut buffer);
            let node = Node::Char('X');
            renderer.render(node);
        }

        let output = String::from_utf8(buffer.get_ref().clone()).unwrap();

        assert!(output.contains('X'));

        assert!(output.contains("\u{1b}["));
    }
}
