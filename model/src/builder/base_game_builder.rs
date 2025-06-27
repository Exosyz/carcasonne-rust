//! The `BaseGameBuilder` trait defines functionality for constructing
//! the base tiles for a game. It provides a method that must be implemented
//! by any struct that wants to support adding the base tileset of the game.

use crate::builder::game_builder::GameBuilder;
use crate::builder::tiles_builders::abbey_tiles_builder::AbbeyTileBuilder;
use crate::builder::tiles_builders::road_tiles_builder::RoadTileBuilder;
use crate::builder::tiles_builders::town_tiles_builder::TownTileBuilder;

/// A trait that provides functionality to add a base game configuration or setup.
///
/// Any builder implementing this trait is expected to define how the base game is
/// added or initialized within the overall structure of the game-building process.
///
/// # Required Methods
/// - `add_base_game`: Adds the base game setup, returning a mutable reference to the builder itself for chaining.
///
/// # Use Case
/// This is useful in scenarios where a game has common elements or logic that needs to
/// be initialized as part of a larger customization or game-building process.
pub trait BaseGameBuilder {
    /// Adds the base game systems, components, and functionality to the application.
    ///
    /// This function is typically used to initialize all necessary parts
    /// and systems required for the base game logic. It ensures the application
    /// contains the foundational setup to run the game properly.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to `Self`, allowing method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use model::builder::base_game_builder::BaseGameBuilder;
    /// use model::builder::game_builder::GameBuilder;
    /// let mut app = GameBuilder::default();
    /// app.add_base_game()
    ///    .run();
    /// ```
    fn add_base_game(&mut self) -> &mut Self;
}

impl BaseGameBuilder for GameBuilder {
    /// Adds the base game tiles to the current game setup.
    ///
    /// This method initializes the base set of tiles required for the game,
    /// including abbeys, roads, and towns. Each tile type is added using a
    /// specific builder function together with the number of tiles of that type
    /// to include in the setup.
    ///
    /// # Tiles Added
    ///
    /// - **Abbey Tiles**:
    ///   - `build_a_abbey`: 2 tiles
    ///   - `build_b_abbey`: 4 tiles
    ///
    /// - **Road Tiles**:
    ///   - `build_u_road`: 8 tiles
    ///   - `build_v_road`: 9 tiles
    ///   - `build_w_road`: 4 tiles
    ///   - `build_x_road`: 1 tile
    ///
    /// - **Town Tiles**:
    ///   - `build_c_town`: 1 tile
    ///   - `build_d_town`: 4 tiles
    ///   - `build_e_town`: 5 tiles
    ///   - `build_f_town`: 2 tiles
    ///   - `build_g_town`: 1 tile
    ///   - `build_h_town`: 3 tiles
    ///   - `build_i_town`: 2 tiles
    ///   - `build_j_town`: 3 tiles
    ///   - `build_k_town`: 3 tiles
    ///   - `build_l_town`: 3 tiles
    ///   - `build_m_town`: 2 tiles
    ///   - `build_n_town`: 3 tiles
    ///   - `build_o_town`: 2 tiles
    ///   - `build_p_town`: 3 tiles
    ///   - `build_q_town`: 1 tile
    ///   - `build_r_town`: 3 tiles
    ///   - `build_s_town`: 2 tiles
    ///   - `build_t_town`: 1 tile
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to `Self`, which allows chaining further modifications
    /// to the game setup after adding the tiles.
    /// ```
    fn add_base_game(&mut self) -> &mut Self {
        self
            // Add Abbey
            .add_tile(|t| t.build_a_abbey(), 2)
            .add_tile(|t| t.build_b_abbey(), 4)
            // Add Road
            .add_tile(|t| t.build_u_road(), 8)
            .add_tile(|t| t.build_v_road(), 9)
            .add_tile(|t| t.build_w_road(), 4)
            .add_tile(|t| t.build_x_road(), 1)
            // Add Town
            .add_tile(|t| t.build_c_town(), 1)
            .add_tile(|t| t.build_d_town(), 4)
            .add_tile(|t| t.build_e_town(), 5)
            .add_tile(|t| t.build_f_town(), 2)
            .add_tile(|t| t.build_g_town(), 1)
            .add_tile(|t| t.build_h_town(), 3)
            .add_tile(|t| t.build_i_town(), 2)
            .add_tile(|t| t.build_j_town(), 3)
            .add_tile(|t| t.build_k_town(), 3)
            .add_tile(|t| t.build_l_town(), 3)
            .add_tile(|t| t.build_m_town(), 2)
            .add_tile(|t| t.build_n_town(), 3)
            .add_tile(|t| t.build_o_town(), 2)
            .add_tile(|t| t.build_p_town(), 3)
            .add_tile(|t| t.build_q_town(), 1)
            .add_tile(|t| t.build_r_town(), 3)
            .add_tile(|t| t.build_s_town(), 2)
            .add_tile(|t| t.build_t_town(), 1)
    }
}
