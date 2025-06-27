use crate::builder::side_builder::SideBuilder;
use crate::side::Side;
use crate::tile::{Tile, TileExtension};

/// A builder struct for constructing tile objects with configurable sides and
/// optional tile extensions.
///
/// # Fields
/// * `north` - The northern Side configuration of the tile.
/// * `south` - The southern Side configuration of the tile.
/// * `east` - The eastern Side configuration of the tile.
/// * `west` - The western Side configuration of the tile.
/// * `tile_extension` - An optional extension to provide additional properties or
///   behavior for the tile.
///
/// # Derives
/// * `Default` - The `TileBuilder` struct can be instantiated with default values
///   for all fields.
/// * `Clone` - Allows creating a duplicate instance of a `TileBuilder`.
///
/// # Usage
/// The `TileBuilder` offers a structured approach to configure and build tile
/// structures by specifying the attributes for each tile side along with any
/// additional extensions as needed.
#[derive(Default, Clone)]
pub struct TileBuilder {
    north: Side,
    south: Side,
    east: Side,
    west: Side,
    tile_extension: TileExtension,
}

impl TileBuilder {
    /// Builds a `Side` instance using the provided builder function.
    ///
    /// This function simplifies the process of creating a `Side` by allowing the caller to specify
    /// a closure or function that configures the `SideBuilder`. The closure is called with a mutable
    /// reference to the `SideBuilder`, which can be customized as needed to define the desired
    /// `Side` properties. Once the customization is complete, the builder constructs the `Side` object.
    ///
    /// # Arguments
    ///
    /// * `side_builder` - A closure or function that takes a mutable reference to a `SideBuilder`
    /// and returns a mutable reference to the same `SideBuilder` after configuration.
    ///
    /// # Returns
    ///
    /// * `Side` - The constructed `Side` instance with the specified configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// let side = TileBuilder::build_side(|builder| {
    ///     builder.set_property1("value1")
    ///            .set_property2("value2")
    /// });
    /// ```
    ///
    /// In this example, the `SideBuilder::set_property1` and `set_property2` methods would
    /// configure the builder, and the resulting configured `Side` is returned.
    pub fn build_side(side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder) -> Side {
        let mut builder = SideBuilder::default();
        side_builder(&mut builder);

        builder.build()
    }

    /// Sets the north side of an object using a provided `SideBuilder` closure.
    ///
    /// This function allows the user to define the north side of a structure
    /// by passing a closure that takes a mutable reference to a `SideBuilder`,
    /// which can be used to configure the side. Once the closure is executed,
    /// the configured side is assigned to the `north` property of the object.
    ///
    /// # Arguments
    ///
    /// * `side_builder` - A closure that takes a mutable reference to a
    ///   `SideBuilder` and returns an updated mutable reference to it.
    ///
    /// # Returns
    ///
    /// * A mutable reference to `Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// let mut builder = TileBuilder::default();
    /// builder.north(|side| side.set_color("red").set_width(10));
    /// ```
    pub fn north(
        &mut self,
        side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder,
    ) -> &mut Self {
        self.north = Self::build_side(side_builder);
        self
    }

    /// Sets the `south` side of the current object using a custom builder.
    ///
    /// This method allows you to define or modify the `south` side of an object by passing a closure
    /// that operates on a `SideBuilder`. The closure provides a convenient way to customize the side
    /// configuration. Once the closure is executed, the result is stored as the `south` side of the object.
    ///
    /// # Arguments
    ///
    /// * `side_builder` - A closure that accepts a mutable reference to a `SideBuilder` and outputs
    ///     the modified `SideBuilder`. This is used to configure the `south` side.
    ///
    /// # Returns
    ///
    /// Returns `&mut Self`, allowing method chaining to configure additional properties of the object.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// let mut builder = TileBuilder::default();
    /// builder.south(|side| side.set_width(10).set_color("red"));
    /// ```
    ///
    /// In this example, the `south` side is configured with a width of 10 and a color of "red", and
    /// the method can be chained to further customize or configure the object.
    ///
    /// # Notes
    ///
    /// This method depends on an internal function `Self::build_side` to create the configuration for
    /// the `south` side.
    pub fn south(
        &mut self,
        side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder,
    ) -> &mut Self {
        self.south = Self::build_side(side_builder);
        self
    }

