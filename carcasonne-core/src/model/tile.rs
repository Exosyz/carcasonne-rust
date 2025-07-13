use crate::model::tile_extension::TileExtension;
use crate::model::tile_feature::TileFeature;

/// Represents a tile in the game, composed of visual and behavioral elements.
///
/// A `Tile` combines a set of structural features (like roads or cities)
/// with optional extended behavior through a `TileExtension` trait object.
#[derive(Debug, Clone)]
pub struct Tile {
    /// The features present on the tile (e.g., roads, cities).
    pub tile_features: Vec<TileFeature>,

    /// An optional extension providing additional behavior or metadata.
    ///
    /// This is implemented as a trait object, allowing different tile types
    /// (e.g., monastery) to extend base functionality.
    pub tile_extension: Option<Box<dyn TileExtension>>,
}
