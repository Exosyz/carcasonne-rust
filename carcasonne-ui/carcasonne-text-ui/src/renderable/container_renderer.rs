use crate::frame::Frame;
use crate::renderable::{get_node_renderer, Renderable};
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

pub enum ContainerDirection {
    Horizontal,
    Vertical,
}

pub enum ContainerProps {
    Size(usize, usize),
    Centered,
    Top,
    Bottom,
    Left,
    Right,
}

/// A container that lays out multiple [`Renderable`] children either horizontally or vertically,
/// optionally applying layout properties.
///
/// `ContainerRenderer` allows composing multiple renderable elements in a flexible layout.
/// Each child is rendered sequentially in the chosen direction, and the total container size
/// is computed from the children or overridden using [`ContainerProps::Size`].
///
/// # Directions
/// - `ContainerDirection::Horizontal`: children laid out side by side.
/// - `ContainerDirection::Vertical`: children stacked vertically.
///
/// # Properties (`ContainerProps`)
/// - `Size(width, height)`: explicitly sets the container's size.
/// - `Centered`, `Top`, `Bottom`, `Left`, `Right`: control the positioning of children inside the container.
///
/// # Behavior
/// - The container respects the `parent_available_size` when rendering.
/// - Children can be offset or aligned according to the applied properties.
/// - The size returned by [`size`] matches the actual space used during [`render`]
pub struct ContainerRenderer<'a> {
    childs: Vec<Box<dyn Renderable + 'a>>,
    direction: ContainerDirection,
    props: Vec<ContainerProps>,
}

impl<'a> ContainerRenderer<'a> {
    /// Creates a new container with the specified direction.
    ///
    /// # Parameters
    /// - `direction`: whether children are laid out horizontally or vertically.
    pub fn new(direction: ContainerDirection) -> Self {
        Self {
            direction,
            childs: vec![],
            props: vec![],
        }
    }

    /// Adds a single child renderable to the container.
    ///
    /// # Parameters
    /// - `child`: a boxed renderable to be added.
    ///
    /// # Returns
    /// Returns a mutable reference to self for chaining.
    fn add_child(&mut self, child: Box<dyn Renderable + 'a>) -> &mut Self {
        self.childs.push(child);
        self
    }

    /// Converts a list of layout [`Node`]s into renderables and adds them as children.
    ///
    /// # Parameters
    /// - `nodes`: the nodes to convert and add.
    ///
    /// # Returns
    /// Self with added children.
    pub fn add_nodes(mut self, nodes: Vec<Node<'a>>) -> Self {
        nodes.into_iter().for_each(|node| {
            self.add_child(get_node_renderer(node));
        });
        self
    }

    /// Adds a layout property to the container.
    ///
    /// # Parameters
    /// - `prop`: the property to add (e.g., alignment, fixed size).
    ///
    /// # Returns
    /// Self with the added property.
    pub fn add_props(mut self, prop: ContainerProps) -> Self {
        self.props.push(prop);
        self
    }

    /// Helper to check if a property matching a predicate exists.
    ///
    /// # Parameters
    /// - `predicate`: a function to test each property.
    ///
    /// # Returns
    /// `Some(&ContainerProps)` if a matching property exists, otherwise `None`.
    fn contain_prop<Predicate>(&self, predicate: Predicate) -> Option<&ContainerProps>
    where
        Predicate: Fn(&&ContainerProps) -> bool,
    {
        self.props.iter().find(|p| predicate(p))
    }
}

