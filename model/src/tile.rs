use crate::side::Side;

#[derive(Debug, Default, Copy, Clone)]
pub enum TileExtension {
    #[default]
    None,
    TownShield(usize),
    Abbey,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Tile {
    north: Side,
    south: Side,
    east: Side,
    west: Side,
    tile_extension: TileExtension,
}

impl Tile {}

#[derive(Default)]
pub struct TileBuilder {
    north: Side,
    south: Side,
    east: Side,
    west: Side,
    tile_extension: TileExtension,
}

impl TileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn north(mut self, north: Side) -> Self {
        self.north = north;
        self
    }

    pub fn south(mut self, south: Side) -> Self {
        self.south = south;
        self
    }

    pub fn east(mut self, east: Side) -> Self {
        self.east = east;
        self
    }

    pub fn west(mut self, west: Side) -> Self {
        self.west = west;
        self
    }

    pub fn tile_extension(mut self, tile_extension: TileExtension) -> Self {
        self.tile_extension = tile_extension;
        self
    }

    pub fn build(self) -> Tile {
        Tile {
            north: self.north,
            south: self.south,
            east: self.east,
            west: self.west,
            tile_extension: self.tile_extension,
        }
    }
}
