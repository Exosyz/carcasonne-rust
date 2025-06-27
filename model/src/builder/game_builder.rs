use crate::board::{Board, BoardBuilder};
use crate::builder::player_builder::PlayerBuilder;
use crate::builder::scoreboard_builder::ScoreBoardBuilder;
use crate::builder::tile_builder::TileBuilder;
use crate::game::Game;
use crate::player::Player;
use crate::scoreboard::ScoreBoard;
use crate::tile::Tile;

/// A builder struct for creating and initializing a `Game` instance.
/// The `GameBuilder` struct provides a flexible way to set up and configure the components
/// needed for a game, such as the players, tiles, score board, and board.
///
/// # Attributes
///
/// * `players` - A vector of `Player` objects representing the participants of the game.
/// * `available_tiles` - A vector of `Tile` objects that can be used in the game.
/// * `score_board` - A `ScoreBoard` object used to manage and track the scores of the players.
/// * `board` - Represents the main `Board` where the game takes place.
///
/// # Derive Attributes
///
/// * `Default` - Provides a default implementation for `GameBuilder` to initialize with empty or default values.
/// * `Clone` - Allows the `GameBuilder` instance to be cloned.
///
/// # Usage
///
/// ```
/// use model::builder::game_builder::GameBuilder;
/// let builder = GameBuilder::default();
/// let builder_clone = builder.clone();
/// ```
///
/// This struct is intended to be used as part of constructing and initializing a `Game`.
#[derive(Default, Clone)]
pub struct GameBuilder {
    players: Vec<Player>,
    available_tiles: Vec<Tile>,
    score_board: ScoreBoard,
    board: Board,
}

impl GameBuilder {
    /// Configures the board object for the current instance.
    ///
    /// This method allows for customization of a `Board` object through a builder pattern.
    /// A closure is passed as an argument which operates on a `BoardBuilder` to build
    /// the desired configuration of the `Board`. The built `Board` object is then stored
    /// within the current instance (`self`).
    ///
    /// # Parameters
    ///
    /// - `board_builder`: A closure that accepts a mutable reference to a `BoardBuilder` and
    ///   returns a mutable reference to the same `BoardBuilder`. This closure is used to
    ///   define how the `Board` should be constructed.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to `self`, enabling method chaining.
    ///
    /// # Example
    ///
    /// ```
    /// use model::builder::game_builder::GameBuilder;
    ///
    /// let mut instance = GameBuilder::default();
    /// instance.board(|builder| {
    ///     builder
    ///         .set_width(10)
    ///         .set_height(20)
    ///         .enable_feature_x();
    /// });
    /// ```
    ///
    /// In the above example, the `board` method is used to configure a `Board` object
    /// for `instance`, setting its width, height, and enabling a custom feature.
    pub fn board(
        &mut self,
        board_builder: impl FnOnce(&mut BoardBuilder) -> &mut BoardBuilder,
    ) -> &mut Self {
        let mut builder = BoardBuilder::default();
        board_builder(&mut builder);

        let mut board = Board::default();
        builder.build(&mut board);

        self.board = board;
        self
    }

    /// Resets the current board to its default state and returns a mutable reference to `self`.
    ///
    /// This method sets the `board` field of the implementing struct to a default instance of
    /// the `Board` type. It allows for method chaining by returning a mutable reference to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::builder::game_builder::GameBuilder;
    /// let mut game = GameBuilder::default();
    /// game.default_board()
    ///     .start_game();
    /// ```
    ///
    /// In the above example, the board is reset to its default configuration,
    /// and other methods can be called on the same instance due to method chaining.
    ///
    /// # Returns
    ///
    /// A mutable reference to `self`, allowing for method chaining.
    pub fn default_board(&mut self) -> &mut Self {
        self.board = Board::default();
        self
    }

    /// Configures the score board for the current instance using a builder pattern.
    ///
    /// This method accepts a closure that takes a mutable reference to a `ScoreBoardBuilder`,
    /// allowing the caller to customize the score board. After building the score board
    /// via the builder, the created score board is assigned to the current instance.
    ///
    /// # Arguments
    ///
    /// * `score_board_builder` - A closure or function that accepts a mutable reference to a `ScoreBoardBuilder`
    ///   and customizes it according to the user's requirements. The closure should return the modified builder.
    ///
    /// # Returns
    ///
    /// This method returns a mutable reference to the current instance (`Self`) to allow
    /// method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::game_builder::GameBuilder;
    /// let mut instance = GameBuilder::default();
    /// instance.scores(|builder| {
    ///     builder.add_score("Player1", 42)
    ///            .add_score("Player2", 36)
    /// });
    /// ```
    ///
    /// In this example, the `scores` method is used to configure the score board with
    /// player scores using the provided `ScoreBoardBuilder`.
    ///
    pub fn scores(
        &mut self,
        score_board_builder: impl FnOnce(&mut ScoreBoardBuilder) -> &mut ScoreBoardBuilder,
    ) -> &mut Self {
        let mut builder = ScoreBoardBuilder::default();
        score_board_builder(&mut builder);

        self.score_board = builder.build();
        self
    }

