use crate::layout::node::Node;

/// A trait for rendering a layout tree.
///
/// A `Renderer` is responsible for taking a root `Node` (typically representing
/// the entire UI or game screen) and producing a visual output.
///
/// Implementors may render to different targets, such as a terminal, GUI, or image buffer.
pub trait Renderer {
    /// Renders the provided main layout node.
    ///
    /// # Arguments
    ///
    /// * `node` - The root node of the layout tree to render.
    fn render(&mut self, node: Node);
}
