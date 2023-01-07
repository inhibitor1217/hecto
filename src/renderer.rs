use std::io;

use crate::{color::Color, row::Row};

pub struct Highlight {
    start: usize,
    end: usize,
    color: Option<Color>,
    background_color: Option<Color>,
}

impl Highlight {
    pub fn new(
        start: usize,
        end: usize,
        color: Option<Color>,
        background_color: Option<Color>,
    ) -> Self {
        Self {
            start,
            end,
            color,
            background_color,
        }
    }
}

pub trait RenderOutput {
    fn draw(
        &mut self,
        content: &str,
        color: Option<Color>,
        background_color: Option<Color>,
    ) -> Result<()>;

    fn draw_line(
        &mut self,
        content: &str,
        color: Option<Color>,
        background_color: Option<Color>,
    ) -> Result<()>;
}

pub type Error = io::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub fn render<Out>(
    out: &mut Out,
    row: &Row,
    range: (usize, usize),
    highlights: &[Highlight],
) -> Result<()>
where
    Out: RenderOutput,
{
    let (start, end) = range;

    out.draw_line(format!("{}\r", row.render(start, end)).as_str(), None, None)?;

    Ok(())
}
