use carcasonne_core::layout::point::Point;
use carcasonne_core::model::tile_feature::Edge;

/// Utility functions for rendering tiles in a grid system.
/// Provides geometric calculations for tile-based rendering including
/// edge positioning, circles, lines, and size calculations.
pub struct TileRendererUtils;

impl TileRendererUtils {
    /// Returns the center point of an edge on a tile of given size.
    ///
    /// # Arguments
    /// * `size` - The size of the square tile grid (size x size)
    /// * `edge` - The edge (North, East, South, West) to get the center point for
    ///
    /// # Returns
    /// Point representing the center of the specified edge
    ///
    /// # Examples
    /// For a 5x5 tile:
    /// - North edge center: (2, 0)
    /// - East edge center: (4, 2)
    /// - South edge center: (2, 4)
    /// - West edge center: (0, 2)
    pub fn edge(size: usize, edge: &Edge) -> Point {
        match edge {
            Edge::North => Point::new(size / 2, 0),
            Edge::East => Point::new(size - 1, size / 2),
            Edge::South => Point::new(size / 2, size - 1),
            Edge::West => Point::new(0, size / 2),
        }
    }

    /// Returns the center point of a square tile grid.
    ///
    /// # Arguments
    /// * `size` - The size of the square tile grid
    ///
    /// # Returns
    /// Point at the center of the grid
    ///
    /// # Examples
    /// - 5x5 grid: center at (2, 2)
    /// - 6x6 grid: center at (3, 3)
    pub fn center(size: usize) -> Point {
        Point::new(size / 2, size / 2)
    }

    /// Calculates a middle point between two points with grid-aware rounding.
    /// Uses smart rounding based on the tile center to ensure proper grid alignment.
    ///
    /// # Arguments
    /// * `size` - The size of the tile grid (used for center calculation)
    /// * `from` - Starting point
    /// * `to` - Ending point
    ///
    /// # Returns
    /// Point representing the middle position with grid-aware rounding
    ///
    /// # Algorithm
    /// - Calculate floating-point midpoint
    /// - Round up if midpoint > grid center, otherwise round down
    /// - Ensures consistent behavior across different grid positions
    pub fn middle(size: usize, from: Point, to: Point) -> Point {
        let mid_x = (from.x + to.x) as f32 / 2.0;
        let mid_y = (from.y + to.y) as f32 / 2.0;

        let center = size as f32 / 2.0;

        let x = if mid_x > center {
            mid_x.ceil() as usize
        } else {
            mid_x.floor() as usize
        };

        let y = if mid_y > center {
            mid_y.ceil() as usize
        } else {
            mid_y.floor() as usize
        };

        Point::new(x, y)
    }

    /// Generates points forming a circle using Manhattan distance.
    /// Creates a filled diamond/rhombus shape rather than a true circle.
    ///
    /// # Arguments
    /// * `center` - Center point of the circle
    /// * `circle_size` - Radius of the circle (Manhattan distance)
    /// * `size` - Size of the tile grid (for boundary checking)
    ///
    /// # Returns
    /// Vector of all points within the circle that fit in the grid
    ///
    /// # Algorithm
    /// - Uses Manhattan distance (|dx| + |dy|) for diamond-shaped pattern
    /// - Includes all points where Manhattan distance ≤ radius
    /// - Automatically clips points outside the grid boundaries via `offset_point`
    ///
    /// # Note
    /// Despite the name "circle", this function creates a diamond/rhombus pattern
    /// due to the use of Manhattan distance instead of Euclidean distance.
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

