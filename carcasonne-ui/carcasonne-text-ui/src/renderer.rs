use carcasonne_core::game_state::GameState;
use carcasonne_core::renderer::Renderer;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Default, Debug)]
pub struct TextRenderer;

impl TextRenderer {
    pub fn new() -> Self {
        enable_raw_mode().ok();
        Self {}
    }
}

impl Drop for TextRenderer {
    fn drop(&mut self) {
        disable_raw_mode().ok();
    }
}

impl Renderer for TextRenderer {
    fn render(&self, state: &GameState) {
        match state {
            GameState::Menu => println!("Menu"),
            GameState::Playing => println!("Playing"),
            GameState::Stop => println!("Stop"),
        }
    }
}
