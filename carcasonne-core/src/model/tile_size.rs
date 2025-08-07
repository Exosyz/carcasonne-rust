pub enum TileSize {
    Small,
    Medium,
    Large,
    XtraLarge,
}

impl TileSize {
    pub fn get_size(&self) -> usize {
        match self {
            TileSize::Small => 5,
            TileSize::Medium => 7,
            TileSize::Large => 9,
            TileSize::XtraLarge => 11,
        }
    }

    pub fn increase_size(&mut self) {
        match self {
            TileSize::Small => *self = TileSize::Medium,
            TileSize::Medium => *self = TileSize::Large,
            TileSize::Large => *self = TileSize::XtraLarge,
            TileSize::XtraLarge => *self = TileSize::XtraLarge,
        }
    }

    pub fn decrease_size(&mut self) {
        match self {
            TileSize::Small => *self = TileSize::Small,
            TileSize::Medium => *self = TileSize::Small,
            TileSize::Large => *self = TileSize::Medium,
            TileSize::XtraLarge => *self = TileSize::Large,
        }
    }
}
