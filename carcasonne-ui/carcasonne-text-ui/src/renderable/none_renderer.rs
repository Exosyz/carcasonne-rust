use crate::frame::Frame;
use crate::renderable::Renderable;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

/// A renderer that produces no output.
///
/// `NoneRenderer` is useful as a placeholder when a `Renderable` is required,
/// but nothing should actually be drawn. Its reported size is always `(0,0)`,
/// and calling [`Renderable::render`] has no effect.
pub struct NoneRenderer;

impl Renderable for NoneRenderer {
    fn render(&self, _frame: &mut Frame, _parent_available_size: Size, _point: Point) {}

    fn size(&self, _parent_available_size: Size) -> Size {
        Size::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use carcasonne_core::layout::point::Point;
    use carcasonne_core::layout::size::Size;

    #[test]
    fn size_is_always_zero() {
        let r = NoneRenderer;
        assert_eq!(r.size(Size::new(50, 50)), Size::new(0, 0));
    }

    #[test]
    fn render_does_nothing_on_non_empty_frame() {
        let mut frame = Frame::new(Size::new(2, 2));
        let before = frame.cells.clone();

        let r = NoneRenderer;
        r.render(&mut frame, Size::new(10, 10), Point::new(1, 1));

        assert_eq!(frame.cells, before);
    }

    #[test]
    fn render_on_empty_frame_is_safe() {
        let mut frame = Frame::new(Size::new(0, 0));
        let r = NoneRenderer;
        r.render(&mut frame, Size::new(10, 10), Point::new(0, 0));
        // No panic, still empty
        assert!(frame.cells.is_empty());
    }
}
