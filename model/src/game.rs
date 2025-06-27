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
use crate::board::Board;
use crate::player::Player;
use crate::scoreboard::ScoreBoard;
use crate::tile::Tile;
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