    /// Configures the east side of the object with the given builder function.
    ///
    /// This method takes a closure or other function-like object that operates
    /// on a `SideBuilder` to configure the properties of the east side. Once the
    /// side is built, the configuration will be assigned to the `east` field of
    /// the object.
    ///
    /// # Parameters
    /// - `side_builder`: A function or closure that receives a mutable reference
    ///   to a `SideBuilder` and returns a mutable reference to the same `SideBuilder`
    ///   after applying the desired configuration.
    ///
    /// # Returns
    /// Returns a mutable reference to `self` to allow method chaining.
    ///
    /// # Example
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// let tile = TileBuilder::default()
    ///     .east(|builder| builder.set_color("red").set_size(10))
    ///     .build();
    /// ```
    ///
    /// In this example, the `east` side of `object` is configured with the given
    /// properties using the `SideBuilder`.
    pub fn east(
        &mut self,
        side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder,
    ) -> &mut Self {
        self.east = Self::build_side(side_builder);
        self
    }

    /// Sets the western side of the structure using the provided `side_builder` function.
    ///
    /// This method takes a closure or function implementing the `FnOnce` trait, which is used
    /// to configure a `SideBuilder` instance for the western side. The resulting `SideBuilder`
    /// configuration is then assigned to the `west` attribute of the current structure.
    ///
    /// # Arguments
    /// * `side_builder` - A closure or function that accepts a mutable reference to a `SideBuilder`
    ///   and returns a mutable reference to it after applying the desired configurations.
    ///
    /// # Returns
    /// A mutable reference to `Self`, allowing for method chaining and further configuration.
    ///
    /// # Example
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// let tile = TileBuilder::default()
    ///     .west(|side| side.set_color("blue").add_window(2))
    ///     .build();
    /// ```
    ///
    /// In this example, the `west` method is called to set the color and number of windows
    /// on the western side of the structure.
    pub fn west(
        &mut self,
        side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder,
    ) -> &mut Self {
        self.west = Self::build_side(side_builder);
        self
    }

    /// Sets the tile extension for the current object.
    ///
    /// This method updates the `tile_extension` property with the provided value
    /// and allows for method chaining by returning a mutable reference to `self`.
    ///
    /// # Parameters
    /// - `tile_extension`: The new `TileExtension` value to set.
    ///
    /// # Returns
    /// A mutable reference to the current instance of the caller (`Self`), enabling
    /// method chaining.
    ///
    /// # Example
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::tile::{ TileExtension};
    /// let tile = TileBuilder::default()
    ///     .tile_extension(TileExtension::None)
    ///     .other_method();
    /// ```
    pub fn tile_extension(&mut self, tile_extension: TileExtension) -> &mut Self {
        self.tile_extension = tile_extension;
        self
    }

    /// Constructs and returns a `Tile` instance using the current state of the builder.
    ///
    /// This method takes the fields from the builder instance (`self`) and uses them to create
    /// and return a new `Tile` object. The values for the `Tile` fields (`east`, `north`, `south`,
    /// `west`, and `tile_extension`) are directly copied from the corresponding fields of the builder.
    ///
    /// # Returns
    ///
    /// A `Tile` instance is initialized with the builder's current field values.
    ///
    /// # Example
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::side::SideKind;
    /// use model::tile::{TileExtension};
    ///
    /// let builder = TileBuilder::default()
    ///     .east(|b| b.kind(SideKind::Meadow).section(1))
    ///     .north(|b| b.kind(SideKind::Road).section(1))
    ///     .west(|b| b.kind(SideKind::Town).section(1))
    ///     .south(|b| b.kind(SideKind::Road).section(2))
    ///     .tile_extension(TileExtension::None);
    ///
    /// let tile = builder.build();
    /// ```
    pub fn build(&self) -> Tile {
        Tile {
            east: self.east,
            north: self.north,
            south: self.south,
            west: self.west,
            tile_extension: self.tile_extension,
        }
    }
}
