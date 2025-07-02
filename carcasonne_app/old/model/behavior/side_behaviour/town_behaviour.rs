use crate::behavior::side_behaviour::{Position, SectionId, SideBehavior};
use crate::direction::Direction;
use crate::renderer::ascii_renderer::RenderChar;
use crate::side::{Side, SideKind};

#[derive(Debug, Clone, Default)]
pub struct TownBehavior;

impl SideBehavior for TownBehavior {
    fn handle_side(&self, side: &Side) -> Option<(SectionId, Position, RenderChar)> {
        let coordinates = match side.direction {
            Direction::North => (0, 1),
            Direction::East => (1, 2),
            Direction::South => (2, 1),
            Direction::West => (1, 0),
        };

        Some((side.section, coordinates, RenderChar::Town))
    }
    fn handle_pair(&self, side1: &Side, side2: &Side) -> Option<(Position, RenderChar)> {
        if side1.kind == SideKind::Town && side2.kind == SideKind::Town {
            let coordinates = match (side1.direction, side2.direction) {
                (Direction::North, Direction::East) => Some((0, 2)),
                (Direction::East, Direction::South) => Some((2, 2)),
                (Direction::South, Direction::West) => Some((2, 0)),
                (Direction::North, Direction::West) => Some((0, 0)),
                _ => None,
            };
            if let Some(coordinates) = coordinates {
                Some((coordinates, RenderChar::Town))
            } else {
                None
            }
        } else {
            None
        }
    }
}
