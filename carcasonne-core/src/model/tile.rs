use crate::model::tile_extension::TileExtension;
use crate::model::tile_feature::TileFeature;

/// Represents a tile in the game, composed of visual and behavioral elements.
///
/// A `Tile` combines a set of structural features (like roads or cities)
/// with optional extended behavior through a `TileExtension` trait object.
#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_id: String,
    /// The features present on the tile (e.g., roads, cities).
    pub tile_features: Vec<TileFeature>,

    /// An optional extension providing additional behavior or metadata.
    ///
    /// This is implemented as a trait object, allowing different tile types
    /// (e.g., monastery) to extend base functionality.
    pub tile_extension: Option<Box<dyn TileExtension>>,
}

impl Tile {
    pub fn rotate_left(&mut self) -> &mut Self {
        self.tile_features.iter_mut().for_each(|f| {
            f.rotate_left();
        });

        self
    }

    pub fn rotate_right(&mut self) -> &mut Self {
        self.tile_features.iter_mut().for_each(|f| {
            f.rotate_right();
        });

        self
    }
}
