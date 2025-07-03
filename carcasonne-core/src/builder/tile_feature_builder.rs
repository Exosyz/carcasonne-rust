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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tile_feature::{Road, Shield, Town};
    use std::any::TypeId;
    #[test]
    fn test_tile_feature_builder_with_town() {
        let edges = vec![Edge::North, Edge::East];

        let feature = TileFeatureBuilder::new(Box::new(Town {}))
            .edges(edges.clone())
            .build();

        assert_eq!(feature.edges, edges);
        assert!(feature.enhancement.is_none());
        assert_eq!(
            feature.feature_type.as_ref().type_id(),
            TypeId::of::<Town>()
        );
    }

    #[test]
    fn test_tile_feature_builder_with_road() {
        let edges = vec![Edge::South];

        let feature = TileFeatureBuilder::new(Box::new(Road {}))
            .edges(edges.clone())
            .build();

        assert_eq!(feature.edges, edges);
        assert!(feature.enhancement.is_none());
        assert_eq!(
            feature.feature_type.as_ref().type_id(),
            TypeId::of::<Road>()
        );
    }

    #[test]
    fn test_tile_feature_builder_with_enhancement() {
        let edges = vec![Edge::West];

        let feature = TileFeatureBuilder::new(Box::new(Town {}))
            .edges(edges.clone())
            .enhancement(Box::new(Shield {}))
            .build();

        assert_eq!(feature.edges, edges);
        assert!(feature.enhancement.is_some());
        assert_eq!(
            feature.feature_type.as_ref().type_id(),
            TypeId::of::<Town>()
        );
        assert_eq!(
            feature.enhancement.unwrap().as_ref().type_id(),
            TypeId::of::<Shield>()
        );
    }
}
