use crate::model::tile_extension::TileExtension;
use crate::model::tile_feature::TileFeature;

#[derive(Debug, Clone)]
pub struct Tile {
    pub(crate) tile_features: Vec<TileFeature>,
    pub(crate) tile_extension: Option<Box<dyn TileExtension>>,
}
