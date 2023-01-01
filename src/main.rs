use std::{
    error::Error,
    io::{self, Read},
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

type MainError = Box<dyn Error>;

fn main() -> Result<(), MainError> {
    enable_raw_mode()?;

    for byte in io::stdin().bytes() {
        let ch = byte.unwrap() as char;
        println!("{}", ch);

        if ch == 'q' {
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}
