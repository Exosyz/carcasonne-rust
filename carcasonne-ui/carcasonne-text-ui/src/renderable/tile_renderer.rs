mod tile_charset;
mod tile_renderer_builder;
mod tile_renderer_helper;
mod tile_renderer_utils;

use crate::frame::Frame;
use crate::renderable::tile_renderer::tile_renderer_builder::TileRendererBuilder;
use crate::renderable::{fit_within_bounds, Renderable};
use carcasonne_core::layout::point::Point;
use carcasonne_core::layout::size::Size;
use carcasonne_core::model::rotation::Rotation;
use carcasonne_core::model::tile::Tile;
use carcasonne_core::model::tile_size::TileSize;

/// A `Renderable` implementation that draws a [`Tile`] with a given rotation
/// and size onto a [`Frame`].
///
/// `TileRenderer` acts as a thin wrapper around [`TileRendererBuilder`],
/// which generates the 2D character representation of a tile.
/// It then writes those characters into the frame at the specified position.
///
/// # Parameters
/// - `tile` - The game tile to render (terrain, city, road, etc.).
/// - `rotation` - The rotation applied before rendering.
/// - `size` - The logical size of the tile (e.g. small, medium, large).
///
/// # Guarantees
/// - [`Renderable::size`] always returns a square size of `(n, n)`
///   where `n = size.get_size()`.
/// - [`Renderable::render`] will never write outside of this bounding box.
/// - Rendering does not mutate the `Tile`, `Rotation`, or `TileSize`.
pub struct TileRenderer<'a> {
    tile: &'a Tile,
    rotation: &'a Rotation,
    size: &'a TileSize,
}

impl<'a> TileRenderer<'a> {
    pub fn new(tile: &'a Tile, rotation: &'a Rotation, size: &'a TileSize) -> Self {
        Self {
            tile,
            rotation,
            size,
        }
    }
}

impl<'a> Renderable for TileRenderer<'a> {
    fn render(&self, frame: &mut Frame, parent_available_size: Size, point: Point) {
        let chars = TileRendererBuilder::build(self.size.get_size(), self.tile, self.rotation);

        chars
            .iter()
            .enumerate()
            .filter(|(j, _)| j < &parent_available_size.height)
            .for_each(|(j, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(i, _)| i < &parent_available_size.width)
                    .for_each(|(i, c)| frame.char_simple(point + Point::new(i, j), *c))
            });
    }

    fn size(&self, parent_available_size: Size) -> Size {
        let size = self.size.get_size();
        fit_within_bounds(Size::new(size, size), parent_available_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::Frame;
    use carcasonne_core::factory::tile_factory::abbey_tiles_factory::AbbeyTileBuilder;
    use carcasonne_core::factory::tile_factory::TileFactory;
    use carcasonne_core::layout::point::Point;
    use carcasonne_core::layout::size::Size;
    use carcasonne_core::model::rotation::Rotation;
    use carcasonne_core::model::tile::Tile;
    use carcasonne_core::model::tile_size::TileSize;

    fn dummy_tile() -> Tile {
        TileFactory::build_a_abbey()
    }

    // --- new() ---
    #[test]
    fn test_new_initializes_fields() {
        let tile = dummy_tile();
        let rot = Rotation::R0;
        let size = TileSize::Small;

        let r = TileRenderer::new(&tile, &rot, &size);

        assert!(std::ptr::eq(r.tile, &tile));
        assert!(std::ptr::eq(r.rotation, &rot));
        assert!(std::ptr::eq(r.size, &size));
    }

    // --- size() ---
    #[test]
    fn test_size_matches_tile_size() {
        let tile = dummy_tile();
        let rot = Rotation::R0;

        for s in [TileSize::Small, TileSize::Medium, TileSize::Large].iter() {
            let r = TileRenderer::new(&tile, &rot, s);
            let expected = Size::new(s.get_size(), s.get_size());
            assert_eq!(r.size(Size::new(50, 50)), expected);
        }
    }

    #[test]
    fn test_size_respects_parent_bounds() {
        let tile = dummy_tile();
        let rot = Rotation::R0;
        let size = TileSize::Large;
        let r = TileRenderer::new(&tile, &rot, &size);

        // Large tile but parent only 5x5
        let bounded = r.size(Size::new(5, 5));
        assert!(bounded.width <= 5 && bounded.height <= 5);
    }

    // --- render() ---
    #[test]
    fn test_render_places_chars_in_frame() {
        let tile = dummy_tile();
        let rot = Rotation::R0;
        let size = TileSize::Small;
        let renderer = TileRenderer::new(&tile, &rot, &size);

        let mut frame = Frame::new(Size::new(size.get_size(), size.get_size()));
        renderer.render(&mut frame, Size::new(50, 50), Point::new(0, 0));

        let mut has_char = false;
        for y in 0..size.get_size() {
            for x in 0..size.get_size() {
                if frame.cells[y][x].symbol != ' ' {
                    has_char = true;
                }
            }
        }
        assert!(
            has_char,
            "Expected at least one non-empty char in rendered frame"
        );
    }
}
