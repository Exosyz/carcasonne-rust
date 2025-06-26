//! The `Game` struct represents the state of a game, including players, the game board, a
//! scoreboard, and a collection of available tiles.
//!
//! # Fields
//! * `players` - A vector of `Player` structs representing the players in the game.
//! * `score_board` - A `ScoreBoard` struct used to manage the scores of the players.
//! * `available_tiles` - A vector of `Tile` structs representing the tiles left to be used in the game.
//! * `board` - A `Board` struct representing the game board.
//!
//! # Examples
//! ```
//! use model::game::Game;
//! let mut game = Game::default();
//! game.shuffle_available_tiles();
//! if let Some(tile) = game.get_next_tile() {
//!     // Use the tile in the gameplay logic.
//! }
//! ```
use crate::board::{Board, BoardBuilder};
use crate::player::{Player, PlayerBuilder};
use crate::scoreboard::{ScoreBoard, ScoreBoardBuilder};
use crate::tile::{Tile, TileBuilder};
use rand::prelude::SliceRandom;
use rand::thread_rng;

/// Struct representing the state of the game.
///
/// # Fields
///
/// * `players` - A `Vec` containing the list of players currently in the game.
///   Each player is represented by a `Player` struct.
///
/// * `score_board` - A `ScoreBoard` instance that keeps track of the players' scores
///   and other scoring-related details in the game.
///
/// * `available_tiles` - A `Vec` of `Tile` instances representing the pool of tiles
///   that have not yet been used or placed on the board during the game.
///
/// * `board` - A `Board` instance representing the game board where tiles are played.
///
/// # Derives
///
/// * `Debug` - Enables formatted output of this struct, useful for debugging purposes.
/// * `Default` - Provides a default implementation for creating an instance of `Game`.
/// * `Clone` - Allows this `Game` struct to be cloned, producing an exact duplicate of the instance.
///
/// This struct is central to representing the overall state and progress of the game,
/// combining the current players, scores, tiles, and board into a cohesive structure.
#[derive(Debug, Default, Clone)]
pub struct Game {
    pub players: Vec<Player>,
    pub score_board: ScoreBoard,
    pub available_tiles: Vec<Tile>,
    pub board: Board,
}

impl Game {
    /// Shuffles the available tiles in the game or collection randomly.
    ///
    /// This method uses a random number generator (`thread_rng`) to shuffle
    /// the elements in the `available_tiles` collection and modifies it in place.
    /// It ensures that every possible ordering of the tiles has an equal probability
    /// of occurring. This is particularly useful for scenarios such as resetting
    /// or randomizing game states before the start of a new round.
    ///
    /// # Returns
    ///
    /// A reference to the current instance of the object, allowing method
    /// chaining for later operations.
    ///
    /// # Example
    /// ```
    /// use model::game::Game;
    /// let mut game = Game::default();
    /// game.shuffle_available_tiles()
    ///     .start_round();
    /// ```
    ///
    /// In this example, the `available_tiles` are shuffled, and then the game round starts.
    ///
    /// # Note
    /// Ensure that `available_tiles` is not empty before calling this method,
    /// as it may have no effect on an empty collection.
    ///
    /// # Panics
    /// This function does not explicitly panic but be cautious of the
    /// state of `available_tiles` and other mutable behaviors in the context
    /// in which this method is used.
    pub fn shuffle_available_tiles(&mut self) -> &Self {
        self.available_tiles.shuffle(&mut thread_rng());
        self
    }

    /// Retrieves the next available tile from the set of available tiles.
    ///
    /// This method removes and returns the last tile in the `available_tiles` collection if one is present.
    /// If the collection is empty, it returns `None`.
    ///
    /// # Returns
    /// * `Some(Tile)` - The next available tile if there are any remaining tiles.
    /// * `None` - If the `available_tiles` collection is empty.
    ///
    /// # Example
    /// ```rust
    /// use model::game::Game;
    ///
    /// let mut game = Game::default();
    /// if let Some(tile) = game.get_next_tile() {
    ///     println!("Got a tile: {:?}", tile);
    /// } else {
    ///     println!("No more tiles available!");
    /// }
    /// ```
    pub fn get_next_tile(&mut self) -> Option<Tile> {
        self.available_tiles.pop()
    }
}

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
/// use model::game::GameBuilder;
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
    /// use model::game::GameBuilder;
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
    /// use model::game::GameBuilder;
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
    /// use model::game::GameBuilder;
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
    /// use model::game::GameBuilder;
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
    /// use model::game::GameBuilder;
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
    /// use model::game::{Game, GameBuilder};
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
