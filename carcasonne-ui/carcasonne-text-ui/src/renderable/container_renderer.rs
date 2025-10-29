use crate::frame::Frame;
use crate::renderable::container_renderer::layout_analyser::LayoutAnalyser;
use crate::renderable::helpers::renderable_helper::RenderableProps;
use crate::renderable::Renderable;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

mod layout_analyser;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerProps {
    FullSize(ContainerDirection),
    Centered,
    Top,
    Bottom,
}

/// A container that arranges its children either horizontally or vertically.
#[derive(Default)]
pub struct ContainerRenderer<'a> {
    pub direction: ContainerDirection,
    pub children: Vec<Box<dyn Renderable + 'a>>,
    pub props: Vec<ContainerProps>,
}

impl<'a> ContainerRenderer<'a> {
    pub fn new(
        direction: ContainerDirection,
        children: Vec<Box<dyn Renderable + 'a>>,
        props: Vec<ContainerProps>,
    ) -> Self {
        Self {
            direction,
            children,
            props,
        }
    }

    pub fn has_prop(&self, prop: ContainerProps) -> bool {
        self.props.contains(&prop)
    }

    pub(crate) fn has_full_size_in_direction(&self, dir: ContainerDirection) -> bool {
        self.props.iter().any(|p| match p {
            ContainerProps::FullSize(d) => *d == dir,
            _ => false,
        })
    }
}

impl<'a> Renderable for ContainerRenderer<'a> {
    fn render(&self, frame: &mut Frame, parent_available_size: Size, origin: Point) -> Size {
        let analysis = LayoutAnalyser::new(self).analyse(parent_available_size);

        let mut child_origin = origin;
        let mut container_size = Size::new(0, 0);
        for c in self.children.iter() {
            let available_size = if c.has_full_size_in_direction(self.direction) {
                analysis.full_size_child_size.unwrap()
            } else {
                parent_available_size
            };

            let child_size = c.render(frame, available_size, child_origin);

            match self.direction {
                ContainerDirection::Horizontal => {
                    child_origin.x += child_size.width;
                    container_size.width = child_origin.x;
                    container_size.height = child_size.height.max(container_size.height);
                }
                ContainerDirection::Vertical => {
                    child_origin.y += child_size.height;
                    container_size.width = child_size.width.max(container_size.width);
                    container_size.height = child_origin.y;
                }
            }
        }

        container_size
    }

    fn size(&self, parent_available_size: Size) -> Size {
        let (is_horizontal_full_size, is_vertical_full_size) = (
            self.has_prop(ContainerProps::FullSize(ContainerDirection::Horizontal)),
            self.has_prop(ContainerProps::FullSize(ContainerDirection::Vertical)),
        );
        if is_horizontal_full_size && is_vertical_full_size {
            return parent_available_size;
        }

        let analysis = LayoutAnalyser::new(self).analyse(parent_available_size);
        let base_size = analysis
            .full_size_child_size
            .unwrap_or(analysis.normal_child_size);

        match (is_horizontal_full_size, is_vertical_full_size) {
            (true, _) => Size::new(parent_available_size.width, base_size.height),
            (_, true) => Size::new(base_size.width, parent_available_size.height),
            _ => base_size,
        }
    }

