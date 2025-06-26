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

/// A struct to build and configure a `Player` with customizable attributes.
///
/// The `PlayerBuilder` struct provides a way to specify key properties of a `Player`
/// such as their `name` and `quantity`. It implements the `Default` trait for providing
/// default values and the `Clone` trait for creating deep copies of an instance.
///
/// # Fields
/// - `name` (`String`): The name of the player. Defaults to an empty string (`""`).
/// - `quantity` (`usize`): The quantity associated with the player, such as the number
///   of players or units. Defaults to `0`.
///
/// # Traits Implemented
/// - `Default`: Enables the creation of a `PlayerBuilder` with default values for its fields.
/// - `Clone`: Allows for the creation of an identical copy of a `PlayerBuilder`.
///
/// # Example
/// ```
/// use model::player::PlayerBuilder;
///
/// // Create a new PlayerBuilder with default values
/// let default_player = PlayerBuilder::default();
/// assert_eq!(default_player.name, "");
/// assert_eq!(default_player.quantity, 0);
///
/// // Create a customized PlayerBuilder
/// let custom_player = PlayerBuilder {
///     name: "Alice".to_string(),
///     quantity: 5,
/// };
/// assert_eq!(custom_player.name, "Alice");
/// assert_eq!(custom_player.quantity, 5);
///
/// // Clone the PlayerBuilder
/// let cloned_player = custom_player.clone();
/// assert_eq!(cloned_player.name, "Alice");
/// assert_eq!(cloned_player.quantity, 5);
/// ```
#[derive(Default, Clone)]
pub struct PlayerBuilder {
    pub name: String,
    pub quantity: usize,
}

impl PlayerBuilder {
    /// Sets the `name` field of the current object and returns a mutable reference to the object itself.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that represents the name to be assigned to the object.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - A mutable reference to the current instance, allowing for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::player::PlayerBuilder;
    ///
    /// let mut instance = PlayerBuilder::default();
    /// instance.name("example name").other_method();
    /// ```
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    /// Sets the quantity for the current instance.
    ///
    /// # Arguments
    /// * `quantity` - The amount to be set for the instance.
    ///
    /// # Returns
    /// * A mutable reference to the current instance, allowing method chaining.
    ///
    /// # Example
    /// ```rust
    /// use model::player::PlayerBuilder;
    ///
    /// let mut item = PlayerBuilder::default();
    /// item.quantity(5);
    /// ```
    pub fn quantity(&mut self, quantity: usize) -> &mut Self {
        self.quantity = quantity;
        self
    }

    /// Builds and returns an instance of the `Player` structure.
    ///
    /// This method creates a new `Player` using the current state of the builder object.
    /// It clones the `name` field from the builder and assigns it to the `name` field of the new `Player`.
    ///
    /// # Returns
    ///
    /// A `Player` instance with its `name` field initialized using the builder's `name` property.
    ///
    /// # Example
    /// ```
    /// use model::player::PlayerBuilder;
    ///
    /// let builder = PlayerBuilder::default()
    ///     .name("Alice");
    /// let player = builder.build();
    /// assert_eq!(player.name, "Alice");
    /// ```
    pub fn build(&self) -> Player {
        Player {
            name: self.name.to_string(),
        }
    }
}
