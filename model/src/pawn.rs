//! The `PawnKind` enum represents different types of pawns.
//!
//! Currently, it only includes a `Basic` variant which contains a `usize` value.
//!
//! # Variants
//!
//! * `Basic(usize)` - Represents a basic type of pawn with an associated `usize` value.

/// An enumeration representing different types of pawns.
///
/// # Variants
///
/// * `Basic(usize)` - A basic type of pawn with an associated numerical value.
///   The `usize` value can represent attributes such as rank, level, or any other
///   measurable property of the pawn.
///
/// # Examples
///
/// ```rust
/// use model::pawn::PawnKind;
/// let pawn = PawnKind::Basic(5);
/// match pawn {
///     PawnKind::Basic(value) => println!("This is a basic pawn with a value: {}", value),
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum PawnKind {
    Basic(usize),
}

impl Default for PawnKind {
    /// Provides the default implementation for the `PawnKind` type.
    ///
    /// # Returns
    /// A `PawnKind` instance with the default value:
    /// - `PawnKind::Basic(1)`
    ///
    /// This method is typically used when a default value of `PawnKind`
    /// is required. The default initializes a `Basic` pawn with a value of `1`.
    ///
    /// # Example
    /// ```rust
    /// use model::pawn::PawnKind;
    /// let default_pawn = PawnKind::default();
    /// assert_eq!(default_pawn, PawnKind::Basic(1));
    /// ```
    fn default() -> Self {
        PawnKind::Basic(1)
    }
}

/// Represents a `Pawn` in a game or simulation with a specific kind.
///
/// # Fields
/// - `kind`: The type or category of the `Pawn`. This is represented by the `PawnKind` enum.
///
/// # Derives
/// - `Default`: Provides a default implementation for the `Pawn` struct, where `kind` is initialized
///   with its default value defined by the `PawnKind` type.
///
/// This struct can be used to model various types of pawns, and leveraging the `Default` trait
/// allows for convenient instantiation with sensible defaults.
///
/// # Example
/// ```
/// use model::pawn::Pawn;
/// let default_pawn = Pawn::default();
/// ```
#[derive(Default)]
pub struct Pawn {
    kind: PawnKind,
}
