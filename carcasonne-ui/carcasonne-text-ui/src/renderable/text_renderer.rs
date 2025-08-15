use crate::frame::Frame;
use crate::renderable::Renderable;
use carcasonne_core::layout::node::NodeTag;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

pub struct TextRenderer<'a> {
    str: &'a str,
    tags: Vec<NodeTag>,
    cursor: Option<Point>,
}

impl<'a> TextRenderer<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            tags: Vec::new(),
            cursor: None,
        }
    }

    pub fn push_tags(mut self, tags: Vec<NodeTag>) -> Self {
        tags.into_iter().for_each(|node| {
            self.tags.push(node);
        });
        self
    }

    pub fn set_cursor(mut self, cursor: Point) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

impl<'a> Renderable for TextRenderer<'a> {
    fn render(&self, frame: &mut Frame, point: Point) {
        if let Some(cursor) = self.cursor {
            frame.set_cursor(Some(point + cursor));
        }
        self.str.lines().enumerate().for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, c)| frame.char(point + Point::new(x, y), c, &self.tags));
        })
    }

    fn size(&self) -> Size {
        let max_line_length = self.str.lines().map(|l| l.len()).max().unwrap_or(0);
        Size::new(max_line_length, self.str.lines().count())
    }
}
