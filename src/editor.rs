use std::io::{self, Stdout, Write};

use crate::{
    document::Document,
    position::Position,
    terminal::{Key, KeyCode, KeyModifiers, Terminal},
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
            self.process_key(key);
            self.sanitize_position();
            self.scroll();
        }
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        let window_width = self.window_width();
        let window_height = self.window_height();
        let Position { x: offset_x, y: offset_y } = self.offset;
        let welcome_message_row = window_height / 3;

        for row_idx in 0..window_height {
            let row_idx = row_idx + offset_y;

            self.terminal.clear_line()?;

            let line = if let Some(row) = self.document.row(row_idx) {
                format!("{}\r", row.render(offset_x, offset_x + window_width))
            } else if self.document.is_empty() && row_idx == welcome_message_row {
                Editor::welcome_message(window_width)
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

    fn process_key(&mut self, key: Key) {
        match key {
            // In most cases we will use ctrl+q for quitting,
            // but apparently VSCode skips sending ctrl+q to the terminal.
            (_, KeyCode::Char('q')) => self.quit = true,
            (KeyModifiers::NONE, KeyCode::Left) => self.position.x = self.position.x.saturating_sub(1),
            (KeyModifiers::NONE, KeyCode::Right) => self.position.x += 1,
            (KeyModifiers::NONE, KeyCode::Up) => self.position.y = self.position.y.saturating_sub(1),
            (KeyModifiers::NONE, KeyCode::Down) => self.position.y += 1,
            _ => {}
        }
    }

    fn sanitize_position(&mut self) {
        let doc_height = self.document.height();
        let Position { x: mut position_x, y: mut position_y } = self.position;

        if position_y >= doc_height {
            position_y = doc_height - 1;
        }

        let width = self.document.width_at(&self.position);
        if position_x > width {
            position_x = width;
        }

        self.position = Position::at(position_x, position_y);
    }

    fn scroll(&mut self) {
        let window_width = self.window_width();
        let window_height = self.window_height();

        let Position { x: position_x, y: position_y } = self.position;
        let Position { x: mut offset_x, y: mut offset_y } = self.offset;

        if position_x < offset_x {
            offset_x = position_x;
        }
        if position_y < offset_y {
            offset_y = position_y;
        }
        if position_x >= offset_x + window_width {
            offset_x = position_x - window_width + 1;
        }
        if position_y >= offset_y + window_height {
            offset_y = position_y - window_height + 1;
        }

        self.offset = Position::at(offset_x, offset_y);
    }

    fn window_width(&self) -> usize {
        self.terminal.size().width as usize
    }

    fn window_height(&self) -> usize {
        self.terminal.size().height as usize - 1 // Last line is for status bar
    }

    fn die(&mut self, e: &Error) {
        self.terminal.clear().unwrap(); // We cannot handle error here, already dying
        panic!("{}", e);
    }
}
