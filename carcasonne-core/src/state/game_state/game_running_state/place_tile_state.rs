use crate::action::Action;
use crate::context::game_context::GameContext;
use crate::input_handler::InputEvent;
use crate::layout::node::Node;
use crate::model::rotation::Rotation;
use crate::model::tile::Tile;
use crate::model::tile_size::TileSize;
use crate::state::game_state::game_running_state::select_tile_state::SelectTileState;
use crate::state::StateResult::{Stay, Transition};
use crate::state::{State, StateResult};

pub struct PlaceTileState {
    tile: Tile,
    rotation: Rotation,
    tile_size: TileSize,
}

impl PlaceTileState {
    pub fn new(tile: Tile) -> Self {
        Self {
            tile,
            rotation: Rotation::R0,
            tile_size: TileSize::Small,
        }
    }
}

impl State for PlaceTileState {
    fn update(&mut self, context: &mut GameContext, action: Action) -> StateResult {
        match action {
            Action::RotateLeft => {
                self.rotation.rotate_left();
                self.tile.rotate_left();
                Stay(true)
            }
            Action::RotateRight => {
                self.rotation.rotate_right();
                self.tile.rotate_right();
                Stay(true)
            }
            Action::ZoomIn => {
                self.tile_size.decrease_size();
                Stay(true)
            }
            Action::ZoomOut => {
                self.tile_size.increase_size();
                Stay(true)
            }
            Action::Validate => {
                //TODO context.place_tile(self.tile);

                Transition(Box::new(SelectTileState::new()))
            }
            _ => Stay(false),
        }
    }

    fn draw(&'_ self) -> Node<'_> {
        Node::Framed(Box::new(Node::Tile(
            &self.tile,
            &self.rotation,
            &self.tile_size,
        )))
    }
    fn handle_input(&self, event: InputEvent) -> Action {
        match event {
            InputEvent::Select => Action::Validate,
            InputEvent::Char('a') => Action::RotateLeft,
            InputEvent::Char('e') => Action::RotateRight,
            InputEvent::Char('o') => Action::ZoomIn,
            InputEvent::Char('l') => Action::ZoomOut,
            _ => Action::None,
        }
    }

    fn need_input(&self) -> bool {
        true
    }
}
