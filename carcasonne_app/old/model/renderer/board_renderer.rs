use crate::board::Board;
use crate::renderer::ascii_renderer::AsciiRenderer;
use crate::renderer::base_renderer::{GraphicRenderer, UnicodeRenderer};

pub trait BoardRenderer {
    fn render(&self, Board: &Board);
}
impl BoardRenderer for UnicodeRenderer {
    fn render(&self, Board: &Board) {
        panic!("Not implemented");
    }
}

impl BoardRenderer for GraphicRenderer {
    fn render(&self, Board: &Board) {
        panic!("Not implemented");
    }
}

impl BoardRenderer for AsciiRenderer {
    fn render(&self, Board: &Board) {
        panic!("Not implemented");
    }
}
