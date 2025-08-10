mod char_renderer;
mod container_renderer;
mod node;
mod text_renderer;
mod tile_renderer;

use crate::frame::Frame;
use crate::renderable::char_renderer::CharRenderer;
use crate::renderable::container_renderer::{
    ContainerDirection, ContainerProps, ContainerRenderer,
};
use crate::renderable::text_renderer::TextRenderer;
use carcasonne_core::color::Color;
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::node::NodeTag::{Bold, Foreground};
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

pub fn get_node_renderer<'a>(node: Node<'a>) -> Box<dyn Renderable + 'a> {
    match node {
        Node::None => todo!(),
        Node::Char(c) => Box::new(CharRenderer::new(c, vec![])),
        Node::RichChar(c, tags) => Box::new(CharRenderer::new(c, tags)),
        Node::Text(str) => Box::new(TextRenderer::<'a>::new(str, vec![])),
        Node::RichText(str, tags) => Box::new(TextRenderer::<'a>::new(str, tags)),
        Node::Error(str) => Box::new(TextRenderer::<'a>::new(
            str,
            vec![Bold, Foreground(Color::Red)],
        )),
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
        _ => todo!(),
        /*
        Node::Tile(tile, rotation, size) => NodeRenderer::tile(frame, point, tile, rotation, size),
        Node::Menu(elems, selected_index) => {
            NodeRenderer::menu(frame, point, elems, selected_index)
        }
        Node::Input(str, position) => NodeRenderer::input(frame, point, str, position),
        Node::MultiLineText(str) => NodeRenderer::multiline_text(frame, point, str),
        Node::MultiLineRichText(str, tags) => {
            NodeRenderer::multiline_rich_text(frame, point, str, tags)
        }
         */
    }
}

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
