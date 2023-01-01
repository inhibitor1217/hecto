#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod position;
mod terminal;

use std::error::Error;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use editor::Editor;
use terminal::Terminal;

type MainError = Box<dyn Error>;

fn main() -> Result<(), MainError> {
    enable_raw_mode()?;

    let mut terminal = Terminal::new()?;
    let mut editor = Editor::new(&mut terminal);
    editor.run();

    disable_raw_mode()?;

    Ok(())
}
