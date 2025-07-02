use crate::builder::game_builder::GameBuilder;
use crate::factory::tile_factory::abbey_tiles_factory::AbbeyTileBuilder;
use crate::factory::tile_factory::road_tiles_factory::RoadTileBuilder;
use crate::factory::tile_factory::town_tiles_factory::TownTileBuilder;
use crate::factory::tile_factory::TileFactory;
use crate::model::game::GameTiles;

pub struct GameTilesFactory;

impl GameTilesFactory {
    pub fn build_base_game() -> GameTiles {
        GameBuilder::new()
            // Add Abbey
            .add_tiles(TileFactory::build_a_abbey(), 2)
            .add_tiles(TileFactory::build_b_abbey(), 4)
            // Add Road
            .add_tiles(TileFactory::build_u_road(), 8)
            .add_tiles(TileFactory::build_v_road(), 9)
            .add_tiles(TileFactory::build_x_road(), 1)
            .add_tiles(TileFactory::build_w_road(), 4)
            // Add Town
            .add_tiles(TileFactory::build_c_town(), 1)
            .add_tiles(TileFactory::build_d_town(), 4)
            .add_tiles(TileFactory::build_e_town(), 5)
            .add_tiles(TileFactory::build_f_town(), 2)
            .add_tiles(TileFactory::build_g_town(), 1)
            .add_tiles(TileFactory::build_h_town(), 3)
            .add_tiles(TileFactory::build_i_town(), 2)
            .add_tiles(TileFactory::build_j_town(), 3)
            .add_tiles(TileFactory::build_k_town(), 3)
            .add_tiles(TileFactory::build_l_town(), 3)
            .add_tiles(TileFactory::build_m_town(), 2)
            .add_tiles(TileFactory::build_n_town(), 3)
            .add_tiles(TileFactory::build_o_town(), 2)
            .add_tiles(TileFactory::build_p_town(), 3)
            .add_tiles(TileFactory::build_q_town(), 1)
            .add_tiles(TileFactory::build_r_town(), 3)
            .add_tiles(TileFactory::build_s_town(), 2)
            .add_tiles(TileFactory::build_t_town(), 1)
            .build()
    }
}
