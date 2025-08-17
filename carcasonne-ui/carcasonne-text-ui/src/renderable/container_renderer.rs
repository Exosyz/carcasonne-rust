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
    // TODO
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
}

impl<'a> Renderable for ContainerRenderer<'a> {
    fn render(&self, frame: &mut Frame, point: Point) {
        let start_point = point;

        if let Some(ContainerProps::Contained) =
            self.contain_prop(|p| matches!(p, ContainerProps::Contained))
        {
            Size::new(2, 2);
        }

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
}
