use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::board::Board;
use crate::player::{Player, PlayerBuilder};
use crate::scoreboard::ScoreBoard;
use crate::tile::Tile;

#[derive(Debug, Default)]
pub struct Game {
    pub players: Vec<Player>,
    pub scores: ScoreBoard,
    pub available_tiles: Vec<Tile>,
    pub board: Board,
}

impl Game {
    pub fn shuffle_available_tiles(mut self) -> Game {
        self.available_tiles.shuffle(&mut thread_rng());
        self
    }
}

#[derive(Default)]
pub struct GameBuilder {
    players: Vec<Player>,
    available_tiles: Vec<Tile>,
    scores: ScoreBoard,
    board: Board,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn board(mut self, board: Board) -> Self {
        self.board = board;
        self
    }

    pub fn scores(mut self, scores: ScoreBoard) -> Self {
        self.scores = scores;
        self
    }

    pub fn add_player(
        mut self,
        player_builder: impl FnOnce(&mut PlayerBuilder) -> &mut PlayerBuilder,
    ) -> Self {
        let mut builder = PlayerBuilder::new();
        let player = player_builder(&mut builder);
        self.players.push(player.build());
        self
    }

    pub fn add_tile(mut self, tile: Tile, quantity: usize) -> Self {
        for _ in 0..quantity {
            self.available_tiles.push(tile);
        }
        self
    }

    pub fn build(self) -> Game {
        Game {
            players: self.players,
            scores: self.scores,
            available_tiles: self.available_tiles,
            board: self.board,
        }
    }
}
