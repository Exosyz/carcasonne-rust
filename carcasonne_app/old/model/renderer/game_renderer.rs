use crate::game::Game;
use crate::renderer::base_renderer::{
    GraphicRenderer, UnicodeRenderer,
};

pub trait GameRenderer {
    fn render(&self, game: &mut Game);
}

impl GameRenderer for GraphicRenderer {
    fn render(&self, game: &mut Game) {}
}

impl GameRenderer for UnicodeRenderer {
    fn render(&self, game: &mut Game) {}
}
