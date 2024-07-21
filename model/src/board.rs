use crate::tile::Tile;

#[derive(Debug, Default)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
}

impl Board {}

#[derive(Default)]
pub struct BoardBuilder {
    tiles: Vec<Vec<Tile>>,
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Board {
        Board { tiles: self.tiles }
    }
}
