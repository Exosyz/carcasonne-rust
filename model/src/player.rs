#[derive(Debug, Default, Eq, Hash, PartialEq)]
pub struct Player {
    name: String,
}
#[derive(Default, Clone)]
pub struct PlayerBuilder {
    name: String,
    quantity: usize,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&mut self, name: String) -> &Self {
        self.name = name;
        self
    }

    pub fn quantity(&mut self, quantity: usize) -> &Self {
        self.quantity = quantity;
        self
    }

    pub fn build(&self) -> Player {
        Player {
            name: self.name.clone(),
        }
    }
}
