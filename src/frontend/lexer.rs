use std::process::{ExitCode, exit};

use crate::utils::logging::{LogLevel, print_log};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Count(usize, usize),
    StartTime(u64, usize),
    EndTime(u64, usize),
    Subtitle((Vec<String>, usize)),
    Eof,
}

#[derive(PartialEq)]
pub enum LexState {
    Counter,
    Time,
    Sub,
}

/// Represents environment for tokenizing a `.srt` file.
pub struct Lexer {
    input: Vec<String>,
    curr_loc: Option<usize>,
    state: LexState,
    verbose: bool,
    strict: bool,
    issues: usize,
    last_count: (usize, usize), // count, line
}

fn remove_bom(s: &mut String) {
    if s.starts_with('\u{feff}') {
        s.remove(0);
    }
}

impl Lexer {
    /// Create a new instance of the lexer.
    pub fn new(input: Vec<String>, verbose: bool, strict: bool) -> Result<Self, ExitCode> {
        let mut input = input;

        if input.is_empty() {
            print_log(LogLevel::Error, "File is empty.");
            return Err(ExitCode::FAILURE);
        }

        if !input[input.len() - 1].is_empty() {
            if verbose {
                print_log(
                    LogLevel::Info,
                    "Re-added extra empty line at the end as it was removed unintentionally.",
                );
            }
            input.push(String::from(""));
        }

        if let Some(bom) = input[0].chars().nth(0) {
            if bom == '\u{feff}' {
                let original_length = input[0].len();
                remove_bom(&mut input[0]);

                if input[0].len() < original_length && verbose {
                    print_log(LogLevel::Info, "Detected BOM.");
                }
            }
        }

        Ok(Self {
            curr_loc: Some(0),
            state: LexState::Counter,
            issues: 0,
            input,
            verbose,
            strict,
            last_count: (0, 0),
        })
    }

    fn advance(&mut self) {
        if self.curr_loc.unwrap() + 1 < self.input.len() {
            self.curr_loc = Some(self.curr_loc.unwrap() + 1);
        } else {
            self.curr_loc = None;
        }
    }

    fn check_time_digit_padding(&mut self, hh: &str, mm: &str, ss: &str, ms: &str) {
        if !(hh.len() == 2 && mm.len() == 2 && ss.len() == 2 && ms.len() == 3) {
            if self.verbose {
                print_log(
                    LogLevel::Warning,
                    &format!(
                        "(line {}) Padding on digits are not OK. Expected 00:00:00,000.",
                        self.curr_loc.unwrap() + 1
                    ),
                );
            }
            self.issues += 1;
        }
    }

    fn lex_time(&mut self, time: String) -> Result<u64, ExitCode> {
        let mut dials: Vec<&str> = time.split(':').collect();

        if dials.len() < 3 || dials.len() > 3 {
            print_log(
                LogLevel::Error,
                &format!(
                    "(line {}) Invalid timestamp provided.",
                    self.curr_loc.unwrap() + 1
                ),
            );
            return Err(ExitCode::FAILURE);
        }

        let last = dials[dials.len() - 1].split(',').collect::<Vec<_>>();
        let ms = last[1];
        dials[2] = last[0];

        if self.strict {
            self.check_time_digit_padding(dials[0], dials[1], dials[2], ms);
        }

        let ms: u64 = match ms.parse() {
            Ok(num) => num,
            Err(e) => {
                print_log(
                    LogLevel::Error,
                    &format!(
                        "(line {}) Could not parse timestamp [{}].",
                        self.curr_loc.unwrap() + 1,
                        e
                    ),
                );
                return Err(ExitCode::FAILURE);
            }
        };

        let mut dials: Vec<u64> = dials
            .iter()
            .map(|xs| {
                let result = xs.parse::<u64>();
                match result {
                    Ok(num) => num,
                    Err(e) => {
                        print_log(
                            LogLevel::Error,
                            &format!(
                                "(line {}) Could not parse timestamp [{}].",
                                self.curr_loc.unwrap() + 1,
                                e
                            ),
                        );
                        // TODO: handle this exit more gracefully
                        exit(1);
                    }
                }
            })
            .collect();

        dials[0] *= 3_600_000; // hour dial
        dials[1] *= 60000; // minute dial
        dials[2] *= 1000; // second dial

        let squish: u64 = dials.iter().sum();

        Ok(squish + ms)
    }

    /// Generate tokens from a `.srt` file.
    pub fn lex(&mut self) -> Result<(Vec<Token>, usize), ExitCode> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(curr_loc) = self.curr_loc {
            match self.state {
                LexState::Counter => {
                    let num: Result<usize, _> = self.input[curr_loc].parse();

                    match num {
                        Ok(result) => {
                            if (result - self.last_count.0) != 1 {
                                self.issues += 1;
                                print_log(
                                    LogLevel::Warning,
                                    &format!(
                                        "(line {}) The last sequential subtitle count was {} (line {}), but now we're at {}. The difference is >1. Check your file, something possibly went wrong. ",
                                        curr_loc + 1,
                                        self.last_count.0,
                                        self.last_count.1,
                                        result
                                    ),
                                );
                            }

                            if self.strict && self.last_count == (0, 0) && result != 1 {
                                print_log(
                                    LogLevel::Warning,
                                    &format!(
                                        "(line {}) This is supposed to be the first subtitle in this file but the sequential counter is not `1` (found `{}`).",
                                        curr_loc + 1,
                                        result
                                    ),
                                );
                            }

                            self.last_count = (result, curr_loc + 1);
                            tokens.push(Token::Count(result, curr_loc + 1));
                            self.state = LexState::Time;
                            self.advance();
                        }
                        Err(_) => {
                            print_log(
                                LogLevel::Error,
                                &format!(
                                    "(line {}) Expected a clean sequential counter but the line has unexpected values.",
                                    curr_loc + 1
                                ),
                            );
                            return Err(ExitCode::FAILURE);
                        }
                    }
                }
                LexState::Time => {
                    let line = self.input[curr_loc].clone();
                    let times: Vec<&str> = line.split("-->").collect();

                    if times.len() < 2 || times.len() > 2 {
                        print_log(
                            LogLevel::Error,
                            &format!(
                                "(line {}) Expected a valid timestamp after the sequential counter.",
                                curr_loc + 1
                            ),
                        );
                        return Err(ExitCode::FAILURE);
                    }

                    let begin = self.lex_time(times[0].trim().into())?;
                    let end = self.lex_time(times[1].trim().into())?;
                    tokens.push(Token::StartTime(begin, curr_loc + 1));
                    tokens.push(Token::EndTime(end, curr_loc + 1));
                    self.state = LexState::Sub;
                    self.advance();
                }
                LexState::Sub => {
                    let mut lines: Vec<String> = vec![];

                    loop {
                        let line = self.input[self.curr_loc.unwrap()].clone();
                        self.advance();
                        if line.is_empty() {
                            break;
                        }

                        lines.push(line);
                    }
                    tokens.push(Token::Subtitle((lines, curr_loc + 1)));
                    self.state = LexState::Counter;
                }
            }
        }

        tokens.push(Token::Eof);

        Ok((tokens, self.issues))
    }
}
