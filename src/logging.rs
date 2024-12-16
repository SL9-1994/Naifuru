use log::LevelFilter;

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Info,
}

impl From<LogLevel> for log::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Info => log::LevelFilter::Info,
        }
    }
}

pub fn init_logger(log_level: LevelFilter) {
    env_logger::Builder::new().filter_level(log_level).init();
}
