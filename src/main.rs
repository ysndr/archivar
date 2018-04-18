#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate sloggers;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate error_chain;

mod action;
mod app;
mod command;
mod constants;
mod error;
mod logger;

use app::Archivar as App;
use app::Config;
use error::*;
use logger::Logger;
use slog::Level;
use std::cmp::min;

fn main() {
    let _ret = run();
}

fn run() {
    let (log_level, matches) = app::parse_args();

    let log_level = match log_level {
        1...3 => Level::from_usize(3 + min(log_level as usize, 3)).unwrap(),
        _ => Level::Warning,
    };

    let log = Logger::new(&log_level);

    let config = Config::new(&log, matches);
    let mut app = App::new(&config);

    let result = app.build_command().and_then(|_| app.build_actions());

    info!(log, "info;");
    debug!(log, "debug");
    trace!(log, "trace");

    if let Err(e) = result {
        match *e.kind() {
            ErrorKind::Clap(_) => println!("{}", e),
            _ => {
                crit!(log, "error: {}", e);
                for e in e.iter().skip(1) {
                    println!("caused by: {}", e);
                }

                // The backtrace is not always generated. Try to run this example
                // with `RUST_BACKTRACE=1`.
                if let Some(backtrace) = e.backtrace() {
                    println!("backtrace: {:?}", backtrace);
                }
            }
        }
    };
}
