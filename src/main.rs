#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate log;
extern crate fern;
extern crate chrono;

extern crate libarchivar;

// mod template;

mod logger;


use libarchivar::app::Archivar as App;
use libarchivar::app::Args;
use libarchivar::error::*;
use libarchivar::structopt::*;

fn main() {
    let args = Args::from_args();

    let level = logger::level_from_verbosity(args.verbosity);
    logger::setup_logger(level).expect("could not set logger");

    let context = App::setup_context(&args);

    let app = App::new(args.sub, context);

    if let Err(e) = run(&app) {
        let mut mapp = app;
        mapp.context.shell().error(e).is_ok();
    }
}

fn run(app: &App) -> Result<()> {
    app.shell().info(format!("{:?}", app)).unwrap();
    app.run()
}
