pub mod place_tile_state;
pub mod select_tile_state;

use crate::action::Action;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::state::game_state::stop_state::StopState;
use crate::state::StateResult::{Exit, Stay, Transition};
use crate::state::{State, StateResult};

pub struct GameRunningState {
    pub current_state: Option<Box<dyn State>>,
}

impl GameRunningState {
    pub fn new(default_state: Box<dyn State>) -> Self {
        Self {
            current_state: Some(default_state),
        }
    }
}

impl State for GameRunningState {
    fn update(&mut self, action: Action) -> StateResult {
        match &mut self.current_state {
            Some(current_state) => match current_state.update(action) {
                Transition(new_state) => {
                    self.current_state = Some(new_state);
                    Stay(true)
                }
                Stay(need_input) => Stay(need_input),
                Exit => Transition(Box::new(StopState {})),
            },
            None => unreachable!("You cannot have a None substate"),
        }
    }

    fn draw(&'_ self) -> Node<'_> {
        match &self.current_state {
            Some(state) => {
                let draw_result = state.draw();

                if let Node::None = draw_result {
                    return Node::None;
                }

                Node::VerticalContainer(vec![
                    Node::Text("Game Is Running"),
                    Node::Framed(Box::new(state.draw())),
                ])
            }
            None => Node::None,
        }
    }
    fn handle_input(&self, event: InputEvent) -> Action {
        match &self.current_state {
            Some(state) => state.handle_input(event),
            None => Action::None,
        }
    }

    fn need_input(&self) -> bool {
        match &self.current_state {
            Some(state) => state.need_input(),
            None => false,
        }
    }
}
