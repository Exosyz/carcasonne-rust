use crate::game::Game;
use carcasonne_text_ui::renderer::TextRenderer;

mod game;

fn main() {
    //let game_tiles = GameTilesFactory::build_base_game();
    //println!("{:#?}", game_tiles);

    Game::new(TextRenderer {}).run();
}
