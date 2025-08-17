use crate::frame::Frame;
use crate::renderable::Renderable;
use carcasonne_core::layout::node::NodeTag;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

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
        assert_eq!(renderer.size(), Size::new(5, 1));
    }

    #[test]
    fn test_size_multi_line() {
        let renderer = TextRenderer::new("Hi\nRust");
        assert_eq!(renderer.size(), Size::new(4, 2));
    }

    #[test]
    fn test_render_text() {
        let mut frame = Frame::new(Size::new(5, 1));
        let renderer = TextRenderer::new("Test");
        renderer.render(&mut frame, Point::new(0, 0));

        for (i, c) in "Test".chars().enumerate() {
            let cell = frame.cells[0][i].clone();
            assert_eq!(cell.symbol, c);
        }
    }

    #[test]
    fn test_render_with_tags() {
        let mut frame = Frame::new(Size::new(4, 1));
        let renderer = TextRenderer::new("AB").push_tags(vec![NodeTag::Bold, NodeTag::Underline]);
        renderer.render(&mut frame, Point::new(0, 0));

        let cell = frame.cells[0][0].clone();
        assert!(cell.tags.contains(&CellTag::Bold));
        assert!(cell.tags.contains(&CellTag::Underline));
    }

    #[test]
    fn test_render_with_cursor() {
        let mut frame = Frame::new(Size::new(5, 2));
        let renderer = TextRenderer::new("X").set_cursor(Point::new(2, 1));
        renderer.render(&mut frame, Point::new(0, 0));

        let cursor = frame.cursor;
        assert_eq!(cursor, Some(Point::new(2, 1)));
    }
}
