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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tile_feature::Edge::{East, North, South, West};
    use crate::model::tile_feature::{Edge, Road};
    use std::any::TypeId;

    fn assert_road_edges(tile: &Tile, expected_edges: Vec<Vec<Edge>>) {
        assert_eq!(tile.tile_features.len(), expected_edges.len());
        for (feature, expected) in tile.tile_features.iter().zip(expected_edges) {
            assert_eq!(
                feature.feature_type.as_ref().type_id(),
                TypeId::of::<Road>()
            );
            assert_eq!(feature.edges, expected);
        }
    }

    #[test]
    fn test_build_u_road() {
        let tile = TileFactory::build_u_road();
        assert_road_edges(&tile, vec![vec![North, South]]);
    }

    #[test]
    fn test_build_v_road() {
        let tile = TileFactory::build_v_road();
        assert_road_edges(&tile, vec![vec![North, West]]);
    }

    #[test]
    fn test_build_w_road() {
        let tile = TileFactory::build_w_road();
        assert_road_edges(&tile, vec![vec![North], vec![West], vec![South]]);
    }

    #[test]
    fn test_build_x_road() {
        let tile = TileFactory::build_x_road();
        assert_road_edges(
            &tile,
            vec![vec![North], vec![West], vec![South], vec![East]],
        );
    }
}
