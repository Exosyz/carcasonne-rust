use carcasonne_core::layout::point::Point;
use carcasonne_core::model::tile_feature::Edge;

pub struct TileRendererHelper;

impl TileRendererHelper {
    pub fn edge(size: usize, edge: &Edge) -> Point {
        match edge {
            Edge::North => Point::new(size / 2, 0),
            Edge::East => Point::new(size - 1, size / 2),
            Edge::South => Point::new(size / 2, size - 1),
            Edge::West => Point::new(0, size / 2),
        }
    }
    pub fn center(size: usize) -> Point {
        Point::new(size / 2, size / 2)
    }

    pub fn circle(center: Point, circle_size: usize, size: usize) -> Vec<Point> {
        let mut points = Vec::new();

        for dy in -(circle_size as isize)..=(circle_size as isize) {
            for dx in -(circle_size as isize)..=(circle_size as isize) {
                let dist = dx.abs() + dy.abs();
                if dist <= circle_size as isize
                    && let Some(p) = Self::offset_point(center, dx, dy, size)
                {
                    points.push(p);
                }
            }
        }

        points
    }

    fn straight_line(from: Point, to: Point) -> Vec<Point> {
        let mut points = Vec::new();

        if from.x == to.x {
            // Ligne vertical
            let (start, end) = if from.y <= to.y {
                (from.y, to.y)
            } else {
                (to.y, from.y)
            };
            for y in start..=end {
                points.push(Point::new(from.x, y));
            }
        } else if from.y == to.y {
            // Ligne horizontal
            let (start, end) = if from.x <= to.x {
                (from.x, to.x)
            } else {
                (to.x, from.x)
            };
            for x in start..=end {
                points.push(Point::new(x, from.y));
            }
        } else {
            panic!("straight_line only supports horizontal or vertical lines");
        }

        points
    }

    pub fn line(from: Point, to: Point, size: usize) -> Vec<Point> {
        if from.x == to.x || from.y == to.y {
            return TileRendererHelper::straight_line(from, to);
        }

        let center = TileRendererHelper::center(size);

        let mut points = Vec::new();
        points.extend(TileRendererHelper::straight_line(from, center));
        points.extend(TileRendererHelper::straight_line(center, to));
        points.dedup();
        points.retain(|p| *p != center);

        points
    }

    fn offset_point(base: Point, ox: isize, oy: isize, size: usize) -> Option<Point> {
        let nx = base.x as isize + ox;
        let ny = base.y as isize + oy;
        if nx >= 0 && ny >= 0 && nx < size as isize && ny < size as isize {
            Some(Point::new(nx as usize, ny as usize))
        } else {
            None
        }
    }

