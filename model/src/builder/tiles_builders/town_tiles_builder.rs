//! This module defines a trait `TownTileBuilder` and its implementation for `TileBuilder`.

use crate::builder::tile_builder::TileBuilder;
use crate::side::SideKind;
use crate::tile::TileExtension;

/// A trait that defines a builder for creating various types of town tiles.
///
/// This trait provides methods for incrementally constructing different town tiles,
/// identified by specific letters. The builder pattern allows for a flexible and
/// chained construction process, where each method modifies the builder's internal state
/// and returns a mutable reference to itself for further chaining.
pub trait TownTileBuilder {
    /// Constructs or initiates the setup process for building C-town within the current context.
    ///
    /// This method is typically used to add or configure elements associated with C-town,
    /// such as infrastructure, layout, or any specific logic tied to C-town's construction.
    ///
    /// # Returns
    /// A mutable reference to the current object (`Self`), enabling method chaining for further
    /// configuration or building processes.
    ///
    /// # Example
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_c_town()
    ///        .finalize();
    /// ```
    ///
    /// # Note
    /// Ensure that any prerequisite conditions required for building C-town are
    /// satisfied before calling this method.
    fn build_c_town(&mut self) -> &mut Self;
    /// Constructs and initializes D-Town for the current instance.
    ///
    /// This method is used to build or configure a D-Town entity and update the
    /// current structure accordingly. After calling this method, the instance is
    /// returned for further chaining or modification.
    ///
    /// ### Example
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_d_town()
    ///             .other_method();
    /// ```
    ///
    /// ### Returns
    /// A mutable reference to `Self`, allowing method chaining.
    ///
    /// ### Note
    /// Ensure that the instance is properly initialized before calling `build_d_town`
    /// to avoid undefined behavior or logical inconsistencies in your program.
    fn build_d_town(&mut self) -> &mut Self;
    /// Constructs or initiates the building process for an e-town within the context of the current structure.
    ///
    /// This method is a part of a builder pattern and allows chaining multiple method calls.
    /// It modifies the internal state of the object to reflect the addition or construction of an e-town.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of the type implementing this method.
    /// This allows method chaining by returning `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_e_town()
    ///        .other_method()
    ///        .finalize();
    /// ```
    fn build_e_town(&mut self) -> &mut Self;
    /// Configures the `f_town` setting for the current builder object.
    ///
    /// This method is part of a builder pattern. It allows the modification of
    /// the existing object to set up parameters related to `f_town`.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the builder object, enabling method
    /// chaining to apply further customizations or configurations.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_f_town()
    ///        .other_config_method();
    /// ```
    ///
    /// # Note
    /// Ensure that all required configurations are completed before calling
    /// methods that finalize or consume the builder.
    ///
    fn build_f_town(&mut self) -> &mut Self;
    /// Builds or initializes the "G Town" feature for the object this method is called on.
    ///
    /// This method mutably borrows the instance of the object, allowing it to modify its state.
    /// After modifying, it returns a mutable reference to `Self` (the same object), enabling method chaining.
    ///
    /// # Returns
    /// A mutable reference to the current object (`Self`), enabling further method calls chained to this method.
    ///
    /// # Example
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_g_town()
    ///        .another_method();
    /// ```
    fn build_g_town(&mut self) -> &mut Self;
    /// Constructs and adds an "H Town" building or entity to the current city or simulation setup.
    ///
    /// This method is typically used to include "H Town" as part of the larger structure being built
    /// or simulated. The method modifies the current instance and then returns a mutable reference
    /// to allow for method chaining.
    ///
    /// # Returns
    ///
    /// A mutable reference to `Self`, allowing method chaining or further modifications.
    ///
    /// # Example
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_h_town()
    ///             .build_marketplace()
    ///             .build_park();
    /// ```
    ///
    /// In the above example, the `build_h_town` method is called to include
    /// "H Town" in the `city_builder`, followed by the addition of other entities.
    ///
    /// # Note
    ///
    /// Ensure that the necessary dependencies or configurations required for creating "H Town"
    /// are already set up before invoking this method.
    ///
    /// # Errors
    ///
    /// This method will not produce errors by itself but ensures the context or resources
    /// required for adding "H Town" are valid.
    ///
    /// # Mutability
    ///
    /// Since this method modifies the current instance, a mutable reference to the instance
    /// (`&mut self`) is required.
    fn build_h_town(&mut self) -> &mut Self;
    /// Constructs or builds the "I-Town" structure or feature in the current context.
    /// This function modifies the current instance and allows method chaining.
    ///
    /// # Returns
    /// A mutable reference to `Self`, enabling chained method calls.
    ///
    /// # Example
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_i_town()
    ///        .another_method();
    /// ```
    fn build_i_town(&mut self) -> &mut Self;
    /// Initiates the construction of the "J Town" structure within the entity.
    ///
    /// This method modifies the current instance by beginning the process
    /// of building "J Town" and allows for method chaining by returning a mutable
    /// reference to itself. The specific behavior and structure of "J Town" should
    /// be defined within the implementation.
    ///
    /// # Example
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_j_town();
    /// ```
    ///
    /// # Returns
    /// * `&mut Self` - A mutable reference to the current instance to support method chaining.
    ///
    /// # Usage
    /// This method is typically used in a builder pattern to chain multiple
    /// method calls for configuring or constructing an entity.
    ///
    /// # Note
    /// Ensure that necessary pre-conditions (if any) for building "J Town" are
    /// satisfied before invoking this method.
    fn build_j_town(&mut self) -> &mut Self;
    /// Constructs or builds the "K Town" structure or entity within the current context.
    ///
    /// This method modifies the internal state of the instance to include or initialize the
    /// "K Town" component or functionality. It applies the necessary configurations or steps
    /// required for the construction of "K Town."
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance (`Self`) allowing method chaining.
    ///
    /// # Examples
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_k_town()
    ///        .finalize();
    /// ```
    ///
    /// This example demonstrates using `build_k_town` within a chain of method calls,
    /// modifying the builder and continuing with further configurations or actions.
    fn build_k_town(&mut self) -> &mut Self;
    /// Builds or constructs the "L-Town" structure or configuration within the context of the type
    /// implementing this method. This method modifies the current instance and allows chaining
    /// of additional method calls.
    ///
    /// # Returns
    /// * `&mut Self` - A mutable reference to the current instance, enabling method chaining.
    ///
    /// # Example
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_l_town()
    ///        .some_other_method();
    /// ```
    ///
    /// # Notes
    /// This method assumes that all necessary prerequisites for building "L-Town"
    /// have already been satisfied before its invocation.
    fn build_l_town(&mut self) -> &mut Self;
    /// Configures and builds the "M Town". This method modifies the current object
    /// and allows for method chaining.
    ///
    /// # Returns
    ///
    /// A mutable reference to `Self` to allow chaining further method calls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_m_town()
    ///        .finalize();
    /// ```
    fn build_m_town(&mut self) -> &mut Self;
    /// Adds an N-Town to the current object or configuration. This function is typically
    /// used in a builder pattern to chain multiple customization or setup steps.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to `self` to allow method chaining.
    ///
    /// # Example
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_n_town()
    ///       .setup_city()
    ///       .finalize();
    /// ```
    ///
    /// # Note
    /// Specific details about what constitutes an "N-Town" should be explained or
    /// sought from the larger context of this method's implementation.
    fn build_n_town(&mut self) -> &mut Self;
    /// Constructs or configures "O-Town" within the current context or object.
    ///
    /// This method is used to initialize or build the O-Town functionality
    /// associated with the object implementing this function. It returns
    /// a mutable reference to `Self`, enabling method chaining.
    ///
    /// # Example
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_o_town()
    ///        .finalize();
    /// ```
    ///
    /// # Returns
    /// * `&mut Self` - A mutable reference to the current instance for method chaining.
    ///
    /// # Notes
    /// * Ensure the method is called in the appropriate sequence if other
    ///   dependencies exist in the builder process.
    fn build_o_town(&mut self) -> &mut Self;
    /// Configures and builds a "P Town" in the current context.
    ///
    /// This method is part of a builder pattern, allowing chained calls to configure
    /// and construct a desired object or structure. Calling `build_p_town` applies
    /// the necessary setup for "P Town" and returns a mutable reference to `self`,
    /// enabling further configuration.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of the object (`self`) to allow
    /// for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_p_town()
    ///        .another_method();
    /// ```
    ///
    /// # Notes
    ///
    /// - Ensure any required prerequisites for building "P Town" are completed before
    /// calling this method.
    /// - This method modifies the current instance's state.
    fn build_p_town(&mut self) -> &mut Self;
    /// Configures the builder to include a "Q-Town" in the current object being constructed.
    ///
    /// This is a chainable method that modifies the internal state of the builder
    /// to incorporate details for a "Q-Town". The exact implementation depends on
    /// the builder's use case.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - A mutable reference to the builder instance, allowing for
    /// further chained method calls.
    ///
    /// # Example
    ///
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_q_town()
    ///        .build_another_feature();
    /// ```
    fn build_q_town(&mut self) -> &mut Self;
    /// Configures and builds an "R Town" component within the current builder object.
    ///
    /// This method is typically used in a builder pattern to enable chaining of
    /// methods for configuring and constructing a larger structure or system. The `build_r_town`
    /// function performs all necessary operations to set up the "R Town" component and returns
    /// a mutable reference to the builder instance, allowing for further configuration.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - A mutable reference to the builder instance, enabling method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_r_town()
    ///        .configure_other_components()
    ///        .finalize();
    /// ```
    fn build_r_town(&mut self) -> &mut Self;
    /// Constructs or initializes a "'S Town" component and integrates it into the current state of the object.
    ///
    /// This function modifies the state of the object by building the "S Town" component
    /// and allows for method chaining by returning a mutable reference to the same object.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance of the object, allowing for method chaining.
    ///
    /// # Example
    /// ```
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_s_town()
    ///        .build_other_component();
    /// ```
    ///
    /// # Note
    ///
    /// Ensure that this function is called at the appropriate stage in the object's construction
    /// process to avoid any inconsistencies or unexpected behavior.
    fn build_s_town(&mut self) -> &mut Self;
    /// Configures and builds the T-Town configuration for the current instance of the object.
    ///
    /// This method allows the user to modify the current object by adding or configuring
    /// settings specific to the T-Town. It applies the desired configuration and then
    /// returns a mutable reference to the modified instance, enabling method chaining.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - A mutable reference to the current instance of the object, allowing
    ///   for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::tile_builder::TileBuilder;
    /// use model::builder::tiles_builders::town_tiles_builder::TownTileBuilder;
    ///
    /// let mut builder = TileBuilder::default();
    /// builder.build_t_town()
    /// ```
    fn build_t_town(&mut self) -> &mut Self;
}

impl TownTileBuilder for TileBuilder {
    fn build_c_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Town).section(1))
            .east(|s| s.kind(SideKind::Town).section(1))
            .tile_extension(TileExtension::TownShield(1))
    }
    fn build_d_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Road).section(1))
    }
    fn build_e_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }
    fn build_f_town(&mut self) -> &mut Self {
        self.build_g_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_g_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Meadow))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Town).section(1))
    }

    fn build_h_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Meadow))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Town).section(2))
    }

    fn build_i_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(2))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }

    fn build_j_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Road).section(1))
    }

    fn build_k_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Road).section(1))
    }

    fn build_l_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Road).section(2))
            .east(|s| s.kind(SideKind::Road).section(3))
    }

    fn build_m_town(&mut self) -> &mut Self {
        self.build_n_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_n_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }

    fn build_o_town(&mut self) -> &mut Self {
        self.build_p_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_p_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Road).section(1))
    }

    fn build_q_town(&mut self) -> &mut Self {
        self.build_r_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_r_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Town).section(1))
    }

    fn build_s_town(&mut self) -> &mut Self {
        self.build_t_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_t_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Town).section(1))
    }
}
