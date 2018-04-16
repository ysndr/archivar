#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

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
use error::*;

fn main() {
    println!("Hello, world!");
    let mut app = App::default();
    let result = app.match_args()
        .and(app.configure_logger())
        .and(app.build_command())
        .and(app.build_actions());

    match result {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::Clap => println!("{}", e),
        Err(e) => {
            println!("error: {}", e);
            for e in e.iter().skip(1) {
                println!("caused by: {}", e);
            }

            // The backtrace is not always generated. Try to run this example
            // with `RUST_BACKTRACE=1`.
            if let Some(backtrace) = e.backtrace() {
                println!("backtrace: {:?}", backtrace);
            }

            ::std::process::exit(1);
        }
    }
}