    pub fn center_size(size: usize) -> usize {
        match size {
            0..3 => 0,
            3..8 => 1,
            8..12 => 2,
            12_usize.. => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use carcasonne_core::layout::point::Point;

    pub fn debug_tile(points: &[Point], size: usize) {
        for y in 0..size {
            for x in 0..size {
                if points.contains(&Point::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    macro_rules! test_tile_center {
        (
            $(
                $name:ident: size = $size:expr => Point($x:expr, $y:expr);
            )*
        ) => {
            $(
                #[test]
                fn $name() {
                    let result = TileRendererHelper::center($size);
                    let expected = Point::new($x, $y);

                    println!("\n--- Debug for {} ---", stringify!($name));
                    debug_tile(&[result], $size);

                    assert_eq!(
                        result, expected,
                        "Expected center({}, {}) for size {}, got {:?}", $x, $y, $size, result
                    );
                }
            )*
        };
    }

    test_tile_center! {
        test_center_even: size = 6 => Point(3, 3);
        test_center_odd:  size = 5 => Point(2, 2);
        test_center_zero: size = 0 => Point(0, 0);
        test_center_large: size = 101 => Point(50, 50);
    }

    macro_rules! test_tile_circle {
        (
            $(
                $name:ident: center = ($x:expr, $y:expr), total_size = $total_size:expr, size = $size:expr => [$($expected:expr),* $(,)?];
            )*
        ) => {
            $(
                #[test]
                fn $name() {
                    let center = Point::new($x, $y);
                    let circle = TileRendererHelper::circle(center, $size, $total_size);

                    println!("\n--- Debug for {} ---", stringify!($name));
                    debug_tile(&circle, $total_size);

                    let expected: Vec<Point> = vec![$($expected),*];
                    assert_eq!(
                        circle.into_iter().collect::<std::collections::HashSet<_>>(),
                        expected.into_iter().collect::<std::collections::HashSet<_>>(),
                        "Mismatch on radius({:?}, {})", center, $size
                    );
                }
            )*
        };
    }

    test_tile_circle! {
        test_radius_zero_centered: center = (2, 2), total_size = 5, size = 0 => [
            Point::new(2, 2)
        ];

        test_radius_one_centered: center = (2, 2), total_size = 5, size = 1 => [
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(2, 3),
            Point::new(1, 2),
            Point::new(2, 1)
        ];

        test_radius_two_centered: center = (2, 2), total_size = 5, size = 2 => [
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(1, 2),
            Point::new(2, 3),
            Point::new(2, 1),
            Point::new(4, 2),
            Point::new(0, 2),
            Point::new(3, 3),
            Point::new(1, 3),
            Point::new(3, 1),
            Point::new(1, 1),
            Point::new(2, 4),
            Point::new(2, 0),
        ];

        test_radius_two_top_left: center = (0, 0), total_size = 5, size = 2 => [
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(2, 0),
            Point::new(1, 1),
            Point::new(0, 2),
        ];
    }
    macro_rules! test_edge_for_size {
        ($name:ident, $half:expr, $size:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    TileRendererHelper::edge($size, &Edge::North),
                    Point::new($half, 0)
                );
                assert_eq!(
                    TileRendererHelper::edge($size, &Edge::East),
                    Point::new($size, $half)
                );
                assert_eq!(
                    TileRendererHelper::edge($size, &Edge::South),
                    Point::new($half, $size)
                );
                assert_eq!(
                    TileRendererHelper::edge($size, &Edge::West),
                    Point::new(0, $half)
                );
            }
        };
    }

    test_edge_for_size!(test_edge_1, 0, 1);
    test_edge_for_size!(test_edge_3, 1, 3);
    test_edge_for_size!(test_edge_5, 2, 5);
    test_edge_for_size!(test_edge_7, 3, 7);
    test_edge_for_size!(test_edge_9, 4, 9);

    macro_rules! test_straight_line {
        (
            $(
                $name:ident:
                    from = ($fx:expr, $fy:expr),
                    to = ($tx:expr, $ty:expr),
                    size = $size:expr
                    => [$(($ex:expr, $ey:expr)),* $(,)?];
            )*
        ) => {
            $(
                #[test]
                fn $name() {
                    let from = Point::new($fx, $fy);
                    let to = Point::new($tx, $ty);
                    let result = TileRendererHelper::straight_line(from, to);

                    let expected: Vec<Point> = vec![
                        $(Point::new($ex, $ey)),*
                    ];

                    println!("\n--- Debug for {} (line) ---", stringify!($name));
                    debug_tile(&result, $size);
                    println!("\n--- Expected for {} (line) ---", stringify!($name));
                    debug_tile(&expected, $size);

                    assert_eq!(
                        result.into_iter().collect::<std::collections::HashSet<_>>(),
                        expected.into_iter().collect::<std::collections::HashSet<_>>(),
                        "Mismatch on line from {:?} to {:?} with size {}", from, to, $size
                    );
                }
            )*
        };
    }

    test_straight_line! {
        test_straight_line_horizontal:
            from = (0, 0),
            to = (4, 0),
            size = 5
            => [(0, 0), (1, 0), (2, 0), (3, 0),(4, 0)];

        test_straight_line_vertical:
            from = (2, 0),
            to = (2, 5),
            size = 5
            => [(2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5)];

        test_straight_line_horizontal_9:
            from = (0, 0),
            to = (8, 0),
            size = 9
            => [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0)];

        test_straight_line_vertical_9:
            from = (4, 0),
            to = (4, 8),
            size = 9
            => [(4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7), (4, 8)];

        test_straight_line_horizontal_reverse:
            from = (4, 0),
            to = (0, 0),
            size = 5
            => [(4, 0), (3, 0), (2, 0), (1, 0), (0, 0)];

        test_straight_line_vertical_reverse:
            from = (2, 5),
            to = (2, 0),
            size = 5
            => [(2, 5), (2, 4), (2, 3), (2, 2), (2, 1), (2, 0)];

        test_straight_line_single_point:
            from = (2, 2),
            to = (2, 2),
            size = 5
            => [(2, 2)];
    }

    macro_rules! test_line {
        (
            $(
                $name:ident:
                    from = ($fx:expr, $fy:expr),
                    to = ($tx:expr, $ty:expr),
                    size = $size:expr
                    => [$(($ex:expr, $ey:expr)),* $(,)?];
            )*
        ) => {
            $(
                #[test]
                fn $name() {
                    let from = Point::new($fx, $fy);
                    let to = Point::new($tx, $ty);
                    let line = TileRendererHelper::line(from, to, $size);

                    let expected: Vec<Point> = vec![
                        $(Point::new($ex, $ey)),*
                    ];

                    println!("\n--- Debug for {} (line_with_smooth_curve) ---", stringify!($name));
                    debug_tile(&line, $size);
                    println!("\n--- Expected for {} (line_with_smooth_curve) ---", stringify!($name));
                    debug_tile(&expected, $size);

                    assert_eq!(
                        line.into_iter().collect::<std::collections::HashSet<_>>(),
                        expected.into_iter().collect::<std::collections::HashSet<_>>(),
                        "Mismatch on line_with_smooth_curve from {:?} to {:?}", from, to
                    );
                }
            )*
        };
    }

    test_line! {
        test_line_left_to_right:
            from = (0, 4),
            to = (8, 4),
            size = 9
            => [
                (0,4), (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (7,4), (8,4)
            ];

        test_line_right_to_left:
            from = (8, 4),
            to = (0, 4),
            size = 9
            => [
                (8,4), (7,4), (6,4), (5,4), (4,4), (3,4), (2,4), (1,4), (0,4)
            ];

        test_line_top_to_bottom:
            from = (4, 0),
            to = (4, 8),
            size = 9
            => [
                (4,0), (4,1), (4,2), (4,3), (4,4), (4,5), (4,6), (4,7), (4,8)
            ];

        test_line_bottom_to_top:
            from = (4, 8),
            to = (4, 0),
            size = 9
            => [
                (4,8), (4,7), (4,6), (4,5), (4,4), (4,3), (4,2), (4,1), (4,0)
            ];

        test_line_left_to_top:
            from = (0, 4),
            to = (4, 0),
            size = 9
            => [
                (0,4), (1,4), (2,4), (3,4),
                (4,3), (4,2), (4,1), (4,0)
            ];

        test_line_left_to_bottom:
            from = (0, 4),
            to = (4, 8),
            size = 9
            => [
                (0,4), (1,4), (2,4), (3,4),
                (4,5), (4,6), (4,7), (4,8)
            ];

        test_line_right_to_top:
            from = (8, 4),
            to = (4, 0),
            size = 9
            => [
                (8,4), (7,4), (6,4), (5,4),
                (4,3), (4,2), (4,1), (4,0)
            ];

        test_line_right_to_bottom:
            from = (8, 4),
            to = (4, 8),
            size = 9
            => [
                (8,4), (7,4), (6,4), (5,4),
                (4,5), (4,6), (4,7), (4,8)
            ];

        test_line_top_to_left:
            from = (4, 0),
            to = (0, 4),
            size = 9
            => [
                (4,0), (4,1), (4,2), (4,3),
                (3,4), (2,4), (1,4), (0,4)
            ];

        test_line_top_to_right:
            from = (4, 0),
            to = (8, 4),
            size = 9
            => [
                (4,0), (4,1), (4,2), (4,3),
                (5,4), (6,4), (7,4), (8,4)
            ];

        test_line_bottom_to_left:
            from = (4, 8),
            to = (0, 4),
            size = 9
            => [
                (4,8), (4,7), (4,6), (4,5),
                (3,4), (2,4), (1,4), (0,4)
            ];

        test_line_bottom_to_right:
            from = (4, 8),
            to = (8, 4),
            size = 9
            => [
                (4,8), (4,7), (4,6), (4,5),
                (5,4), (6,4), (7,4), (8,4)
            ];
    }
}
