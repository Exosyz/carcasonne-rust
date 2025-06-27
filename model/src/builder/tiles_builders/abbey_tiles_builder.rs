use crate::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
use crate::side::SideKind;
use crate::tile::{TileBuilder, TileExtension};

/// A trait defining the builder pattern for creating Abbey tiles.
///
/// This trait provides a set of methods to construct various components of an "Abbey"
/// tile, allowing for customization and step-by-step composition.
///
/// # Required Methods
/// Implementors of this trait need to provide concrete implementations of the following methods:
pub trait AbbeyTileBuilder {
    /// Constructs or builds an "abbey" as part of this instance's state.
    ///
    /// This method modifies the current instance, allowing the chaining of
    /// additional method calls. The specifics of what constitutes building an
    /// "abbey" depend on the implementation details within the struct this
    /// function belongs to.
    ///
    /// # Returns
    ///
    /// This method returns a mutable reference to the current instance, allowing
    /// for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::tile::TileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_a_abbey().finalize();
    /// ```
    ///
    /// Here, `build_a_abbey` is called on the `builder` instance, and its result
    /// is chained with a call to `finalize`.
    ///
    /// # Note
    ///
    /// Ensure that this method is used with the appropriate context of the struct
    /// it belongs to, as it modifies the internal state.
    fn build_a_abbey(&mut self) -> &mut Self;
    /// Constructs and adds a "B Abbey" structure to the current entity or context.
    ///
    /// This method modifies the entity or context it is called upon by adding
    /// a specific structure referred to as a "B Abbey". After adding this
    /// structure, it returns a mutable reference to the current instance, allowing
    /// for method chaining.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to `self` to facilitate further chained
    /// method calls.
    ///
    /// # Example
    ///
    /// ```
    /// use model::tile::TileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_b_abbey()
    ///        .some_other_method();
    /// ```
    ///
    /// # Notes
    ///
    /// - The exact behavior of "B Abbey" should be described in further detail in the context
    ///   where this method is implemented.
    /// - Ensure the functionality of this method aligns with the requirements of the broader
    ///   application.
    fn build_b_abbey(&mut self) -> &mut Self;
}

impl AbbeyTileBuilder for TileBuilder {
    /// Builds an abbey on the tile by extending the current configuration.
    ///
    /// This method modifies the current builder to include an abbey as a tile extension.
    /// It internally calls `build_e_town` to initialize an empty town and then appends
    /// an abbey to the configuration using `tile_extension`.
    ///
    /// # Returns
    /// A mutable reference to `Self` to allow method chaining.
    ///
    /// # Example
    /// ```rust
    /// use model::tile::TileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_a_abbey();
    /// ```
    ///
    /// # Panics
    /// This method does not explicitly panic, but it assumes that
    /// `build_e_town` and `tile_extension` methods are valid and functional.
    ///
    /// # Notes
    /// - Ensure that `TileExtension::Abbey` is a valid enum variant in your code.
    /// - This method assumes that a town must be built before adding the abbey.
    ///
    /// # See Also
    /// - [`build_e_town`](#method.build_e_town)
    /// - [`tile_extension`](#method.tile_extension)
    fn build_a_abbey(&mut self) -> &mut Self {
        self.build_e_town().tile_extension(TileExtension::Abbey)
    }
    /// Configures and builds a tile with an abbey structure. The method modifies
    /// the tile by setting all four sides (north, west, south, and east) as meadows.
    /// Additionally, it applies the `TileExtension::Abbey` to the tile, signifying
    /// that this tile represents an abbey.
    ///
    /// # Returns
    /// A mutable reference to `Self` to facilitate method chaining while building the tile.
    ///
    /// # Examples
    /// ```rust
    /// use model::tile::TileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_b_abbey();
    /// // The tile now has all sides set to `SideKind::Meadow`
    /// // and an abbey extension added.
    /// ```
    ///
    /// # Usage
    /// - This method is designed for creating specific tiles in the context
    ///   of board or tile-based games.
    /// - It assumes the builder API allows chaining of configuration methods.
    ///
    /// # Preconditions
    /// - The `TileExtension::Abbey` and `SideKind::Meadow` types must be valid
    ///   within the context of the application.
    ///
    fn build_b_abbey(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Meadow))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
            .tile_extension(TileExtension::Abbey)
    }
}
