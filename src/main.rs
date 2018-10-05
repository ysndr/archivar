#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

// mod template;

mod logger;

use archivar::app::Archivar as App;
use archivar::app::Args;
use archivar::error::*;
use archivar::structopt::*;

fn main() {
    let args = Args::from_args();

    let level = logger::level_from_verbosity(args.verbosity);
    logger::setup_logger(level).expect("could not set logger");

    let context = App::setup_context(&args);

    let app = App::new(args.sub, context);

    if let Err(e) = run(&app) {
        let mapp = app;
        mapp.context.shell().error(e).is_ok();
    }
}

fn run(app: &App) -> Result<()> {
    app.shell().info(format!("{:?}", app)).unwrap();
    app.run()
}
