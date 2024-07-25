use crate::side::{Side, SideBuilder};

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

#[derive(Default, Clone)]
pub struct TileBuilder {
    north: Side,
    south: Side,
    east: Side,
    west: Side,
    tile_extension: TileExtension,
}

impl TileBuilder {
    fn build_side(side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder) -> Side {
        let mut builder = SideBuilder::default();
        side_builder(&mut builder);

        let mut side = Side::default();
        builder.build(&mut side);
        side
    }

    pub fn north(
        &mut self,
        side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder,
    ) -> &mut Self {
        self.north = Self::build_side(side_builder);
        self
    }

    pub fn south(
        &mut self,
        side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder,
    ) -> &mut Self {
        self.south = Self::build_side(side_builder);
        self
    }

    pub fn east(
        &mut self,
        side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder,
    ) -> &mut Self {
        self.east = Self::build_side(side_builder);
        self
    }

    pub fn west(
        &mut self,
        side_builder: impl FnOnce(&mut SideBuilder) -> &mut SideBuilder,
    ) -> &mut Self {
        self.west = Self::build_side(side_builder);
        self
    }

    pub fn tile_extension(&mut self, tile_extension: TileExtension) -> &mut Self {
        self.tile_extension = tile_extension;
        self
    }

    pub fn build(self, tile: &mut Tile) {
        tile.north = self.north;
        tile.south = self.south;
        tile.east = self.east;
        tile.west = self.west;
        tile.tile_extension = self.tile_extension;
    }
}
