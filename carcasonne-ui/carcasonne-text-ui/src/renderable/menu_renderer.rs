use crate::char_drawing::CharDrawing;
use crate::frame::Frame;
use crate::renderable::container_renderer::{ContainerDirection, ContainerRenderer};
use crate::renderable::{get_node_renderer, Renderable};
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::node::NodeTag::Underline;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;
use std::cmp::min;

pub struct MenuRenderer<'a> {
    elems: Vec<&'a str>,
    selected_index: usize,
}

impl<'a> MenuRenderer<'a> {
    pub fn new(elems: Vec<&'a str>, selected_index: usize) -> Self {
        Self {
            elems,
            selected_index,
        }
    }
}

impl<'a> Renderable for MenuRenderer<'a> {
    fn render(&self, frame: &mut Frame, parent_available_size: Size, point: Point) -> Size {
        let childs = self
            .elems
            .iter()
            .enumerate()
            .map(|(i, s)| {
                Node::HorizontalContainer(vec![
                    Node::Char(if i == self.selected_index {
                        CharDrawing::SelectedRadio.into()
                    } else {
                        CharDrawing::Radio.into()
                    }),
                    Node::Char(CharDrawing::None.into()),
                    if i == self.selected_index {
                        Node::RichText(s, vec![Underline])
                    } else {
                        Node::Text(s)
                    },
                ])
            })
            .map(|node| get_node_renderer(node))
            .collect();

        ContainerRenderer::new(ContainerDirection::Vertical, childs, vec![]).render(
            frame,
            parent_available_size,
            point,
        )
    }

    fn size(&self, parent_available_size: Size) -> Size {
        Size::new(
            min(
                self.elems.iter().map(|s| s.len()).max().unwrap_or(0) + 2,
                parent_available_size.width,
            ),
            min(self.elems.len(), parent_available_size.height),
        )
    }

    fn debug(&self, size: usize) -> String {
        let ident = "\t".repeat(size);
        let indent_inner = "\t".repeat(size + 1);

        let elems = self
            .elems
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "{ident}MenuRenderer {{\n\
             {indent_inner}elems: [{elems}],\n\
             {indent_inner}selected_index: {}\n\
             {ident}}}\n",
            self.selected_index,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char_drawing::CharDrawing;
    use crate::frame::cell::CellTag;
    use crate::frame::Frame;
    #[test]
    fn test_size_clamped_to_parent() {
        let renderer = MenuRenderer::new(vec!["One", "TwoLong", "3"], 1);
        let parent_size = Size::new(4, 1);
        assert_eq!(renderer.size(parent_size), Size::new(4, 1));
    }

    #[test]
    fn menu_size_computes_width_and_height() {
        let menu = MenuRenderer::new(vec!["One", "TwoLong", "3"], 1);
        let size = menu.size(Size::new(10, 10));

        assert_eq!(size.height, 3);
        assert_eq!(size.width, "TwoLong".len() + 2); // +2 pour le radio + espace
    }

    #[test]
    fn menu_render_places_radio_symbols() {
        let menu = MenuRenderer::new(vec!["A", "B"], 1);
        let mut frame = Frame::new(menu.size(Size::new(10, 10)));

        menu.render(&mut frame, Size::new(10, 10), Point::new(0, 0));

        let first_radio = frame.cells[0][0].symbol;
        assert_eq!(first_radio, CharDrawing::Radio.into());

        let second_radio = frame.cells[1][0].symbol;
        assert_eq!(second_radio, CharDrawing::SelectedRadio.into());
    }

    #[test]
    fn menu_render_underlines_selected_item() {
        let menu = MenuRenderer::new(vec!["Item1", "Item2"], 0);
        let mut frame = Frame::new(menu.size(Size::new(10, 10)));

        menu.render(&mut frame, Size::new(10, 10), Point::new(0, 0));

        // Vérifie que l'élément sélectionné contient un tag Underline
        let mut underline_found = false;
        for y in 0..frame.size.height {
            for x in 0..frame.size.width {
                let cell = frame.cells[y][x].clone();
                if cell.tags.contains(&CellTag::Underline) {
                    underline_found = true;
                }
            }
        }
        assert!(
            underline_found,
            "Expected selected item to have Underline tag"
        );
    }
}
