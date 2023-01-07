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

pub struct CommonSyntaxHighlighter {
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
}

impl CommonSyntaxHighlighter {
    const NUMBER_COLOR: Color = Color::Rgb {
        r: 220,
        g: 163,
        b: 163,
    };

    const STRING_COLOR: Color = Color::DarkCyan;

    const COMMENT_COLOR: Color = Color::DarkGrey;

    const PRIMARY_KEYWORD_COLOR: Color = Color::Rgb {
        r: 181,
        g: 137,
        b: 0,
    };

    const SECONDARY_KEYWORD_COLOR: Color = Color::Rgb {
        r: 86,
        g: 156,
        b: 214,
    };

    pub fn new() -> Self {
        Self {
            primary_keywords: vec![
                "as".to_string(),
                "break".to_string(),
                "const".to_string(),
                "continue".to_string(),
                "crate".to_string(),
                "else".to_string(),
                "enum".to_string(),
                "extern".to_string(),
                "false".to_string(),
                "fn".to_string(),
                "for".to_string(),
                "if".to_string(),
                "impl".to_string(),
                "in".to_string(),
                "let".to_string(),
                "loop".to_string(),
                "match".to_string(),
                "mod".to_string(),
                "move".to_string(),
                "mut".to_string(),
                "pub".to_string(),
                "ref".to_string(),
                "return".to_string(),
                "self".to_string(),
                "Self".to_string(),
                "static".to_string(),
                "struct".to_string(),
                "super".to_string(),
                "trait".to_string(),
                "true".to_string(),
                "type".to_string(),
                "unsafe".to_string(),
                "use".to_string(),
                "where".to_string(),
                "while".to_string(),
                "dyn".to_string(),
                "abstract".to_string(),
                "become".to_string(),
                "box".to_string(),
                "do".to_string(),
                "final".to_string(),
                "macro".to_string(),
                "override".to_string(),
                "priv".to_string(),
                "typeof".to_string(),
                "unsized".to_string(),
                "virtual".to_string(),
                "yield".to_string(),
                "async".to_string(),
                "await".to_string(),
                "try".to_string(),
            ],
            secondary_keywords: vec![
                "bool".to_string(),
                "char".to_string(),
                "i8".to_string(),
                "i16".to_string(),
                "i32".to_string(),
                "i64".to_string(),
                "isize".to_string(),
                "u8".to_string(),
                "u16".to_string(),
                "u32".to_string(),
                "u64".to_string(),
                "usize".to_string(),
                "f32".to_string(),
                "f64".to_string(),
                "str".to_string(),
            ],
        }
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

        for primary_keyword in &self.primary_keywords {
            let mut start = 0;
            while let Some(index) = line[start..].find(format!("{} ", primary_keyword).as_str()) {
                start = start + index;
                let end = start + primary_keyword.len();
                highlights.push(Highlight::new(
                    start,
                    end,
                    Some(Self::PRIMARY_KEYWORD_COLOR),
                    None,
                ));
                start = end;
            }
        }

        for secondary_keyword in &self.secondary_keywords {
            let mut start = 0;
            while let Some(index) = line[start..].find(format!("{} ", secondary_keyword).as_str()) {
                start = start + index;
                let end = start + secondary_keyword.len();
                highlights.push(Highlight::new(
                    start,
                    end,
                    Some(Self::SECONDARY_KEYWORD_COLOR),
                    None,
                ));
                start = end;
            }
        }

        let mut word_start = 0;
        let mut is_separator = true;
        let mut is_number = false;
        let mut in_string = false;
        for (i, g) in line.graphemes(true).enumerate() {
            if let Some(ch) = g.chars().next() {
                if is_separator {
                    word_start = i;
                    is_number = true;
                }

                if !in_string {
                    is_separator = Self::is_separator(ch);
                }

                if is_separator {
                    if is_number {
                        highlights.push(Highlight::new(
                            word_start,
                            i,
                            Some(Self::NUMBER_COLOR),
                            None,
                        ));
                    }
                } else {
                    is_number = is_number && Self::is_number(ch);

                    if ch == '"' || ch == '\'' {
                        if !in_string {
                            in_string = true;
                        } else {
                            highlights.push(Highlight::new(
                                word_start,
                                i + 1,
                                Some(Self::STRING_COLOR),
                                None,
                            ));
                            in_string = false;
                        }
                    }
                }
            }
        }

        line.find("//").map(|pos| {
            highlights.push(Highlight::new(
                pos,
                line.len(),
                Some(Self::COMMENT_COLOR),
                None,
            ))
        });

        highlights
    }
}
