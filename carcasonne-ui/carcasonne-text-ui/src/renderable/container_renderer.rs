use crate::char_drawing::CharDrawing;
use crate::frame::Frame;
use crate::renderable::{get_node_renderer, Renderable};
use carcasonne_core::layout::node::Node;
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;

pub enum ContainerDirection {
    Horizontal,
    Vertical,
}

pub enum ContainerProps {
    Size(usize, usize),
    Contained,
    Centered,
    Top,
    Bottom,
    Left,
    Right,
}

pub struct ContainerRenderer<'a> {
    childs: Vec<Box<dyn Renderable + 'a>>,
    direction: ContainerDirection,
    props: Vec<ContainerProps>,
}

impl<'a> ContainerRenderer<'a> {
    pub fn new(direction: ContainerDirection) -> Self {
        Self {
            direction,
            childs: vec![],
            props: vec![],
        }
    }
    fn add_child(&mut self, child: Box<dyn Renderable + 'a>) -> &mut Self {
        self.childs.push(child);
        self
    }

    pub fn add_nodes(mut self, nodes: Vec<Node<'a>>) -> Self {
        nodes.into_iter().for_each(|node| {
            self.add_child(get_node_renderer(node));
        });
        self
    }

    pub fn add_props(mut self, prop: ContainerProps) -> Self {
        self.props.push(prop);
        self
    }

    fn contain_prop<Predicate>(&self, predicate: Predicate) -> Option<&ContainerProps>
    where
        Predicate: Fn(&&ContainerProps) -> bool,
    {
        self.props.iter().find(|p| predicate(p))
    }

