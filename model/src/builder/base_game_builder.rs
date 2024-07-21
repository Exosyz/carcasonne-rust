use crate::builder::tiles_builders::{AbbeyTileBuilder, RoadTileBuilder, TownTileBuilder};
use crate::game::GameBuilder;
use crate::tile::TileBuilder;

pub trait BaseGameBuilder {
    fn add_base_game(self) -> Self;
}

impl BaseGameBuilder for GameBuilder {
    fn add_base_game(self) -> Self {
        self
            // Add Abbey
            .add_tile(TileBuilder::new().build_A_abbey().build(), 2)
            .add_tile(TileBuilder::new().build_B_abbey().build(), 4)
            // Add Road
            .add_tile(TileBuilder::new().build_U_road().build(), 8)
            .add_tile(TileBuilder::new().build_V_road().build(), 9)
            .add_tile(TileBuilder::new().build_W_road().build(), 4)
            .add_tile(TileBuilder::new().build_X_road().build(), 1)
            // Add Town
            .add_tile(TileBuilder::new().build_C_town().build(), 1)
            .add_tile(TileBuilder::new().build_D_town().build(), 4)
            .add_tile(TileBuilder::new().build_E_town().build(), 5)
            .add_tile(TileBuilder::new().build_F_town().build(), 2)
            .add_tile(TileBuilder::new().build_G_town().build(), 1)
            .add_tile(TileBuilder::new().build_H_town().build(), 3)
            .add_tile(TileBuilder::new().build_I_town().build(), 2)
            .add_tile(TileBuilder::new().build_J_town().build(), 3)
            .add_tile(TileBuilder::new().build_K_town().build(), 3)
            .add_tile(TileBuilder::new().build_L_town().build(), 3)
            .add_tile(TileBuilder::new().build_M_town().build(), 2)
            .add_tile(TileBuilder::new().build_N_town().build(), 3)
            .add_tile(TileBuilder::new().build_O_town().build(), 2)
            .add_tile(TileBuilder::new().build_P_town().build(), 3)
            .add_tile(TileBuilder::new().build_Q_town().build(), 1)
            .add_tile(TileBuilder::new().build_R_town().build(), 3)
            .add_tile(TileBuilder::new().build_S_town().build(), 2)
            .add_tile(TileBuilder::new().build_T_town().build(), 1)
    }
}
