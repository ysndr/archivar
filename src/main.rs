#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

extern crate clap;


mod app;
mod constants;
mod command;
mod error;
mod action;
mod logger;

use app::Archivar as App;

fn main() {
    println!("Hello, world!");
    let mut app = App::default();
    app.match_args();
    app.configure_logger();
    app.build_command();
    app.build_actions();

}
