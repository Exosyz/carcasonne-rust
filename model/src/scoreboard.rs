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
