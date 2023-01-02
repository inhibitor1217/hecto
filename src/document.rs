use std::{fs, io};

use crate::{row::Row, position::Position};

#[derive(Default)]
pub struct Document {
    pub filename: Option<String>,
    rows: Vec<Row>,
}

impl Document {
    pub fn new() -> Self {
        Self { filename: None, rows: vec![] }
    }

    pub fn open(filename: &str) -> Result<Self, io::Error> {
        let content = fs::read_to_string(filename)?;
        Ok(Self {
            filename: Some(filename.to_string()),
            rows: content.lines().map(Row::from).collect(),
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn width_at(&self, position: &Position) -> usize {
        self.row(position.y)
            .map(Row::len)
            .unwrap_or_default()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
