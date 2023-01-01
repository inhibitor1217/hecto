use std::io::{self, Stdout};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{size, Clear, ClearType},
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    stdout: Stdout,
    size: Size,
}

pub type Error = io::Error;
pub type Result<T> = std::result::Result<T, Error>;

impl Terminal {
    pub fn new() -> Result<Self> {
        let (width, height) = size()?;
        Ok(Self {
            stdout: io::stdout(),
            size: Size { width, height },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> Result<()> {
        execute!(self.stdout, MoveTo(x, y))
    }

    pub fn clear(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All))
    }
}
