use slog;
use slog::*;
use slog_async;
use slog_term;
use std::cmp::min;
use std::io;
use std::ops::Deref;

fn eat_timestamp(writer: &mut io::Write) -> io::Result<()> {
    write!(writer, "::")
}

#[derive(Debug)]
pub struct Logger {
    logger: slog::Logger,
}

impl Logger {
    pub fn new(log_level: &Level) -> Self {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::CompactFormat::new(decorator)
            .use_custom_timestamp(eat_timestamp)
            .build()
            .fuse();
        let drain = slog::LevelFilter::new(drain, *log_level).ignore_res();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logger = slog::Logger::root(drain, o!());

        debug!(&logger, "logger ready");

        Logger { logger }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new(&Level::Warning)
    }
}

impl Deref for Logger {
    type Target = slog::Logger;

    fn deref(&self) -> &slog::Logger {
        &self.logger
    }
}