    /// Draws a straight line between two points (horizontal or vertical only).
    ///
    /// # Arguments
    /// * `from` - Starting point
    /// * `to` - Ending point
    ///
    /// # Returns
    /// Vector of points forming the line from start to end (inclusive)
    ///
    /// # Panics
    /// Panics if the line is diagonal (not horizontal or vertical)
    ///
    /// # Algorithm
    /// - Detects if line is vertical (same x) or horizontal (same y)
    /// - Handles both forward and reverse directions
    /// - Includes both endpoints in the result
    fn straight_line(from: Point, to: Point) -> Vec<Point> {
        let mut points = Vec::new();

        if from.x == to.x {
            // Vertical line - iterate through y coordinates
            let (start, end) = if from.y <= to.y {
                (from.y, to.y)
            } else {
                (to.y, from.y)
            };
            for y in start..=end {
                points.push(Point::new(from.x, y));
            }
        } else if from.y == to.y {
            // Horizontal line - iterate through x coordinates
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

    /// Creates a line between two points, using the grid center as a waypoint for diagonal connections.
    /// For straight lines (horizontal/vertical), draws directly.
    /// For diagonal connections, routes through the grid center.
    ///
    /// # Arguments
    /// * `from` - Starting point
    /// * `to` - Ending point
    /// * `size` - Size of the tile grid
    ///
    /// # Returns
    /// Vector of points forming the connection, excluding the center waypoint
    ///
    /// # Algorithm
    /// - If line is straight (horizontal/vertical): draw directly
    /// - If diagonal: draw from->center + center->to, then remove center and duplicates
    /// - This creates L-shaped paths for diagonal connections
    pub fn line(from: Point, to: Point, size: usize) -> Vec<Point> {
        // Direct path for straight lines
        if from.x == to.x || from.y == to.y {
            return TileRendererUtils::straight_line(from, to);
        }

        // L-shaped path through center for diagonal connections
        let center = TileRendererUtils::center(size);

        let mut points = Vec::new();
        points.extend(TileRendererUtils::straight_line(from, center));
        points.extend(TileRendererUtils::straight_line(center, to));
        points.dedup(); // Remove duplicate points
        points.retain(|p| *p != center); // Remove center waypoint

        points
    }

    /// Safely calculates a point offset from a base point, with boundary checking.
    ///
    /// # Arguments
    /// * `base` - Base point to offset from
    /// * `ox` - X offset (can be negative)
    /// * `oy` - Y offset (can be negative)
    /// * `size` - Size of the grid for boundary checking
    ///
    /// # Returns
    /// Some(Point) if the resulting point is within bounds, None otherwise
    ///
    /// # Safety
    /// Prevents coordinate underflow/overflow by checking bounds before conversion
    fn offset_point(base: Point, ox: isize, oy: isize, size: usize) -> Option<Point> {
        let nx = base.x as isize + ox;
        let ny = base.y as isize + oy;

        // Check bounds before converting back to usize
        if nx >= 0 && ny >= 0 && nx < size as isize && ny < size as isize {
            Some(Point::new(nx as usize, ny as usize))
        } else {
            None
        }
    }

    // Size thresholds for determining appropriate circle/feature sizes
    const SMALL_SIZE: usize = 3; // Tiles smaller than 3x3
    const MEDIUM_SIZE: usize = 6; // Tiles 3x3 to 5x5
    const LARGE_SIZE: usize = 8; // Tiles 6x6 to 7x7
    const XLARGE_SIZE: usize = 10; // Tiles 8x8 to 9x9

    /// Determines appropriate center feature size based on overall tile size.
    /// Scales feature size proportionally to tile size for visual consistency.
    ///
    /// # Arguments
    /// * `size` - The size of the tile grid
    ///
    /// # Returns
    /// Appropriate radius/size for center features (0-4)
    ///
    /// # Size Mapping
    /// - 0-2: size 0 (single point)
    /// - 3-5: size 1 (3x3 area)
    /// - 6-7: size 2 (5x5 area)
    /// - 8-9: size 3 (7x7 area)
    /// - 10+: size 4 (9x9 area)
    pub fn center_size(size: usize) -> usize {
        match size {
            0..Self::SMALL_SIZE => 0,
            Self::SMALL_SIZE..Self::MEDIUM_SIZE => 1,
            Self::MEDIUM_SIZE..Self::LARGE_SIZE => 2,
            Self::LARGE_SIZE..Self::XLARGE_SIZE => 3,
            Self::XLARGE_SIZE.. => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use carcasonne_core::layout::point::Point;

    /// Debug utility to visualize points on a grid.
    /// Prints a visual representation where '#' represents points and '.' represents empty space.
    ///
    /// # Arguments
    /// * `points` - Slice of points to display
    /// * `size` - Size of the grid to display
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

    /// Macro for testing tile center calculations across different sizes.
    /// Generates test cases that verify center point calculation and provide visual debugging.
    macro_rules! test_tile_center {
        (
            $(
                $name:ident: size = $size:expr => Point($x:expr, $y:expr);
            )*
        ) => {
            $(
                #[test]
                fn $name() {
                    let result = TileRendererUtils::center($size);
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

    // Test cases for center calculation
    test_tile_center! {
        test_center_even: size = 6 => Point(3, 3);   // Even-sized grid
        test_center_odd:  size = 5 => Point(2, 2);   // Odd-sized grid
        test_center_zero: size = 0 => Point(0, 0);   // Edge case: zero size
        test_center_large: size = 101 => Point(50, 50); // Large grid
    }

    /// Macro for testing circle generation with visual debugging.
    /// Tests circle algorithm across different centers, sizes, and grid boundaries.
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
                    let circle = TileRendererUtils::circle(center, $size, $total_size);

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

    // Test cases for circle generation
    test_tile_circle! {
        test_radius_zero_centered: center = (2, 2), total_size = 5, size = 0 => [
            Point::new(2, 2)  // Single point for radius 0
        ];

        test_radius_one_centered: center = (2, 2), total_size = 5, size = 1 => [
            Point::new(2, 2), // Center
            Point::new(3, 2), // Right
            Point::new(2, 3), // Down
            Point::new(1, 2), // Left
            Point::new(2, 1)  // Up
        ];

        test_radius_two_centered: center = (2, 2), total_size = 5, size = 2 => [
            Point::new(2, 2), // Center
            Point::new(3, 2), Point::new(1, 2), // Horizontal radius 1
            Point::new(2, 3), Point::new(2, 1), // Vertical radius 1
            Point::new(4, 2), Point::new(0, 2), // Horizontal radius 2
            Point::new(3, 3), Point::new(1, 3), Point::new(3, 1), Point::new(1, 1), // Diagonal radius 1
            Point::new(2, 4), Point::new(2, 0), // Vertical radius 2
        ];

        test_radius_two_top_left: center = (0, 0), total_size = 5, size = 2 => [
            Point::new(0, 0), // Center at edge - tests boundary clipping
            Point::new(1, 0), Point::new(0, 1), // Adjacent points
            Point::new(2, 0), Point::new(1, 1), Point::new(0, 2), // Radius 2 points
        ];
    }

    /// Macro for testing edge position calculations.
    /// Verifies that edge centers are calculated correctly for different tile sizes.
    macro_rules! test_edge_for_size {
        ($name:ident, $half:expr, $size:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    TileRendererUtils::edge($size, &Edge::North),
                    Point::new($half, 0) // Top edge, center horizontally
                );
                assert_eq!(
                    TileRendererUtils::edge($size, &Edge::East),
                    Point::new($size - 1, $half) // Right edge, center vertically
                );
                assert_eq!(
                    TileRendererUtils::edge($size, &Edge::South),
                    Point::new($half, $size - 1) // Bottom edge, center horizontally
                );
                assert_eq!(
                    TileRendererUtils::edge($size, &Edge::West),
                    Point::new(0, $half) // Left edge, center vertically
                );
            }
        };
    }

    // Test edge calculations for various sizes
    test_edge_for_size!(test_edge_1, 0, 1); // 1x1 tile (all edges at corners)
    test_edge_for_size!(test_edge_3, 1, 3); // 3x3 tile
    test_edge_for_size!(test_edge_5, 2, 5); // 5x5 tile
    test_edge_for_size!(test_edge_7, 3, 7); // 7x7 tile
    test_edge_for_size!(test_edge_9, 4, 9); // 9x9 tile

    /// Macro for testing straight line generation.
    /// Tests horizontal and vertical line drawing with visual debugging.
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
                    let result = TileRendererUtils::straight_line(from, to);

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

    // Test cases for straight line generation
    test_straight_line! {
        test_straight_line_horizontal:
            from = (0, 0), to = (4, 0), size = 5
            => [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)];

        test_straight_line_vertical:
            from = (2, 0), to = (2, 5), size = 5
            => [(2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5)];

        test_straight_line_horizontal_9:
            from = (0, 0), to = (8, 0), size = 9
            => [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0)];

        test_straight_line_vertical_9:
            from = (4, 0), to = (4, 8), size = 9
            => [(4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7), (4, 8)];

        test_straight_line_horizontal_reverse:
            from = (4, 0), to = (0, 0), size = 5
            => [(4, 0), (3, 0), (2, 0), (1, 0), (0, 0)];

        test_straight_line_vertical_reverse:
            from = (2, 5), to = (2, 0), size = 5
            => [(2, 5), (2, 4), (2, 3), (2, 2), (2, 1), (2, 0)];

        test_straight_line_single_point:
            from = (2, 2), to = (2, 2), size = 5
            => [(2, 2)];
    }

    /// Macro for testing the general line function (including L-shaped diagonal paths).
    /// Tests both straight lines and diagonal connections routed through center.
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
                    let line = TileRendererUtils::line(from, to, $size);

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

    // Test cases for general line function
    test_line! {
        // Straight line tests (should behave identically to straight_line)
        test_line_left_to_right:
            from = (0, 4), to = (8, 4), size = 9
            => [(0,4), (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (7,4), (8,4)];

        test_line_right_to_left:
            from = (8, 4), to = (0, 4), size = 9
            => [(8,4), (7,4), (6,4), (5,4), (4,4), (3,4), (2,4), (1,4), (0,4)];

        test_line_top_to_bottom:
            from = (4, 0), to = (4, 8), size = 9
            => [(4,0), (4,1), (4,2), (4,3), (4,4), (4,5), (4,6), (4,7), (4,8)];

        test_line_bottom_to_top:
            from = (4, 8), to = (4, 0), size = 9
            => [(4,8), (4,7), (4,6), (4,5), (4,4), (4,3), (4,2), (4,1), (4,0)];

        // Diagonal line tests (L-shaped paths through center, excluding center point)
        test_line_left_to_top:
            from = (0, 4), to = (4, 0), size = 9
            => [(0,4), (1,4), (2,4), (3,4),    // Horizontal segment to center
                (4,3), (4,2), (4,1), (4,0)];   // Vertical segment from center

        test_line_left_to_bottom:
            from = (0, 4), to = (4, 8), size = 9
            => [(0,4), (1,4), (2,4), (3,4),    // Horizontal segment to center
                (4,5), (4,6), (4,7), (4,8)];   // Vertical segment from center

        test_line_right_to_top:
            from = (8, 4), to = (4, 0), size = 9
            => [(8,4), (7,4), (6,4), (5,4),    // Horizontal segment to center
                (4,3), (4,2), (4,1), (4,0)];   // Vertical segment from center

        test_line_right_to_bottom:
            from = (8, 4), to = (4, 8), size = 9
            => [(8,4), (7,4), (6,4), (5,4),    // Horizontal segment to center
                (4,5), (4,6), (4,7), (4,8)];   // Vertical segment from center

        test_line_top_to_left:
            from = (4, 0), to = (0, 4), size = 9
            => [(4,0), (4,1), (4,2), (4,3),    // Vertical segment to center
                (3,4), (2,4), (1,4), (0,4)];   // Horizontal segment from center

        test_line_top_to_right:
            from = (4, 0), to = (8, 4), size = 9
            => [(4,0), (4,1), (4,2), (4,3),    // Vertical segment to center
                (5,4), (6,4), (7,4), (8,4)];   // Horizontal segment from center

        test_line_bottom_to_left:
            from = (4, 8), to = (0, 4), size = 9
            => [(4,8), (4,7), (4,6), (4,5),    // Vertical segment to center
                (3,4), (2,4), (1,4), (0,4)];   // Horizontal segment from center

        test_line_bottom_to_right:
            from = (4, 8), to = (8, 4), size = 9
            => [(4,8), (4,7), (4,6), (4,5),    // Vertical segment to center
                (5,4), (6,4), (7,4), (8,4)];   // Horizontal segment from center
    }
}
