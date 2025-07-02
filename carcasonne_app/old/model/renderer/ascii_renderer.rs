use crate::behavior::side_behaviour::SideBehavior;
use crate::side::SideKind;
use crate::tile::{Tile, TileExtension};
use std::collections::HashMap;

mod game_ascii_renderer;
mod tile_ascii_renderer;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum RenderChar {
    Meadow,
    Town,
    RoadVertical,
    RoadHorizontal,
    RoadCrossing,
    RoadCorner,
    Placeholder,
    Custom(char),
}

#[derive(Clone, Debug)]
pub struct AsciiPalette {
    pub road_vertical: char,
    pub road_horizontal: char,
    pub road_crossing: char,
    pub road_corner: char,
    pub meadow: char,
    pub town: char,
    pub placeholder: char,
}

impl Default for AsciiPalette {
    fn default() -> Self {
        Self {
            road_vertical: '|',
            road_horizontal: '-',
            road_crossing: '+',
            road_corner: 'L',
            meadow: '.',
            town: '#',
            placeholder: '@',
        }
    }
}

impl RenderChar {
    pub fn with_palette(self, palette: &AsciiPalette) -> char {
        match self {
            RenderChar::RoadVertical => palette.road_vertical,
            RenderChar::RoadHorizontal => palette.road_horizontal,
            RenderChar::RoadCrossing => palette.road_crossing,
            RenderChar::RoadCorner => palette.road_corner,
            RenderChar::Meadow => palette.meadow,
            RenderChar::Town => palette.town,
            RenderChar::Placeholder => palette.placeholder,
            RenderChar::Custom(c) => c,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct AsciiRenderer {
    pub tile_size: usize,
    pub behaviors: HashMap<SideKind, Box<dyn SideBehavior>>,
    pub palette: Option<AsciiPalette>,
}

impl AsciiRenderer {
    pub(crate) fn get_behavior(&self, kind: SideKind) -> Option<&dyn SideBehavior> {
        self.behaviors.get(&kind).map(|b| &**b)
    }

    pub(crate) fn render_center(
        &self,
        tile: &Tile,
        sections: &HashMap<usize, Vec<RenderChar>>,
    ) -> Option<RenderChar> {
        match tile.tile_extension {
            TileExtension::Abbey => return Some(RenderChar::Custom('A')),
            TileExtension::TownShield => return Some(RenderChar::Custom('S')),
            TileExtension::None => (),
        }
        /*
        let tree = Some(
            *sections
                .iter()
                .max_by_key(|(_, v)| v.len())
                .map(|(_, v)| v)
                .into_iter()
                .fold(HashMap::new(), |mut counts, item| {
                    *counts.entry(item).or_insert(0) += 1;
                    counts
                })
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(val, _)| val.first())??,
        );
            // TODO c'est la merde peut etre voir pour simplifer
        match sections.keys().len() {
            1 => Some(*sections.iter().next().map(|(_, c)| c.first())??),
            2 => match (tile.north.kind, tile.west.kind,tile.south.kind, tile.east.kind) {
                (SideKind::Town, SideKind::Town, SideKind::Town, SideKind::Town) => None,
                _ => None,
            },
            3 => None,
            4 => match (tile.north.kind, tile.west.kind,tile.south.kind, tile.east.kind) {
                (SideKind::Road, SideKind::Road, SideKind::Road, SideKind::Road) => Some(RenderChar::RoadCrossing),
                (SideKind::Town, SideKind::Town, _, _) => None,
                ( _, _, SideKind::Town, SideKind::Town) => None,
                (SideKind::Town, _, SideKind::Town, _) => None,
                (_, SideKind::Town, _, SideKind::Town) => None,
                (SideKind::Town,_,  _, SideKind::Town) => None,
                _ => Some(RenderChar::Placeholder),
            },
            _ => None,
        }*/
        None
    }
}
