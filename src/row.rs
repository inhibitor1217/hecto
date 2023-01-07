use std::cmp::{max, min};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

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

impl ToString for Row {
    fn to_string(&self) -> String {
        self.string.clone()
    }
}

impl Row {
    pub fn new() -> Row {
        Row {
            string: String::new(),
            len: 0,
        }
    }

    pub fn render(&self, start: usize, end: usize) -> String {
        let end = max(start, min(end, self.len()));
        if start > end {
            return String::new();
        }
        self.string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect()
    }

    pub fn to_position(&self, raw_pos: usize) -> usize {
        let mut pos = 0;
        let mut width = 0;
        for ch in self.string.graphemes(true) {
            width += ch.len();
            if width > raw_pos {
                break;
            }
            pos += 1;
        }
        pos
    }

    pub fn to_raw_position(&self, pos: usize) -> usize {
        self.string
            .graphemes(true)
            .take(pos)
            .map(UnicodeWidthStr::width)
            .sum()
    }

    pub fn insert_at(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            let mut s = String::new();
            for (i, ch) in self.string.graphemes(true).enumerate() {
                if i == at {
                    s.push(c);
                }
                s.push_str(ch);
            }
            self.string = s;
        }

        self.len += 1;
    }

    pub fn delete_at(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }

        let mut s = String::new();
        for (i, ch) in self.string.graphemes(true).enumerate() {
            if i != at {
                s.push_str(ch);
            }
        }
        self.string = s;
        self.len -= 1;
    }

    pub fn append(&mut self, row: &Row) {
        self.string.push_str(&row.string);
        self.len += row.len();
    }

    pub fn split_at(&mut self, at: usize) -> (Row, Row) {
        let left = &self.string.graphemes(true).take(at).collect::<String>()[..];
        let right = &self.string.graphemes(true).skip(at).collect::<String>()[..];
        (
            Row::from(left),
            Row::from(right),
        )
    }

    pub fn search(&self, query: &str, after: usize) -> Option<usize> {
        let substr = &self.string.graphemes(true)
            .skip(after)
            .collect::<String>()[..];

        substr
            .find(query)
            .map(|raw_pos| self.to_position(raw_pos))
            .map(|pos| pos + after)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
