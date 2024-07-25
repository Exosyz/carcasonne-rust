use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::board::{Board, BoardBuilder};
use crate::player::{Player, PlayerBuilder};
use crate::scoreboard::{ScoreBoard, ScoreBoardBuilder};
use crate::tile::{Tile, TileBuilder};

#[derive(Debug, Default, Clone)]
pub struct Game {
    pub players: Vec<Player>,
    pub score_board: ScoreBoard,
    pub available_tiles: Vec<Tile>,
    pub board: Board,
}

impl Game {
    pub fn shuffle_available_tiles(&mut self) -> &Self {
        self.available_tiles.shuffle(&mut thread_rng());
        self
    }

    pub fn get_next_tile(&mut self) -> Option<Tile> {
        self.available_tiles.pop()
    }
}

#[derive(Default, Clone)]
pub struct GameBuilder {
    players: Vec<Player>,
    available_tiles: Vec<Tile>,
    score_board: ScoreBoard,
    board: Board,
}

impl GameBuilder {
    pub fn board(
        &mut self,
        board_builder: impl FnOnce(&mut BoardBuilder) -> &mut BoardBuilder,
    ) -> &mut Self {
        let mut builder = BoardBuilder::default();
        board_builder(&mut builder);

        let mut board = Board::default();
        builder.build(&mut board);

        self.board = board;
        self
    }

    pub fn default_board(&mut self) -> &mut Self {
        self.board = Board::default();
        self
    }

    pub fn scores(
        &mut self,
        score_board_builder: impl FnOnce(&mut ScoreBoardBuilder) -> &mut ScoreBoardBuilder,
    ) -> &mut Self {
        let mut builder = ScoreBoardBuilder::default();
        score_board_builder(&mut builder);

        let mut score_board = ScoreBoard::default();
        builder.build(&mut score_board);

        self.score_board = score_board;
        self
    }

    pub fn add_player(
        &mut self,
        player_builder: impl FnOnce(&mut PlayerBuilder) -> &mut PlayerBuilder,
    ) -> &mut Self {
        let mut builder = PlayerBuilder::default();
        player_builder(&mut builder);

        let mut player = Player::default();
        builder.build(&mut player);

        self.players.push(player);
        self
    }

    pub fn add_tile(
        &mut self,
        tile_builder: impl FnOnce(&mut TileBuilder) -> &mut TileBuilder,
        quantity: usize,
    ) -> &mut Self {
        let mut builder = TileBuilder::default();
        tile_builder(&mut builder);

        let mut tile = Tile::default();
        builder.build(&mut tile);

        for _ in 0..quantity {
            self.available_tiles.push(tile.clone());
        }
        self
    }

    pub fn build(self, game: &mut Game) {
        game.players = self.players;
        game.available_tiles = self.available_tiles;
        game.board = self.board;
        game.score_board = self.score_board;
    }
}
