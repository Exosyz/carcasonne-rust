use crate::action::Action;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::state::game_state::playing_state::place_tile_state::PlaceTileState;
use crate::state::game_state::playing_state::PlayingStateResult::{Continue, ExitToStop};
use crate::state::game_state::playing_state::{GameContext, PlayingState, PlayingStateResult};

pub struct SelectTileState {}

impl PlayingState for SelectTileState {
    fn update_game(&mut self, _action: Action, context: &mut GameContext) -> PlayingStateResult {
        if let Some(tile) = context.select_random_tile() {
            Continue(Box::new(PlaceTileState::new(tile)))
        } else {
            ExitToStop
        }
    }
    fn draw(&self) -> Node {
        Node::None
    }

    fn handle_input(&self, _: InputEvent) -> Action {
        Action::None
    }

    fn need_input(&self) -> bool {
        false
    }
}
