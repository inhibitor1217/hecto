use std::cmp::{max, min};

use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(str: &str) -> Self {
        Self {
            string: String::from(str),
            len: str.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = max(start, min(end, self.len()));
        self.string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect()
    }

    pub fn insert_at(&mut self, at: usize, c: char) {
        self.string.insert(at, c);
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
