mod tile_charset;
mod tile_renderer_helper;
mod tile_renderer_utils;

use crate::renderable::tile_renderer::tile_charset::TileCharset;
use crate::renderable::tile_renderer::tile_renderer_helper::TileRendererHelper;
use carcasonne_core::layout::point::Point;
use carcasonne_core::model::rotation::Rotation;
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
    pub(crate) fn tile(size: usize, tile: &Tile, rotation: &Rotation) -> Vec<Vec<char>> {
        let mut chars = vec![vec!['.'; size]; size];

        Self::apply(
            tile.tile_features
                .iter()
                .fold(HashMap::new(), |mut acc, feature| {
                    Self::render_tile_feature(size, feature, rotation, &mut acc);
                    acc
                }),
            &mut chars,
        );

        Self::apply(
            Self::render_extension(size, &tile.tile_extension),
            &mut chars,
        );

        chars
    }

    fn apply(points: HashMap<Point, TileCharset>, map: &mut [Vec<char>]) {
        points
            .iter()
            .for_each(|(point, char)| map[point.y][point.x] = (*char).into());
    }

    fn render_tile_feature(
        size: usize,
        feature: &TileFeature,
        rotation: &Rotation,
        map: &mut HashMap<Point, TileCharset>,
    ) {
        let any = feature.feature_type.as_any();
        if any.downcast_ref::<Town>().is_some() {
            TileRendererHelper::render_town_edges(size, map, &feature.edges);
        } else if any.downcast_ref::<Road>().is_some() {
            TileRendererHelper::render_road_edges(size, map, &feature.edges);
        };

        Self::render_tile_enhancement(size, map, &feature.enhancement, rotation);
    }

    fn render_tile_enhancement(
        size: usize,
        map: &mut HashMap<Point, TileCharset>,
        feature_enhancement: &Option<Box<dyn TileFeatureEnhancement>>,
        rotation: &Rotation,
    ) {
        let Some(ext) = feature_enhancement else {
            return;
        };
        if ext.as_any().downcast_ref::<Shield>().is_some() {
            TileRendererHelper::render_shield_enhancement(size, map, rotation);
        }
    }

    fn render_extension(
        size: usize,
        extension: &Option<Box<dyn TileExtension>>,
    ) -> HashMap<Point, TileCharset> {
        let mut map = HashMap::new();
        let Some(ext) = extension else {
            return map;
        };

        if ext.as_any().downcast_ref::<Abbey>().is_some() {
            TileRendererHelper::render_abbey_extension(size, &mut map);
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

    fn convert_char(c: char) -> Option<char> {
        match c {
            '.' => Some(TileCharset::None.into()),
            'A' => Some(TileCharset::Abbey.into()),
            '║' => Some(TileCharset::VerticalRoad.into()),
            '═' => Some(TileCharset::HorizontalRoad.into()),
            '#' => Some(TileCharset::Town.into()),
            'S' => Some(TileCharset::Shield.into()),
            '╝' => Some(TileCharset::CornerTopLeft.into()),
            '╚' => Some(TileCharset::CornerTopRight.into()),
            '╗' => Some(TileCharset::CornerBottomLeft.into()),
            '╔' => Some(TileCharset::CornerBottomRight.into()),
            '◻' => Some(TileCharset::EndRoad.into()),
            _ => None,
        }
    }

    fn rotate_char_left(c: &char) -> char {
        match c {
            '║' => '═',
            '═' => '║',
            '╝' => '╗',
            '╚' => '╝',
            '╗' => '╔',
            '╔' => '╚',
            c => *c,
        }
    }

    pub fn convert_test_string_to_tile_string(test_string: &str) -> Result<String, String> {
        test_string
            .chars()
            .map(|c| {
                if c.is_whitespace() {
                    Ok(c)
                } else {
                    convert_char(c).ok_or_else(|| format!("Unknown test character: '{c}'"))
                }
            })
            .collect()
    }

    fn rotate_left_expected(expected: String) -> String {
        // map to matrix
        let mut expected_matrix = expected
            .trim()
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // rotate 2D matrix 90° clockwise
        expected_matrix = if expected_matrix.is_empty() || expected_matrix[0].is_empty() {
            Vec::new()
        } else {
            let rows = expected_matrix.len();
            let cols = expected_matrix[0].len();

            (0..cols)
                .rev()
                .map(|col| (0..rows).map(|row| expected_matrix[row][col]).collect())
                .collect()
        };

        //map chars
        expected_matrix
            .iter()
            .map(|row| row.iter().map(rotate_char_left).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn assert_tile(tile_factory_fn: fn() -> Tile, size: usize, expected: &str) {
        let mut tile = tile_factory_fn();
        let expected = expected.trim().to_string();
        let mut rotated_expected = expected.clone();
        assert_tile_render(&tile, size, rotated_expected.as_str(), Rotation::R0);

        // rotate 90°
        tile.rotate_left();
        rotated_expected = rotate_left_expected(rotated_expected.to_string());
        assert_tile_render(&tile, size, rotated_expected.as_str(), Rotation::R90);

        // rotate 180°
        tile.rotate_left();
        rotated_expected = rotate_left_expected(rotated_expected);
        assert_tile_render(&tile, size, rotated_expected.as_str(), Rotation::R180);

        // rotate 270°
        tile.rotate_left();
        rotated_expected = rotate_left_expected(rotated_expected);
        assert_tile_render(&tile, size, rotated_expected.as_str(), Rotation::R270);

        // rotate 360°
        tile.rotate_left();
        rotated_expected = rotate_left_expected(rotated_expected);
        assert_tile_render(&tile, size, rotated_expected.as_str(), Rotation::R0);

        assert_eq!(rotated_expected, expected);
    }

    fn assert_tile_render(tile: &Tile, size: usize, expected: &str, rotation: Rotation) {
        let rendered = TileRenderer::tile(size, &tile, &rotation);
        let rendered_str = rendered
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        let expected_test_str = expected.trim();
        let expected_str = convert_test_string_to_tile_string(expected_test_str)
            .unwrap_or_else(|e| panic!("Failed to convert test charset: {e}"));
        let display_rotation = rotation.to_degrees();

        println!(
            "\nRendered ({display_rotation}) ({} chars):\n{}",
            rendered_str.len(),
            rendered_str
        );
        println!(
            "\nExpected ({display_rotation}) ({} chars):\n{}",
            expected_str.len(),
            expected_str
        );
        println!("--- Real debug output ---");
        println!(
            "Expected ({display_rotation}) ({} chars):\n{}",
            expected_test_str.len(),
            expected_test_str
        );
        println!("--- End debug output ---\n");

        assert_eq!(rendered_str, expected_str);
    }

    // Abbey tiles - build_a_abbey
    #[test]
    fn test_tile_a_abbey_5() {
        assert_tile(
            TileFactory::build_a_abbey,
            5,
            "
.....
..A..
.AAA.
..A..
..║..",
        );
    }

    #[test]
    fn test_tile_a_abbey_7() {
        assert_tile(
            TileFactory::build_a_abbey,
            7,
            "
.......
...A...
..AAA..
.AAAAA.
..AAA..
...A...
...║...",
        );
    }

    #[test]
    fn test_tile_a_abbey_9() {
        assert_tile(
            TileFactory::build_a_abbey,
            9,
            "
.........
....A....
...AAA...
..AAAAA..
.AAAAAAA.
..AAAAA..
...AAA...
....A....
....║....",
        );
    }

    #[test]
    fn test_tile_a_abbey_11() {
        assert_tile(
            TileFactory::build_a_abbey,
            11,
            "
...........
.....A.....
....AAA....
...AAAAA...
..AAAAAAA..
.AAAAAAAAA.
..AAAAAAA..
...AAAAA...
....AAA....
.....A.....
.....║.....",
        );
    }

    // Abbey tiles - build_b_abbey
    #[test]
    fn test_tile_b_abbey_5() {
        assert_tile(
            TileFactory::build_b_abbey,
            5,
            "
.....
..A..
.AAA.
..A..
.....",
        );
    }

    #[test]
    fn test_tile_b_abbey_7() {
        assert_tile(
            TileFactory::build_b_abbey,
            7,
            "
.......
...A...
..AAA..
.AAAAA.
..AAA..
...A...
.......",
        );
    }

    #[test]
    fn test_tile_b_abbey_9() {
        assert_tile(
            TileFactory::build_b_abbey,
            9,
            "
.........
....A....
...AAA...
..AAAAA..
.AAAAAAA.
..AAAAA..
...AAA...
....A....
.........",
        );
    }

    #[test]
    fn test_tile_b_abbey_11() {
        assert_tile(
            TileFactory::build_b_abbey,
            11,
            "
...........
.....A.....
....AAA....
...AAAAA...
..AAAAAAA..
.AAAAAAAAA.
..AAAAAAA..
...AAAAA...
....AAA....
.....A.....
...........",
        );
    }

    // Town tiles - build_c_town
    #[test]
    fn test_tile_c_town_5() {
        assert_tile(
            TileFactory::build_c_town,
            5,
            "
#####
#S###
#####
#####
#####",
        );
    }

    #[test]
    fn test_tile_c_town_7() {
        assert_tile(
            TileFactory::build_c_town,
            7,
            "
#######
#S#####
#######
#######
#######
#######
#######",
        );
    }

    #[test]
    fn test_tile_c_town_9() {
        assert_tile(
            TileFactory::build_c_town,
            9,
            "
#########
##S######
#SSS#####
##S######
#########
#########
#########
#########
#########",
        );
    }

    #[test]
    fn test_tile_c_town_11() {
        assert_tile(
            TileFactory::build_c_town,
            11,
            "
###########
##S########
#SSS#######
##S########
###########
###########
###########
###########
###########
###########
###########",
        );
    }

    // Town tiles - build_d_town
    #[test]
    fn test_tile_d_town_5() {
        assert_tile(
            TileFactory::build_d_town,
            5,
            "
.###.
..#..
═════
.....
.....",
        );
    }

    #[test]
    fn test_tile_d_town_7() {
        assert_tile(
            TileFactory::build_d_town,
            7,
            "
.#####.
..###..
...#...
═══════
.......
.......
.......",
        );
    }

    #[test]
    fn test_tile_d_town_9() {
        assert_tile(
            TileFactory::build_d_town,
            9,
            "
.#######.
..#####..
...###...
....#....
═════════
.........
.........
.........
.........",
        );
    }

    #[test]
    fn test_tile_d_town_11() {
        assert_tile(
            TileFactory::build_d_town,
            11,
            "
.#########.
..#######..
...#####...
....###....
.....#.....
═══════════
...........
...........
...........
...........
...........",
        );
    }

    // Town tiles - build_e_town
    #[test]
    fn test_tile_e_town_5() {
        assert_tile(
            TileFactory::build_e_town,
            5,
            "
.###.
..#..
.....
.....
.....",
        );
    }

    #[test]
    fn test_tile_e_town_7() {
        assert_tile(
            TileFactory::build_e_town,
            7,
            "
.#####.
..###..
...#...
.......
.......
.......
.......",
        );
    }

    #[test]
    fn test_tile_e_town_9() {
        assert_tile(
            TileFactory::build_e_town,
            9,
            "
.#######.
..#####..
...###...
....#....
.........
.........
.........
.........
.........",
        );
    }

    #[test]
    fn test_tile_e_town_11() {
        assert_tile(
            TileFactory::build_e_town,
            11,
            "
.#########.
..#######..
...#####...
....###....
.....#.....
...........
...........
...........
...........
...........
...........",
        );
    }

    // Town tiles - build_f_town
    #[test]
    fn test_tile_f_town_5() {
        assert_tile(
            TileFactory::build_f_town,
            5,
            "
#...#
#####
###S#
#####
#...#",
        );
    }

    #[test]
    fn test_tile_f_town_7() {
        assert_tile(
            TileFactory::build_f_town,
            7,
            "
#.....#
##...##
#######
#####S#
#######
##...##
#.....#",
        );
    }

    #[test]
    fn test_tile_f_town_9() {
        assert_tile(
            TileFactory::build_f_town,
            9,
            "
#.......#
##.....##
###...###
######S##
#####SSS#
######S##
###...###
##.....##
#.......#",
        );
    }

    #[test]
    fn test_tile_f_town_11() {
        assert_tile(
            TileFactory::build_f_town,
            11,
            "
#.........#
##.......##
###.....###
####...####
########S##
#######SSS#
########S##
####...####
###.....###
##.......##
#.........#",
        );
    }

    // Town tiles - build_g_town
    #[test]
    fn test_tile_g_town_5() {
        assert_tile(
            TileFactory::build_g_town,
            5,
            "
#...#
#####
#####
#####
#...#",
        );
    }

    #[test]
    fn test_tile_g_town_7() {
        assert_tile(
            TileFactory::build_g_town,
            7,
            "
#.....#
##...##
#######
#######
#######
##...##
#.....#",
        );
    }

    #[test]
    fn test_tile_g_town_9() {
        assert_tile(
            TileFactory::build_g_town,
            9,
            "
#.......#
##.....##
###...###
#########
#########
#########
###...###
##.....##
#.......#",
        );
    }

    #[test]
    fn test_tile_g_town_11() {
        assert_tile(
            TileFactory::build_g_town,
            11,
            "
#.........#
##.......##
###.....###
####...####
###########
###########
###########
####...####
###.....###
##.......##
#.........#",
        );
    }

    // Town tiles - build_i_town
    #[test]
    fn test_tile_i_town_5() {
        assert_tile(
            TileFactory::build_i_town,
            5,
            "
.###.
#.#..
##...
#....
.....",
        );
    }

    #[test]
    fn test_tile_i_town_7() {
        assert_tile(
            TileFactory::build_i_town,
            7,
            "
.#####.
#.###..
##.#...
###....
##.....
#......
.......",
        );
    }

    #[test]
    fn test_tile_i_town_9() {
        assert_tile(
            TileFactory::build_i_town,
            9,
            "
.#######.
#.#####..
##.###...
###.#....
####.....
###......
##.......
#........
.........",
        );
    }

    #[test]
    fn test_tile_i_town_11() {
        assert_tile(
            TileFactory::build_i_town,
            11,
            "
.#########.
#.#######..
##.#####...
###.###....
####.#.....
#####......
####.......
###........
##.........
#..........
...........",
        );
    }

    // Town tiles - build_j_town
    #[test]
    fn test_tile_j_town_5() {
        assert_tile(
            TileFactory::build_j_town,
            5,
            "
.###.
..#..
...╔═
..╔╝.
..║..",
        );
    }

    #[test]
    fn test_tile_j_town_7() {
        assert_tile(
            TileFactory::build_j_town,
            7,
            "
.#####.
..###..
...#...
.....╔═
....╔╝.
...╔╝..
...║...",
        );
    }

    #[test]
    fn test_tile_j_town_9() {
        assert_tile(
            TileFactory::build_j_town,
            9,
            "
.#######.
..#####..
...###...
....#....
......╔══
.....╔╝..
....╔╝...
....║....
....║....",
        );
    }

    #[test]
    fn test_tile_j_town_11() {
        assert_tile(
            TileFactory::build_j_town,
            11,
            "
.#########.
..#######..
...#####...
....###....
.....#.....
........╔══
.......╔╝..
......╔╝...
.....╔╝....
.....║.....
.....║.....",
        );
    }

    // Town tiles - build_k_town
    #[test]
    fn test_tile_k_town_5() {
        assert_tile(
            TileFactory::build_k_town,
            5,
            "
.###.
..#..
═╗...
.╚╗..
..║..",
        );
    }

    #[test]
    fn test_tile_k_town_7() {
        assert_tile(
            TileFactory::build_k_town,
            7,
            "
.#####.
..###..
...#...
═╗.....
.╚╗....
..╚╗...
...║...",
        );
    }

    #[test]
    fn test_tile_k_town_9() {
        assert_tile(
            TileFactory::build_k_town,
            9,
            "
.#######.
..#####..
...###...
....#....
══╗......
..╚╗.....
...╚╗....
....║....
....║....",
        );
    }

    #[test]
    fn test_tile_k_town_11() {
        assert_tile(
            TileFactory::build_k_town,
            11,
            "
.#########.
..#######..
...#####...
....###....
.....#.....
══╗........
..╚╗.......
...╚╗......
....╚╗.....
.....║.....
.....║.....",
        );
    }

    // Town tiles - build_l_town
    #[test]
    fn test_tile_l_town_5() {
        assert_tile(
            TileFactory::build_l_town,
            5,
            "
.###.
..#..
══◻══
..║..
..║..",
        );
    }

    #[test]
    fn test_tile_l_town_7() {
        assert_tile(
            TileFactory::build_l_town,
            7,
            "
.#####.
..###..
...#...
═══◻═══
...║...
...║...
...║...",
        );
    }

    #[test]
    fn test_tile_l_town_9() {
        assert_tile(
            TileFactory::build_l_town,
            9,
            "
.#######.
..#####..
...###...
....#....
════◻════
....║....
....║....
....║....
....║....",
        );
    }

    #[test]
    fn test_tile_l_town_11() {
        assert_tile(
            TileFactory::build_l_town,
            11,
            "
.#########.
..#######..
...#####...
....###....
.....#.....
═════◻═════
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....",
        );
    }

    // Town tiles - build_m_town
    #[test]
    fn test_tile_m_town_5() {
        assert_tile(
            TileFactory::build_m_town,
            5,
            "
#####
.##S#
..###
...##
....#",
        );
    }

    #[test]
    fn test_tile_m_town_7() {
        assert_tile(
            TileFactory::build_m_town,
            7,
            "
#######
.####S#
..#####
...####
....###
.....##
......#",
        );
    }

    #[test]
    fn test_tile_m_town_9() {
        assert_tile(
            TileFactory::build_m_town,
            9,
            "
#########
.#####S##
..###SSS#
...###S##
....#####
.....####
......###
.......##
........#",
        );
    }

    #[test]
    fn test_tile_m_town_11() {
        assert_tile(
            TileFactory::build_m_town,
            11,
            "
###########
.#######S##
..#####SSS#
...#####S##
....#######
.....######
......#####
.......####
........###
.........##
..........#",
        );
    }

    // Town tiles - build_n_town
    #[test]
    fn test_tile_n_town_5() {
        assert_tile(
            TileFactory::build_n_town,
            5,
            "
#####
.####
..###
...##
....#",
        );
    }

    #[test]
    fn test_tile_n_town_7() {
        assert_tile(
            TileFactory::build_n_town,
            7,
            "
#######
.######
..#####
...####
....###
.....##
......#",
        );
    }

    #[test]
    fn test_tile_n_town_9() {
        assert_tile(
            TileFactory::build_n_town,
            9,
            "
#########
.########
..#######
...######
....#####
.....####
......###
.......##
........#",
        );
    }

    #[test]
    fn test_tile_n_town_11() {
        assert_tile(
            TileFactory::build_n_town,
            11,
            "
###########
.##########
..#########
...########
....#######
.....######
......#####
.......####
........###
.........##
..........#",
        );
    }

    // Town tiles - build_o_town
    #[test]
    fn test_tile_o_town_5() {
        assert_tile(
            TileFactory::build_o_town,
            5,
            "
#####
#S##.
###╔═
##╔╝.
#.║..",
        );
    }

    #[test]
    fn test_tile_o_town_7() {
        assert_tile(
            TileFactory::build_o_town,
            7,
            "
#######
#S####.
#####..
####.╔═
###.╔╝.
##.╔╝..
#..║...",
        );
    }

    #[test]
    fn test_tile_o_town_9() {
        assert_tile(
            TileFactory::build_o_town,
            9,
            "
#########
##S#####.
#SSS###..
##S###...
#####.╔══
####.╔╝..
###.╔╝...
##..║....
#...║....",
        );
    }

    #[test]
    fn test_tile_o_town_11() {
        assert_tile(
            TileFactory::build_o_town,
            11,
            "
###########
##S#######.
#SSS#####..
##S#####...
#######....
######..╔══
#####..╔╝..
####..╔╝...
###..╔╝....
##...║.....
#....║.....",
        );
    }

    // Town tiles - build_p_town
    #[test]
    fn test_tile_p_town_5() {
        assert_tile(
            TileFactory::build_p_town,
            5,
            "
#####
####.
###╔═
##╔╝.
#.║..",
        );
    }

    #[test]
    fn test_tile_p_town_7() {
        assert_tile(
            TileFactory::build_p_town,
            7,
            "
#######
######.
#####..
####.╔═
###.╔╝.
##.╔╝..
#..║...",
        );
    }

    #[test]
    fn test_tile_p_town_9() {
        assert_tile(
            TileFactory::build_p_town,
            9,
            "
#########
########.
#######..
######...
#####.╔══
####.╔╝..
###.╔╝...
##..║....
#...║....",
        );
    }

    #[test]
    fn test_tile_p_town_11() {
        assert_tile(
            TileFactory::build_p_town,
            11,
            "
###########
##########.
#########..
########...
#######....
######..╔══
#####..╔╝..
####..╔╝...
###..╔╝....
##...║.....
#....║.....",
        );
    }

    // Town tiles - build_q_town
    #[test]
    fn test_tile_q_town_5() {
        assert_tile(
            TileFactory::build_q_town,
            5,
            "
#####
#S###
#####
##.##
#...#",
        );
    }

    #[test]
    fn test_tile_q_town_7() {
        assert_tile(
            TileFactory::build_q_town,
            7,
            "
#######
#S#####
#######
#######
###.###
##...##
#.....#",
        );
    }

    #[test]
    fn test_tile_q_town_9() {
        assert_tile(
            TileFactory::build_q_town,
            9,
            "
#########
##S######
#SSS#####
##S######
#########
####.####
###...###
##.....##
#.......#",
        );
    }

    #[test]
    fn test_tile_q_town_11() {
        assert_tile(
            TileFactory::build_q_town,
            11,
            "
###########
##S########
#SSS#######
##S########
###########
###########
#####.#####
####...####
###.....###
##.......##
#.........#",
        );
    }

    // Town tiles - build_r_town
    #[test]
    fn test_tile_r_town_5() {
        assert_tile(
            TileFactory::build_r_town,
            5,
            "
#####
#####
#####
##.##
#...#",
        );
    }

    #[test]
    fn test_tile_r_town_7() {
        assert_tile(
            TileFactory::build_r_town,
            7,
            "
#######
#######
#######
#######
###.###
##...##
#.....#",
        );
    }

    #[test]
    fn test_tile_r_town_9() {
        assert_tile(
            TileFactory::build_r_town,
            9,
            "
#########
#########
#########
#########
#########
####.####
###...###
##.....##
#.......#",
        );
    }

    #[test]
    fn test_tile_r_town_11() {
        assert_tile(
            TileFactory::build_r_town,
            11,
            "
###########
###########
###########
###########
###########
###########
#####.#####
####...####
###.....###
##.......##
#.........#",
        );
    }

    // Town tiles - build_s_town
    #[test]
    fn test_tile_s_town_5() {
        assert_tile(
            TileFactory::build_s_town,
            5,
            "
#####
#S###
#####
##║##
#.║.#",
        );
    }

    #[test]
    fn test_tile_s_town_7() {
        assert_tile(
            TileFactory::build_s_town,
            7,
            "
#######
#S#####
#######
#######
###║###
##.║.##
#..║..#",
        );
    }

    #[test]
    fn test_tile_s_town_9() {
        assert_tile(
            TileFactory::build_s_town,
            9,
            "
#########
##S######
#SSS#####
##S######
#########
####║####
###.║.###
##..║..##
#...║...#",
        );
    }

    #[test]
    fn test_tile_s_town_11() {
        assert_tile(
            TileFactory::build_s_town,
            11,
            "
###########
##S########
#SSS#######
##S########
###########
###########
#####║#####
####.║.####
###..║..###
##...║...##
#....║....#",
        );
    }

    // Town tiles - build_t_town
    #[test]
    fn test_tile_t_town_5() {
        assert_tile(
            TileFactory::build_t_town,
            5,
            "
#####
#####
#####
##║##
#.║.#",
        );
    }

    #[test]
    fn test_tile_t_town_7() {
        assert_tile(
            TileFactory::build_t_town,
            7,
            "
#######
#######
#######
#######
###║###
##.║.##
#..║..#",
        );
    }

    #[test]
    fn test_tile_t_town_9() {
        assert_tile(
            TileFactory::build_t_town,
            9,
            "
#########
#########
#########
#########
#########
####║####
###.║.###
##..║..##
#...║...#",
        );
    }

    #[test]
    fn test_tile_t_town_11() {
        assert_tile(
            TileFactory::build_t_town,
            11,
            "
###########
###########
###########
###########
###########
###########
#####║#####
####.║.####
###..║..###
##...║...##
#....║....#",
        );
    }

    // Road tiles - build_u_road
    #[test]
    fn test_tile_u_road_5() {
        assert_tile(
            TileFactory::build_u_road,
            5,
            "
..║..
..║..
..║..
..║..
..║..",
        );
    }

    #[test]
    fn test_tile_u_road_7() {
        assert_tile(
            TileFactory::build_u_road,
            7,
            "
...║...
...║...
...║...
...║...
...║...
...║...
...║...",
        );
    }

    #[test]
    fn test_tile_u_road_9() {
        assert_tile(
            TileFactory::build_u_road,
            9,
            "
....║....
....║....
....║....
....║....
....║....
....║....
....║....
....║....
....║....",
        );
    }

    #[test]
    fn test_tile_u_road_11() {
        assert_tile(
            TileFactory::build_u_road,
            11,
            "
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....",
        );
    }

    // Road tiles - build_v_road
    #[test]
    fn test_tile_v_road_5() {
        assert_tile(
            TileFactory::build_v_road,
            5,
            "
.....
.....
═╗...
.╚╗..
..║..",
        );
    }

    #[test]
    fn test_tile_v_road_7() {
        assert_tile(
            TileFactory::build_v_road,
            7,
            "
.......
.......
.......
═╗.....
.╚╗....
..╚╗...
...║...",
        );
    }

    #[test]
    fn test_tile_v_road_9() {
        assert_tile(
            TileFactory::build_v_road,
            9,
            "
.........
.........
.........
.........
══╗......
..╚╗.....
...╚╗....
....║....
....║....",
        );
    }

    #[test]
    fn test_tile_v_road_11() {
        assert_tile(
            TileFactory::build_v_road,
            11,
            "
...........
...........
...........
...........
...........
══╗........
..╚╗.......
...╚╗......
....╚╗.....
.....║.....
.....║.....",
        );
    }

    // Road tiles - build_w_road
    #[test]
    fn test_tile_w_road_5() {
        assert_tile(
            TileFactory::build_w_road,
            5,
            "
.....
.....
══◻══
..║..
..║..",
        );
    }

    #[test]
    fn test_tile_w_road_7() {
        assert_tile(
            TileFactory::build_w_road,
            7,
            "
.......
.......
.......
═══◻═══
...║...
...║...
...║...",
        );
    }

    #[test]
    fn test_tile_w_road_9() {
        assert_tile(
            TileFactory::build_w_road,
            9,
            "
.........
.........
.........
.........
════◻════
....║....
....║....
....║....
....║....",
        );
    }

    #[test]
    fn test_tile_w_road_11() {
        assert_tile(
            TileFactory::build_w_road,
            11,
            "
...........
...........
...........
...........
...........
═════◻═════
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....",
        );
    }

    // Road tiles - build_x_road
    #[test]
    fn test_tile_x_road_5() {
        assert_tile(
            TileFactory::build_x_road,
            5,
            "
..║..
..║..
══◻══
..║..
..║..",
        );
    }

    #[test]
    fn test_tile_x_road_7() {
        assert_tile(
            TileFactory::build_x_road,
            7,
            "
...║...
...║...
...║...
═══◻═══
...║...
...║...
...║...",
        );
    }

    #[test]
    fn test_tile_x_road_9() {
        assert_tile(
            TileFactory::build_x_road,
            9,
            "
....║....
....║....
....║....
....║....
════◻════
....║....
....║....
....║....
....║....",
        );
    }

    #[test]
    fn test_tile_x_road_11() {
        assert_tile(
            TileFactory::build_x_road,
            11,
            "
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....
═════◻═════
.....║.....
.....║.....
.....║.....
.....║.....
.....║.....",
        );
    }
}