    fn debug(&self, tabs: usize) -> String {
        let indent = "\t".repeat(tabs);
        let indent_inner = "\t".repeat(tabs + 1);
        let children_debug = self
            .children
            .iter()
            .map(|s| s.debug(tabs + 2))
            .collect::<Vec<_>>()
            .join("\n");

        let props = self
            .props
            .iter()
            .map(|s| format!("{s:?}"))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "{indent}ContainerRenderer {{\n\
             {indent_inner}direction: {:?},\n\
             {indent_inner}props: [{props}],\n\
             {indent_inner}children: [\n{children_debug}\n{indent_inner}],\n\
             {indent}}}",
            self.direction
        )
    }

    fn as_container(&self) -> Option<&ContainerRenderer<'a>> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{ContainerDirection, ContainerProps, ContainerRenderer};
    use crate::frame::{Frame, FrameDebug};
    use crate::renderable::container_renderer::ContainerProps::FullSize;
    use crate::renderable::framed_renderer::FramedRenderer;
    use crate::renderable::mock_renderer::MockRenderable;
    use crate::renderable::text_renderer::TextRenderer;
    use crate::renderable::Renderable;
    use carcasonne_core::layout::point::Point;
    use carcasonne_core::layout::size::Size;

    #[test]
    fn test_container_horizontal_rendering() {
        let child1 = Box::new(MockRenderable::new(Size::new(5, 3)));
        let child2 = Box::new(MockRenderable::new(Size::new(7, 4)));
        let child3 = Box::new(MockRenderable::new(Size::new(3, 2)));

        let container = ContainerRenderer::new(
            ContainerDirection::Horizontal,
            vec![child1, child2, child3],
            vec![],
        );

        let mut frame = Frame::new(Size::new(20, 10));
        let rendered_size = container.render(&mut frame, Size::new(20, 10), Point::new(0, 0));
        frame.debug();

        // Expected to span the summed widths of the children in a horizontal layout
        assert_eq!(rendered_size, Size::new(15, 4)); // max height among children
    }

    #[test]
    fn test_container_vertical_rendering() {
        let child1 = Box::new(MockRenderable::new(Size::new(5, 3)));
        let child2 = Box::new(MockRenderable::new(Size::new(7, 4)));
        let child3 = Box::new(MockRenderable::new(Size::new(3, 2)));

        let container = ContainerRenderer::new(
            ContainerDirection::Vertical,
            vec![child1, child2, child3],
            vec![],
        );

        let mut frame = Frame::new(Size::new(10, 20));
        let rendered_size = container.render(&mut frame, Size::new(10, 20), Point::new(0, 0));
        frame.debug();

        // Expected to span the summed heights of the children in a vertical layout
        assert_eq!(rendered_size, Size::new(7, 9)); // max width among children
    }

    #[test]
    fn test_container_fullsize_child() {
        let fullsize_child = Box::new(FramedRenderer::new(Box::new(ContainerRenderer::new(
            ContainerDirection::Horizontal,
            vec![Box::new(TextRenderer::new("test"))],
            vec![FullSize(ContainerDirection::Horizontal)],
        ))));
        let container =
            ContainerRenderer::new(ContainerDirection::Horizontal, vec![fullsize_child], vec![]);

        let mut frame = Frame::new(Size::new(20, 10));
        let rendered_size = container.render(&mut frame, Size::new(20, 10), Point::new(0, 0));
        frame.debug();

        // FullSize child should take the parent's size
        assert_eq!(rendered_size, Size::new(20, 3));
    }

    #[test]
    fn test_empty_container_rendering() {
        let container = ContainerRenderer::new(ContainerDirection::Horizontal, vec![], vec![]);

        let mut frame = Frame::new(Size::new(20, 10));
        let rendered_size = container.render(&mut frame, Size::new(20, 10), Point::new(0, 0));
        frame.debug();

        // Empty container should have size 0
        assert_eq!(rendered_size, Size::new(0, 0));
    }

    #[test]
    fn test_container_with_centered_child() {
        let child = Box::new(MockRenderable::new(Size::new(5, 3)));

        let container = ContainerRenderer::new(
            ContainerDirection::Horizontal,
            vec![child],
            vec![ContainerProps::Centered],
        );

        let mut frame = Frame::new(Size::new(20, 10));
        let rendered_size = container.render(&mut frame, Size::new(20, 10), Point::new(0, 0));
        frame.debug();

        // The centered child should align in the parent's area
        assert_eq!(rendered_size, Size::new(5, 3));
    }
}

