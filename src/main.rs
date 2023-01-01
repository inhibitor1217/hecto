use std::{
    error::Error,
    io::{self, Write},
};

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

type MainError = Box<dyn Error>;

fn _die(e: io::Error) {
    panic!("{}", e);
}

fn main() -> Result<(), MainError> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    loop {
        match read()? {
            Event::Key(event) => match (event.modifiers, event.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                (_, KeyCode::Char(c)) => write!(stdout, "{:?} ({}) \r\n", c as u8, c)?,
                (_, code) => write!(stdout, "{:?} \r\n", code)?,
            },
            _ => {}
        }
    }

    disable_raw_mode()?;

    Ok(())
}
