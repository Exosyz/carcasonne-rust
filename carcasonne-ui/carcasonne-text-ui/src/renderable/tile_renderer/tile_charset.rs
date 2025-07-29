#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileCharset {
    None,
    Abbey,
    VerticalRoad,
    HorizontalRoad,
    Crossroad,
    EndRoad,
    CornerTopLeft,
    CornerTopRight,
    CornerBottomLeft,
    CornerBottomRight,
    Town,
    Shield,
}

impl From<TileCharset> for char {
    fn from(value: TileCharset) -> Self {
        match value {
            TileCharset::None => '.',
            TileCharset::Abbey => 'A',
            TileCharset::Town => '#',
            TileCharset::HorizontalRoad => '═',
            TileCharset::VerticalRoad => '║',
            TileCharset::Crossroad => '╬',
            TileCharset::EndRoad => '◻',
            TileCharset::Shield => 'S',
            TileCharset::CornerTopLeft => '╝',
            TileCharset::CornerTopRight => '╚',
            TileCharset::CornerBottomLeft => '╗',
            TileCharset::CornerBottomRight => '╔',
        }
    }
}
