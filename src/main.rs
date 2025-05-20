use clap::Parser;
use std::{error, io, path::PathBuf, process::ExitCode};
use tui::App;
use utils::logging::{LogLevel, print_log};

use ratatui::{
    Terminal,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
};

mod frontend;
mod tui;
mod utils;

#[derive(Parser)]
#[command(name = "srt-linter")]
#[command(version = "0.2.0")]
#[command(about = "Look for issues inside SubRip text (.srt) files.")]
struct Cli {
    #[arg(
        long,
        short,
        help = "Logs additional information about internal actions"
    )]
    verbose: bool,
    #[arg(long, short, help = "Enforces stricter rules for suspicious behavior")]
    strict: bool,
    #[arg(long, short, help = "Shows a TUI at the end")]
    tui: bool,
    #[arg(value_parser = clap::value_parser!(PathBuf))]
    file_path: PathBuf,
}

struct State {
    file_path: PathBuf,
    content: Vec<String>,
}

// the error variant is primarily for TUI errors.
fn main() -> Result<ExitCode, Box<dyn error::Error>> {
    let cli = Cli::parse();
    let mut state = State {
        file_path: cli.file_path,
        content: vec![],
    };

    if let Ok(lines) = utils::file::read_lines(state.file_path) {
        for line in lines.map_while(Result::ok) {
            state.content.push(line);
        }
    }

    // do semantic analysis
    let lexer_result = frontend::lexer::Lexer::new(state.content, cli.verbose, cli.strict);

    let mut lexer = match lexer_result {
        Ok(res) => res,
        Err(e) => return Ok(e),
    };

    let lexed_result = lexer.lex();

    let (tokens, issues) = match lexed_result {
        Ok(res) => res,
        Err(e) => return Ok(e),
    };

    if issues > 0 {
        print_log(
            LogLevel::Warning,
            &format!("File is semantically OK except for {} issue(s).", issues),
        );
    } else {
        print_log(LogLevel::Success, "File is semantically OK.");
    }

    // parse the file
    let mut parser = frontend::parser::Parser::new(tokens, cli.strict);
    let (subtitles, lines, issues) = parser.parse(); // subtitles, line number, issues

    if issues > 0 {
        print_log(
            LogLevel::Warning,
            &format!(
                "File is structurally OK except for {} issue(s). Read {} line(s).",
                issues, lines
            ),
        );
    } else {
        print_log(
            LogLevel::Success,
            &format!("File is structurally OK. Read {} line(s).", lines),
        );
    }

    if cli.tui {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        let mut app = App::new(lines);

        tui::run_tui(&mut terminal, (&subtitles, lines), &mut app)?;

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
    }

    Ok(ExitCode::SUCCESS)
}
