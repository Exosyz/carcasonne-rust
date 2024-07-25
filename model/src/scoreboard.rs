use std::collections::HashMap;

use crate::player::Player;

#[derive(Debug, Default, Clone)]
pub struct ScoreBoard {
    scores: HashMap<Player, usize>,
}

#[derive(Default)]
pub struct ScoreBoardBuilder {
    scores: HashMap<Player, usize>,
}

impl ScoreBoardBuilder {
    pub fn add_player(mut self, player: Player) -> ScoreBoardBuilder {
        self.scores.insert(player, 0);
        self
    }

    pub fn build(self, score_board: &mut ScoreBoard) {
        score_board.scores = self.scores;
    }
}
