use crate::builder::tile_builder::TileBuilder;
use crate::factory::tile_factory::TileFactory;
use crate::model::tile::Tile;
use crate::model::tile_feature::Edge::{East, North, South, West};

/// A trait for constructing predefined town tile variants.
///
/// Each method returns a `Tile` representing a unique configuration of town features,
/// based on the base game's tile definitions. These can include shielded towns,
/// connected roads, and various edge combinations.
pub trait TownTileBuilder {
    /// Builds town tile variant C.
    fn build_c_town() -> Tile;
    /// Builds town tile variant D.
    fn build_d_town() -> Tile;
    /// Builds town tile variant E.
    fn build_e_town() -> Tile;
    /// Builds town tile variant F.
    fn build_f_town() -> Tile;
    /// Builds town tile variant G.
    fn build_g_town() -> Tile;
    /// Builds town tile variant H.
    fn build_h_town() -> Tile;
    /// Builds town tile variant I.
    fn build_i_town() -> Tile;
    /// Builds town tile variant J.
    fn build_j_town() -> Tile;
    /// Builds town tile variant K.
    fn build_k_town() -> Tile;
    /// Builds town tile variant L.
    fn build_l_town() -> Tile;
    /// Builds town tile variant M.
    fn build_m_town() -> Tile;
    /// Builds town tile variant N.
    fn build_n_town() -> Tile;
    /// Builds town tile variant O.
    fn build_o_town() -> Tile;
    /// Builds town tile variant P.
    fn build_p_town() -> Tile;
    /// Builds town tile variant Q.
    fn build_q_town() -> Tile;
    /// Builds town tile variant R.
    fn build_r_town() -> Tile;
    /// Builds town tile variant S.
    fn build_s_town() -> Tile;
    /// Builds town tile variant T.
    fn build_t_town() -> Tile;
}

impl TownTileBuilder for TileFactory {
    fn build_c_town() -> Tile {
        TileBuilder::new("C")
            .add_shielded_town(vec![North, West, South, East])
            .build()
    }
    fn build_d_town() -> Tile {
        TileBuilder::new("D")
            .add_town(vec![North])
            .add_road(vec![West, East])
            .build()
    }
    fn build_e_town() -> Tile {
        TileBuilder::new("E").add_town(vec![North]).build()
    }
    fn build_f_town() -> Tile {
        TileBuilder::new("F")
            .add_shielded_town(vec![West, East])
            .build()
    }

    fn build_g_town() -> Tile {
        TileBuilder::new("G").add_town(vec![West, East]).build()
    }

    fn build_h_town() -> Tile {
        TileBuilder::new("H")
            .add_town(vec![West])
            .add_town(vec![East])
            .build()
    }

    fn build_i_town() -> Tile {
        TileBuilder::new("I")
            .add_town(vec![North])
            .add_town(vec![West])
            .build()
    }

    fn build_j_town() -> Tile {
        TileBuilder::new("J")
            .add_town(vec![North])
            .add_road(vec![South, East])
            .build()
    }

    fn build_k_town() -> Tile {
        TileBuilder::new("K")
            .add_town(vec![North])
            .add_road(vec![West, South])
            .build()
    }

    fn build_l_town() -> Tile {
        TileBuilder::new("L")
            .add_town(vec![North])
            .add_road(vec![West])
            .add_road(vec![South])
            .add_road(vec![East])
            .build()
    }

    fn build_m_town() -> Tile {
        TileBuilder::new("M")
            .add_shielded_town(vec![North, East])
            .build()
    }

    fn build_n_town() -> Tile {
        TileBuilder::new("N").add_town(vec![North, East]).build()
    }

    fn build_o_town() -> Tile {
        TileBuilder::new("O")
            .add_shielded_town(vec![North, West])
            .add_road(vec![South, East])
            .build()
    }

    fn build_p_town() -> Tile {
        TileBuilder::new("P")
            .add_town(vec![North, West])
            .add_road(vec![South, East])
            .build()
    }

    fn build_q_town() -> Tile {
        TileBuilder::new("Q")
            .add_shielded_town(vec![North, West, East])
            .build()
    }

    fn build_r_town() -> Tile {
        TileBuilder::new("R")
            .add_town(vec![North, West, East])
            .build()
    }

    fn build_s_town() -> Tile {
        TileBuilder::new("S")
            .add_shielded_town(vec![North, West, East])
            .add_road(vec![South])
            .build()
    }

    fn build_t_town() -> Tile {
        TileBuilder::new("T")
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
    ( $( $name:ident => $tile_expr:expr, $towns:expr, $roads:expr, $tile_id:expr );* $(;)? ) => {
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
                assert_eq!(tile.tile_id, $tile_id, "Wrong tile ID for {}", stringify!($name));
            }
        )*
        };
    }

    generate_town_tile_tests! {
        c_town => TileFactory::build_c_town(), vec![(&[North, West, South, East][..], true)], Vec::<&[Edge]>::new(), "C";
        d_town => TileFactory::build_d_town(), vec![(&[North][..], false)], vec![&[West, East][..]], "D";
        e_town => TileFactory::build_e_town(), vec![(&[North][..], false)], Vec::<&[Edge]>::new(), "E";
        f_town => TileFactory::build_f_town(), vec![(&[West, East][..], true)], Vec::<&[Edge]>::new(), "F";
        g_town => TileFactory::build_g_town(), vec![(&[West, East][..], false)], Vec::<&[Edge]>::new(), "G";
        h_town => TileFactory::build_h_town(), vec![
            (&[West][..], false),
            (&[East][..], false)
        ], Vec::<&[Edge]>::new(), "H";
        i_town => TileFactory::build_i_town(), vec![
            (&[North][..], false),
            (&[West][..], false)
        ], Vec::<&[Edge]>::new(), "I";
        j_town => TileFactory::build_j_town(), vec![(&[North][..], false)], vec![&[South, East][..]], "J";
        k_town => TileFactory::build_k_town(), vec![(&[North][..], false)], vec![&[West, South][..]], "K";
        l_town => TileFactory::build_l_town(), vec![(&[North][..], false)], vec![&[West][..], &[South][..], &[East][..]], "L";
        m_town => TileFactory::build_m_town(), vec![(&[North, East][..], true)], Vec::<&[Edge]>::new(), "M";
        n_town => TileFactory::build_n_town(), vec![(&[North, East][..], false)], Vec::<&[Edge]>::new(), "N";
        o_town => TileFactory::build_o_town(), vec![(&[North, West][..], true)], vec![&[South, East][..]], "O";
        p_town => TileFactory::build_p_town(), vec![(&[North, West][..], false)], vec![&[South, East][..]], "P";
        q_town => TileFactory::build_q_town(), vec![(&[North, West, East][..], true)], Vec::<&[Edge]>::new(), "Q";
        r_town => TileFactory::build_r_town(), vec![(&[North, West, East][..], false)], Vec::<&[Edge]>::new(), "R";
        s_town => TileFactory::build_s_town(), vec![(&[North, West, East][..], true)], vec![&[South][..]], "S";
        t_town => TileFactory::build_t_town(), vec![(&[North, West, East][..], false)], vec![&[South][..]], "T";
    }
}
