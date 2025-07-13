use crate::action::Action;
use crate::factory::game_factory::GameTilesFactory;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::state::game_state::playing_state::select_tile_state::SelectTileState;
use crate::state::game_state::playing_state::PlayingPhase;
use crate::state::StateResult::{Continue, Skip};
use crate::state::{State, StateResult};

pub struct MenuState {}

impl State for MenuState {
    fn update(&mut self, action: Action) -> StateResult {
        match action {
            Action::StartGame => Continue(Box::new(PlayingPhase::new(
                Box::new(SelectTileState {}),
                GameTilesFactory::build_base_game(),
            ))),
            _ => Skip,
        }
    }

    fn draw(&self) -> Node {
        Node::Text("Press <Enter> to start playing")
    }

    fn handle_input(&self, event: InputEvent) -> Action {
        match event {
            InputEvent::Quit => Action::Quit,
            InputEvent::Enter => Action::StartGame,
            _ => Action::None,
        }
    }

    fn need_input(&self) -> bool {
        true
    }
}
