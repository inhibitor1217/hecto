use std::io;

use crossterm::terminal::size;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

pub type Error = io::Error;
pub type Result<T> = std::result::Result<T, Error>;

impl Terminal {
    pub fn new() -> Result<Self> {
        let (width, height) = size()?;
        Ok(Self {
            size: Size { width, height },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
