use crate::frame::Frame;
use crate::renderable::{fit_within_bounds, Renderable};
use carcasonne_core::layout::node::NodeTag;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

/// Rendu d'un seul caractère avec un ensemble de tags.
///
/// Un `CharRenderer` encapsule un caractère (`symbol`) et une liste de tags
/// [`NodeTag`] qui seront appliqués lors du rendu.
pub struct CharRenderer {
    symbol: char,
    tags: Vec<NodeTag>,
}

impl CharRenderer {
    pub fn new(symbol: char, tags: Vec<NodeTag>) -> Self {
        Self { symbol, tags }
    }
}

impl Renderable for CharRenderer {
    fn render(&self, frame: &mut Frame, parent_available_size: Size, point: Point) {
        if parent_available_size.width < 1 || parent_available_size.height < 1 {
            return;
        }
        frame.char(point, self.symbol, &self.tags);
    }

    fn size(&self, parent_available_size: Size) -> Size {
        fit_within_bounds(Size::new(1, 1), parent_available_size)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::cell::CellTag;
    use crate::frame::Frame;
    use carcasonne_core::layout::node::NodeTag;
    use carcasonne_core::layout::point::Point;
    use carcasonne_core::layout::size::Size;

    #[test]
    fn test_size_is_one_by_one() {
        let renderer = CharRenderer::new('X', vec![]);
        assert_eq!(renderer.size(Size::new(50, 50)), Size::new(1, 1));
    }

    #[test]
    fn test_render_simple_char() {
        let mut frame = Frame::new(Size::new(2, 2));
        let renderer = CharRenderer::new('Z', vec![]);
        renderer.render(&mut frame, Size::new(2, 2), Point::new(0, 0));

        let cell = frame.cells[0][0].clone();
        assert_eq!(cell.symbol, 'Z');
        assert!(cell.tags.is_empty());
    }

    #[test]
    fn test_render_char_with_tags() {
        let mut frame = Frame::new(Size::new(2, 2));
        let renderer = CharRenderer::new('B', vec![NodeTag::Bold, NodeTag::Underline]);
        renderer.render(&mut frame, Size::new(2, 2), Point::new(1, 1));

        let cell = frame.cells[1][1].clone();
        assert_eq!(cell.symbol, 'B');
        assert!(cell.tags.contains(&CellTag::Bold));
        assert!(cell.tags.contains(&CellTag::Underline));
    }

    #[test]
    fn test_render_skips_if_parent_too_small() {
        let mut frame = Frame::new(Size::new(1, 1));
        let renderer = CharRenderer::new('K', vec![]);
        // parent_available_size est 0x0 → pas de rendu
        renderer.render(&mut frame, Size::new(0, 0), Point::new(0, 0));

        // la cellule doit rester vide (Frame initialise à ' ')
        let cell = frame.cells[0][0].clone();
        assert_eq!(cell.symbol, ' ');
        assert!(cell.tags.is_empty());
    }
}
