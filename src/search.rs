use crate::position::Position;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hit {
    pub position: Position,
    pub highlight: (Position, Position),
}

impl Hit {
    pub fn new(from: Position, to: Position) -> Self {
        Self {
            position: from,
            highlight: (from, to),
        }
    }
}
