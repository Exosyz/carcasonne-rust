use crate::model::tile_feature::{Edge, TileFeature, TileFeatureEnhancement, TileFeatureType};

/// Builder pattern for constructing `TileFeature` instances.
///
/// Supports setting the feature type, the edges it spans, and an optional enhancement.
/// Methods consume and return `self` for chaining.
pub struct TileFeatureBuilder {
    feature_type: Box<dyn TileFeatureType>,
    edges: Vec<Edge>,
    enhancement: Option<Box<dyn TileFeatureEnhancement>>,
}

impl TileFeatureBuilder {
    /// Creates a new `TileFeatureBuilder` with the given feature type.
    ///
    /// # Arguments
    ///
    /// * `feature` - The core feature type (e.g., Town, Road).
    pub fn new(feature: Box<dyn TileFeatureType>) -> Self {
        Self {
            feature_type: feature,
            edges: vec![],
            enhancement: None,
        }
    }

    /// Sets the edges that this feature occupies on the tile.
    ///
    /// # Arguments
    ///
    /// * `edge` - A vector of `Edge` values representing tile edges.
    pub fn edges(mut self, edge: Vec<Edge>) -> Self {
        self.edges = edge;
        self
    }

    /// Sets an optional enhancement for this tile feature.
    ///
    /// # Arguments
    ///
    /// * `enhancement` - A boxed trait object implementing `TileFeatureEnhancement`.
    pub fn enhancement(mut self, enhancement: Box<dyn TileFeatureEnhancement>) -> Self {
        self.enhancement = Some(enhancement);
        self
    }

    /// Builds the final `TileFeature` instance.
    pub fn build(self) -> TileFeature {
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
