use std::io::{self, Stdout, Write};

use crate::{
    document::Document,
    position::Position,
    terminal::{Key, KeyCode, KeyModifiers, Size, Terminal},
};

type Error = io::Error;
type Result<T> = std::result::Result<T, Error>;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor<'a> {
    stdout: Stdout,
    terminal: &'a mut Terminal,
    document: Document,
    position: Position,
    offset: Position,
    quit: bool,
}

impl<'a> Editor<'a> {
    pub fn new(terminal: &'a mut Terminal) -> Self {
        Self {
            stdout: io::stdout(),
            terminal,
            document: Document::new(),
            position: Position::zero(),
            offset: Position::zero(),
            quit: false,
        }
    }

    pub fn from_file(terminal: &'a mut Terminal, filename: &'a str) -> Self {
        Self {
            stdout: io::stdout(),
            terminal,
            position: Position::zero(),
            document: Document::open(filename).unwrap_or_default(),
            offset: Position::zero(),
            quit: false,
        }
    }

    pub fn run(&mut self) {
        if let Err(e) = self.run_loop() {
            self.die(&e);
        }
    }

    fn run_loop(&mut self) -> Result<()> {
        loop {
            self.terminal.hide_cursor()?;

            self.terminal.move_cursor_to(&Position::zero())?;
            self.draw()?;
            self.terminal
                .move_cursor_to(&self.position.diff(&self.offset))?;
            self.terminal.show_cursor()?;

            if self.quit {
                self.terminal.clear()?;
                self.terminal.move_cursor_to(&Position::zero())?;
                write!(self.stdout, "Goodbye! :)\r\n")?;
                break;
            }

            let key = Terminal::read_key()?;
            self.process_key(key)?;
        }
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        let Size { width, height } = *self.terminal.size();
        let width = width as usize;
        let height = height as usize;
        let welcome_message_row = height / 3;

        for row_idx in 0..height - 1 {
            let row_idx = row_idx + self.offset.y;

            self.terminal.clear_line()?;

            let line = if let Some(row) = self.document.row(row_idx) {
                format!("{}\r", row.render(self.offset.x, self.offset.x + width))
            } else if self.document.is_empty() && row_idx == welcome_message_row {
                Editor::welcome_message(width)
            } else {
                Editor::empty_line()
            };
            writeln!(self.stdout, "{line}")?;
        }
        Ok(())
    }

    fn empty_line() -> String {
        String::from("~\r")
    }

    fn welcome_message(width: usize) -> String {
        let msg = format!("Hecto editor -- version {VERSION}");
        let len = msg.len();
        let padding = width.saturating_sub(len) / 2;
        let pad = " ".repeat(padding.saturating_sub(1));
        format!("~{pad}{}\r", &msg[..len])
    }

    fn process_key(&mut self, key: Key) -> Result<()> {
        // TODO
        Ok(())
    }

    fn die(&mut self, e: &Error) {
        self.terminal.clear().unwrap(); // We cannot handle error here, already dying
        panic!("{}", e);
    }
}
