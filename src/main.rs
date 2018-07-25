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
extern crate structopt;

#[macro_use]
extern crate error_chain;

// mod action;
mod app;
mod args;
mod constants;
mod error;
mod logger;
// mod template;

use app::Archivar as App;
use args::Args;
use error::*;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    let app = App::new(args);

    if let Err(e) = run(&app) {
        let mut mapp = app;
        mapp.context.shell().error(e).is_ok();
    }
}

fn run(app: &App) -> Result<()> {
    app.shell().info(format!("{:?}", app)).unwrap();

    Err("oof".into())

    // let actions = app.make_actions();

    // app.execute(&*actions)
}
