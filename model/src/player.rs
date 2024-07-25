#[derive(Debug, Default, Eq, Hash, PartialEq, Clone)]
pub struct Player {
    name: String,
}
#[derive(Default, Clone)]
pub struct PlayerBuilder {
    name: String,
    quantity: usize,
}

impl PlayerBuilder {
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn quantity(&mut self, quantity: usize) -> &mut Self {
        self.quantity = quantity;
        self
    }

    pub fn build(self, player: &mut Player) {
        player.name = self.name;
    }
}
