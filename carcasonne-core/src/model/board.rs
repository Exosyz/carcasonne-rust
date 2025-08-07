use crate::model::player::Player;
use crate::model::tile::Tile;
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }
    pub fn zero() -> Position {
        Position::new(0, 0)
    }

    pub fn set_min(&mut self, other: &Position) {
        self.x = self.x.min(other.x);
        self.y = self.y.min(other.y);
    }

    pub fn set_max(&mut self, other: &Position) {
        self.x = self.x.max(other.x);
        self.y = self.y.max(other.y);
    }
}

#[derive(Debug)]
pub struct PlacedTile<'a>(&'a Tile, &'a Player);

#[derive(Debug)]
pub struct Board<'board> {
    placed_tiles: HashMap<Position, PlacedTile<'board>>,
    min: Position,
    max: Position,
}

pub enum BoardError {
    InvalidPosition,
}

impl Default for Board<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'board> Board<'board> {
    pub fn new() -> Board<'board> {
        Board {
            min: Position::zero(),
            max: Position::zero(),
            placed_tiles: HashMap::new(),
        }
    }

    pub fn is_valid_position(&self, position: &Position) -> Result<bool, BoardError> {
        if self.placed_tiles.is_empty() && position != &Position::zero() {
            return Err(BoardError::InvalidPosition);
        }

        Ok(self.placed_tiles.contains_key(position))
    }

    pub fn place_tile(
        &mut self,
        position: Position,
        tile: &'board Tile,
        player: &'board Player,
    ) -> Result<(), BoardError> {
        if let Ok(true) = self.is_valid_position(&position) {
            self.min.set_min(&position);
            self.max.set_max(&position);
            self.placed_tiles.insert(position, PlacedTile(tile, player));
            Ok(())
        } else {
            Err(BoardError::InvalidPosition)
        }
    }
}
