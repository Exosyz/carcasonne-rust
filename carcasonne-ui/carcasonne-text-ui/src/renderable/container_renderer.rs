mod tests;

use crate::frame::Frame;
use crate::renderable::{get_node_renderer, Renderable};
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

/// Defines the primary layout direction of a container.
///
/// - [`Horizontal`] — children are placed side by side, advancing along the X axis.
/// - [`Vertical`] — children are placed on top of each other, advancing along the Y axis.
pub enum ContainerDirection {
    Horizontal,
    Vertical,
}

/// Defines layout properties that can be applied to a container.
///
/// These properties influence how the container itself or its children
/// should behave when computing size and rendering.
///
/// - [`FullSize(ContainerDirection)`] — expands to take all available space in a given direction.
/// - [`Centered`] — aligns content in the center (not yet implemented).
/// - [`Top`], [`Bottom`], [`Left`], [`Right`] — alignment hints for positioning
pub enum ContainerProps {
    FullSize(ContainerDirection),
    Centered,
    Top,
    Bottom,
    Left,
    Right,
}

/// Internal structure used to analyze the layout of a container before rendering.
///
/// - [`non_full_sized_size`] — total accumulated size of children that do not expand.
/// - [`full_sized_count`] — number of children marked with [`ContainerProps::FullSize`].
struct LayoutAnalysis {
    non_full_sized_size: Size,
    full_sized_count: usize,
}

/// Type alias for a cursor advancement function.
///
/// An `AdvanceCursor` shifts the rendering position and reduces
/// the remaining available space after a child has been rendered.
type AdvanceCursor = Box<dyn FnMut(&mut Size, &mut Point, Size)>;

/// Type alias for a size accumulator function.
///
/// A `SizeAccumulator` combines the size of children into a cumulative size,
/// used during layout analysis.
type SizeAccumulator = Box<dyn FnMut(&mut Size, Size)>;

/// A container renderer that arranges child [`Renderable`] elements
/// either horizontally or vertically, applying layout properties as needed.
///
/// Containers can be nested to create complex layouts. Each child can be
/// another container or a leaf renderer (e.g., text, framed element).
pub struct ContainerRenderer<'a> {
    children: Vec<Box<dyn Renderable + 'a>>,
    direction: ContainerDirection,
    props: Vec<ContainerProps>,
}

impl<'a> ContainerRenderer<'a> {
    /// Creates a new container with the given direction.
    pub fn new(direction: ContainerDirection) -> Self {
        Self {
            direction,
            children: vec![],
            props: vec![],
        }
    }

    /// Adds a child [`Renderable`] element to the container.
    ///
    /// Typically used with `Box::new(TextRenderer::new(...))`
    /// or nested containers.
    fn add_child(&mut self, child: Box<dyn Renderable + 'a>) -> &mut Self {
        self.children.push(child);
        self
    }

    /// Converts a list of [`Node`]s into renderable children and adds them.
    pub fn add_nodes(mut self, nodes: Vec<Node<'a>>) -> Self {
        nodes.into_iter().for_each(|node| {
            self.add_child(get_node_renderer(node));
        });
        self
    }

    /// Adds a layout property ([`ContainerProps`]) to the container.
    ///
    /// Useful for marking a container as full-size in a direction.
    pub fn add_props(mut self, prop: ContainerProps) -> Self {
        self.props.push(prop);
        self
    }

    /// Checks if the container has a property matching the given predicate.
    fn contain_prop<Predicate>(&self, predicate: Predicate) -> Option<&ContainerProps>
    where
        Predicate: Fn(&ContainerProps) -> bool,
    {
        self.props.iter().find(|p| predicate(p))
    }

    /// Returns true if the container has at least one [`FullSize`] property.
    fn has_full_size_prop(&self) -> bool {
        self.contain_prop(|p| matches!(p, ContainerProps::FullSize(_)))
            .is_some()
    }

    /// Returns true if the container has a [`FullSize`] property in the given direction.
    fn has_full_size_in_direction(&self, direction: ContainerDirection) -> bool {
        self.contain_prop(|p| matches!(p, ContainerProps::FullSize(d) if std::mem::discriminant(d) == std::mem::discriminant(&direction)))
            .is_some()
    }

    /// Creates the function responsible for advancing the cursor
    /// after rendering a child in the container’s direction.
    fn create_advance_cursor(&self) -> AdvanceCursor {
        match self.direction {
            ContainerDirection::Horizontal => Box::new(
                |available_size: &mut Size, position: &mut Point, child_size: Size| {
                    position.x += child_size.width;
                    available_size.width -= child_size.width;
                },
            ),
            ContainerDirection::Vertical => Box::new(
                |available_size: &mut Size, position: &mut Point, child_size: Size| {
                    position.y += child_size.height;
                    available_size.height -= child_size.height;
                },
            ),
        }
    }

    /// Creates the function that accumulates the size of non-full-size children.
    fn create_size_accumulator(&self) -> SizeAccumulator {
        match self.direction {
            ContainerDirection::Horizontal => {
                Box::new(|accumulated_size: &mut Size, child_size: Size| {
                    accumulated_size.width += child_size.width;
                    accumulated_size.height = accumulated_size.height.max(child_size.height);
                })
            }
            ContainerDirection::Vertical => {
                Box::new(|accumulated_size: &mut Size, child_size: Size| {
                    accumulated_size.height += child_size.height;
                    accumulated_size.width = accumulated_size.width.max(child_size.width);
                })
            }
        }
    }

