use carcasonne_core::game_state::GameState;
use carcasonne_core::renderer::{Renderer, UIComponent};

pub struct GameStateRenderer<'a>(pub &'a GameState);
impl<'a> UIComponent for GameStateRenderer<'a> {
    fn render(&self, renderer: &dyn Renderer) {
        match self.0 {
            GameState::Playing => renderer.render("PLAY"),
            GameState::Menu => renderer.render("MENU"),
            GameState::Stop => renderer.render("STOP"),
        }
    }
}
