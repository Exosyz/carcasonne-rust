use crate::builder::tile_builder::TileBuilder;
use crate::factory::tile_factory::TileFactory;
use crate::model::tile::Tile;
use crate::model::tile_feature::Edge::South;

pub trait AbbeyTileBuilder {
    fn build_a_abbey() -> Tile;
    fn build_b_abbey() -> Tile;
}

impl AbbeyTileBuilder for TileFactory {
    fn build_a_abbey() -> Tile {
        TileBuilder::new().add_road(vec![South]).add_abbey().build()
    }
    fn build_b_abbey() -> Tile {
        TileBuilder::new().add_abbey().build()
    }
}
