use crate::action::Action;
use crate::context::game_context::GameContext;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::state::game_state::game_running_state::select_tile_state::SelectTileState;
use crate::state::StateResult::Transition;
use crate::state::{State, StateResult};

pub struct SelectPlayerState {}

impl SelectPlayerState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for SelectPlayerState {
    fn update(&mut self, context: &mut GameContext, _action: Action) -> StateResult {
        context.set_next_player();
        Transition(Box::new(SelectTileState::new()))
    }
    fn draw(&'_ self) -> Node<'_> {
        Node::None
    }

    fn handle_input(&self, _: InputEvent) -> Action {
        Action::None
    }

    fn need_input(&self) -> bool {
        false
    }
}
