pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn at(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::at(0, 0)
    }
}
