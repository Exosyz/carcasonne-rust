use crate::frame::Frame;
use crate::renderable::Renderable;
use carcasonne_core::layout::node::NodeTag;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

pub struct CharRenderer {
    char: char,
    tags: Vec<NodeTag>,
}

impl CharRenderer {
    pub fn new(char: char, tags: Vec<NodeTag>) -> Self {
        Self { char, tags }
    }
}

impl Renderable for CharRenderer {
    fn render(&self, frame: &mut Frame, point: Point) {
        frame.char(point, self.char, &self.tags);
    }

    fn size(&self) -> Size {
        Size::new(1, 1)
    }
}
