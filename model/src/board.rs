//! Represents a game board composed of a 2-dimensional grid of tiles.
use crate::tile::Tile;

/// The `Board` struct represents a two-dimensional game board composed of tiles.
///
/// # Fields
/// - `tiles`: A 2D vector (`Vec<Vec<Tile>>`) that holds the `Tile` elements representing
///            the state and arrangement of the board.
///
/// # Traits
/// - `Debug`: Allows the `Board` to be formatted using the `{:?}` formatter, which is useful
///            for debugging output.
/// - `Default`: Provides a default initial state for the `Board`. This can be created
///              using `Board::default()`.
/// - `Clone`: Enables the `Board` to be deeply cloned.
///
/// # Example
/// ```
/// use model::board::Board;
/// let board = Board::default(); // Create a default board
/// println!("{:?}", board); // Debug prints the board
/// ```
#[derive(Debug, Default, Clone)]
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
}

impl Board {}

/// A builder for constructing a `Board` with a customizable layout of tiles.
///
/// The `BoardBuilder` struct provides a convenient way to assemble a board
/// by defining its structure using a 2D vector of `Tile` objects.
///
/// # Fields
///
/// * `tiles` - A two-dimensional vector of `Tile` objects representing the layout
///   of the board.
///
/// # Usage
///
/// The `Default` trait is derived from `BoardBuilder`, allowing you to create
/// a new instance of the builder with default values:
///
/// ```
/// use model::board::BoardBuilder;
/// let builder = BoardBuilder::default();
/// ```
///
/// You can then customize the tiles on the board by setting the `tiles` property
/// or using additional builder methods (if implemented).
///
/// This struct is typically used to construct an instance of a `Board` by
/// applying the specified configurations.
#[derive(Default)]
pub struct BoardBuilder {
    tiles: Vec<Vec<Tile>>,
}

impl BoardBuilder {
    /// Sets the `tiles` field of the provided `Board` instance to the value stored in the
    /// builder.
    ///
    /// This method takes a mutable reference to a `Board` instance and updates its `tiles`
    /// field with the value stored in the builder.
    ///
    /// # Parameters
    ///
    /// - `self`: Consumes the builder object, transferring ownership of the `tiles` data.
    /// - `board`: A mutable reference to the `Board` instance that will be updated with
    ///   the builder's `tiles`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::board::{Board, BoardBuilder};
    /// use model::tile::Tile;
    /// let builder = BoardBuilder::default();
    /// let mut board = Board::default();
    /// builder.build(&mut board);
    /// assert_eq!(board.tiles.len(), 1);
    /// ```
    ///
    /// # Notes
    /// This function will replace any existing `tiles` in the provided `Board` instance
    /// with the builder's `tiles`. Use this method with caution if the `Board` already
    /// contains data that should be preserved.
    pub fn build(self, board: &mut Board) {
        board.tiles = self.tiles;
    }
}
