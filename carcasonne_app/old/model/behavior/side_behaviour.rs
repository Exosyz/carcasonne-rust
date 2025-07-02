pub mod meadow_behaviour;
pub mod road_behaviour;
pub mod town_behaviour;

use crate::renderer::ascii_renderer::RenderChar;
use crate::side::Side;
use std::fmt::Debug;

type SectionId = usize;
type Position = (usize, usize);

pub trait SideBehavior: SideBehaviorClone + Debug {
    fn handle_side(&self, side: &Side) -> Option<(SectionId, Position, RenderChar)>;
    fn handle_pair(&self, side1: &Side, side2: &Side) -> Option<(Position, RenderChar)>;
}

pub trait SideBehaviorClone {
    fn clone_box(&self) -> Box<dyn SideBehavior>;
}

impl<T> SideBehaviorClone for T
where
    T: 'static + SideBehavior + Clone,
{
    fn clone_box(&self) -> Box<dyn SideBehavior> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SideBehavior> {
    fn clone(&self) -> Box<dyn SideBehavior> {
        self.clone_box()
    }
}
