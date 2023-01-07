use crate::{color::Color};

pub struct Highlight {
    pub start: usize,
    pub end: usize,
    pub color: Option<Color>,
    pub background_color: Option<Color>,
}

impl Highlight {
    pub fn new(
        start: usize,
        end: usize,
        color: Option<Color>,
        background_color: Option<Color>,
    ) -> Self {
        Self {
            start,
            end,
            color,
            background_color,
        }
    }
}

pub trait Highlighter {
    fn highlight(&self, line: &str) -> Vec<Highlight>;
}
