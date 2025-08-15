use crate::char_drawing::CharDrawing;
use crate::frame::Frame;
use crate::renderable::container_renderer::{ContainerDirection, ContainerRenderer};
use crate::renderable::Renderable;
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::node::NodeTag::Underline;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

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
    fn render(&self, frame: &mut Frame, point: Point) {
        let built_menu = self
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
            .collect();

        ContainerRenderer::new(ContainerDirection::Vertical)
            .add_nodes(built_menu)
            .render(frame, point);
    }

    fn size(&self) -> Size {
        Size::new(
            self.elems.iter().map(|s| s.len()).max().unwrap_or(0) + 2,
            self.elems.len(),
        )
    }
}
