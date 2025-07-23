mod tile_charset;
mod tile_renderer_helper;

use crate::renderable::tile_renderer::tile_charset::TileCharset;
use crate::renderable::tile_renderer::tile_renderer_helper::TileRendererHelper;
use carcasonne_core::layout::point::Point;
use carcasonne_core::model::tile::Tile;
use carcasonne_core::model::tile_extension::{Abbey, TileExtension};
use carcasonne_core::model::tile_feature::{
    Road, Shield, TileFeature, TileFeatureEnhancement, Town,
};
use std::collections::HashMap;

pub struct TileRenderer;

impl TileRenderer {
    /// Renders a tile using a square grid of placeholder characters.
    ///
    /// This is a stub implementation: the tile is filled with `.` characters
    /// and does not yet reflect actual tile features.
    ///
    /// # Arguments
    /// * `frame` - The drawing buffer.
    /// * `point` - The top-left corner where the tile will be drawn.
    /// * `tile` - The tile to render
    pub(crate) fn tile(size: usize, tile: &Tile) -> Vec<Vec<char>> {
        let mut chars = vec![vec!['.'; size]; size];

        if [""].iter().any(|s| *s == tile.tile_id) {
            dbg!(&tile);
        }

        tile.tile_features.iter().for_each(|feature| {
            Self::apply(Self::render_tile_feature(size, feature), &mut chars);
        });

        Self::apply(
            Self::render_extension(size, &tile.tile_extension),
            &mut chars,
        );

        chars
    }

    fn apply(points: HashMap<Point, TileCharset>, map: &mut [Vec<char>]) {
        for (point, char) in points {
            if point.y >= map.len() || point.x >= map[0].len() {
                dbg!(point);
            }
            map[point.y][point.x] = char.into()
        }
    }

    fn render_tile_feature(size: usize, feature: &TileFeature) -> HashMap<Point, TileCharset> {
        let mut map = HashMap::new();

        if feature
            .feature_type
            .as_any()
            .downcast_ref::<Town>()
            .is_some()
        {
            let town_size =
                TileRendererHelper::center_size(size) + if feature.edges.len() > 1 { 1 } else { 0 };

            feature
                .edges
                .iter()
                .flat_map(|edge| {
                    TileRendererHelper::circle(
                        TileRendererHelper::edge(size, edge),
                        town_size,
                        size,
                    )
                })
                .for_each(|point| {
                    map.insert(point, TileCharset::Town);
                });
        } else if feature
            .feature_type
            .as_any()
            .downcast_ref::<Road>()
            .is_some()
        {
            let center = TileRendererHelper::center(size);
            feature
                .edges
                .iter()
                .flat_map(|edge| {
                    TileRendererHelper::line(TileRendererHelper::edge(size, edge), center, size)
                })
                .filter(|point| *point != center)
                .for_each(|point| {
                    map.insert(point, TileCharset::VerticalRoad);
                });

            if feature.edges.len() > 1 {
                map.insert(center, TileCharset::Crossroad);
            }
        };

        Self::render_tile_enhancement(&mut map, &feature.enhancement, size);

        map
    }

    fn render_tile_enhancement(
        map: &mut HashMap<Point, TileCharset>,
        feature_enhancement: &Option<Box<dyn TileFeatureEnhancement>>,
        size: usize,
    ) {
        if let Some(ext) = feature_enhancement
            && ext.as_any().downcast_ref::<Shield>().is_some()
            && let Some(p) = [
                Point::new(1, 1),
                Point::new(size - 2, 1),
                Point::new(size - 2, TileRendererHelper::center(size).y),
            ]
            .iter()
            .find(|p| map.contains_key(p))
        {
            map.insert(*p, TileCharset::Shield);
        }
    }

