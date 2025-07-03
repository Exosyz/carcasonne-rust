use crate::game_state::GameState;

pub trait Renderer {
    fn render(&self, state: &GameState);
}
