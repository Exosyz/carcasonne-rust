use crate::color::Color;
use crate::model::player::Player;

#[derive(Default)]
pub struct MainMenuContext {
    players: Vec<Player>,
    available_colors: Vec<Color>,
    max_player: usize,
}

impl MainMenuContext {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            available_colors: vec![Color::Red, Color::Blue, Color::Green, Color::Yellow],
            max_player: 4,
        }
    }

    pub fn try_add_player(&mut self, name: String) -> Result<(), String> {
        if self.players.len() >= self.max_player {
            return Err("Max player reached".into());
        }

        if self.players.iter().filter(|p| p.name == name).count() > 0 {
            return Err("Player already exists".into());
        }

        match self.available_colors.pop() {
            Some(color) => {
                self.players.push(Player::new(name, color));
                Ok(())
            }
            None => Err("No more colors available".into()),
        }
    }

    pub fn remove_player(&mut self, name: String) -> Result<(), String> {
        match self.players.iter().position(|p| p.name == name) {
            Some(index) => {
                let player = self.players.remove(index);
                self.available_colors.push(player.color);
                Ok(())
            }
            None => Err("Player does not exist".into()),
        }
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
}
