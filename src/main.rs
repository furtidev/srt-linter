use clap::Parser;
use std::{path::PathBuf, process::ExitCode};
use utils::logging::{LogLevel, print_log};

mod frontend;
mod utils;

#[derive(Parser)]
#[command(name = "srt-linter")]
#[command(version = "0.1.0")]
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
    #[arg(value_parser = clap::value_parser!(PathBuf))]
    file_path: PathBuf,
}

struct State {
    file_path: PathBuf,
    content: Vec<String>,
}

fn main() -> ExitCode {
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
        Err(e) => return e,
    };

    let lexed_result = lexer.lex();

    let (tokens, issues) = match lexed_result {
        Ok(res) => res,
        Err(e) => return e,
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
    let (_, lines) = parser.parse(); // subtitles, line number
    print_log(
        LogLevel::Success,
        &format!("File is structurally OK. Read {} line(s).", lines),
    );

    ExitCode::SUCCESS
}
