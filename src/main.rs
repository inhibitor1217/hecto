use std::{
    error::Error,
    io::{self, Read, Write},
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

type MainError = Box<dyn Error>;

fn to_ctrl_byte(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

fn die(e: io::Error) {
    panic!("{}", e);
}

fn main() -> Result<(), MainError> {
    enable_raw_mode()?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for byte in stdin.bytes() {
        match byte {
            Ok(b) => {
                let c = b as char;

                if c.is_control() {
                    write!(stdout, "{:?} \r\n", b)?;
                } else {
                    write!(stdout, "{:?} ({})\r\n", b, c)?;
                }

                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(e) => die(e),
        }
    }

    disable_raw_mode()?;

    Ok(())
}
