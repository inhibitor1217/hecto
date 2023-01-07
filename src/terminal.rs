use std::io::{self, Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode as CrossTermKeyCode, KeyModifiers as CrossTermKeyModifiers},
    execute,
    style::{SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{size, Clear, ClearType},
};

use crate::position::Position;
use crate::{color::Color, renderer::RenderOutput};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    stdout: Stdout,
    size: Size,
    cursor_position: Position,
}

pub type Error = io::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub type KeyModifiers = CrossTermKeyModifiers;
pub type KeyCode = CrossTermKeyCode;
pub type Key = (KeyModifiers, KeyCode);

impl Terminal {
    pub fn new() -> Result<Self> {
        let (width, height) = size()?;
        Ok(Self {
            stdout: io::stdout(),
            size: Size { width, height },
            cursor_position: Position::zero(),
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        execute!(self.stdout, Show)
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        execute!(self.stdout, Hide)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn move_cursor_to(&mut self, position: &Position) -> Result<()> {
        self.cursor_position = Position::at(position.x, position.y);
        execute!(self.stdout, MoveTo(position.x as u16, position.y as u16))
    }

    pub fn read_key() -> Result<Key> {
        loop {
            if let Event::Key(event) = read()? {
                return Ok((event.modifiers, event.code));
            }
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    pub fn clear_line(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::CurrentLine))
    }
}

impl RenderOutput for Terminal {
    fn draw(&mut self, content: &str, color: Option<Color>, bg_color: Option<Color>) -> Result<()> {
        if let Some(color) = color {
            execute!(self.stdout, SetForegroundColor(color))?;
        }
        if let Some(bg_color) = bg_color {
            execute!(self.stdout, SetBackgroundColor(bg_color))?;
        }

        write!(self.stdout, "{content}")?;
        execute!(self.stdout, SetForegroundColor(Color::Reset))?;
        execute!(self.stdout, SetBackgroundColor(Color::Reset))?;
        Ok(())
    }

    fn draw_line(
        &mut self,
        line: &str,
        color: Option<Color>,
        bg_color: Option<Color>,
    ) -> Result<()> {
        let is_last_line = self.cursor_position.y == self.size.height as usize - 1;
        let newline = if is_last_line { "" } else { "\n" };

        self.draw(&format!("{line}{newline}"), color, bg_color)
    }

    fn style(
        content: &str,
        color: Option<Color>,
        background_color: Option<Color>,
    ) -> String {
        let mut styled = content.to_string();
        if let Some(color) = color {
            styled = styled.with(color).to_string();
        }
        if let Some(background_color) = background_color {
            styled = styled.on(background_color).to_string();
        }
        styled
    }
}