    /// Adds a new player to the current list of players.
    ///
    /// # Parameters
    /// - `player_builder`: A closure that takes a mutable reference to a `PlayerBuilder` and returns
    ///   the same builder after configuring it. This allows for customizing the player's properties
    ///   before it is added.
    ///
    /// # Returns
    /// A mutable reference to `Self` for method chaining.
    ///
    /// # Example
    /// ```
    /// use model::builder::game_builder::GameBuilder;
    ///
    /// let mut manager = GameBuilder::default();
    /// manager.add_player(|builder| {
    ///     builder.name("Player1".parse().unwrap()).set_level(5)
    /// });
    /// ```
    pub fn add_player(
        &mut self,
        player_builder: impl FnOnce(&mut PlayerBuilder) -> &mut PlayerBuilder,
    ) -> &mut Self {
        let mut builder = PlayerBuilder::default();
        player_builder(&mut builder);

        self.players.push(builder.build());
        self
    }

    /// Adds a specified quantity of tiles to the `available_tiles` collection in the current object.
    ///
    /// # Parameters
    /// - `tile_builder`: A closure or function that takes a mutable reference to a `TileBuilder`
    ///   instance, allowing customization of the tile being added.
    /// - `quantity`: The number of tiles to add to the `available_tiles` collection.
    ///
    /// # Returns
    /// - Returns `&mut Self` for method chaining.
    ///
    /// # Behavior
    /// 1. Initializes a default instance of `TileBuilder`.
    /// 2. Passes the `TileBuilder` instance to the provided `tile_builder` function for customization.
    /// 3. Uses the customized `TileBuilder` to create a `Tile` instance.
    /// 4. Clones the created tile and adds the specified quantity of that tile to the
    ///    `available_tiles` collection.
    ///
    /// # Example
    /// ```
    /// use model::builder::game_builder::GameBuilder;
    /// use model::tile::Tile;
    /// let mut tile_manager = GameBuilder::default();
    ///
    /// tile_manager.add_tile(
    ///     |builder| builder.set_type(Tile::default()),
    ///     5
    /// );
    /// ```
    /// In this example, the closure configures the tile's type and color before adding
    /// 5 instances of it to the `available_tiles` collection.
    ///
    /// # Notes
    /// - The tile customization logic is determined by the `tile_builder` closure.
    /// - The `Tile` instance is cloned for each additional quantity to ensure each tile is independent.
    pub fn add_tile(
        &mut self,
        tile_builder: impl FnOnce(&mut TileBuilder) -> &mut TileBuilder,
        quantity: usize,
    ) -> &mut Self {
        let mut builder = TileBuilder::default();
        tile_builder(&mut builder);

        let tile = builder.build();
        for _ in 0..quantity {
            self.available_tiles.push(tile);
        }
        self
    }

    /// Finalizes the construction of the `Game` object by transferring the state
    /// from the builder to the provided mutable reference to a `Game` instance.
    ///
    /// This method is called after a builder has been configured with the desired
    /// state. It populates the `Game` object with various components such as players,
    /// the available tiles, the game board, and the score board.
    ///
    /// # Parameters
    /// - `game`: A mutable reference to the `Game` instance that will be
    ///   initialized or updated with the state from the builder.
    ///
    /// # Behavior
    /// - Replaces the `players` field of the `Game` instance with the `players` field
    ///   from the builder.
    /// - Replaces the `available_tiles` field of the `Game` instance with the
    ///   `available_tiles` field from the builder.
    /// - Replaces the `board` field of the `Game` instance with the `board`
    ///   field from the builder.
    /// - Replaces the `score_board` field of the `Game` instance with the `score_board`
    ///   field from the builder.
    ///
    /// # Example
    /// ```
    /// use model::builder::game_builder::GameBuilder;
    /// let game_builder = GameBuilder::default()
    ///     .add_player(|b| b.name("Alice").quantity(1))
    ///     .default_board();
    ///
    /// let mut game = game_builder.build();
    /// ```
    pub fn build(&self) -> Game {
        Game {
            players: self.players.clone(),
            score_board: self.score_board.clone(),
            available_tiles: self.available_tiles.clone(),
            board: self.board.clone(),
        }
    }
}
