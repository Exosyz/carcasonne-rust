use crate::color::Color;

/// Represents a player in the game.
///
/// Each player has a unique name and an associated color for identification.
pub struct Player {
    /// The player's name.
    pub name: String,

    /// The player's associated color.
    pub color: Color,
}

impl Player {
    /// Creates a new player with the specified name and color.
    ///
    /// # Arguments
    ///
    /// * `name` - A `String` representing the player's name.
    /// * `color` - A `Color` representing the player's color.
    ///
    /// # Returns
    ///
    /// A new `Player` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use carcasonne_core::color::Color;
    /// use carcasonne_core::model::player::Player;
    ///
    /// let player = Player::new("Alice".to_string(), Color::Red);
    /// ```
    pub fn new(name: String, color: Color) -> Self {
        Self { name, color }
    }
}
