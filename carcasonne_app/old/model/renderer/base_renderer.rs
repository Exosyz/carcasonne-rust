use crate::renderer::ascii_renderer::AsciiRenderer;
use crate::tile::Tile;

#[derive(Debug, Clone)]
pub enum GameRendererType {
    Ascii(AsciiRenderer),
    Unicode(UnicodeRenderer),
    Graphic(GraphicRenderer),
}

impl Default for GameRendererType {
    fn default() -> Self {
        Self::Ascii(AsciiRenderer::default())
    }
}

#[derive(Debug)]
pub enum RenderOutput {
    Ascii(Vec<Vec<char>>),
    Graphic(/* par exemple une texture ou un sprite */),
}

#[derive(Clone, Debug)]
pub struct UnicodeRenderer {}

#[derive(Clone, Debug)]
pub struct GraphicRenderer {}

pub trait TileRenderer {
    fn render(&self, rotated_tile: &Tile) -> RenderOutput;
}
