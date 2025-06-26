//! This module defines a `ScoreBoard` and its corresponding builder, `ScoreBoardBuilder`,
//! to manage and initialize player scores in a game.
use crate::player::Player;
use std::collections::HashMap;

/// A `ScoreBoard` struct that keeps track of player scores in a game.
///
/// This struct is used to store and manage the scores of players using a `HashMap`, where
/// each player's score is associated with their unique identifier.
///
/// # Fields
/// - `scores`:
///   A `HashMap` with keys of type `Player` and values of type `usize`.
///   This map stores the scores of each player, where the `Player` identifies
///   the individual and the `usize` represents their score.
///
/// # Traits
/// Implements the following traits:
/// - `Debug`: Enables formatting the `ScoreBoard` instance for debugging output.
/// - `Default`: Provides a default empty `ScoreBoard` instance.
/// - `Clone`: Allows for creating a duplicate of a `ScoreBoard` instance.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use model::player::Player;
/// use model::scoreboard::ScoreBoard;
///
/// let mut scoreboard = ScoreBoard::default();
///
/// let player = Player { name: String::from("Alice") };
/// scoreboard.scores.insert(player.clone(), 42);
///
/// println!("{:?}", scoreboard);
/// ```
#[derive(Debug, Default, Clone)]
pub struct ScoreBoard {
    pub scores: HashMap<Player, usize>,
}

/// A builder structure for creating and managing a scoreboard to track player scores.
///
/// # Generic Parameters
/// - `'a`: A lifetime parameter that represents the lifetime of the `Player` type used as keys
///   in the `HashMap`.
///
/// # Fields
/// - `scores`: A `HashMap` that maps `Player` instances to their respective scores (`usize`).
///
/// # Derives
/// This struct implements the `Default` trait, allowing instances of `ScoreBoardBuilder`
/// to be created with the default state, where the `scores` field is an empty `HashMap`.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use model::scoreboard::ScoreBoardBuilder;
///
/// let scoreboard_builder = ScoreBoardBuilder::default();
/// assert!(scoreboard_builder.scores.is_empty());
/// ```
#[derive(Default)]
pub struct ScoreBoardBuilder {
    pub scores: HashMap<Player, usize>,
}

impl ScoreBoardBuilder {
    /// Adds a player to the scoreboard with an initial score of 0.
    ///
    /// This method takes ownership of the `ScoreBoardBuilder` instance and adds a new player
    /// to the `scores` collection. The player's score is initialized to 0. The method then
    /// returns the modified builder for further modifications or building the final scoreboard.
    ///
    /// # Parameters
    ///
    /// * `player`: The `Player` instance representing the player to be added to the scoreboard.
    ///
    /// # Returns
    ///
    /// Returns a `ScoreBoardBuilder` instance with the updated scores.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::player::PlayerBuilder;
    /// use model::scoreboard::ScoreBoardBuilder;
    /// let player = PlayerBuilder::default().name("John").build();
    /// let scoreboard = ScoreBoardBuilder::default()
    ///     .add_player(player)
    ///     .build();
    /// ```
    ///
    /// # Note
    ///
    /// This method assumes players are unique keys in the scoreboard. If `add_player` is called
    /// with a player that already exists in the score map, the existing score will be overwritten
    /// to `0`.
    pub fn add_player(&mut self, player: Player) -> &mut Self {
        self.scores.insert(player, 0);
        self
    }

    /// Builds and returns a new `ScoreBoard` instance using the current state of the builder.
    ///
    /// # Returns
    ///
    /// A new `ScoreBoard` instance where the `scores` field is a clone of the builder's `scores`.
    ///
    /// # Example
    ///
    /// ```
    /// use model::scoreboard::ScoreBoardBuilder;
    /// let builder = ScoreBoardBuilder::default();
    /// let scoreboard = builder.build();
    /// ```
    pub fn build(&self) -> ScoreBoard {
        ScoreBoard {
            scores: self.scores.clone(),
        }
    }
}
