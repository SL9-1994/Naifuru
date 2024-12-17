use log::{Level, LevelFilter};
use std::io::Write;

/// Represents available logging levels for the application.
/// Used for configuring the logging verbosity through CLI arguments.
#[derive(Debug, Clone, Copy, clap::ValueEnum, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Info,
    Debug,
}

/// Converts the application's LogLevel to the standard log crate's LevelFilter.
///
/// # Mapping
/// * Error -> LevelFilter::Error
/// * Info  -> LevelFilter::Info
/// * Debug -> LevelFilter::Debug
impl From<LogLevel> for log::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
        }
    }
}

/// Initializes the application logger with the specified log level.
///
/// # Arguments
/// * `log_level` - The maximum log level to display
///
/// # Returns
/// Result indicating whether logger initialization was successful
pub fn init_logger(log_level: LevelFilter) -> Result<(), log::SetLoggerError> {
    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp_secs()
        .format(|buf, record| {
            let level = match record.level() {
                Level::Error => "\x1b[31mERROR\x1b[0m", // red
                Level::Info => "\x1b[32mINFO\x1b[0m",   // green
                Level::Debug => "\x1b[34mDEBUG\x1b[0m", // blue
                Level::Warn => "\x1b[33mWARN\x1b[0m",   // yellow
                Level::Trace => "\x1b[35mTRACE\x1b[0m", // magenta
            };
            writeln!(buf, "{}: {}", level, record.args())
        })
        .try_init()
}
