use crate::tile::Tile;

#[derive(Debug, Default, Clone)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
}

impl Board {}

#[derive(Default)]
pub struct BoardBuilder {
    tiles: Vec<Vec<Tile>>,
}

impl BoardBuilder {
    pub fn build(self, board: &mut Board) {
        board.tiles = self.tiles;
    }
}
