use crate::position::Position;

pub struct Hit {
    pub query: String,
    pub position: Position,
    pub highlight: (Position, Position),
}

impl Hit {
    pub fn new(
        query: String,
        from: Position,
        to: Position,
    ) -> Self {
        Self {
            query,
            position: from,
            highlight: (from, to),
        }
    }
}
