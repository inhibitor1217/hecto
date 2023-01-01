use std::cmp::{max, min};

pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(str: &str) -> Self {
        Self {
            string: String::from(str),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = max(start, min(end, self.string.len()));
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}