#[cfg(test)]
mod integration_test {
    use crate::frame::{Frame, FrameDebug};
    use crate::renderable::container_renderer::{
        ContainerDirection, ContainerProps, ContainerRenderer,
    };
    use crate::renderable::framed_renderer::FramedRenderer;
    use crate::renderable::text_renderer::TextRenderer;
    use crate::renderable::Renderable;
    use carcasonne_core::layout::point::Point;
    use carcasonne_core::layout::size::Size;

    #[test]
    fn test_container_with_nested_elements_framed() {
        let nested_container = Box::new(FramedRenderer::new(Box::new(ContainerRenderer::new(
            ContainerDirection::Vertical,
            vec![
                Box::new(FramedRenderer::new(Box::new(TextRenderer::new("Item 1")))),
                Box::new(FramedRenderer::new(Box::new(TextRenderer::new("Item 2")))),
                Box::new(FramedRenderer::new(Box::new(TextRenderer::new("Item 3")))),
            ],
            vec![],
        ))));

        let main_container = FramedRenderer::new(Box::new(ContainerRenderer::new(
            ContainerDirection::Horizontal,
            vec![
                nested_container,
                Box::new(FramedRenderer::new(Box::new(TextRenderer::new(
                    "Side Item",
                )))),
            ],
            vec![],
        )));

        let mut frame = Frame::new(Size::new(40, 20));
        let rendered_size = main_container.render(&mut frame, Size::new(40, 20), Point::new(0, 0));

        println!("--- Rendered Frame ---");
        frame.debug();

        assert_eq!(rendered_size.width, 23);
        assert_eq!(rendered_size.height, 13);
    }

    #[test]
    fn test_complex_layout_with_fullsize_and_centered_children_framed() {
        let child1 = Box::new(FramedRenderer::new(Box::new(TextRenderer::new(
            "Full-size Child",
        ))));
        let child2 = Box::new(FramedRenderer::new(Box::new(TextRenderer::new(
            "Centered Child",
        ))));

        let container = FramedRenderer::new(Box::new(ContainerRenderer::new(
            ContainerDirection::Vertical,
            vec![
                Box::new(FramedRenderer::new(child1)),
                Box::new(FramedRenderer::new(Box::new(ContainerRenderer::new(
                    ContainerDirection::Horizontal,
                    vec![child2],
                    vec![ContainerProps::Centered],
                )))),
            ],
            vec![ContainerProps::FullSize(ContainerDirection::Vertical)],
        )));

        let mut frame = Frame::new(Size::new(50, 20));
        let rendered_size = container.render(&mut frame, Size::new(50, 20), Point::new(0, 0));

        println!("--- Debug Complex Layout (Framed) ---");
        frame.debug();

        assert_eq!(rendered_size.width, 21);
        assert_eq!(rendered_size.height, 20);
    }

    #[test]
    fn test_mixed_renderers_with_text_and_frames_framed() {
        let container = FramedRenderer::new(Box::new(ContainerRenderer::new(
            ContainerDirection::Horizontal,
            vec![
                Box::new(FramedRenderer::new(Box::new(TextRenderer::new(
                    "Left Panel",
                )))),
                Box::new(FramedRenderer::new(Box::new(ContainerRenderer::new(
                    ContainerDirection::Vertical,
                    vec![
                        Box::new(FramedRenderer::new(Box::new(TextRenderer::new(
                            "Top Section",
                        )))),
                        Box::new(FramedRenderer::new(Box::new(TextRenderer::new(
                            "Bottom Section",
                        )))),
                    ],
                    vec![],
                )))),
            ],
            vec![ContainerProps::FullSize(ContainerDirection::Horizontal)],
        )));

        let mut frame = Frame::new(Size::new(50, 15));
        let rendered_size = container.render(&mut frame, Size::new(50, 15), Point::new(0, 0));

        println!("--- Debug Mixed Renderers (Framed) ---");
        frame.debug();

        assert_eq!(rendered_size.width, 50);
        assert_eq!(rendered_size.height, 10);
    }
}
