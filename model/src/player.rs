//! Represents a player in the system with a name attribute.
//!
//! # Fields
//!
//! * `name` - A string slice that holds the player's name.
//!
//! This structure derives common traits for debugging (`Debug`), default initialization (`Default`), equality comparison (`Eq`, `PartialEq`), hashing (`Hash`), and cloning (`Clone`).

/// Represents a player in the system.
///
/// This structure models a player with a single field, `name`, to store the
/// player's name. It derives several traits to enable common operations:
/// - `Debug` for formatting and debugging purposes.
/// - `Default` to allow creating a default instance with empty values.
/// - `Eq` and `PartialEq` to compare players for equality.
/// - `Hash` to use the struct as a key in hashed collections like `HashMap`.
/// - `Clone` to create deep copies of instances.
///
/// # Fields
///
/// * `name` - A `String` representing the name of the player.
///
/// # Derivable Traits
///
/// * `Debug`: Enables use of the `{:?}` formatter to inspect the `Player`.
/// * `Default`: Allows creation of a `Player` using `Player::default()`,
///   initializing with an empty `name`.
/// * `Eq` and `PartialEq`: Enables comparison of two `Player` instances for equality.
/// * `Hash`: Allows the struct to be used with collections like `HashMap`
///   or `HashSet`.
/// * `Clone`: Enables creating exact copies of the `Player`.
///
/// # Example
///
/// ```rust
/// use model::player::Player;
///
/// let player = Player {
///     name: String::from("Alice"),
/// };
/// println!("{:?}", player); // Output: Player { name: "Alice" }
///
/// let default_player = Player::default();
/// println!("{:?}", default_player); // Output: Player { name: "" }
///
/// let player_clone = player.clone();
/// assert_eq!(player, player_clone);
/// ```
#[derive(Debug, Default, Eq, Hash, PartialEq, Clone)]
pub struct Player {
    pub name: String,
}
