extern crate libarchivar;
extern crate pretty_assertions;
extern crate assert_fs;
extern crate predicates;
extern crate log;

use libarchivar::app::Archivar as App;

use assert_fs::prelude::*;

mod utils;

use libarchivar::logger;

use utils::*;


#[test]
fn test_init_ok() {
    logger::setup_logger(logger::level_from_verbosity(3)).unwrap();

    // setuo
    let temp = assert_fs::TempDir::new().unwrap();
    let args = make_args("init", &["-p", temp.path().to_str().unwrap()], &[]);
    let app = App::new(args);

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
    let args = make_args("init", &["-p", temp.path().to_str().unwrap()], &[]);
    let app = App::new(args);
    

    // run
    app.run().unwrap();
    app.run().unwrap(); // same arguments = directory shuld already exist now  

    // test
    temp.child(".archivar").assert(predicates::path::is_file());
    
    //cleanup
    temp.close().unwrap();
}
