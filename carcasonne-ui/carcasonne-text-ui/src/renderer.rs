use crate::frame::Frame;
use carcasonne_core::layout::node::Node;
use carcasonne_core::renderer::Renderer;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Attribute, Print, ResetColor, SetAttribute};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{
    execute, queue,
    terminal::{Clear, ClearType},
};
use std::io::Write;

/// A renderer that outputs the game view as styled text to the terminal.
///
/// This renderer uses the `crossterm` crate to manage terminal output, including
/// cursor movement, styling (colors, attributes), and raw mode control.
///
/// - Enables raw mode and switches to the alternate screen buffer on creation,
///   allowing a clean and isolated rendering environment.
/// - Restores the terminal to the original state on a drop.
///
/// The renderer converts a UI tree (`Node`) into a `Frame`, then
/// draws the frame cell by cell with the appropriate styles.
///
/// # Type Parameters
///
/// * `W`: The output stream implementing `Write`. This allows flexibility
///   in where the output is sent (stdout, buffer, tests, etc.).
#[derive(Default, Debug)]
pub struct ConsoleRenderer<W: Write> {
    out: W,
}

impl<W: Write> ConsoleRenderer<W> {
    /// Creates a new `TextRenderer`, enabling raw mode and switching
    /// to the alternate screen buffer.
    ///
    /// # Arguments
    ///
    /// * `out` - The output stream to write terminal data to.
    ///
    /// # Panics
    ///
    /// Does not panic but silently ignores errors from terminal setup.
    pub fn new(mut out: W) -> Self {
        enable_raw_mode().ok();
        execute!(out, EnterAlternateScreen).ok();
        Self { out }
    }
}

impl<W: Write> Drop for ConsoleRenderer<W> {
    /// Disables raw mode and leaves the alternate screen buffer
    /// to restore the terminal to its original state.
    fn drop(&mut self) {
        disable_raw_mode().ok();
        execute!(self.out, LeaveAlternateScreen).ok();
    }
}

impl<W: Write> Renderer for ConsoleRenderer<W> {
    /// Renders the given root `Node` by converting it into a `Frame`,
    /// then printing each cell's symbol with its styles at the correct
    /// terminal position.
    ///
    /// Clears the terminal before rendering to avoid artifacts.
    /// Applies foreground/background colors and text attributes per cell.
    /// Shows or hides the cursor based on the frame's cursor position.
    ///
    /// # Arguments
    ///
    /// * `node` - The root node representing the UI to render.
    fn render(&mut self, node: Node) {
        execute!(self.out, Clear(ClearType::All), MoveTo(0, 0),).ok();

        let mut frame: Frame = node.into();

        for i in 0..frame.size.height {
            for j in 0..frame.size.width {
                let cell = frame.cells[i][j].to_owned();
                cell.apply_tags(&mut self.out);

                queue!(
                    self.out,
                    MoveTo(j as u16, i as u16),
                    Print(cell.symbol),
                    ResetColor,
                    SetAttribute(Attribute::Reset),
                )
                .ok();
            }
        }

        if let Some(point) = frame.cursor {
            queue!(self.out, MoveTo(point.x as u16, point.y as u16), Show).ok();
        } else {
            queue!(self.out, Hide).ok();
        }

        self.out.flush().ok();
        frame.set_cursor(None);
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
            let mut renderer = ConsoleRenderer::new(&mut buffer);
            let node = Node::Char('X');
            renderer.render(node);
        }

        let output = String::from_utf8(buffer.get_ref().clone()).unwrap();

        assert!(
            output.contains('X'),
            "Output should contain a rendered character"
        );

        // Check if terminal escape sequences are present (for styling/control)
        assert!(
            output.contains("\u{1b}["),
            "Output should contain ANSI escape sequences"
        );
    }
}
