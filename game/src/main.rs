use model::builder::base_game_builder::BaseGameBuilder;
use model::builder::game_builder::GameBuilder;

fn main() {
    let mut game_builder = GameBuilder::default();

    game_builder
        .default_board()
        .add_player(|b| b.name("Manon"))
        .add_player(|b| b.name("Raph"))
        .add_base_game();

    let mut game = game_builder.build();

    game.shuffle_available_tiles();

    // TODO Ajouter la rotation des joueurs pour qu'il pioche une carte

    // TODO Afficher le board
    // TODO     - aVec des lettres dans un premier temps
    // TODO     - aVec des multichars dans un second temps

    println!("{:?}", game.get_next_tile());
}
