mod char_renderer;
mod container_renderer;
mod menu_renderer;
mod node;
mod none_renderer;
mod text_renderer;
mod tile_renderer;

use crate::frame::Frame;
use crate::renderable::char_renderer::CharRenderer;
use crate::renderable::container_renderer::{
    ContainerDirection, ContainerProps, ContainerRenderer,
};
use crate::renderable::menu_renderer::MenuRenderer;
use crate::renderable::none_renderer::NoneRenderer;
use crate::renderable::text_renderer::TextRenderer;
use crate::renderable::tile_renderer::TileRenderer;
use carcasonne_core::color::Color;
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::node::NodeTag::{Bold, Foreground};
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

pub fn get_node_renderer<'a>(node: Node<'a>) -> Box<dyn Renderable + 'a> {
    match node {
        Node::None => Box::new(NoneRenderer),
        Node::Char(c) => Box::new(CharRenderer::new(c, vec![])),
        Node::RichChar(c, tags) => Box::new(CharRenderer::new(c, tags)),
        Node::Text(str) => Box::new(TextRenderer::<'a>::new(str)),
        Node::RichText(str, tags) => Box::new(TextRenderer::<'a>::new(str).push_tags(tags)),
        Node::Error(str) => {
            Box::new(TextRenderer::<'a>::new(str).push_tags(vec![Bold, Foreground(Color::Red)]))
        }
        Node::Input(str, cursor) => Box::new(TextRenderer::new(str).set_cursor(cursor)),
        Node::VerticalContainer(elems) => {
            Box::new(ContainerRenderer::new(ContainerDirection::Vertical).add_nodes(elems))
        }
        Node::HorizontalContainer(elems) => {
            Box::new(ContainerRenderer::new(ContainerDirection::Horizontal).add_nodes(elems))
        }
        Node::Framed(elem) => Box::new(
            ContainerRenderer::new(ContainerDirection::Horizontal)
                .add_nodes(vec![*elem])
                .add_props(ContainerProps::Contained),
        ),
        Node::Tile(tile, rotation, size) => Box::new(TileRenderer::new(tile, rotation, size)),
        Node::Menu(options, selected_index) => Box::new(MenuRenderer::new(options, selected_index)),
        Node::FullScreenContainer(_) => todo!(),
    }
}

/// A trait representing an object that can be rendered onto a `Frame`.
/// Notes for implementors:
/// - The `render` method should not attempt to draw outside the bounds of the `Frame`.
/// - The `size` method must be consistent with what `render` will actually occupy,
///   otherwise layout engines may misplace or clip the renderable.
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
