use std::io::{self, Write};

use crossterm::event::{read, Event, KeyCode, KeyModifiers};

pub type Error = io::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Editor {}

impl Editor {
    pub fn new() -> Editor {
        Self {}
    }

    pub fn run(&self) -> Result<()> {
        let mut stdout = io::stdout();

        loop {
            if let Event::Key(event) = read()? {
                match (event.modifiers, event.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                    (_, KeyCode::Char(c)) => write!(stdout, "{:?} ({c}) \r\n", c as u8)?,
                    (_, code) => write!(stdout, "{code:?} \r\n")?,
                }
            }
        }

        Ok(())
    }
}
