mod node;

use crate::frame::Frame;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

/// A trait representing an object that can be rendered onto a `Frame`.
pub trait Renderable {
    /// Renders the object onto the provided frame starting at the specified position.
    ///
    /// # Parameters
    ///
    /// * `frame` - The mutable frame where the object will be rendered.
    /// * `point` - The top-left position on the frame to start rendering.
    fn render(&self, frame: &mut Frame, point: Point);
    /// Returns the size that the rendered object will occupy.
    ///
    /// # Returns
    ///
    /// A `Size` struct representing the width and height in characters.
    fn size(&self) -> Size;
}
