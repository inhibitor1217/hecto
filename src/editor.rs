use std::io::{self, Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{Clear, ClearType},
};

use crate::terminal::Terminal;

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
            self.refresh_screen()?;

            if self.quit {
                break;
            }

            self.draw()?;
            self.position_cursor_at_default()?;

            let key = Self::read_key()?;
            self.process_key(key)?;
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    fn draw(&mut self) -> Result<()> {
        for _ in 0..self.terminal.size().height {
            write!(self.stdout, "~\r\n")?;
        }
        Ok(())
    }

    fn position_cursor_at_default(&mut self) -> Result<()> {
        execute!(self.stdout, MoveTo(0, 0))
    }

    fn read_key() -> Result<(KeyModifiers, KeyCode)> {
        loop {
            if let Event::Key(event) = read()? {
                return Ok((event.modifiers, event.code));
            }
        }
    }

    fn process_key(&mut self, key: (KeyModifiers, KeyCode)) -> Result<()> {
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
        self.refresh_screen().unwrap(); // We cannot handle error here, already dying
        panic!("{}", e);
    }
}
