use model::board::BoardBuilder;
use model::builder::base_game_builder::BaseGameBuilder;
use model::game::GameBuilder;
use model::player::PlayerBuilder;

fn main() {
    let game = GameBuilder::new()
        .board(BoardBuilder::new().build())
        .add_player(PlayerBuilder::new().name(String::from("Manon")).build())
        .add_player(PlayerBuilder::new().name(String::from("Raph")).build())
        .add_player(PlayerBuilder::new().name(String::from("B")).build())
        .add_base_game()
        .build();

    println!("{:?}", game.available_tiles);

    let game = game.shuffle_available_tiles();

    println!("{:?}", game.available_tiles);
}
