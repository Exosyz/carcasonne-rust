use crate::renderer::ascii_renderer::{AsciiRenderer, RenderChar};
use crate::renderer::base_renderer::{RenderOutput, TileRenderer};
use crate::tile::Tile;
use std::collections::HashMap;

impl TileRenderer for AsciiRenderer {
    fn render(&self, rotated_tile: &Tile) -> RenderOutput {
        let mut output = vec![vec![RenderChar::Meadow; self.tile_size]; self.tile_size];

        let mut sections: HashMap<usize, Vec<RenderChar>> = HashMap::new();

        let sides = [
            &rotated_tile.north,
            &rotated_tile.east,
            &rotated_tile.south,
            &rotated_tile.west,
        ];

        for side in sides.iter() {
            if let Some(behavior) = self.get_behavior(side.kind) {
                if let Some((section, (row, col), c)) = behavior.handle_side(side) {
                    if row < self.tile_size && col < self.tile_size {
                        sections.entry(section).or_default().push(c);
                        output[row][col] = c;
                    }
                }
            }
        }

        let pairs = [
            (&rotated_tile.north, &rotated_tile.east),
            (&rotated_tile.east, &rotated_tile.south),
            (&rotated_tile.south, &rotated_tile.west),
            (&rotated_tile.west, &rotated_tile.north),
        ];

        for &(side1, side2) in pairs.iter() {
            if side1.kind == side2.kind {
                if let Some(behavior) = self.get_behavior(side1.kind) {
                    if let Some(((row, col), c)) = behavior.handle_pair(side1, side2) {
                        if row < self.tile_size && col < self.tile_size {
                            output[row][col] = c;
                        }
                    }
                }
            }
        }

        if let Some(c) = self.render_center(rotated_tile, &sections) {
            output[1][1] = c;
        }

        let palette = self.palette.clone().unwrap_or_default();
        let char_output: Vec<Vec<char>> = output
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|rc| rc.with_palette(&palette))
                    .collect()
            })
            .collect();

        RenderOutput::Ascii(char_output)
    }
}
