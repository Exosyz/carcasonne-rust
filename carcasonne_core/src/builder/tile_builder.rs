use crate::builder::tile_feature_builder::TileFeatureBuilder;
use crate::model::tile::Tile;
use crate::model::tile_extension::{Abbey, TileExtension};
use crate::model::tile_feature::{Edge, Road, Shield, TileFeature, Town};

pub(crate) struct TileBuilder {
    tile_features: Vec<TileFeature>,
    tile_extension: Option<Box<dyn TileExtension>>,
}

impl TileBuilder {
    pub(crate) fn new() -> Self {
        Self {
            tile_features: Vec::new(),
            tile_extension: None,
        }
    }

    pub(crate) fn add_town(mut self, edges: Vec<Edge>) -> Self {
        self.tile_features.push(
            TileFeatureBuilder::new(Box::new(Town {}))
                .edges(edges)
                .build(),
        );
        self
    }

    pub(crate) fn add_shielded_town(mut self, edges: Vec<Edge>) -> Self {
        self.tile_features.push(
            TileFeatureBuilder::new(Box::new(Town {}))
                .edges(edges)
                .enhancement(Box::new(Shield {}))
                .build(),
        );
        self
    }

    pub(crate) fn add_road(mut self, edges: Vec<Edge>) -> Self {
        self.tile_features.push(
            TileFeatureBuilder::new(Box::new(Road {}))
                .edges(edges)
                .build(),
        );
        self
    }

    pub(crate) fn add_abbey(mut self) -> Self {
        self.tile_extension = Some(Box::new(Abbey {}));
        self
    }

    pub(crate) fn build(self) -> Tile {
        Tile {
            tile_features: self.tile_features,
            tile_extension: self.tile_extension,
        }
    }
}
