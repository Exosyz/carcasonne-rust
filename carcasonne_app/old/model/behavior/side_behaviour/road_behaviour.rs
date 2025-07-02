use crate::behavior::side_behaviour::{Position, SectionId, SideBehavior};
use crate::direction::Direction;
use crate::renderer::ascii_renderer::RenderChar;
use crate::side::{Side, SideKind};

#[derive(Debug, Clone, Default)]
pub struct RoadBehavior;

impl SideBehavior for RoadBehavior {
    fn handle_side(&self, side: &Side) -> Option<(SectionId, Position, RenderChar)> {
        let (coordinates, char) = match side.direction {
            Direction::North => ((0, 1), RenderChar::RoadVertical),
            Direction::East => ((1, 2), RenderChar::RoadHorizontal),
            Direction::South => ((2, 1), RenderChar::RoadVertical),
            Direction::West => ((1, 0), RenderChar::RoadHorizontal),
        };

        Some((side.section, coordinates, char))
    }
    fn handle_pair(&self, side1: &Side, side2: &Side) -> Option<(Position, RenderChar)> {
        if side1.kind == SideKind::Road && side2.kind == SideKind::Road {
            let coordinates = match (side1.direction, side2.direction) {
                /*( (Direction::North, Direction::South) => Some(((1, 1), RenderChar::RoadVertical)),
                 (Direction::East, Direction::West) => Some(((2, 2), RenderChar::RoadHorizontal)),
                Direction::North, Direction::East) => Some(((2, 0), RenderChar::RoadVertical)),
                 (Direction::East, Direction::South) => Some(((2, 0), RenderChar::RoadVertical)),
                 (Direction::South, Direction::West) => Some(((2, 0), RenderChar::RoadVertical)),
                 (Direction::North, Direction::West) => Some(((0, 0), RenderChar::RoadVertical)),
                  */
                _ => None,
            };
            if let Some((coord, char)) = coordinates {
                Some((coord, char))
            } else {
                None
            }
        } else {
            None
        }
    }
}
