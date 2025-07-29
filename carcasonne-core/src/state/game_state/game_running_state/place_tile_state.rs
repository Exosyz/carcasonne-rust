use crate::action::Action;
use crate::color::Color;
use crate::context::game_context::GameContext;
use crate::input_handler::InputEvent;
use crate::layout::node::{Node, NodeTag};
use crate::model::rotation::Rotation;
use crate::model::tile::Tile;
use crate::state::game_state::game_running_state::select_tile_state::SelectTileState;
use crate::state::StateResult::Transition;
use crate::state::{State, StateResult};

pub struct PlaceTileState {
    tile: Tile,
    game_context: GameContext,
}

impl PlaceTileState {
    pub fn new(game_context: GameContext, tile: Tile) -> Self {
        Self { tile, game_context }
    }
}

impl State for PlaceTileState {
    fn update(&mut self, _: Action) -> StateResult {
        Transition(Box::new(SelectTileState::new(std::mem::take(
            &mut self.game_context,
        ))))
    }

    fn draw(&'_ self) -> Node<'_> {
        Node::HorizontalContainer(vec![
            Node::Framed(Box::new(Node::Tile(&self.tile, &Rotation::R0))),
            Node::None,
            Node::Framed(Box::new(Node::MultiLineRichText(
                Box::leak(format!("{:#?}", self.tile).into_boxed_str()),
                vec![NodeTag::Foreground(Color::Blue)],
            ))),
        ])
    }
    fn handle_input(&self, event: InputEvent) -> Action {
        match event {
            InputEvent::Select => Action::Validate,
            _ => Action::None,
        }
    }

    fn need_input(&self) -> bool {
        true
    }
}
