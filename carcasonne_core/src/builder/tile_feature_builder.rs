use crate::model::tile_feature::{Edge, TileFeature, TileFeatureEnhancement, TileFeatureType};

pub(super) struct TileFeatureBuilder {
    feature_type: Box<dyn TileFeatureType>,
    edges: Vec<Edge>,
    enhancement: Option<Box<dyn TileFeatureEnhancement>>,
}

impl TileFeatureBuilder {
    pub(super) fn new(feature: Box<dyn TileFeatureType>) -> Self {
        Self {
            feature_type: feature,
            edges: vec![],
            enhancement: None,
        }
    }

    pub(super) fn edges(mut self, edge: Vec<Edge>) -> Self {
        self.edges = edge;
        self
    }

    pub(super) fn enhancement(mut self, enhancement: Box<dyn TileFeatureEnhancement>) -> Self {
        self.enhancement = Some(enhancement);
        self
    }

    pub(super) fn build(self) -> TileFeature {
        TileFeature {
            edges: self.edges,
            feature_type: self.feature_type,
            enhancement: self.enhancement,
        }
    }
}
