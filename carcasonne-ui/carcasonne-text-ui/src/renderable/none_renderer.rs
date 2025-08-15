use crate::frame::Frame;
use crate::renderable::Renderable;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

pub struct NoneRenderer;

impl Renderable for NoneRenderer {
    fn render(&self, _frame: &mut Frame, _point: Point) {}
    fn size(&self) -> Size {
        Size::new(0, 0)
    }
}
