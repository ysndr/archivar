use std::ops::Deref;

use slog;
use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::types::{Format, Severity};
use sloggers::Build;

#[derive(Debug)]
pub struct Logger {
    logger: slog::Logger,
}

impl Logger {
    pub fn new(level: Severity) -> Self {
        let logger = TerminalLoggerBuilder::new()
            .format(Format::Compact)
            .level(level)
            .build()
            .unwrap();

        info!(logger, "Logging ready!");
        error!(logger, "Logging ready!");
        warn!(logger, "Logging ready!");
        Logger { logger }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new(Severity::Info)
    }
}

impl Deref for Logger {
    type Target = slog::Logger;

    fn deref(&self) -> &slog::Logger {
        &self.logger
    }
}
