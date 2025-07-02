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
