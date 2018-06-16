#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate shell;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

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
mod template;

use app::Archivar as App;
use app::Config;
use command::Command;
use error::*;
use logger::Logger;
use shell::Shell;

fn main() {
    if let Err(e) = run() {
        match *e.kind() {
            ErrorKind::Clap(_) => crit!(Logger::default(), "{}", e),
            _ => {
                crit!(Logger::default(), "error: {}", e);
                for e in e.iter().skip(1) {
                    crit!(Logger::default(), "caused by: {}", e);
                }

                // The backtrace is not always generated. Try to run this example
                // with `RUST_BACKTRACE=1`.
                if let Some(backtrace) = e.backtrace() {
                    crit!(Logger::default(), "backtrace: {:?}", backtrace);
                }
            }
        }
    }
}

fn run() -> Result<()> {
    let (log_level, matches) = app::parse_args()?;

    let log = Logger::new(log_level);
    let shell = Shell::new();
    let command = Command::from_matches(&matches, &log)?;

    println!("got command {:?}", command);
    let cwd = std::env::current_dir().unwrap();
    let config = Config::new(&log, &shell, cwd.as_path());

    let mut app = App::new(config, command);

    app.make_actions()
}
