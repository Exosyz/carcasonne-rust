use crate::builder::tile_feature_builder::TileFeatureBuilder;
use crate::model::tile::Tile;
use crate::model::tile_extension::{Abbey, TileExtension};
use crate::model::tile_feature::{Edge, Road, Shield, TileFeature, Town};

/// A builder for constructing complex `Tile` instances.
///
/// This builder supports adding multiple features such as towns and roads,
/// optionally enhanced (e.g., shielded towns), and setting tile extensions like Abbeys.
///
/// The builder methods consume and return `self` for ergonomic chaining.
pub struct TileBuilder {
    tile_features: Vec<TileFeature>,
    tile_extension: Option<Box<dyn TileExtension>>,
}

impl TileBuilder {
    /// Creates a new empty `TileBuilder`.
    pub fn new() -> Self {
        Self {
            tile_features: Vec::new(),
            tile_extension: None,
        }
    }

    /// Adds a town feature covering the specified edges.
    ///
    /// # Arguments
    ///
    /// * `edges` - The edges of the tile that the town feature occupies.
    pub fn add_town(mut self, edges: Vec<Edge>) -> Self {
        self.tile_features.push(
            TileFeatureBuilder::new(Box::new(Town {}))
                .edges(edges)
                .build(),
        );
        self
    }

    /// Adds a town feature with a shield enhancement on the specified edges.
    ///
    /// Shields typically grant bonus points when scoring the town.
    ///
    /// # Arguments
    ///
    /// * `edges` - The edges of the tile that the shielded town occupies.
    pub fn add_shielded_town(mut self, edges: Vec<Edge>) -> Self {
        self.tile_features.push(
            TileFeatureBuilder::new(Box::new(Town {}))
                .edges(edges)
                .enhancement(Box::new(Shield {}))
                .build(),
        );
        self
    }

    /// Adds a road feature covering the specified edges.
    ///
    /// # Arguments
    ///
    /// * `edges` - The edges of the tile that the road feature occupies.
    pub fn add_road(mut self, edges: Vec<Edge>) -> Self {
        self.tile_features.push(
            TileFeatureBuilder::new(Box::new(Road {}))
                .edges(edges)
                .build(),
        );
        self
    }

    /// Adds an Abbey tile extension.
    ///
    /// Abbeys are special tile extensions that typically affect scoring or placement.
    pub fn add_abbey(mut self) -> Self {
        self.tile_extension = Some(Box::new(Abbey {}));
        self
    }

    /// Finalizes the builder and returns the constructed `Tile`.
    pub fn build(self) -> Tile {
        Tile {
            tile_features: self.tile_features,
            tile_extension: self.tile_extension,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tile_feature::Edge;
    use std::any::TypeId;

    #[test]
    fn test_new_tile_builder() {
        let builder = TileBuilder::new();
        assert!(builder.tile_features.is_empty());
        assert!(builder.tile_extension.is_none());
    }

    #[test]
    fn test_add_town() {
        let edges = vec![Edge::North, Edge::East];
        let tile = TileBuilder::new().add_town(edges.clone()).build();

        assert_eq!(tile.tile_features.len(), 1);
        let feature = &tile.tile_features[0];

        assert_eq!(feature.edges, edges);
        assert!(feature.enhancement.is_none());
        assert_eq!(
            feature.feature_type.as_ref().type_id(),
            TypeId::of::<Town>()
        );
    }

    #[test]
    fn test_add_shielded_town() {
        let edges = vec![Edge::South, Edge::West];
        let tile = TileBuilder::new().add_shielded_town(edges.clone()).build();

        assert_eq!(tile.tile_features.len(), 1);
        let feature = &tile.tile_features[0];

        assert_eq!(feature.edges, edges);
        assert!(feature.enhancement.is_some());
        assert_eq!(
            feature.feature_type.as_ref().type_id(),
            TypeId::of::<Town>()
        );
        assert_eq!(
            feature.enhancement.clone().unwrap().as_ref().type_id(),
            TypeId::of::<Shield>()
        );
    }

    #[test]
    fn test_add_road() {
        let edges = vec![Edge::North];
        let tile = TileBuilder::new().add_road(edges.clone()).build();

        assert_eq!(tile.tile_features.len(), 1);
        let feature = &tile.tile_features[0];
        println!("abbey {:?}", TypeId::of::<Abbey>());

        assert_eq!(feature.edges, edges);
        assert!(feature.enhancement.is_none());
        assert_eq!(
            feature.feature_type.as_ref().type_id(),
            TypeId::of::<Road>()
        );
    }

    #[test]
    fn test_add_abbey() {
        let tile = TileBuilder::new().add_abbey().build();
        assert!(tile.tile_extension.is_some());
        assert_eq!(
            tile.tile_extension.unwrap().as_ref().type_id(),
            TypeId::of::<Abbey>()
        );
    }

    #[test]
    fn test_combined_tile() {
        let edges_town = vec![Edge::North, Edge::South];
        let edges_road = vec![Edge::East, Edge::West];

        let tile = TileBuilder::new()
            .add_town(edges_town.clone())
            .add_road(edges_road.clone())
            .add_abbey()
            .build();

        assert_eq!(tile.tile_features.len(), 2);
        assert_eq!(
            tile.tile_features[0].feature_type.as_ref().type_id(),
            TypeId::of::<Town>()
        );
        assert_eq!(
            tile.tile_features[1].feature_type.as_ref().type_id(),
            TypeId::of::<Road>()
        );
        assert!(tile.tile_extension.is_some());
    }
}
