#![warn(clippy::all, clippy::pedantic)]
mod editor;

use std::error::Error;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use editor::Editor;

type MainError = Box<dyn Error>;

fn main() -> Result<(), MainError> {
    enable_raw_mode()?;

    let mut editor = Editor::new();
    editor.run();

    disable_raw_mode()?;

    Ok(())
}
