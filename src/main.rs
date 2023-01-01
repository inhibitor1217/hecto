mod editor;

use std::error::Error;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use editor::{Editor, EditorError};

type MainError = Box<dyn Error>;

fn die(e: EditorError) -> ! {
    panic!("{}", e);
}

fn main() -> Result<(), MainError> {
    enable_raw_mode()?;

    let editor = Editor::new();
    editor.run().map_err(die)?;

    disable_raw_mode()?;

    Ok(())
}
