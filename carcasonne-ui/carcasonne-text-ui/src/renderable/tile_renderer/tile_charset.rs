#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileCharset {
    None,
    Abbey,
    VerticalRoad,
    HorizontalRoad,
    Crossroad,
    Town,
    Shield,
}

impl From<TileCharset> for char {
    fn from(value: TileCharset) -> Self {
        match value {
            TileCharset::None => '.',
            TileCharset::Abbey => 'A',
            TileCharset::HorizontalRoad => '═',
            TileCharset::VerticalRoad => '║',
            TileCharset::Town => '#',
            TileCharset::Crossroad => '╬',
            TileCharset::Shield => 'S',
        }
    }
}
