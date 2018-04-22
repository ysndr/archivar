use slog;
use slog::*;

use slog_async;
use slog_term;
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
    pub fn new(verbosity: u64) -> Self {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::CompactFormat::new(decorator)
            .use_custom_timestamp(eat_timestamp)
            .build()
            .fuse();
        let level = match verbosity {
            0 => Level::Warning,
            1 => Level::Info,
            2 => Level::Debug,
            _ => Level::Trace,
        };

        let drain = slog::LevelFilter::new(drain, level).ignore_res();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logger = slog::Logger::root(drain, o!());
        debug!(&logger, "logger ready");

        Logger { logger }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new(0)
    }
}

impl Deref for Logger {
    type Target = slog::Logger;

    fn deref(&self) -> &slog::Logger {
        &self.logger
    }
}