    fn render_container(&self, frame: &mut Frame, point: Point) {
        let outer_size = self.size();

        if outer_size.width < 2 || outer_size.height < 2 {
            return;
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
    }
}

impl<'a> Renderable for ContainerRenderer<'a> {
    fn render(&self, frame: &mut Frame, point: Point) {
        let start_point = if let Some(ContainerProps::Contained) =
            self.contain_prop(|p| matches!(p, ContainerProps::Contained))
        {
            self.render_container(frame, point);
            point + Point::new(1, 1)
        } else {
            point
        };

        match self.direction {
            ContainerDirection::Horizontal => {
                let mut current_x = start_point.x;
                self.childs.iter().for_each(|elem| {
                    let size = elem.size();
                    elem.render(frame, Point::new(current_x, start_point.y));
                    current_x += size.width;
                })
            }
            ContainerDirection::Vertical => {
                let mut current_y = start_point.y;
                self.childs.iter().for_each(|elem| {
                    let size = elem.size();
                    elem.render(frame, Point::new(start_point.x, current_y));
                    current_y += size.height;
                })
            }
        }
    }
    fn size(&self) -> Size {
        let contained_size = if let Some(ContainerProps::Contained) =
            self.contain_prop(|p| matches!(p, ContainerProps::Contained))
        {
            Size::new(2, 2)
        } else {
            Size::new(0, 0)
        };

        let size = if let Some(ContainerProps::Size(width, height)) =
            self.contain_prop(|p| matches!(p, ContainerProps::Size(_, _)))
        {
            Size::new(*width, *height)
        } else {
            match self.direction {
                ContainerDirection::Horizontal => self
                    .childs
                    .iter()
                    .map(|e| e.size())
                    .fold(Size::new(0, 0), |acc, s| {
                        Size::new(acc.width + s.width, acc.height.max(s.height))
                    }),
                ContainerDirection::Vertical => self
                    .childs
                    .iter()
                    .map(|e| e.size())
                    .fold(Size::new(0, 0), |acc, s| {
                        Size::new(acc.width.max(s.width), acc.height + s.height)
                    }),
            }
        };
        size + contained_size
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::Frame;
    use carcasonne_core::layout::node::Node;
    use carcasonne_core::layout::point::Point;
    use carcasonne_core::layout::size::Size;

    #[test]
    fn empty_container_has_zero_size() {
        let c = ContainerRenderer::new(ContainerDirection::Horizontal);
        assert_eq!(c.size(), Size::new(0, 0));

        let mut frame = Frame::new(Size::new(5, 3));
        c.render(&mut frame, Point::new(0, 0));
        for y in 0..3 {
            for x in 0..5 {
                assert_eq!(frame.cells[y][x].symbol, ' ');
            }
        }
    }

    #[test]
    fn horizontal_container_renders_children_side_by_side() {
        let nodes = vec![Node::Char('A'), Node::Char('B')];
        let c = ContainerRenderer::new(ContainerDirection::Horizontal)
            .add_props(ContainerProps::Contained)
            .add_nodes(nodes);

        let size = c.size();
        assert!(size.width >= 2);
        assert!(size.height >= 1);

        let mut frame = Frame::new(Size::new(size.width, size.height));
        c.render(&mut frame, Point::new(0, 0));

        // Vérifie que les caractères sont présents
        let mut found_a = false;
        let mut found_b = false;
        for y in 0..size.height {
            for x in 0..size.width {
                let sym = frame.cells[y][x].symbol;
                if sym == 'A' {
                    found_a = true;
                }
                if sym == 'B' {
                    found_b = true;
                }
            }
        }
        assert!(found_a);
        assert!(found_b);
    }

    #[test]
    fn vertical_container_renders_children_stacked() {
        let nodes = vec![Node::Char('X'), Node::Char('Y')];
        let c = ContainerRenderer::new(ContainerDirection::Vertical).add_nodes(nodes);

        let size = c.size();
        assert!(size.width >= 1);
        assert!(size.height >= 2);

        let mut frame = Frame::new(Size::new(size.width, size.height));
        c.render(&mut frame, Point::new(0, 0));

        let mut found_x = false;
        let mut found_y = false;
        for y in 0..size.height {
            for x in 0..size.width {
                let sym = frame.cells[y][x].symbol;
                if sym == 'X' {
                    found_x = true;
                }
                if sym == 'Y' {
                    found_y = true;
                }
            }
        }
        assert!(found_x);
        assert!(found_y);
    }

    #[test]
    fn contained_adds_extra_size() {
        let nodes = vec![Node::Char('C')];
        let c = ContainerRenderer::new(ContainerDirection::Horizontal)
            .add_nodes(nodes)
            .add_props(ContainerProps::Contained);

        let size = c.size();
        assert!(size.width >= 2);
        assert!(size.height >= 2);
    }

    #[test]
    fn size_prop_overrides_children() {
        let nodes = vec![Node::Char('Z')];
        let c = ContainerRenderer::new(ContainerDirection::Vertical)
            .add_nodes(nodes)
            .add_props(ContainerProps::Size(5, 4));

        assert_eq!(c.size(), Size::new(5, 4));
    }

    #[test]
    fn size_and_contained_are_combined() {
        let nodes = vec![Node::Char('Q')];
        let c = ContainerRenderer::new(ContainerDirection::Vertical)
            .add_nodes(nodes)
            .add_props(ContainerProps::Size(3, 2))
            .add_props(ContainerProps::Contained);

        let size = c.size();
        assert!(size.width >= 5); // 3 + 2 contained
        assert!(size.height >= 4); // 2 + 2 contained
    }

    #[test]
    fn contained_renders_frame_edges() {
        let nodes = vec![Node::Char('A')];
        let c = ContainerRenderer::new(ContainerDirection::Horizontal)
            .add_nodes(nodes)
            .add_props(ContainerProps::Contained);

        let size = c.size();
        let mut frame = Frame::new(Size::new(size.width, size.height));
        c.render(&mut frame, Point::new(0, 0));

        // Vérifie les coins
        assert_eq!(frame.cells[0][0].symbol, CharDrawing::CornerTopLeft.into());
        assert_eq!(
            frame.cells[0][size.width - 1].symbol,
            CharDrawing::CornerTopRight.into()
        );
        assert_eq!(
            frame.cells[size.height - 1][0].symbol,
            CharDrawing::CornerBottomLeft.into()
        );
        assert_eq!(
            frame.cells[size.height - 1][size.width - 1].symbol,
            CharDrawing::CornerBottomRight.into()
        );

        // Vérifie les lignes horizontales
        for x in 1..(size.width - 1) {
            assert_eq!(frame.cells[0][x].symbol, CharDrawing::Horizontal.into());
            assert_eq!(
                frame.cells[size.height - 1][x].symbol,
                CharDrawing::Horizontal.into()
            );
        }

        // Vérifie les lignes verticales
        for y in 1..(size.height - 1) {
            assert_eq!(frame.cells[y][0].symbol, CharDrawing::Vertical.into());
            assert_eq!(
                frame.cells[y][size.width - 1].symbol,
                CharDrawing::Vertical.into()
            );
        }
    }

    #[test]
    fn horizontal_container_children_offset_by_contained() {
        let nodes = vec![Node::Char('A'), Node::Char('B')];
        let c = ContainerRenderer::new(ContainerDirection::Horizontal)
            .add_nodes(nodes)
            .add_props(ContainerProps::Contained);

        let mut frame = Frame::new(c.size());
        c.render(&mut frame, Point::new(0, 0));

        // Le premier enfant doit être à (1,1) à cause du padding contained
        assert_eq!(frame.cells[1][1].symbol, 'A');
        assert_eq!(frame.cells[1][2].symbol, 'B');
    }

    #[test]
    fn vertical_container_children_offset_by_contained() {
        let nodes = vec![Node::Char('X'), Node::Char('Y')];
        let c = ContainerRenderer::new(ContainerDirection::Vertical)
            .add_nodes(nodes)
            .add_props(ContainerProps::Contained);

        let mut frame = Frame::new(c.size());
        c.render(&mut frame, Point::new(0, 0));

        // Les enfants doivent commencer à (1,1)
        assert_eq!(frame.cells[1][1].symbol, 'X');
        assert_eq!(frame.cells[2][1].symbol, 'Y');
    }

    #[test]
    fn horizontal_size_accumulates_children_width() {
        let nodes = vec![Node::Char('A'), Node::Char('B')];
        let c = ContainerRenderer::new(ContainerDirection::Horizontal).add_nodes(nodes);

        let size = c.size();

        let expected_width = 2;
        assert_eq!(size.width, expected_width);
    }

    #[test]
    fn vertical_size_accumulates_children_height() {
        let nodes = vec![Node::Char('X'), Node::Char('Y')];
        let c = ContainerRenderer::new(ContainerDirection::Vertical).add_nodes(nodes);

        let size = c.size();

        let expected_height = 2;
        let expected_width = 1;
        assert_eq!(size.height, expected_height);
        assert_eq!(size.width, expected_width);
    }
}
