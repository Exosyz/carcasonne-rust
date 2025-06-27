use crate::player::Player;

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
/// // Create a new PlayerBuilder with default values
/// use model::builder::player_builder::PlayerBuilder;
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
    /// use model::builder::player_builder::PlayerBuilder;
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
    /// use model::builder::player_builder::PlayerBuilder;
    ///
    /// let mut instance = PlayerBuilder::default();
    /// instance.quantity(5);
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
    ///
    /// use model::builder::player_builder::PlayerBuilder;
    ///
    /// let mut instance = PlayerBuilder::default()
    ///     .name("Alice");
    /// let player = instance.build();
    /// assert_eq!(player.name, "Alice");
    /// ```
    pub fn build(&self) -> Player {
        Player {
            name: self.name.to_string(),
        }
    }
}
