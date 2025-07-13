use crate::action::Action;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::state::StateResult::ExitToStop;
use crate::state::{State, StateResult};

pub struct StopState {}

impl State for StopState {
    fn update(&mut self, _: Action) -> StateResult {
        ExitToStop
    }

    fn draw(&self) -> Node {
        Node::Text("Fin du jeu")
    }
    fn handle_input(&self, _: InputEvent) -> Action {
        Action::None
    }

    fn need_input(&self) -> bool {
        false
    }
}
