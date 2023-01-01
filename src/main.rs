#![warn(clippy::all, clippy::pedantic)]
mod editor;

use std::error;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use editor::Editor;

type MainError = Box<dyn error::Error>;

fn die(e: &editor::Error) {
    panic!("{}", e);
}

fn main() -> Result<(), MainError> {
    enable_raw_mode()?;

    let mut editor = Editor::new();
    if let Err(e) = editor.run() {
        die(&e);
    }

    disable_raw_mode()?;

    Ok(())
}
