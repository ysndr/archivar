#[macro_use]
extern crate slog;
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
use error::*;

fn main() {
    let mut app = App::default();
    error!(&app.logger, "error 1");
    warn!(&app.logger, "warning 1");

    let result = app.match_args()
        .and(app.configure_logger())
        .and(app.build_command())
        .and(app.build_actions());

    error!(&app.logger, "error");
    warn!(&app.logger, "warning");

    if let Err(e) = &result {
        match *e.kind() {
            ErrorKind::Clap(_) => println!("{}", e),
            _ => {
                error!(app.logger, "error: {}", e);
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
    };
}