    fn render_extension(
        size: usize,
        extension: &Option<Box<dyn TileExtension>>,
    ) -> HashMap<Point, TileCharset> {
        let mut map = HashMap::new();

        if let Some(ext) = extension
            && ext.as_any().downcast_ref::<Abbey>().is_some()
        {
            for point in TileRendererHelper::circle(
                TileRendererHelper::center(size),
                TileRendererHelper::center_size(size),
                size,
            ) {
                map.insert(point, TileCharset::Abbey);
            }
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use carcasonne_core::factory::tile_factory::abbey_tiles_factory::AbbeyTileBuilder;
    use carcasonne_core::factory::tile_factory::road_tiles_factory::RoadTileBuilder;
    use carcasonne_core::factory::tile_factory::town_tiles_factory::TownTileBuilder;
    use carcasonne_core::factory::tile_factory::TileFactory;

    macro_rules! test_tile_render {
        (
         $($name:ident:
            tile = $tile:expr,
            size = $size:expr,
            expected = $expected:literal;
         )*
    ) => {
            $(
                #[test]
                fn $name() {
                    let rendered = TileRenderer::tile($size, &$tile);
                    let rendered_str = rendered
                        .iter()
                        .map(|row| row.iter().collect::<String>())
                        .collect::<Vec<_>>()
                        .join("\n");

                    let expected_str = $expected.trim();

                    println!("\n--- Debug for {} ---", stringify!($name));
                    println!("{}", rendered_str);
                    println!("\n--- Expected for {} ---", stringify!($name));
                    println!("{}", expected_str);

                    assert_eq!(rendered_str, expected_str,);
                }
            )*
        };
    }

    // Abbey tiles
    test_tile_render! {
        test_tile_a_abbey:
            tile = TileFactory::build_a_abbey(),
            size = 5,
            expected = "
.....
..A..
.AAA.
..A..
..║..";
        test_tile_b_abbey:
            tile = TileFactory::build_b_abbey(),
            size = 5,
            expected = "
.....
..A..
.AAA.
..A..
.....";
    }

    // Town tiles
    test_tile_render! {
        test_tile_c_town:
            tile = TileFactory::build_c_town(),
            size = 5,
            expected = "
#####
#S###
#####
#####
#####";

        test_tile_d_town:
            tile = TileFactory::build_d_town(),
            size = 5,
            expected = "
.###.
..#..
═════
.....
.....";

        test_tile_e_town:
            tile = TileFactory::build_e_town(),
            size = 5,
            expected = "
#####
.###.
..#..
.....
.....";

        test_tile_f_town:
            tile = TileFactory::build_f_town(),
            size = 5,
            expected = "
#...#
#####
###S#
#####
#...#";

        test_tile_g_town:
            tile = TileFactory::build_g_town(),
            size = 5,
            expected = "
#...#
#####
#####
#####
#...#";

        test_tile_i_town:
            tile = TileFactory::build_i_town(),
            size = 5,
            expected = "
.###.
#.#..
##...
#....
.....";

        test_tile_j_town:
            tile = TileFactory::build_j_town(),
            size = 5,
            expected = "
.###.
..#..
..╔══
..║..
..║..";

        test_tile_k_town:
            tile = TileFactory::build_k_town(),
            size = 5,
            expected = "
.###.
..#..
══╗..
..║..
..║..";

        test_tile_l_town:
            tile = TileFactory::build_l_town(),
            size = 5,
            expected = "
.###.
..#..
══◻══
..║..
..║..";

        test_tile_m_town:
            tile = TileFactory::build_m_town(),
            size = 5,
            expected = "
#####
.##S#
..###
...##
....#";

        test_tile_n_town:
            tile = TileFactory::build_n_town(),
            size = 5,
            expected = "
#####
.####
..###
...##
....#";

        test_tile_o_town:
            tile = TileFactory::build_o_town(),
            size = 5,
            expected = "
#####
#S##.
###╔═
##╔╝.
#.║..";

        test_tile_p_town:
            tile = TileFactory::build_p_town(),
            size = 5,
            expected = "
#####
#S##.
###╔═
##╔╝.
#.║..";

        test_tile_q_town:
            tile = TileFactory::build_q_town(),
            size = 5,
            expected = "
#####
#S###
#####
##.##
#...#";

        test_tile_r_town:
            tile = TileFactory::build_r_town(),
            size = 5,
            expected = "
#####
#####
#####
##.##
#...#";

        test_tile_s_town:
            tile = TileFactory::build_s_town(),
            size = 5,
            expected = "
#####
#S###
#####
##║##
#.║.#";

        test_tile_t_town:
            tile = TileFactory::build_t_town(),
            size = 5,
            expected = "
#####
#####
#####
##║##
#.║.#";
    }

    // Road tiles
    test_tile_render! {
        test_tile_u_road:
            tile = TileFactory::build_u_road(),
            size = 5,
            expected = "
..║..
..║..
..║..
..║..
..║..";
        test_tile_b_road:
            tile = TileFactory::build_v_road(),
            size = 5,
            expected = "
.....
.....
══╗.
..║..
..║..";

        test_tile_w_road:
            tile = TileFactory::build_w_road(),
            size = 5,
            expected = "
.....
.....
══◻══
..║..
..║..";
        test_tile_x_road:
            tile = TileFactory::build_x_road(),
            size = 5,
            expected = "
..║..
..║..
══◻══
..║..
..║..";
    }
}
