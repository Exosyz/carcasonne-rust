use crate::action::Action;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::state::StateResult::Exit;
use crate::state::{State, StateResult};

pub struct StopState {}

impl State for StopState {
    fn update(&mut self, _: Action) -> StateResult {
        Exit
    }

    fn draw(&'_ self) -> Node<'_> {
        Node::Text("Fin du jeu")
    }
    fn handle_input(&self, _: InputEvent) -> Action {
        Action::None
    }

    fn need_input(&self) -> bool {
        false
    }
}
