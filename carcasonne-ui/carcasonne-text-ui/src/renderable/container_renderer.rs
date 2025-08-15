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
    fn add_child(&mut self, child: Box<dyn Renderable + 'a>) -> &Self {
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
                        Size::new(acc.width.max(s.width), acc.height + s.height)
                    }),
                ContainerDirection::Vertical => self
                    .childs
                    .iter()
                    .map(|e| e.size())
                    .fold(Size::new(0, 0), |acc, s| {
                        Size::new(acc.width + s.width, acc.height.max(s.height))
                    }),
            }
        };
        size + contained_size
    }
}
