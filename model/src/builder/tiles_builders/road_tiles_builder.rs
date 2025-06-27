//! The `RoadTileBuilder` trait defines a set of methods for constructing specific road tile configurations
//! within a tile-building system. Each method represents a unique tile configuration where roads and meadows
//! are defined on the tile's sides.
//!
//! # Methods
//!
//! - `build_u_road`: Configures a tile with roads on the north and south sides, and meadows on the west and east sides.
//! - `build_v_road`: Configures a tile with roads on the north and west sides, and meadows on the south and east sides.
//! - `build_w_road`: Configures a tile with roads on the north, west, and south sides (each with different sections),
//!   and a meadow on the east side.
//! - `build_x_road`: Configures a tile with roads on all four sides, each associated with a different section.
//!
//! # Implementations
//!
//! The `TileBuilder` struct implements the `RoadTileBuilder` trait, allowing for the construction of various
//! road tile configurations by chaining builder methods that define the tile's sides and their respective properties.
//!
//! # Examples
//!
//! ```rust
//! use model::builder::tile_builder::TileBuilder;
//! use model::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
//!
//! let mut tile_builder = TileBuilder::default();
//!
//! // Build a U-shaped road tile configuration
//! tile_builder.build_u_road();
//!
//! // Build a V-shaped road tile configuration
//! tile_builder.build_v_road();
//!
//! // Build a W-shaped road tile configuration
//! tile_builder.build_w_road();
//!
//! // Build an X-shaped road tile configuration
//! tile_builder.build_x_road();
//! ```

use crate::builder::tile_builder::TileBuilder;
use crate::side::SideKind;

/// A trait that defines a builder pattern for constructing different types of road tiles.
///
/// This trait provides methods for building specific types of road configurations
/// (U-shaped, V-shaped, W-shaped, and X-shaped roads) in a modular and fluent manner.
/// The builder methods return a mutable reference to the implementing object, making it
/// possible to chain method calls.
pub trait RoadTileBuilder {
    /// Constructs a "U"-shaped road in the current context.
    ///
    /// This method allows building a U-shaped road as part of the
    /// overall structure or design being worked on. The method modifies
    /// the current object in place and returns a mutable reference
    /// to `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_u_road()
    ///        .add_straight_section();
    /// ```
    ///
    /// # Returns
    ///
    /// A mutable reference to `self`, allowing for method chaining.
    ///
    /// # Note
    ///
    /// Ensure that the context to build the U-shaped road is valid
    /// before invoking this method to avoid runtime errors.
    fn build_u_road(&mut self) -> &mut Self;
    /// Builds a vertical road in the current context.
    ///
    /// This method is used to construct a vertical road as part of a
    /// larger structure or map within the object it is called upon.
    /// It is a builder-style method that modifies the current object
    /// and returns a mutable reference to itself, allowing method
    /// chaining.
    ///
    /// # Returns
    ///
    /// A mutable reference to the object (`&mut Self`) after the road
    /// has been built, enabling later chained calls to other
    /// methods.
    ///
    /// # Example
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_v_road()
    ///        .some_other_method();
    /// ```
    fn build_v_road(&mut self) -> &mut Self;
    /// Adds a westward road to the current structure being built.
    ///
    /// This method modifies the current object to include a road leading west.
    /// It can be used in a builder pattern to chain multiple method calls.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - A mutable reference to the current instance, allowing for method chaining.
    ///
    /// # Example
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_w_road()
    ///        .build_n_road();
    /// ```
    fn build_w_road(&mut self) -> &mut Self;
    /// Constructs and adds an "X" road structure in the current context.
    /// This method is designed to modify the current object to include an "X" road and returns a mutable
    /// reference to the same object, allowing method chaining for additional operations.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - A mutable reference to the current instance, enabling chained method calls.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_x_road()
    ///        .another_method();
    /// ```
    ///
    /// This call adds an "X" road to the builder object and allows further method chaining.
    fn build_x_road(&mut self) -> &mut Self;
}

impl RoadTileBuilder for TileBuilder {
    /// Configures and builds a "U"-shaped road for a structure, setting each side with specific types and characteristics.
    ///
    /// The method modifies the current object by specifying the types of the sides
    /// (roads or meadows), as well as additional attributes (e.g., sections for roads).
    ///
    /// # Process:
    /// - **North side**: Sets it to a road with 1 section.
    /// - **West side**: Sets it as a meadow.
    /// - **South side**: Sets it to a road with 1 section.
    /// - **East side**: Sets it as a meadow.
    ///
    /// # Returns
    /// - A mutable reference to `Self` to allow method chaining.
    ///
    /// # Example
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_u_road(); // Configures the sides in a "U"-shape as described.
    /// ```
    ///
    /// # Notes
    /// - This method assumes the presence of utility methods such as `north`, `west`, `south`,
    ///   `east` to set each direction, and a `kind` method to determine the type of side (e.g., `Road` or `Meadow`).
    /// - The `section` method appears to further configure `SideKind::Road` details.
    fn build_u_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Meadow))
    }
    /// Builds a "V"-shaped road structure by configuring the sides of an object.
    ///
    /// This method sets up the sides of the structure as follows:
    /// - The north and west sides are configured as roads, each with a section value of 1.
    /// - The south and east sides are configured as meadows.
    ///
    /// # Returns
    ///
    /// A mutable reference to `Self`, allowing method chaining for additional configuration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_v_road();
    /// ```
    ///
    /// In this example, `build_v_road` will configure the object described by `YourBuilder`
    /// with the specified road and meadow settings for each side.
    fn build_v_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }
    ///
    /// Configures the builder to create a structure with roads on the
    /// north, west, and south sides, and a meadow on the east side.
    ///
    /// # Behavior
    /// - The north side is set with a road of section `1`.
    /// - The west side is set with a road of section `2`.
    /// - The south side is set with a road of section `3`.
    /// - The east side is set as a meadow.
    ///
    /// # Returns
    /// A mutable reference to the builder instance (`Self`), enabling
    /// method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_w_road();
    /// ```
    ///
    /// After calling this method, the structure will have roads on
    /// three sides (north, west, south) and a meadow on the east side.
    ///
    fn build_w_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Road).section(2))
            .south(|s| s.kind(SideKind::Road).section(3))
            .east(|s| s.kind(SideKind::Meadow))
    }
    /// Configures the current object to build an "X-shaped" road by defining roads
    ///  in all four cardinal directions (north, west, south, and east).
    ///
    /// This method modifies the current state by setting up roads on each side
    /// with specified `SideKind` and section numbers.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to `self` to allow method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_x_road();
    /// ```
    ///
    /// In this example, the builder object will have roads defined in all
    /// four cardinal directions:
    /// - **North**: Road with section 1
    /// - **West**: Road with section 2
    /// - **South**: Road with section 3
    /// - **East**: Road with section 4
    ///
    /// # Side Effects
    ///
    /// - Calls the `north`, `west`, `south`, and `east` methods on the current object,
    ///   modifying its internal state.
    /// - Each of these methods is configured to use `SideKind::Road` with specific
    ///   section indices.
    fn build_x_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Road).section(2))
            .south(|s| s.kind(SideKind::Road).section(3))
            .east(|s| s.kind(SideKind::Road).section(4))
    }
}
