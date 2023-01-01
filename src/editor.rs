use std::io::{self, Stdout, Write};

use crate::terminal::{Key, KeyCode, KeyModifiers, Terminal};

type Error = io::Error;
type Result<T> = std::result::Result<T, Error>;

pub struct Editor<'a> {
    stdout: Stdout,
    terminal: &'a mut Terminal,
    quit: bool,
}

impl<'a> Editor<'a> {
    pub fn new(terminal: &'a mut Terminal) -> Self {
        Self {
            stdout: io::stdout(),
            terminal,
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

            self.draw()?;
            self.terminal.move_cursor_to(0, 0)?;
            self.terminal.show_cursor()?;

            if self.quit {
                self.terminal.clear()?;
                write!(self.stdout, "Goodbye! :)\r\n")?;
                break;
            }

            let key = Terminal::read_key()?;
            self.process_key(key)?;
        }
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        for _ in 0..self.terminal.size().height - 1 {
            self.terminal.clear_line()?;
            write!(self.stdout, "~\r\n")?;
        }
        Ok(())
    }

    fn process_key(&mut self, key: Key) -> Result<()> {
        match key {
            (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                self.quit = true;
                Ok(())
            }
            (_, KeyCode::Char(c)) => write!(self.stdout, "{:?} ({c}) \r\n", c as u8),
            (_, code) => write!(self.stdout, "{code:?} \r\n"),
        }
    }

    fn die(&mut self, e: &Error) {
        self.terminal.clear().unwrap(); // We cannot handle error here, already dying
        panic!("{}", e);
    }
}
