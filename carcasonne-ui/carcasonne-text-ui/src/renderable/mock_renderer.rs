use crate::frame::Frame;
use crate::renderable::container_renderer::ContainerRenderer;
use crate::renderable::Renderable;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

pub struct MockRenderable {
    size: Size,
}

impl MockRenderable {
    pub fn new(size: Size) -> Self {
        Self { size }
    }
}

impl Renderable for MockRenderable {
    fn render(&self, frame: &mut Frame, parent_available_size: Size, point: Point) -> Size {
        for y in 0..self.size.height.min(parent_available_size.height) {
            for x in 0..self.size.width.min(parent_available_size.width) {
                frame.char(point + Point::new(x, y), '#', &[]);
            }
        }

        Size {
            width: self.size.width.min(parent_available_size.width),
            height: self.size.height.min(parent_available_size.height),
        }
    }

    fn size(&self, parent_available_size: Size) -> Size {
        Size::new(
            self.size.width.min(parent_available_size.width),
            self.size.height.min(parent_available_size.height),
        )
    }

    fn debug(&self, tabs: usize) -> String {
        let indent = "\t".repeat(tabs);
        format!("{indent}MockRenderable {{ size: {:?} }}", self.size)
    }

    fn as_container(&self) -> Option<&ContainerRenderer<'_>> {
        None
    }
}
