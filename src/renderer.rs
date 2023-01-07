use std::io;

use unicode_segmentation::UnicodeSegmentation;

use crate::{color::Color, row::Row, highlight::Highlight};

pub trait RenderOutput {
    fn style(content: &str, color: Option<Color>, background_color: Option<Color>) -> String;

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

    let rendered = row.render(start, end);

    let highlighted = rendered
        .graphemes(true)
        .enumerate()
        .map(|(pos, ch)| {
            let mut color = None;
            let mut background_color = None;
            for highlight in highlights {
                if highlight.start <= start + pos && start + pos < highlight.end {
                    color = highlight.color;
                    background_color = highlight.background_color;
                    break;
                }
            }
            Out::style(ch, color, background_color)
        })
        .collect::<String>();

    out.draw_line(format!("{highlighted}\r").as_str(), None, None)
}
