use crate::builder::tile_builder::TileBuilder;
use crate::factory::tile_factory::TileFactory;
use crate::model::tile::Tile;
use crate::model::tile_feature::Edge::South;

pub trait AbbeyTileBuilder {
    fn build_a_abbey() -> Tile;
    fn build_b_abbey() -> Tile;
}

impl AbbeyTileBuilder for TileFactory {
    fn build_a_abbey() -> Tile {
        TileBuilder::new().add_road(vec![South]).add_abbey().build()
    }
    fn build_b_abbey() -> Tile {
        TileBuilder::new().add_abbey().build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tile_extension::Abbey;
    use crate::model::tile_feature::Edge::South;
    use crate::model::tile_feature::Road;
    use std::any::TypeId;

    #[test]
    fn test_build_a_abbey() {
        let tile = TileFactory::build_a_abbey();

        // Vérifie qu'il y a bien une route sur le bord Sud
        assert_eq!(tile.tile_features.len(), 1);
        let feature = &tile.tile_features[0];
        assert_eq!(feature.edges, vec![South]);
        assert_eq!(
            feature.feature_type.as_ref().type_id(),
            TypeId::of::<Road>()
        );

        // Vérifie qu'il y a bien une abbaye
        assert!(tile.tile_extension.is_some());
        assert_eq!(
            tile.tile_extension.unwrap().as_ref().type_id(),
            TypeId::of::<Abbey>()
        );
    }

    #[test]
    fn test_build_b_abbey() {
        let tile = TileFactory::build_b_abbey();

        // Pas de routes
        assert!(tile.tile_features.is_empty());

        // Présence de l'abbaye
        assert!(tile.tile_extension.is_some());
        assert_eq!(
            tile.tile_extension.unwrap().as_ref().type_id(),
            TypeId::of::<Abbey>()
        );
    }
}