    /// Analyzes the container’s children to calculate:
    /// - total size of fixed-size children,
    /// - number of children with full-size properties.
    fn analyze_layout(&self, parent_available_size: Size) -> LayoutAnalysis {
        let mut analysis = LayoutAnalysis {
            non_full_sized_size: Size::new(0, 0),
            full_sized_count: 0,
        };

        let mut size_accumulator = self.create_size_accumulator();

        for child in self.children.iter() {
            if let Some(container) = child.as_container() {
                if container.has_full_size_prop() {
                    analysis.full_sized_count += 1;
                } else {
                    let child_size = child.size(parent_available_size);
                    size_accumulator(&mut analysis.non_full_sized_size, child_size);
                }
            } else {
                let child_size = child.size(parent_available_size);
                size_accumulator(&mut analysis.non_full_sized_size, child_size);
            }
        }

        analysis
    }

    /// Computes the maximum size available for full-size children
    /// by dividing remaining space equally among them.
    fn calculate_max_full_size(&self, parent_size: Size, analysis: &LayoutAnalysis) -> Size {
        if analysis.full_sized_count == 0 {
            return parent_size;
        }

        match self.direction {
            ContainerDirection::Horizontal => Size::new(
                (parent_size.width - analysis.non_full_sized_size.width)
                    / analysis.full_sized_count,
                parent_size.height,
            ),
            ContainerDirection::Vertical => Size::new(
                parent_size.width,
                (parent_size.height - analysis.non_full_sized_size.height)
                    / analysis.full_sized_count,
            ),
        }
    }

    /// Determines the actual size of a full-size container child,
    /// taking into account whether it expands horizontally, vertically, or both.
    fn get_full_size_container_size(
        &self,
        container: &ContainerRenderer<'a>,
        max_full_size: Size,
        current_size: Size,
    ) -> Size {
        let is_vertical = container.has_full_size_in_direction(ContainerDirection::Vertical);
        let is_horizontal = container.has_full_size_in_direction(ContainerDirection::Horizontal);

        match (is_vertical, is_horizontal) {
            (true, true) => max_full_size,
            (true, false) => Size::new(
                if current_size.width > max_full_size.width {
                    max_full_size.width
                } else {
                    current_size.width
                },
                max_full_size.height,
            ),
            (false, true) => Size::new(
                max_full_size.width,
                if current_size.height > max_full_size.height {
                    max_full_size.height
                } else {
                    current_size.height
                },
            ),
            (false, false) => {
                panic!("Container marked as full-size but has no full-size properties")
            }
        }
    }

    /// Renders a single child and returns its actual rendered size.
    ///
    /// Handles both normal children and full-size children,
    /// applying the correct computed size.
    fn render_child(
        &self,
        child: &(dyn Renderable + 'a),
        frame: &mut Frame,
        available_size: Size,
        position: Point,
        max_full_size: Size,
        current_size: Size,
    ) -> Size {
        if let Some(container) = child.as_container() {
            if container.has_full_size_prop() {
                let container_size =
                    self.get_full_size_container_size(container, max_full_size, current_size);
                container.render(frame, container_size, position);
                container_size
            } else {
                let child_size = child.size(available_size);
                child.render(frame, available_size, position);
                child_size
            }
        } else {
            let child_size = child.size(available_size);
            child.render(frame, available_size, position);
            child_size
        }
    }
}

impl<'a> Renderable for ContainerRenderer<'a> {
    fn render(&self, frame: &mut Frame, parent_available_size: Size, point: Point) {
        let mut current_point = point;
        let mut current_size = self.size(parent_available_size);
        let mut advance_cursor = self.create_advance_cursor();

        // Analyze layout
        let analysis = self.analyze_layout(current_size);
        let max_full_size = self.calculate_max_full_size(current_size, &analysis);

        // Render each child
        for child in self.children.iter() {
            let actual_child_size = self.render_child(
                &**child,
                frame,
                current_size,
                current_point,
                max_full_size,
                current_size,
            );

            advance_cursor(&mut current_size, &mut current_point, actual_child_size);
        }
    }
    fn size(&self, parent_available_size: Size) -> Size {
        let analysis = self.analyze_layout(parent_available_size);

        let mut final_size = analysis.non_full_sized_size;

        // If we have full-size children, adjust dimensions accordingly
        if analysis.full_sized_count > 0 {
            match self.direction {
                ContainerDirection::Horizontal => {
                    final_size.width = parent_available_size.width;
                }
                ContainerDirection::Vertical => {
                    final_size.height = parent_available_size.height;
                }
            }
        }

        // Apply the container's own full-size properties
        let container_width = if self.has_full_size_in_direction(ContainerDirection::Horizontal) {
            parent_available_size.width
        } else {
            final_size.width
        };

        let container_height = if self.has_full_size_in_direction(ContainerDirection::Vertical) {
            parent_available_size.height
        } else {
            final_size.height
        };

        Size::new(
            container_width.min(parent_available_size.width),
            container_height.min(parent_available_size.height),
        )
    }

    fn as_container(&self) -> Option<&ContainerRenderer<'a>> {
        Some(self)
    }
}
