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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tile_feature::{
        Edge,
        Edge::{East, North, South, West},
        Road, Shield, Town,
    };
    use std::any::TypeId;

    macro_rules! generate_town_tile_tests {
    ( $( $name:ident => $tile_expr:expr, $towns:expr, $roads:expr );* $(;)? ) => {
        $(
            #[test]
            fn $name() {
                let tile = $tile_expr;
                let town_features: Vec<_> = tile.tile_features.iter()
                    .filter(|f| f.feature_type.as_ref().type_id() == TypeId::of::<Town>())
                    .collect();
                let road_features: Vec<_> = tile.tile_features.iter()
                    .filter(|f| f.feature_type.as_ref().type_id() == TypeId::of::<Road>())
                    .collect();

                assert_eq!(town_features.len(), $towns.len(), "Wrong number of Town features for {}", stringify!($name));
                for (feature, (edges, has_shield)) in town_features.iter().zip($towns.iter()) {
                    assert_eq!(&feature.edges, edges);
                    match (feature.enhancement.as_ref(), has_shield) {
                        (Some(enhancement), true) => {
                            assert_eq!(enhancement.as_ref().type_id(), TypeId::of::<Shield>())
                        }
                        (None, false) => (),
                        (Some(_), false) => panic!("Unexpected shield"),
                        (None, true) => panic!("Missing shield"),
                    }
                }

                assert_eq!(road_features.len(), $roads.len(), "Wrong number of Road features for {}", stringify!($name));
                for (feature, edges) in road_features.iter().zip($roads.iter()) {
                    assert_eq!(&feature.edges, edges);
                }
            }
        )*
        };
    }

    generate_town_tile_tests! {
        c_town => TileFactory::build_c_town(), vec![(&[North, West, South, East][..], true)], Vec::<&[Edge]>::new();
        d_town => TileFactory::build_d_town(), vec![(&[North][..], false)], vec![&[West, East][..]];
        e_town => TileFactory::build_e_town(), vec![(&[North][..], false)], Vec::<&[Edge]>::new();
        f_town => TileFactory::build_f_town(), vec![(&[West, East][..], true)], Vec::<&[Edge]>::new();
        g_town => TileFactory::build_g_town(), vec![(&[West, East][..], false)], Vec::<&[Edge]>::new();
        h_town => TileFactory::build_h_town(), vec![
            (&[West][..], false),
            (&[East][..], false)
        ], Vec::<&[Edge]>::new();
        i_town => TileFactory::build_i_town(), vec![
            (&[North][..], false),
            (&[West][..], false)
        ], Vec::<&[Edge]>::new();
        j_town => TileFactory::build_j_town(), vec![(&[North][..], false)], vec![&[South, East][..]];
        k_town => TileFactory::build_k_town(), vec![(&[North][..], false)], vec![&[West, East][..]];
        l_town => TileFactory::build_l_town(), vec![(&[North][..], false)], vec![&[West][..], &[South][..], &[East][..]];
        m_town => TileFactory::build_m_town(), vec![(&[North, West][..], true)], Vec::<&[Edge]>::new();
        n_town => TileFactory::build_n_town(), vec![(&[North, West][..], false)], Vec::<&[Edge]>::new();
        o_town => TileFactory::build_o_town(), vec![(&[North, West][..], true)], vec![&[South, East][..]];
        p_town => TileFactory::build_p_town(), vec![(&[North, West][..], false)], vec![&[South, East][..]];
        q_town => TileFactory::build_q_town(), vec![(&[North, West, East][..], true)], Vec::<&[Edge]>::new();
        r_town => TileFactory::build_r_town(), vec![(&[North, West, East][..], false)], Vec::<&[Edge]>::new();
        s_town => TileFactory::build_s_town(), vec![(&[North, West, East][..], true)], vec![&[South][..]];
        t_town => TileFactory::build_t_town(), vec![(&[North, West, East][..], false)], vec![&[South][..]];
    }
}
