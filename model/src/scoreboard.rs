use std::collections::HashMap;

use crate::player::Player;

#[derive(Debug, Default)]
pub struct ScoreBoard {
    scores: HashMap<Player, usize>,
}

#[derive(Default)]
pub struct ScoreBoardBuilder {
    scores: HashMap<Player, usize>,
}

impl ScoreBoardBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn addPlayer(mut self, player: Player) -> ScoreBoardBuilder {
        self.scores.insert(player, 0);
        self
    }

    pub fn build(self) -> ScoreBoard {
        ScoreBoard {
            scores: self.scores,
        }
    }
}
