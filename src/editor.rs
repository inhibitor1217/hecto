use std::io::{self, Write};

use crossterm::event::{read, Event, KeyCode, KeyModifiers};

pub type EditorError = io::Error;
pub type Result<T> = std::result::Result<T, EditorError>;

pub struct Editor {}

impl Editor {
    pub fn new() -> Editor {
        Editor {}
    }

    pub fn run(&self) -> Result<()> {
        let mut stdout = io::stdout();

        Ok(loop {
            match read()? {
                Event::Key(event) => match (event.modifiers, event.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                    (_, KeyCode::Char(c)) => write!(stdout, "{:?} ({}) \r\n", c as u8, c)?,
                    (_, code) => write!(stdout, "{:?} \r\n", code)?,
                },
                _ => {}
            }
        })
    }
}
