extern crate assert_fs;
extern crate libarchivar;
extern crate log;
extern crate predicates;
extern crate pretty_assertions;

use libarchivar::app::Archivar as App;
use libarchivar::app::{Args, Command};

use assert_fs::prelude::*;

mod utils;

use libarchivar::logger;

use utils::*;

#[test]
fn test_init_ok() {
    logger::setup_logger(logger::level_from_verbosity(3)).unwrap();

    // setup
    let temp = assert_fs::TempDir::new().unwrap();
    let sub = Command::Init;
    let args = Args {
        verbosity: 3,
        git_disabled: false,
        path: temp.path().to_owned(),
        sub: sub.clone(),
    };

    let app = App::new(sub, App::setup_context(&args));

    // run
    app.run().unwrap();

    // test
    temp.child(".archivar").assert(predicates::path::is_file());
    temp.child(".archive").assert(predicates::path::is_dir());

    //cleanup
    temp.close().unwrap();
}

#[test]
fn test_init_fail_if_exists() {
    logger::setup_logger(logger::level_from_verbosity(3)).unwrap_or(());

    // setuo
    let temp = assert_fs::TempDir::new().unwrap();
    let sub = Command::Init;
    let args = Args {
        verbosity: 3,
        git_disabled: false,
        path: temp.path().to_owned(),
        sub: sub.clone(),
    };

    let app = App::new(sub, App::setup_context(&args));

    // run
    app.run().unwrap();
    assert!(app.run().is_err()); // same arguments = directory shuld already exist now

    // test
    temp.child(".archivar").assert(predicates::path::is_file());

    //cleanup
    temp.close().unwrap();
}
