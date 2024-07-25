use model::builder::base_game_builder::BaseGameBuilder;
use model::game::{Game, GameBuilder};

fn main() {
    let mut game_builder = GameBuilder::default();

    game_builder
        .default_board()
        .add_player(|b| b.name("Manon".into()))
        .add_player(|b| b.name("Raph".into()))
        .add_base_game();

    let mut game = Game::default();

    game_builder.build(&mut game);

    game.shuffle_available_tiles();

    println!("{:?}", game.get_next_tile());
}
