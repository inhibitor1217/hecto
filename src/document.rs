use std::{fs, io};

use crate::{row::Row, position::Position};

#[derive(Default)]
pub struct Document {
    pub filename: Option<String>,
    rows: Vec<Row>,
    dirty: bool,
}

impl ToString for Document {
    fn to_string(&self) -> String {
        self.rows
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Document {
    pub fn new() -> Self {
        Self {
            filename: None,
            rows: vec![],
            dirty: true,
        }
    }

    pub fn open(filename: &str) -> Result<Self, io::Error> {
        let content = fs::read_to_string(filename)?;
        Ok(Self {
            filename: Some(filename.to_string()),
            rows: content.lines().map(Row::from).collect(),
            dirty: false,
        })
    }

    pub fn save(&mut self) -> Result<(), io::Error> {
        if let Some(filename) = &self.filename {
            fs::write(filename, self.to_string())?;
            self.dirty = false;
        } else {
            // TODO return error and prompt the user to input a filename
        }

        Ok(())
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn row_mut(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
    }

    pub fn width_at(&self, position: &Position) -> usize {
        self.row(position.y)
            .map(Row::len)
            .unwrap_or_default()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn insert_at(&mut self, position: &Position, c: char) {
        if let Some(row) = self.row_mut(position.y) {
            row.insert_at(position.x, c);
            self.dirty = true;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
