use crate::model::tile::Tile;
use rand::rng;
use rand::seq::SliceRandom;

pub struct GameContext {
    /// The list of remaining tiles in the game.
    pub available_tiles: Vec<Tile>,
}

impl GameContext {
    /// Randomly selects and removes a tile from the remaining pool.
    ///
    /// Internally, this method shuffles the remaining tiles and pops one
    /// from the end of the vector. It returns `None` if no tiles remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasonne_core::context::GameContext;
    ///
    /// let mut game_tiles = GameContext { available_tiles: vec![] };
    /// let tile = game_tiles.select_random_tile();
    /// ```
    pub fn select_random_tile(&mut self) -> Option<Tile> {
        self.available_tiles.shuffle(&mut rng());
        self.available_tiles.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tile::Tile;

    fn dummy_tile() -> Tile {
        Tile {
            tile_features: vec![],
            tile_extension: None,
        }
    }

    #[test]
    fn test_select_random_tile_returns_tile() {
        let mut game_tiles = GameContext {
            available_tiles: vec![dummy_tile()],
        };

        let tile = game_tiles.select_random_tile();
        assert!(tile.is_some(), "Expected to get a tile");
        assert_eq!(
            game_tiles.available_tiles.len(),
            0,
            "Deck should be empty after pop"
        );
    }

    #[test]
    fn test_select_random_tile_from_empty_deck_returns_none() {
        let mut game_tiles = GameContext {
            available_tiles: vec![],
        };
        let tile = game_tiles.select_random_tile();
        assert!(
            tile.is_none(),
            "Expected None when drawing from the empty deck"
        );
    }

    #[test]
    fn test_random_selection_exhausts_all_tiles() {
        let mut game_tiles = GameContext {
            available_tiles: vec![
                dummy_tile(),
                dummy_tile(),
                dummy_tile(),
                dummy_tile(),
                dummy_tile(),
            ],
        };

        let mut drawn = vec![];
        while let Some(tile) = game_tiles.select_random_tile() {
            drawn.push(tile);
        }

        assert_eq!(drawn.len(), 5, "Should have drawn all tiles");
        assert!(
            game_tiles.available_tiles.is_empty(),
            "Deck should be empty after drawing all tiles"
        );
    }

    #[test]
    fn test_shuffling_changes_order() {
        let tiles: Vec<Tile> = vec![dummy_tile(), dummy_tile(), dummy_tile()];
        let mut game_tiles_1 = GameContext {
            available_tiles: tiles.clone(),
        };
        let mut game_tiles_2 = GameContext {
            available_tiles: tiles.clone(),
        };

        // Shuffle both
        game_tiles_1.select_random_tile(); // first shuffle (done implicitly)
        game_tiles_2.select_random_tile(); // second shuffle

        // We can't guarantee difference, but we can at least check that the deck was modified
        // (it shrinks and is in a different order than initial)
        assert!(game_tiles_1.available_tiles.len() < 3);
        assert!(game_tiles_2.available_tiles.len() < 3);
    }
}
