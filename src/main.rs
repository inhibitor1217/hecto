use std::{
    error::Error,
    io::{self, Read, Write},
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

type MainError = Box<dyn Error>;

fn main() -> Result<(), MainError> {
    enable_raw_mode()?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for byte in stdin.bytes() {
        let ch = byte.unwrap() as char;
        write!(stdout, "{}", ch)?;

        if ch == 'q' {
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}
