use crate::renderer::base_renderer::{
    GraphicRenderer, RenderOutput, TileRenderer, UnicodeRenderer,
};
use crate::tile::Tile;

impl TileRenderer for UnicodeRenderer {
    fn render(&self, rotated_tile: &Tile) -> RenderOutput {
        panic!("Not implemented");
    }
}

impl TileRenderer for GraphicRenderer {
    fn render(&self, rotated_tile: &Tile) -> RenderOutput {
        panic!("Not implemented");
    }
}
