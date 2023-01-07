use unicode_segmentation::UnicodeSegmentation;

use crate::color::Color;

#[derive(Debug)]
pub struct Highlight {
    pub start: usize,
    pub end: usize,
    pub color: Option<Color>,
    pub background_color: Option<Color>,
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

pub trait Highlighter {
    fn highlight(&self, line: &str) -> Vec<Highlight>;
}

pub struct CommonSyntaxHighlighter;

impl CommonSyntaxHighlighter {
    const NUMBER_COLOR: Color = Color::Rgb {
        r: 220,
        g: 163,
        b: 163,
    };

    pub fn new() -> Self {
        Self
    }

    fn is_number(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    fn is_separator(ch: char) -> bool {
        ch.is_whitespace()
            || ch == '('
            || ch == ')'
            || ch == '{'
            || ch == '}'
            || ch == '['
            || ch == ']'
            || ch == ','
            || ch == ';'
            || ch == '.'
            || ch == '='
            || ch == '+'
            || ch == '-'
            || ch == '*'
            || ch == '/'
            || ch == '%'
            || ch == '!'
            || ch == '~'
            || ch == '<'
            || ch == '>'
            || ch == '&'
            || ch == '|'
            || ch == '^'
            || ch == '?'
            || ch == ':'
    }
}

impl Highlighter for CommonSyntaxHighlighter {
    fn highlight(&self, line: &str) -> Vec<Highlight> {
        let mut highlights = vec![];

        let mut is_separator = true;
        for (i, g) in line.graphemes(true).enumerate() {
            if let Some(ch) = g.chars().next() {
                if Self::is_number(ch) {
                    if is_separator {
                        highlights.push(Highlight::new(i, i + 1, Some(Self::NUMBER_COLOR), None));
                    } else if let Some(Highlight {
                        start: _,
                        end,
                        color: _,
                        background_color: _,
                    }) = highlights.last_mut()
                    {
                        *end += 1;
                    }
                }

                is_separator = Self::is_separator(ch);
            }
        }

        highlights
    }
}
