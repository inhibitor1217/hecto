use std::{fs, io, fmt::Display};

use crate::{row::Row, position::Position};

#[derive(Debug)]
pub enum OperationError {
    Position,
    EmptyFilename,
    IO(io::Error),
}

impl Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position => write!(f, "Invalid position"),
            Self::EmptyFilename => write!(f, "Empty filename"),
            Self::IO(err) => write!(f, "IO error: {err}"),
        }
    }
}

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

    pub fn open(filename: &str) -> Result<Self, OperationError> {
        let content = fs::read_to_string(filename).map_err(OperationError::IO)?;
        Ok(Self {
            filename: Some(filename.to_string()),
            rows: content.lines().map(Row::from).collect(),
            dirty: false,
        })
    }

    pub fn save(&mut self) -> Result<(), OperationError> {
        if let Some(filename) = &self.filename {
            fs::write(filename, self.to_string()).map_err(OperationError::IO)?;
            self.dirty = false;
        } else {
            return Err(OperationError::EmptyFilename);
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

    pub fn translate(&self, position: &Position, offset: &Position) -> Position {
        let raw_x = self.row(position.y)
            .map(|r| r.to_raw_position(position.x))
            .unwrap_or_default();

        let raw_y = position.y;

        Position::at(raw_x, raw_y).diff(offset)
    }

    pub fn insert_at(&mut self, position: &Position, c: char) -> Result<(), OperationError> {
        if let Some(row) = self.row_mut(position.y) {
            row.insert_at(position.x, c);
            self.dirty = true;
            return Ok(());
        }

        Err(OperationError::Position)
    }

    pub fn delete_at(&mut self, position: &Position) -> Result<(), OperationError> {
        if let Some(row) = self.row_mut(position.y) {
            if row.len() > 0 {
                row.delete_at(position.x);
                self.dirty = true;
                return Ok(());
            }
        }

        Err(OperationError::Position)
    }

    pub fn append_row(&mut self) {
        self.rows.push(Row::new());
        self.dirty = true;
    }

    pub fn merge_row(&mut self, position: &Position) -> Result<(), OperationError> {
        if let [prev, cur] = &mut self.rows[position.y.saturating_sub(1)..=position.y] {
            prev.append(cur);
            self.rows.remove(position.y);
            self.dirty = true;
            return Ok(());
        }

        Err(OperationError::Position)
    }

    pub fn split_row(&mut self, position: &Position) -> Result<(), OperationError> {
        if let Some(row) = self.row_mut(position.y) {
            let (left, right) = row.split_at(position.x);
            self.rows[position.y] = left;
            self.rows.insert(position.y + 1, right);
            self.dirty = true;
            return Ok(());
        }

        Err(OperationError::Position)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
