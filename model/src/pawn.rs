enum PawnKind {
    Basic(usize),
}

impl Default for PawnKind {
    fn default() -> Self {
        PawnKind::Basic(1)
    }
}

#[derive(Default)]
pub struct Pawn {
    kind: PawnKind,
}
