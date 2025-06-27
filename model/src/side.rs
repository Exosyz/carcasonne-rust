//! This module provides structures and builder functionality for creating `Side` objects.

/// An enumeration representing different kinds of sides or terrains in a game or mapping application.
///
/// # Variants
///
/// - `Meadow`: Represents a meadow terrain. This is the default variant.
/// - `Town`: Represents a town or urban terrain.
/// - `Road`: Represents a road terrain.
///
/// # Traits
///
/// This enumeration derives the following traits:
/// - `Debug`: Enables printing the value of the enum for debugging purposes.
/// - `Default`: Provides a default value, which is `Meadow`.
/// - `Copy`: Allows the enum to be copied instead of moved.
/// - `Clone`: Enables explicit copying of the enum.
///
/// # Example
///
/// ```rust
/// use model::side::SideKind;
///
/// let default_side = SideKind::default(); // Returns SideKind::Meadow
/// let town = SideKind::Town;
/// let road = SideKind::Road;
///
/// println!("{:?}", default_side); // Prints "Meadow"
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub enum SideKind {
    #[default]
    Meadow,
    Town,
    Road,
}

///
/// Represents a side with a specific section index and a kind (type) of side.
///
/// # Attributes
/// - `section` (`usize`): The index of the section corresponding to this side.
/// - `kind` (`SideKind`): The specific type or kind of this side.
///
/// This struct derives the following traits:
/// - `Debug`: Enables formatting using the `{:?}` formatter.
/// - `Default`: Provides a default implementation for the struct.
/// - `Copy`: Allows bitwise copying of the struct.
/// - `Clone`: Produces a duplicate value of the struct.
///
/// # Example
/// ```
/// use model::builder::side_builder::SideBuilder;
/// use model::side::{Side, SideKind};
/// let side = SideBuilder::default()
///             .kind(SideKind::Meadow)
///             .section(1)
///             .build();
///
/// println!("{:?}", side);
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub struct Side {
    pub(crate) section: usize,
    pub(crate) kind: SideKind,
}

impl Side {}
