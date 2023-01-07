#![warn(clippy::all, clippy::pedantic)]
mod color;
mod document;
mod editor;
mod highlight;
mod position;
mod renderer;
mod row;
mod search;
mod terminal;

use std::{env, error::Error};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use editor::Editor;
use terminal::Terminal;

type MainError = Box<dyn Error>;

fn main() -> Result<(), MainError> {
    enable_raw_mode()?;

    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { Some(&args[1]) } else { None };

    let mut terminal = Terminal::new()?;
    let mut editor = if let Some(filename) = filename {
        Editor::from_file(&mut terminal, filename)
    } else {
        Editor::new(&mut terminal)
    };
    editor.run();

    disable_raw_mode()?;

    Ok(())
}
