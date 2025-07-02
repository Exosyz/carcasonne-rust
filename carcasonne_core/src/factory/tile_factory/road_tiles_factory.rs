use crate::builder::tile_builder::TileBuilder;
use crate::factory::tile_factory::TileFactory;
use crate::model::tile::Tile;
use crate::model::tile_feature::Edge::{East, North, South, West};

pub trait RoadTileBuilder {
    fn build_u_road() -> Tile;
    fn build_v_road() -> Tile;
    fn build_w_road() -> Tile;
    fn build_x_road() -> Tile;
}

impl RoadTileBuilder for TileFactory {
    fn build_u_road() -> Tile {
        TileBuilder::new().add_road(vec![North, South]).build()
    }
    fn build_v_road() -> Tile {
        TileBuilder::new().add_road(vec![North, West]).build()
    }
    fn build_w_road() -> Tile {
        TileBuilder::new()
            .add_road(vec![North])
            .add_road(vec![West])
            .add_road(vec![South])
            .build()
    }
    fn build_x_road() -> Tile {
        TileBuilder::new()
            .add_road(vec![North])
            .add_road(vec![West])
            .add_road(vec![South])
            .add_road(vec![East])
            .build()
    }
}
