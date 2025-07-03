use crate::model::game::GameTiles;
use crate::model::tile::Tile;

pub struct GameBuilder {
    tiles: Vec<Tile>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self { tiles: Vec::new() }
    }

    pub fn add_tiles(mut self, tile: Tile, quantity: usize) -> Self {
        self.tiles.extend(vec![tile; quantity]);
        self
    }

    pub fn build(self) -> GameTiles {
        GameTiles { tiles: self.tiles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::tile_builder::TileBuilder;
    use crate::model::tile_feature::Edge::North;
    use crate::model::tile_feature::{Road, Town};
    use std::any::{Any, TypeId};

    #[test]
    fn test_game_builder_empty() {
        let game = GameBuilder::new().build();
        assert!(game.tiles.is_empty());
    }

    #[test]
    fn test_game_builder_add_tiles_once() {
        let tile = TileBuilder::new().add_town(vec![North]).build();
        let game = GameBuilder::new().add_tiles(tile.clone(), 3).build();

        assert_eq!(game.tiles.len(), 3);
        assert!(game.tiles.iter().all(|t| t.tile_extension.is_none()
            && t.tile_features.len() == 1
            && t.tile_features[0].feature_type.as_ref().type_id() == TypeId::of::<Town>()
            && t.tile_features[0].edges.len() == 1
            && t.tile_features[0].edges[0] == North));
    }

    #[test]
    fn test_game_builder_add_tiles_multiple() {
        let tile1 = TileBuilder::new().add_town(vec![North]).build();
        let tile2 = TileBuilder::new().add_road(vec![North]).build();

        let game = GameBuilder::new()
            .add_tiles(tile1.clone(), 2)
            .add_tiles(tile2.clone(), 3)
            .build();

        assert_eq!(game.tiles.len(), 5);
        assert!(compare_tile_extension::<Town>(&game.tiles[0]));
        assert!(compare_tile_extension::<Town>(&game.tiles[1]));
        assert!(compare_tile_extension::<Road>(&game.tiles[2]));
        assert!(compare_tile_extension::<Road>(&game.tiles[3]));
        assert!(compare_tile_extension::<Road>(&game.tiles[4]));
    }

    fn compare_tile_extension<T: 'static>(tile: &Tile) -> bool {
        tile.tile_extension.is_none()
            && tile.tile_features.len() == 1
            && tile.tile_features[0].feature_type.as_ref().type_id() == TypeId::of::<T>()
            && tile.tile_features[0].edges.len() == 1
            && tile.tile_features[0].edges[0] == North
    }
}
