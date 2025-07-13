use crate::game::Game;
use carcasonne_text_ui::renderer::TextRenderer;
use std::cell::RefCell;
use std::io::stdout;

mod game;

/// Entry point of the application.
///
/// Creates a new game instance with a `TextRenderer` wrapped in a `RefCell`,
/// then starts the game loop by calling `run`.
fn main() {
    Game::new(RefCell::new(TextRenderer::new(stdout()))).run();
}
