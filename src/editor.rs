use std::io::{self, Stdout, Write};

use crossterm::event::{read, Event, KeyCode, KeyModifiers};

pub type Error = io::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Editor {
    stdout: Stdout,
}

impl Editor {
    pub fn new() -> Editor {
        Self {
            stdout: io::stdout(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let key = Self::read_key()?;
            self.process_key(key)?;
        }
    }

    fn read_key() -> Result<(KeyModifiers, KeyCode)> {
        loop {
            if let Event::Key(event) = read()? {
                return Ok((event.modifiers, event.code));
            }
        }
    }

    fn process_key(&mut self, key: (KeyModifiers, KeyCode)) -> Result<()> {
        match key {
            (KeyModifiers::CONTROL, KeyCode::Char('q')) => panic!("Quit program."),
            (_, KeyCode::Char(c)) => write!(self.stdout, "{:?} ({c}) \r\n", c as u8),
            (_, code) => write!(self.stdout, "{code:?} \r\n"),
        }
    }
}
