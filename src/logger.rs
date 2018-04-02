use std::ops::Deref;

use slog;
use slog_term;
use slog_async;
use slog::Drain;

#[derive(Debug)]
pub struct Logger {
    logger: slog::Logger,
}

impl Logger {
    pub fn new(level: slog::Level) -> Self {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::CompactFormat::new(decorator).build().fuse();
        let drain = slog::LevelFilter::new(drain, slog::Level::Warning).ignore_res();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logger = slog::Logger::root(drain, o!());

        Logger { logger }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new(slog::Level::Warning)
    }
}

impl Deref for Logger {
    type Target = slog::Logger;

    fn deref(&self) -> &slog::Logger {
        &self.logger
    }
}
