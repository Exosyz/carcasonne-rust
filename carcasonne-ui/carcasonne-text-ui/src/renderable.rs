mod char_renderer;
mod container_renderer;
mod framed_renderer;
mod menu_renderer;
mod none_renderer;
mod text_renderer;
mod tile_renderer;

use crate::frame::Frame;
use crate::renderable::char_renderer::CharRenderer;
use crate::renderable::container_renderer::{
    ContainerDirection, ContainerProps, ContainerRenderer,
};
use crate::renderable::framed_renderer::FramedRenderer;
use crate::renderable::menu_renderer::MenuRenderer;
use crate::renderable::none_renderer::NoneRenderer;
use crate::renderable::text_renderer::TextRenderer;
use crate::renderable::tile_renderer::TileRenderer;
use carcasonne_core::color::Color;
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::node::NodeTag::{Bold, Foreground};
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;
use crossterm::terminal::size;
use std::cmp::min;

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
        Node::Framed(elem) => Box::new(FramedRenderer::new(get_node_renderer(*elem))),
        Node::Tile(tile, rotation, size) => Box::new(TileRenderer::new(tile, rotation, size)),
        Node::Menu(options, selected_index) => Box::new(MenuRenderer::new(options, selected_index)),
        Node::FullScreenContainer(elem) => {
            let (height, width) = size().unwrap();
            Box::new(
                ContainerRenderer::new(ContainerDirection::Horizontal)
                    .add_nodes(vec![*elem])
                    .add_props(ContainerProps::Size(height.into(), width.into())),
            )
        }
    }
}

/// Trait representing an object that can be rendered onto a [`Frame`].
///
/// # Contract for Implementors
/// - The [`render`] method must **never** attempt to draw outside the bounds of the frame.
/// - The [`size`] method must accurately report the space the renderable will occupy.
///   Otherwise, layout engines may misplace or clip the content.
///
/// # Parameters
/// - `frame`: the [`Frame`] to draw onto.
/// - `parent_available_size`: the maximum space available for rendering.
/// - `point`: the starting point in the frame where rendering begins.
pub trait Renderable {
    /// Renders the object onto the given frame at the specified starting point.
    ///
    /// # Notes
    /// - Must respect `parent_available_size`.
    /// - Should not panic or write outside the frame boundaries.
    fn render(&self, frame: &mut Frame, parent_available_size: Size, point: Point);

    /// Returns the size that this object will occupy when rendered.
    ///
    /// # Notes
    /// - The returned size must be consistent with what `render` actually draws.
    /// - `parent_available_size` should be taken into account, and the size should
    ///   be clamped if necessary.
    fn size(&self, parent_available_size: Size) -> Size;
}

fn fit_within_bounds(current: Size, other: Size) -> Size {
    Size::new(
        min(current.width, other.width),
        min(current.height, other.height),
    )
}
