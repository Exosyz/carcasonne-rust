use crate::builder::tile_builder::TileBuilder;
use crate::factory::tile_factory::TileFactory;
use crate::model::tile::Tile;
use crate::model::tile_feature::Edge::{East, North, South, West};

pub trait TownTileBuilder {
    fn build_c_town() -> Tile;
    fn build_d_town() -> Tile;
    fn build_e_town() -> Tile;
    fn build_f_town() -> Tile;
    fn build_g_town() -> Tile;
    fn build_h_town() -> Tile;
    fn build_i_town() -> Tile;
    fn build_j_town() -> Tile;
    fn build_k_town() -> Tile;
    fn build_l_town() -> Tile;
    fn build_m_town() -> Tile;
    fn build_n_town() -> Tile;
    fn build_o_town() -> Tile;
    fn build_p_town() -> Tile;
    fn build_q_town() -> Tile;
    fn build_r_town() -> Tile;
    fn build_s_town() -> Tile;
    fn build_t_town() -> Tile;
}

impl TownTileBuilder for TileFactory {
    fn build_c_town() -> Tile {
        TileBuilder::new()
            .add_shielded_town(vec![North, West, South, East])
            .build()
    }
    fn build_d_town() -> Tile {
        TileBuilder::new()
            .add_town(vec![North])
            .add_road(vec![West, East])
            .build()
    }
    fn build_e_town() -> Tile {
        TileBuilder::new().add_town(vec![North]).build()
    }
    fn build_f_town() -> Tile {
        TileBuilder::new()
            .add_shielded_town(vec![West, East])
            .build()
    }

    fn build_g_town() -> Tile {
        TileBuilder::new().add_town(vec![West, East]).build()
    }

    fn build_h_town() -> Tile {
        TileBuilder::new()
            .add_town(vec![West])
            .add_town(vec![East])
            .build()
    }

    fn build_i_town() -> Tile {
        TileBuilder::new()
            .add_town(vec![North])
            .add_town(vec![West])
            .build()
    }

    fn build_j_town() -> Tile {
        TileBuilder::new()
            .add_town(vec![North])
            .add_road(vec![South, East])
            .build()
    }

    fn build_k_town() -> Tile {
        TileBuilder::new()
            .add_town(vec![North])
            .add_road(vec![West, East])
            .build()
    }

    fn build_l_town() -> Tile {
        TileBuilder::new()
            .add_town(vec![North])
            .add_road(vec![West])
            .add_road(vec![South])
            .add_road(vec![East])
            .build()
    }

    fn build_m_town() -> Tile {
        TileBuilder::new()
            .add_shielded_town(vec![North, West])
            .build()
    }

    fn build_n_town() -> Tile {
        TileBuilder::new().add_town(vec![North, West]).build()
    }

    fn build_o_town() -> Tile {
        TileBuilder::new()
            .add_shielded_town(vec![North, West])
            .add_road(vec![South, East])
            .build()
    }

    fn build_p_town() -> Tile {
        TileBuilder::new()
            .add_town(vec![North, West])
            .add_road(vec![South, East])
            .build()
    }

    fn build_q_town() -> Tile {
        TileBuilder::new()
            .add_shielded_town(vec![North, West, East])
            .build()
    }

    fn build_r_town() -> Tile {
        TileBuilder::new().add_town(vec![North, West, East]).build()
    }

    fn build_s_town() -> Tile {
        TileBuilder::new()
            .add_shielded_town(vec![North, West, East])
            .add_road(vec![South])
            .build()
    }

    fn build_t_town() -> Tile {
        TileBuilder::new()
            .add_town(vec![North, West, East])
            .add_road(vec![South])
            .build()
    }
}
