use crate::action::Action;
use crate::context::game_context::GameContext;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::state::game_state::game_running_state::place_tile_state::PlaceTileState;
use crate::state::StateResult::{Exit, Transition};
use crate::state::{State, StateResult};

pub struct SelectTileState {
    game_context: GameContext,
}

impl SelectTileState {
    pub fn new(game_context: GameContext) -> Self {
        Self { game_context }
    }
}

impl State for SelectTileState {
    fn update(&mut self, _action: Action) -> StateResult {
        if let Some(tile) = self.game_context.select_random_tile() {
            Transition(Box::new(PlaceTileState::new(
                std::mem::take(&mut self.game_context),
                tile,
            )))
        } else {
            Exit
        }
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
