use std::{io::{self, Stdout, Write}, time::Instant};

use crate::{
    document::Document,
    position::Position,
    terminal::{Key, KeyCode, KeyModifiers, Terminal, Color},
};

type Error = io::Error;
type Result<T> = std::result::Result<T, Error>;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const STATUS_FG_COLOR: Color = Color::Rgb { r: 63, g: 63, b: 63 };
const STATUS_BG_COLOR: Color = Color::Rgb { r: 191, g: 191, b: 191 };

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn new(text: String) -> Self {
        Self {
            text,
            time: Instant::now(),
        }
    }

    fn is_recent(&self) -> bool {
        self.time.elapsed().as_secs() < 5
    }
}

pub struct Editor<'a> {
    stdout: Stdout,
    terminal: &'a mut Terminal,
    document: Document,
    position: Position,
    offset: Position,
    status_message: StatusMessage,
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
            status_message: StatusMessage::new(String::from("ctrl-q - quit")),
            quit: false,
        }
    }

    pub fn from_file(terminal: &'a mut Terminal, filename: &'a str) -> Self {
        let mut status_message = StatusMessage::new(String::from("ctrl-q - quit"));
        let document = Document::open(filename)
            .unwrap_or_else(|_| {
                status_message = StatusMessage::new(format!("Error opening file: {filename}"));
                Document::new()
            });

        Self {
            stdout: io::stdout(),
            terminal,
            position: Position::zero(),
            document,
            offset: Position::zero(),
            status_message,
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
            self.draw_window()?;
            self.draw_status_bar()?;
            self.draw_message_bar()?;
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

    fn draw_window(&mut self) -> Result<()> {
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
            
            self.terminal.draw_line(&line, None, None)?;
        }
        Ok(())
    }

    fn draw_status_bar(&mut self) -> Result<()> {
        let status_bar_pos = Position::at(0, self.window_height());
        self.terminal.move_cursor_to(&status_bar_pos)?;

        let mut filename = self.document.filename.clone().unwrap_or(String::from("[New File]"));
        filename.truncate(20);
        let file_length = self.document.height();
        let file_status = format!("{filename} - {file_length} lines");

        let pos_status = format!("{}/{}", self.position.y + 1, file_length);
        
        // Align file_status to left, pos_status to right
        let pad = " ".repeat(self.window_width().saturating_sub(file_status.len() + pos_status.len()));
        let status_line = format!("{file_status}{pad}{pos_status}");

        self.terminal.clear_line()?;
        self.terminal.draw_line(status_line.as_str(), Some(STATUS_FG_COLOR), Some(STATUS_BG_COLOR))?;

        Ok(())
    }

    fn draw_message_bar(&mut self) -> Result<()> {
        let message_bar_pos = Position::at(0, self.window_height() + 1);
        self.terminal.move_cursor_to(&message_bar_pos)?;

        self.terminal.clear_line()?;
        if self.status_message.is_recent() {
            self.terminal.draw_line(self.status_message.text.as_str(), None, None)?;
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
        let Position { x: mut position_x, y: mut position_y } = self.position;

        match key {
            // In most cases we will use ctrl+q for quitting,
            // but apparently VSCode skips sending ctrl+q to the terminal.
            (_, KeyCode::Char('q')) => self.quit = true,
            (KeyModifiers::NONE, KeyCode::Left) => {
                if position_x > 0 {
                    position_x -= 1;
                } else if position_y > 0 {
                    position_y -= 1;
                    position_x = self.document.width_at(&Position::at(0, position_y));
                }
            },
            (KeyModifiers::NONE, KeyCode::Right) => {
                if position_x < self.document.width_at(&self.position) {
                    position_x += 1;
                } else if position_y < self.document.height().saturating_sub(1){
                    position_y += 1;
                    position_x = 0;
                }
            },
            (KeyModifiers::NONE, KeyCode::Up) => {
                position_y = position_y.saturating_sub(1);
            },
            (KeyModifiers::NONE, KeyCode::Down) => {
                position_y += 1;
            },
            (KeyModifiers::NONE, KeyCode::Home) => {
                position_x = 0;
            },
            (KeyModifiers::NONE, KeyCode::End) => {
                position_x = self.document.width_at(&self.position);
            },
            (KeyModifiers::NONE, KeyCode::PageUp) => {
                position_y = position_y.saturating_sub(self.window_height());
            },
            (KeyModifiers::NONE, KeyCode::PageDown) => {
                position_y += self.window_height();
            },
            (_, KeyCode::Char(c)) => {
                let Position { x, y } = self.position;
                if let Some(row) = self.document.row_mut(y) {
                    row.insert_at(x, c);
                    position_x += 1;
                }
            }
            _ => {},
        }

        self.position = Position::at(position_x, position_y);
    }

    fn sanitize_position(&mut self) {
        let doc_height = self.document.height();
        let Position { x: mut position_x, y: mut position_y } = self.position;

        if position_y >= doc_height {
            position_y = doc_height.saturating_sub(1);
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
        self.terminal.size().height as usize - 2 // Last two lines is for status bar
    }

    fn die(&mut self, e: &Error) {
        self.terminal.clear().unwrap(); // We cannot handle error here, already dying
        panic!("{}", e);
    }
}
