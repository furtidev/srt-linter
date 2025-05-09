// taken from: https://github.com/hitblast/cutler/blob/f1cb8e96940335238cbae3cf309f0dd5708e3d4c/src/util/logging.rs

/// ANSI color codes
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

#[derive(PartialEq)]
pub enum LogLevel {
    Success,
    Error,
    Warning,
    Info,
}

/// Central logger
pub fn print_log(level: LogLevel, msg: &str) {
    let (tag, color) = match level {
        LogLevel::Success => ("SUCCESS", GREEN),
        LogLevel::Error => ("ERROR", RED),
        LogLevel::Warning => ("WARNING", YELLOW),
        LogLevel::Info => ("INFO", BOLD),
    };
    let line = format!("{}[{}]{} {}", color, tag, RESET, msg);
    if level == LogLevel::Error || level == LogLevel::Warning {
        eprintln!("{}", line);
    } else {
        println!("{}", line);
    }
}
