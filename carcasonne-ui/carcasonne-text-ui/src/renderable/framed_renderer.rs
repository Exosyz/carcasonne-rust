use crate::char_drawing::CharDrawing;
use crate::frame::Frame;
use crate::renderable::Renderable;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;
use std::cmp::min;

/// Un `FramedRenderer` est un décorateur qui entoure un élément [`Renderable`]
/// par un cadre ASCII (coins, lignes horizontales et verticales).
///
/// # Calcul de taille
/// La taille totale du cadre est la taille de l'enfant + `(2, 2)`
/// (pour les bordures gauche/droite et haut/bas).
///
/// La taille finale est toujours limitée par `parent_available_size`.
///
/// # Rendu
/// - Si `parent_available_size.width < 2` ou `parent_available_size.height < 2`,
///   rien n'est dessiné.
/// - Sinon, les bordures sont tracées avec [`CharDrawing`] et l'enfant est rendu
///   à l'intérieur, décalé d’un point `(1, 1)`.
pub struct FramedRenderer<'a> {
    child: Box<dyn Renderable + 'a>,
}

impl<'a> FramedRenderer<'a> {
    pub fn new(child: Box<dyn Renderable + 'a>) -> Self {
        Self { child }
    }
}

impl<'a> Renderable for FramedRenderer<'a> {
    fn render(&self, frame: &mut Frame, parent_available_size: Size, point: Point) -> Size {
        let outer_size = self.size(parent_available_size);

        if outer_size.width < 2 || outer_size.height < 2 {
            return Size::new(0, 0);
        }

        let (x0, y0) = (point.x, point.y);
        let (x1, y1) = (x0 + outer_size.width - 1, y0 + outer_size.height - 1);

        let h_char = CharDrawing::Horizontal.into();
        let v_char = CharDrawing::Vertical.into();

        for x in x0..=x1 {
            let top_char = match x {
                x if x == x0 => CharDrawing::CornerTopLeft.into(),
                x if x == x1 => CharDrawing::CornerTopRight.into(),
                _ => h_char,
            };
            let bottom_char = match x {
                x if x == x0 => CharDrawing::CornerBottomLeft.into(),
                x if x == x1 => CharDrawing::CornerBottomRight.into(),
                _ => h_char,
            };

            frame.char_simple(Point::new(x, y0), top_char);
            if y1 != y0 {
                frame.char_simple(Point::new(x, y1), bottom_char);
            }
        }

        for y in (y0 + 1)..y1 {
            frame.char_simple(Point::new(x0, y), v_char);
            frame.char_simple(Point::new(x1, y), v_char);
        }

        self.child.render(
            frame,
            parent_available_size - Size::new(2, 2),
            Point::new(x0 + 1, y0 + 1),
        );

        outer_size
    }

    fn size(&self, parent_available_size: Size) -> Size {
        if parent_available_size.width < 2 || parent_available_size.height < 2 {
            return self.child.size(parent_available_size);
        }

        let size = self.child.size(parent_available_size - Size::new(2, 2)) + Size::new(2, 2);

        Size::new(
            min(size.width, parent_available_size.width),
            min(size.height, parent_available_size.height),
        )
    }

    fn debug(&self, tabs: usize) -> String {
        let ident = "\t".repeat(tabs);
        let indent_inner = "\t".repeat(tabs + 1);

        let child_debug = self.child.debug(tabs + 2);
        format!(
            "{ident}FramedRenderer {{\n\
             {indent_inner}child: {{\n{child_debug}\
             {ident}}}}}",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::TextRenderer;

    #[test]
    fn test_size_adds_frame() {
        let child = TextRenderer::new("Hi");
        let framed = FramedRenderer::new(Box::new(child));
        let size = framed.size(Size::new(10, 10));
        assert_eq!(size, Size::new(4, 3)); // "Hi" = (2,1) + (2,2)
    }

    #[test]
    fn test_size_clamped_to_parent() {
        let child = TextRenderer::new("LongText");
        let framed = FramedRenderer::new(Box::new(child));
        let size = framed.size(Size::new(5, 2));
        assert_eq!(size, Size::new(5, 2)); // clampé au parent
    }

    #[test]
    fn test_render_small_parent_does_nothing() {
        let child = TextRenderer::new("X");
        let framed = FramedRenderer::new(Box::new(child));
        let mut frame = Frame::new(Size::new(1, 1));
        framed.render(&mut frame, Size::new(1, 1), Point::new(0, 0));

        // Rien ne doit être dessiné
        for row in &frame.cells {
            for cell in row {
                assert_eq!(cell.symbol, ' ');
            }
        }
    }

    #[test]
    fn test_render_draws_frame_and_child() {
        let mut frame = Frame::new(Size::new(6, 3));
        let child = TextRenderer::new("Hi");
        let framed = FramedRenderer::new(Box::new(child));

        framed.render(&mut frame, Size::new(6, 3), Point::new(0, 0));

        // Coins
        assert_eq!(frame.cells[0][0].symbol, '┌');
        assert_eq!(frame.cells[0][3].symbol, '┐');
        assert_eq!(frame.cells[2][0].symbol, '└');
        assert_eq!(frame.cells[2][3].symbol, '┘');

        // Texte intérieur
        assert_eq!(frame.cells[1][1].symbol, 'H');
        assert_eq!(frame.cells[1][2].symbol, 'i');
    }

    #[test]
    fn test_render_offset_point() {
        let mut frame = Frame::new(Size::new(8, 5));
        let child = TextRenderer::new("Yo");
        let framed = FramedRenderer::new(Box::new(child));

        // Décalé à (2,1)
        framed.render(&mut frame, Size::new(8, 5), Point::new(2, 1));

        assert_eq!(frame.cells[2][3].symbol, 'Y'); // (2+1,1+1)
        assert_eq!(frame.cells[2][4].symbol, 'o');
    }
}
