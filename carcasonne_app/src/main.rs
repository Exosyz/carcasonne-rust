use carcasonne_core::factory::game_factory::GameTilesFactory;

fn main() {
    let game_tiles = GameTilesFactory::build_base_game();

    println!("{:#?}", game_tiles);
}
