use crate::frame::Frame;
use crate::renderable::Renderable;
use carcasonne_core::layout::node::NodeTag;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

pub struct TextRenderer<'a> {
    str: &'a str,
    tags: Vec<NodeTag>,
}

impl<'a> TextRenderer<'a> {
    pub fn new(str: &'a str, tags: Vec<NodeTag>) -> Self {
        Self { str, tags }
    }
}

impl<'a> Renderable for TextRenderer<'a> {
    fn render(&self, frame: &mut Frame, point: Point) {
        self.str
            .chars()
            .enumerate()
            .for_each(|(i, c)| frame.char(point + Point::new(i, 0), c, &self.tags));
    }

    fn size(&self) -> Size {
        Size::new(self.str.len(), 1)
    }
}
