use crate::frame::Frame;
use crate::renderable::Renderable;
use carcasonne_core::layout::node::NodeTag;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;
use std::cmp::min;

/// Rendu d'un texte multi-ligne avec un ensemble de tags.
///
/// Un `TextRenderer` permet d'afficher une chaîne de caractères (`&str`) sur un
/// [`Frame`], avec un style homogène appliqué à tous les caractères.
///
/// # Taille
/// La taille reportée par [`Renderable::size`] correspond à :
/// - largeur = longueur de la ligne la plus longue
/// - hauteur = nombre de lignes
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
    fn render(&self, frame: &mut Frame, parent_available_size: Size, point: Point) {
        if let Some(cursor) = self.cursor {
            frame.set_cursor(Some(point + cursor));
        }
        self.str
            .lines()
            .enumerate()
            .filter(|(j, _)| j < &parent_available_size.height)
            .for_each(|(j, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(i, _)| i < &parent_available_size.width)
                    .for_each(|(i, c)| frame.char(point + Point::new(i, j), c, &self.tags))
            });
    }

    fn size(&self, parent_available_size: Size) -> Size {
        let max_line_length = self.str.lines().map(|l| l.len()).max().unwrap_or(0);
        Size::new(
            min(max_line_length, parent_available_size.width),
            min(self.str.lines().count(), parent_available_size.height),
        )
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
    fn test_size_single_line() {
        let renderer = TextRenderer::new("Hello");
        assert_eq!(renderer.size(Size::new(50, 50)), Size::new(5, 1));
    }

    #[test]
    fn test_size_clamped_to_parent() {
        let renderer = TextRenderer::new("Hello\nWorld!");
        let parent_size = Size::new(4, 1);
        assert_eq!(renderer.size(parent_size), Size::new(4, 1));
    }

    #[test]
    fn test_size_multi_line() {
        let renderer = TextRenderer::new("Hi\nRust");
        assert_eq!(renderer.size(Size::new(50, 50)), Size::new(4, 2));
    }

    #[test]
    fn test_render_text() {
        let mut frame = Frame::new(Size::new(5, 1));
        let renderer = TextRenderer::new("Test");
        renderer.render(&mut frame, Size::new(5, 1), Point::new(0, 0));

        for (i, c) in "Test".chars().enumerate() {
            let cell = frame.cells[0][i].clone();
            assert_eq!(cell.symbol, c);
            assert!(cell.tags.is_empty());
        }
    }

    #[test]
    fn test_render_with_tags() {
        let mut frame = Frame::new(Size::new(4, 1));
        let renderer = TextRenderer::new("AB").push_tags(vec![NodeTag::Bold, NodeTag::Underline]);
        renderer.render(&mut frame, Size::new(4, 1), Point::new(0, 0));

        let cell = frame.cells[0][0].clone();
        assert_eq!(cell.symbol, 'A');
        assert!(cell.tags.contains(&CellTag::Bold));
        assert!(cell.tags.contains(&CellTag::Underline));
    }

    #[test]
    fn test_render_with_cursor() {
        let mut frame = Frame::new(Size::new(5, 2));
        let renderer = TextRenderer::new("X").set_cursor(Point::new(2, 1));
        renderer.render(&mut frame, Size::new(5, 2), Point::new(0, 0));

        let cursor = frame.cursor;
        assert_eq!(cursor, Some(Point::new(2, 1)));
    }

    #[test]
    fn test_render_out_of_bounds_is_clamped() {
        let mut frame = Frame::new(Size::new(3, 1));
        let renderer = TextRenderer::new("ABCDE");
        renderer.render(&mut frame, Size::new(3, 1), Point::new(0, 0));

        // seulement "ABC" doit être rendu
        let rendered: String = frame.cells[0].iter().map(|c| c.symbol).collect();
        assert_eq!(rendered, "ABC");
    }

    #[test]
    fn test_render_multiline_clamped() {
        let mut frame = Frame::new(Size::new(4, 1));
        let renderer = TextRenderer::new("Line1\nLine2\nLine3");
        renderer.render(&mut frame, Size::new(4, 1), Point::new(0, 0));

        // une seule ligne de 4 caractères doit apparaître
        let rendered: String = frame.cells[0].iter().map(|c| c.symbol).collect();
        assert_eq!(rendered, "Line");
    }
}
