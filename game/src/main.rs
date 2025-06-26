use model::builder::base_game_builder::BaseGameBuilder;
use model::game::GameBuilder;

fn main() {
    let mut game_builder = GameBuilder::default();

    game_builder
        .default_board()
        .add_player(|b| b.name("Manon"))
        .add_player(|b| b.name("Raph"))
        .add_base_game();

    let mut game = game_builder.build();

    game.shuffle_available_tiles();

    println!("{:?}", game.get_next_tile());
}
