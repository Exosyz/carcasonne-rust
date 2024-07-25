#[derive(Debug, Default, Copy, Clone)]
pub enum SideKind {
    #[default]
    Meadow,
    Town,
    Road,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Side {
    section: usize,
    kind: SideKind,
}

impl Side {}

#[derive(Default)]
pub struct SideBuilder {
    section: usize,
    kind: SideKind,
}

impl SideBuilder {
    pub fn section(&mut self, section: usize) -> &mut Self {
        self.section = section;
        self
    }

    pub fn kind(&mut self, kind: SideKind) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn build(self, side: &mut Side) {
        side.section = self.section;
        side.kind = self.kind;
    }
}
