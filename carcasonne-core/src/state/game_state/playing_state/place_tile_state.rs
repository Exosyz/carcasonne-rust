use crate::action::Action;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::model::tile::Tile;
use crate::state::game_state::playing_state::select_tile_state::SelectTileState;
use crate::state::game_state::playing_state::PlayingStateResult::Continue;
use crate::state::game_state::playing_state::{GameContext, PlayingState, PlayingStateResult};

pub struct PlaceTileState {
    tile: Tile,
}

impl PlaceTileState {
    pub fn new(tile: Tile) -> Self {
        Self { tile }
    }
}

impl PlayingState for PlaceTileState {
    fn update_game(&mut self, _action: Action, _context: &mut GameContext) -> PlayingStateResult {
        Continue(Box::new(SelectTileState {}))
    }

    fn draw(&self) -> Node {
        Node::Tile(&self.tile)
    }
    fn handle_input(&self, event: InputEvent) -> Action {
        match event {
            InputEvent::Enter => Action::Validate,
            _ => Action::None,
        }
    }

    fn need_input(&self) -> bool {
        true
    }
}
