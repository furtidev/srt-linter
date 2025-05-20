use crate::utils::logging::{LogLevel, print_log};

use super::lexer::Token;
use std::time::Duration;

/// Repesents a singular record/subtitle.
#[derive(Debug, Clone)]
pub struct Subtitle {
    id: Option<usize>,
    pub start: Option<Duration>,
    end: Option<Duration>,
    pub text: Option<Vec<String>>,
}

/// Represents enviroment for parsing tokens generated earlier into structured data.
pub struct Parser {
    input: Vec<Token>,
    curr_loc: Option<usize>,
    strict: bool,
    issues: usize,
}

impl Parser {
    /// Create a new instance of the parser.
    pub fn new(input: Vec<Token>, strict: bool) -> Self {
        Self {
            curr_loc: Some(0),
            input,
            strict,
            issues: 0,
        }
    }

    fn advance(&mut self) {
        if self.curr_loc.unwrap() + 1 < self.input.len() {
            self.curr_loc = Some(self.curr_loc.unwrap() + 1);
        } else {
            self.curr_loc = None;
        }
    }

    // TODO: make this nicer and handle <font>
    fn check_markup_validity(&mut self, input: (Vec<String>, usize)) {
        let mut open: Vec<usize> = vec![];

        for (idx, line) in input.0.iter().enumerate() {
            let chars = line.chars().collect::<Vec<char>>();

            for (i, &c) in chars.iter().enumerate() {
                if c == '<' && (chars.len() <= i + 2) || (chars.len() <= i + 3) {
                    continue;
                }

                if (chars[i + 1] == 'i' || chars[i + 1] == 'b' || chars[i + 1] == 'u')
                    && chars[i + 2] == '>'
                {
                    open.push((idx) + input.1);
                }

                if chars[i + 1] == '/'
                    && (chars[i + 2] == 'i' || chars[i + 2] == 'b' || chars[i + 2] == 'u')
                    && chars[i + 3] == '>'
                {
                    if open.is_empty() {
                        print_log(
                            LogLevel::Warning,
                            &format!(
                                "(line {}) Stray markup closing tag detected.",
                                (idx) + input.1
                            ),
                        );
                        self.issues += 1;
                        continue;
                    }
                    open.pop();
                }
            }
        }

        if !open.is_empty() {
            self.issues += 1;
            for ln in open {
                print_log(
                    LogLevel::Warning,
                    &format!("(line {}) Unclosed markup detected.", ln),
                );
            }
        }
    }

    // TODO: the markup extension could be checked for here.
    /// Parse the tokens and produce a structured list of all records/subtitles.
    pub fn parse(&mut self) -> (Vec<Subtitle>, usize, usize) {
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
                    total_lines += text.0.len();
                    sub_buf.text = Some(text.0.clone());

                    if self.strict {
                        self.check_markup_validity(text.clone());
                    }

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

        (subtitles, total_lines, self.issues)
    }
}
