use std::io::{self, Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode as CrossTermKeyCode, KeyModifiers as CrossTermKeyModifiers},
    execute,
    terminal::{size, Clear, ClearType},
};

use crate::position::Position;

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

pub type KeyModifiers = CrossTermKeyModifiers;
pub type KeyCode = CrossTermKeyCode;
pub type Key = (KeyModifiers, KeyCode);

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

    pub fn show_cursor(&mut self) -> Result<()> {
        execute!(self.stdout, Show)
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        execute!(self.stdout, Hide)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn move_cursor_to(&mut self, position: &Position) -> Result<()> {
        execute!(self.stdout, MoveTo(position.x as u16, position.y as u16))
    }

    pub fn read_key() -> Result<Key> {
        loop {
            if let Event::Key(event) = read()? {
                return Ok((event.modifiers, event.code));
            }
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    pub fn clear_line(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::CurrentLine))
    }

    pub fn draw_line(&mut self, line: &str) -> Result<()> {
        writeln!(self.stdout, "{line}")
    }
}
