use crate::renderable::container_renderer::{ContainerDirection, ContainerRenderer};
use crate::renderable::helpers::renderable_helper::RenderableProps;
use carcasonne_core::layout::size::Size;

pub struct LayoutAnalysis {
    pub full_size_child_size: Option<Size>,
    pub normal_child_size: Size,
}

pub struct LayoutAnalyser<'a> {
    container_renderer: &'a ContainerRenderer<'a>,
}

impl<'a> LayoutAnalyser<'a> {
    pub fn new(container_renderer: &'a ContainerRenderer<'a>) -> Self {
        Self { container_renderer }
    }

    pub fn analyse(&self, container_available_size: Size) -> LayoutAnalysis {
        let mut normal_child_size = Size::new(0, 0);
        let mut full_size_count = 0;

        for c in self.container_renderer.children.iter() {
            if c.has_full_size_in_direction(self.container_renderer.direction) {
                full_size_count += 1;
            } else {
                let child_size = c.size(container_available_size);

                match self.container_renderer.direction {
                    ContainerDirection::Horizontal => {
                        normal_child_size.width += child_size.width;
                        normal_child_size.height = child_size.height.max(normal_child_size.height);
                    }
                    ContainerDirection::Vertical => {
                        normal_child_size.width = child_size.width.max(normal_child_size.width);
                        normal_child_size.height += child_size.height;
                    }
                }
            }
        }

        let remaining_size = match self.container_renderer.direction {
            ContainerDirection::Horizontal => Size::new(
                container_available_size
                    .width
                    .saturating_sub(normal_child_size.width),
                container_available_size.height,
            ),
            ContainerDirection::Vertical => Size::new(
                container_available_size.width,
                container_available_size
                    .height
                    .saturating_sub(normal_child_size.height),
            ),
        };

        if full_size_count == 0 {
            return LayoutAnalysis {
                full_size_child_size: None,
                normal_child_size,
            };
        }

        let full_size_child_size = match self.container_renderer.direction {
            ContainerDirection::Horizontal => Size::new(
                remaining_size.width / full_size_count,
                container_available_size.height,
            ),
            ContainerDirection::Vertical => Size::new(
                container_available_size.width,
                remaining_size.height / full_size_count,
            ),
        };

        LayoutAnalysis {
            full_size_child_size: Some(full_size_child_size),
            normal_child_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ContainerDirection, ContainerRenderer, LayoutAnalyser};
    use crate::renderable::container_renderer::ContainerProps;
    use crate::renderable::mock_renderer::MockRenderable;
    use carcasonne_core::layout::size::Size;

    #[test]
    fn test_layout_analyser_single_child_horizontal() {
        let child = Box::new(MockRenderable::new(Size::new(5, 3)));
        let container = ContainerRenderer::new(ContainerDirection::Horizontal, vec![child], vec![]);

        let analyser = LayoutAnalyser::new(&container);
        let analysis = analyser.analyse(Size::new(20, 15));

        assert_eq!(analysis.full_size_child_size, None);
        assert_eq!(analysis.normal_child_size, Size::new(5, 3));
    }

    #[test]
    fn test_layout_analyser_with_full_size_child_vertical() {
        let full_size_child = Box::new(ContainerRenderer::new(
            ContainerDirection::Horizontal,
            vec![],
            vec![ContainerProps::FullSize(ContainerDirection::Horizontal)],
        ));
        let container = ContainerRenderer::new(
            ContainerDirection::Horizontal,
            vec![full_size_child],
            vec![],
        );

        let analyser = LayoutAnalyser::new(&container);
        let analysis = analyser.analyse(Size::new(20, 20));

        assert_eq!(analysis.full_size_child_size, Some(Size::new(20, 20)));
        assert_eq!(analysis.normal_child_size, Size::new(0, 0));
    }

    #[test]
    fn test_layout_analyser_multiple_children_horizontal() {
        let child1 = Box::new(MockRenderable::new(Size::new(5, 3)));
        let child2 = Box::new(MockRenderable::new(Size::new(8, 4)));

        let container =
            ContainerRenderer::new(ContainerDirection::Horizontal, vec![child1, child2], vec![]);

        let analyser = LayoutAnalyser::new(&container);
        let analysis = analyser.analyse(Size::new(30, 10));

        assert_eq!(analysis.full_size_child_size, None);
        assert_eq!(analysis.normal_child_size, Size::new(13, 4));
    }

    #[test]
    fn test_layout_analyser_clamping_to_parent_size() {
        let child = Box::new(MockRenderable::new(Size::new(10, 3)));
        let container = ContainerRenderer::new(ContainerDirection::Horizontal, vec![child], vec![]);

        let analyser = LayoutAnalyser::new(&container);
        let analysis = analyser.analyse(Size::new(5, 3));

        assert_eq!(analysis.full_size_child_size, None);
        assert_eq!(analysis.normal_child_size, Size::new(5, 3));
    }
}
