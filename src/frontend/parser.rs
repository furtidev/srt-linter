use crate::utils::logging::{LogLevel, print_log};

use super::lexer::Token;
use std::time::Duration;

/// Repesents a singular record/subtitle.
#[derive(Debug)]
pub struct Subtitle {
    id: Option<usize>,
    start: Option<Duration>,
    end: Option<Duration>,
    text: Option<Vec<String>>,
}

/// Represents enviroment for parsing tokens generated earlier into structured data.
pub struct Parser {
    input: Vec<Token>,
    curr_loc: Option<usize>,
    strict: bool,
}

impl Parser {
    /// Create a new instance of the parser.
    pub fn new(input: Vec<Token>, strict: bool) -> Self {
        Self {
            curr_loc: Some(0),
            input,
            strict,
        }
    }

    fn advance(&mut self) {
        if self.curr_loc.unwrap() + 1 < self.input.len() {
            self.curr_loc = Some(self.curr_loc.unwrap() + 1);
        } else {
            self.curr_loc = None;
        }
    }

    // TODO: the markup extension could be checked for here.
    /// Parse the tokens and produce a structured list of all records/subtitles.
    pub fn parse(&mut self) -> (Vec<Subtitle>, usize) {
        let mut subtitles: Vec<Subtitle> = vec![];
        let mut total_lines: usize = 0;
        let mut sub_buf: Subtitle = Subtitle {
            id: None,
            start: None,
            end: None,
            text: None,
        };

        while let Some(curr_loc) = self.curr_loc {
            match &self.input[curr_loc] {
                Token::Count(num, _) => {
                    sub_buf.id = Some(*num);
                }
                Token::StartTime(ms, _) => {
                    sub_buf.start = Some(Duration::from_millis(*ms));
                }
                Token::EndTime(ms, line) => {
                    sub_buf.end = Some(Duration::from_millis(*ms));

                    if self.strict && sub_buf.start.unwrap() == sub_buf.end.unwrap() {
                        print_log(
                            LogLevel::Warning,
                            &format!(
                                "(subtitle #{}, line {}) Timestamps appear to be the same, this might be unintended.",
                                sub_buf.id.unwrap(),
                                line
                            ),
                        );
                    }
                }
                Token::Subtitle(text) => {
                    total_lines += text.len();
                    sub_buf.text = Some(text.clone());

                    subtitles.push(sub_buf);
                    sub_buf = Subtitle {
                        id: None,
                        start: None,
                        end: None,
                        text: None,
                    };
                }
                Token::Eof => break,
            }
            self.advance();
        }

        (subtitles, total_lines)
    }
}