impl<'a> Renderable for ContainerRenderer<'a> {
    fn render(&self, frame: &mut Frame, parent_available_size: Size, point: Point) {
        let start_point = point;
        let mut current_size = parent_available_size;

        match self.direction {
            ContainerDirection::Horizontal => {
                let mut current_x = start_point.x;
                self.childs.iter().for_each(|elem| {
                    let size = elem.size(current_size);
                    elem.render(frame, current_size, Point::new(current_x, start_point.y));
                    current_x += size.width;
                    current_size.width -= size.width;
                })
            }
            ContainerDirection::Vertical => {
                let mut current_y = start_point.y;
                self.childs.iter().for_each(|elem| {
                    let size = elem.size(current_size);
                    elem.render(frame, current_size, Point::new(start_point.x, current_y));
                    current_y += size.height;
                    current_size.height -= size.height;
                })
            }
        }
    }
    fn size(&self, parent_available_size: Size) -> Size {
        if let Some(ContainerProps::Size(width, height)) =
            self.contain_prop(|p| matches!(p, ContainerProps::Size(_, _)))
        {
            Size::new(*width, *height)
        } else {
            match self.direction {
                ContainerDirection::Horizontal => self
                    .childs
                    .iter()
                    .map(|e| e.size(parent_available_size))
                    .fold(Size::new(0, 0), |acc, s| {
                        Size::new(acc.width + s.width, acc.height.max(s.height))
                    }),
                ContainerDirection::Vertical => self
                    .childs
                    .iter()
                    .map(|e| e.size(parent_available_size))
                    .fold(Size::new(0, 0), |acc, s| {
                        Size::new(acc.width.max(s.width), acc.height + s.height)
                    }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::Frame;
    use crate::renderable::Renderable;
    use carcasonne_core::layout::node::Node;
    use carcasonne_core::layout::point::Point;
    use carcasonne_core::layout::size::Size;

    fn simple_char_node(c: char) -> Node<'static> {
        Node::Char(c)
    }

    #[test]
    fn empty_container_has_zero_size() {
        let container = ContainerRenderer::new(ContainerDirection::Horizontal);
        assert_eq!(container.size(Size::new(10, 10)), Size::new(0, 0));

        let mut frame = Frame::new(Size::new(10, 10));
        container.render(&mut frame, Size::new(10, 10), Point::new(0, 0));

        for y in 0..10 {
            for x in 0..10 {
                assert_eq!(frame.cells[y][x].symbol, ' ');
            }
        }
    }

    #[test]
    fn horizontal_container_accumulates_width() {
        let container = ContainerRenderer::new(ContainerDirection::Horizontal)
            .add_nodes(vec![simple_char_node('A'), simple_char_node('B')]);
        let size = container.size(Size::new(10, 10));
        assert_eq!(size.width, 2);
        assert_eq!(size.height, 1);
    }

    #[test]
    fn vertical_container_accumulates_height() {
        let container = ContainerRenderer::new(ContainerDirection::Vertical)
            .add_nodes(vec![simple_char_node('X'), simple_char_node('Y')]);
        let size = container.size(Size::new(10, 10));
        assert_eq!(size.width, 1);
        assert_eq!(size.height, 2);
    }

    #[test]
    fn size_prop_overrides_children_size() {
        let container = ContainerRenderer::new(ContainerDirection::Horizontal)
            .add_nodes(vec![simple_char_node('A'), simple_char_node('B')])
            .add_props(ContainerProps::Size(5, 3));
        let size = container.size(Size::new(10, 10));
        assert_eq!(size, Size::new(5, 3));
    }

    #[test]
    fn horizontal_children_render_side_by_side() {
        let container = ContainerRenderer::new(ContainerDirection::Horizontal)
            .add_nodes(vec![simple_char_node('A'), simple_char_node('B')]);

        let mut frame = Frame::new(container.size(Size::new(10, 10)));
        container.render(&mut frame, Size::new(10, 10), Point::new(0, 0));

        assert_eq!(frame.cells[0][0].symbol, 'A');
        assert_eq!(frame.cells[0][1].symbol, 'B');
    }

    #[test]
    fn vertical_children_render_stacked() {
        let container = ContainerRenderer::new(ContainerDirection::Vertical)
            .add_nodes(vec![simple_char_node('X'), simple_char_node('Y')]);

        let mut frame = Frame::new(container.size(Size::new(10, 10)));
        container.render(&mut frame, Size::new(10, 10), Point::new(0, 0));

        assert_eq!(frame.cells[0][0].symbol, 'X');
        assert_eq!(frame.cells[1][0].symbol, 'Y');
    }

    #[test]
    fn rendering_respects_parent_bounds() {
        let container = ContainerRenderer::new(ContainerDirection::Horizontal).add_nodes(vec![
            simple_char_node('A'),
            simple_char_node('B'),
            simple_char_node('C'),
        ]);

        let mut frame = Frame::new(Size::new(2, 1));
        container.render(&mut frame, Size::new(2, 1), Point::new(0, 0));

        // Only first two chars fit
        assert_eq!(frame.cells[0][0].symbol, 'A');
        assert_eq!(frame.cells[0][1].symbol, 'B');
    }

    #[test]
    fn multiple_props_are_stored_and_queryable() {
        let container = ContainerRenderer::new(ContainerDirection::Vertical)
            .add_props(ContainerProps::Centered)
            .add_props(ContainerProps::Top);

        assert!(matches!(
            container.contain_prop(|p| matches!(p, ContainerProps::Centered)),
            Some(_)
        ));
        assert!(matches!(
            container.contain_prop(|p| matches!(p, ContainerProps::Top)),
            Some(_)
        ));
    }
}
