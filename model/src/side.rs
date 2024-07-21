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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn section(mut self, section: usize) -> Self {
        self.section = section;
        self
    }

    pub fn kind(mut self, kind: SideKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn build(self) -> Side {
        Side {
            section: self.section,
            kind: self.kind,
        }
    }
}
