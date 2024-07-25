use crate::builder::tiles_builders::{AbbeyTileBuilder, RoadTileBuilder, TownTileBuilder};
use crate::game::GameBuilder;

pub trait BaseGameBuilder {
    fn add_base_game(&mut self) -> &mut Self;
}

impl BaseGameBuilder for GameBuilder {
    fn add_base_game(&mut self) -> &mut Self {
        self
            // Add Abbey
            .add_tile(|t| t.build_A_abbey(), 2)
            .add_tile(|t| t.build_B_abbey(), 4)
            // Add Road
            .add_tile(|t| t.build_U_road(), 8)
            .add_tile(|t| t.build_V_road(), 9)
            .add_tile(|t| t.build_W_road(), 4)
            .add_tile(|t| t.build_X_road(), 1)
            // Add Town
            .add_tile(|t| t.build_C_town(), 1)
            .add_tile(|t| t.build_D_town(), 4)
            .add_tile(|t| t.build_E_town(), 5)
            .add_tile(|t| t.build_F_town(), 2)
            .add_tile(|t| t.build_G_town(), 1)
            .add_tile(|t| t.build_H_town(), 3)
            .add_tile(|t| t.build_I_town(), 2)
            .add_tile(|t| t.build_J_town(), 3)
            .add_tile(|t| t.build_K_town(), 3)
            .add_tile(|t| t.build_L_town(), 3)
            .add_tile(|t| t.build_M_town(), 2)
            .add_tile(|t| t.build_N_town(), 3)
            .add_tile(|t| t.build_O_town(), 2)
            .add_tile(|t| t.build_P_town(), 3)
            .add_tile(|t| t.build_Q_town(), 1)
            .add_tile(|t| t.build_R_town(), 3)
            .add_tile(|t| t.build_S_town(), 2)
            .add_tile(|t| t.build_T_town(), 1)
    }
}
