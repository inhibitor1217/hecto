use crate::{color::Color, row::Row};

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

pub fn highlight_row(row: &Row, highlighters: &[Box<dyn Highlighter>]) -> Vec<Highlight> {
    let mut highlights = Vec::new();
    for highlighter in highlighters {
        let line = row.render(0, row.len());
        highlights.extend(highlighter.highlight(line.as_str()));
    }
    highlights
}
