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
        let screen_width = width as usize;
        let screen_height = (height as usize).saturating_sub(1);
        let Position { x: offx, y: offy } = self.offset;
        let welcome_message_row = screen_height / 3;

        for row_idx in 0..screen_height {
            let row_idx = row_idx + offy;

            self.terminal.clear_line()?;

            let line = if let Some(row) = self.document.row(row_idx) {
                format!("{}\r", row.render(offx, offx + screen_width))
            } else if self.document.is_empty() && row_idx == welcome_message_row {
                Editor::welcome_message(screen_width)
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

        self.sanitize_position();
        self.scroll();

        Ok(())
    }

    fn sanitize_position(&mut self) {
        let doc_height = self.document.height();
        if self.position.y > doc_height {
            self.position.y = doc_height;
        }
        let width = self.document.width_at(&self.position);
        if self.position.x > width {
            self.position.x = width;
        }
    }

    fn scroll(&mut self) {
        let Size { width, height } = *self.terminal.size();
        let screen_width = width as usize;
        let screen_height = (height as usize).saturating_sub(1);

        let Position { x: posx, y: posy } = self.position;
        let Position { x: mut offx, y: mut offy } = self.offset;

        if posx < offx {
            offx = posx;
        }
        if posy < offy {
            offy = posy;
        }
        if posx >= offx + screen_width {
            offx += 1;
        }
        if posy >= offy + screen_height {
            offy += 1;
        }

        self.offset = Position::at(offx, offy);
    }

    fn die(&mut self, e: &Error) {
        self.terminal.clear().unwrap(); // We cannot handle error here, already dying
        panic!("{}", e);
    }
}
