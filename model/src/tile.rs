//! This module defines the `Tile`, `TileBuilder`, `TileExtension` enums and structs.
use crate::side::Side;

/// An enumeration representing the possible extensions that can be associated with a tile.
///
/// # Variants
///
/// * `None` - The default value, indicating no extension is present.
/// * `TownShield(usize)` - Represents a town shield extension. It holds a `usize` value,
///   which can be used to carry additional custom data (e.g., a shield identifier or count).
/// * `Abbey` - Represents an abbey extension.
///
/// # Attributes
///
/// * `Debug` - Enables formatting of the enum for debugging purposes.
/// * `Default` - Provides a default value, which is `None`.
/// * `Copy` - Allows the enum to be duplicated through a bitwise copy.
/// * `Clone` - Provides the ability to explicitly clone the enum.
#[derive(Debug, Default, Copy, Clone)]
pub enum TileExtension {
    #[default]
    None,
    TownShield(usize),
    Abbey,
}

/// A struct representing a tile in a grid or board, with defined sides and a potential tile extension.
///
/// This struct is marked with the following traits:
/// - `Debug`: Enables formatting of the struct using the `{:?}` format.
/// - `Default`: Provides a default value for the struct, where each field is initialized to its
///   respective default value.
/// - `Copy`: Allows the `Tile` struct to be copied instead of moved.
/// - `Clone`: Allows the `Tile` struct to be explicitly cloned.
///
/// # Fields
/// - `north` (`Side`): The northern side of the tile.
/// - `south` (`Side`): The southern side of the tile.
/// - `east` (`Side`): The eastern side of the tile.
/// - `west` (`Side`): The western side of the tile.
/// - `tile_extension` (`TileExtension`): Additional information or properties associated with the tile.
///
/// # Example
/// ```
/// use model::tile::Tile;
///
/// let tile = Tile::default();
/// println!("{:?}", tile);
/// ```
#[derive(Debug, Default, Copy, Clone)]
pub struct Tile {
    pub north: Side,
    pub south: Side,
    pub east: Side,
    pub west: Side,
    pub tile_extension: TileExtension,
}
