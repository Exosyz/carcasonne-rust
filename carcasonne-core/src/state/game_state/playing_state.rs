pub mod place_tile_state;
pub mod select_tile_state;

use crate::action::Action;
pub use crate::context::GameContext;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::model::game::GameTiles;
use crate::state::game_state::playing_state::PlayingStateResult::Continue;
use crate::state::game_state::stop_state::StopState;
use crate::state::StateResult::Skip;
use crate::state::{State, StateResult};

pub struct PlayingPhase {
    pub current_state: Box<dyn PlayingState>,
    pub context: GameContext,
}

impl PlayingPhase {
    pub fn new(default_state: Box<dyn PlayingState>, tiles: GameTiles) -> Self {
        Self {
            current_state: default_state,
            context: GameContext {
                available_tiles: tiles.available_tiles,
            },
        }
    }
}

pub enum PlayingStateResult {
    Continue(Box<dyn PlayingState>),
    ExitToStop,
}

pub trait PlayingState {
    fn update_game(&mut self, action: Action, context: &mut GameContext) -> PlayingStateResult;

    fn draw(&self) -> Node;
    fn handle_input(&self, event: InputEvent) -> Action;

    fn need_input(&self) -> bool {
        true
    }
}

impl State for PlayingPhase {
    fn update(&mut self, action: Action) -> StateResult {
        if let Continue(new_state) = self.current_state.update_game(action, &mut self.context) {
            self.current_state = new_state;
            Skip
        } else {
            StateResult::Continue(Box::new(StopState {}))
        }
    }

    fn draw(&self) -> Node {
        let draw_result = self.current_state.draw();

        if let Node::None = draw_result {
            return Node::None;
        }

        Node::VerticalContainer(vec![
            Box::new(Node::Text("Game Is Running")),
            Box::new(Node::Framed(Box::new(self.current_state.draw()))),
            Box::new(Node::HorizontalContainer(vec![
                Box::new(self.current_state.draw()),
                Box::new(self.current_state.draw()),
                Box::new(self.current_state.draw()),
                Box::new(self.current_state.draw()),
            ])),
        ])
    }
    fn handle_input(&self, event: InputEvent) -> Action {
        self.current_state.handle_input(event)
    }

    fn need_input(&self) -> bool {
        self.current_state.need_input()
    }
}
