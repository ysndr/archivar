extern crate libarchivar;
extern crate pretty_assertions;
extern crate assert_fs;
extern crate predicates;
extern crate log;

use std::io::Read;
use std::fs::File;
use std::env::current_dir;
use libarchivar::app::Archivar as App;

use assert_fs::prelude::*;

mod utils;

use libarchivar::logger;

use utils::*;


#[test]
fn test_template_includes() {
    logger::setup_logger(logger::level_from_verbosity(3)).unwrap_or(());
     // setuo
    let temp = assert_fs::TempDir::new().unwrap();
    let args = make_args(
        "new",
        &["-p", temp.path().to_str().unwrap()], 
        &[
            "test",
            utils::cwd().join("tests/setups/templates/includes.yaml").to_str().unwrap(),
        ]);
    
    App::new(make_args(
        "init",
        &["-p", temp.path().to_str().unwrap()], 
        &[])).run().unwrap();

    let app = App::new(args);


    // run
    app.run().unwrap();

    // test
    let test = temp.child("test");
    test.assert(predicates::path::is_dir());
    for path in &[
        "test/example-new.txt",
        "test/plain.txt",
        "test/files/folder/exists.txt",
        "test/files/folder/inside/exists.txt",
        "test/files/folder/inside/intodeep.txt" ] {
            temp.child(path).assert(predicates::path::is_file());
        }
    //cleanup
    temp.close().unwrap();
}

#[test]
fn test_template_init() {
    logger::setup_logger(logger::level_from_verbosity(3)).unwrap_or(());
     // setuo
    let temp = assert_fs::TempDir::new().unwrap();
    let args = make_args(
        "new",
        &["-p", temp.path().to_str().unwrap()], 
        &[
            "test",
            utils::cwd().join("tests/setups/templates/init.yaml").to_str().unwrap(),
        ]);
    
    App::new(make_args(
        "init",
        &["-p", temp.path().to_str().unwrap()], 
        &[])).run().unwrap();

    let app = App::new(args);


    // run
    app.run().unwrap();

    // test
    temp.child("test/test").assert(predicates::path::is_dir());

    let test_dir = temp.child("test");
    let test_file = temp.child("test/pwd.txt");
    test_file.assert(predicates::path::is_file());

    let mut content = String::new();
    File::open(test_file.path()).unwrap()
        .read_to_string(&mut content).unwrap();
    
    assert_eq!(test_dir.path().to_str().unwrap(), content.trim());

    //cleanup
    temp.close().unwrap();
}

#[test]
fn test_template_dirs() {
    logger::setup_logger(logger::level_from_verbosity(3)).unwrap_or(());
     // setuo
    let temp = assert_fs::TempDir::new().unwrap();
    let args = make_args(
        "new",
        &["-p", temp.path().to_str().unwrap()], 
        &[
            "test",
            utils::cwd().join("tests/setups/templates/new.yaml").to_str().unwrap(),
        ]);
    
    App::new(make_args(
        "init",
        &["-p", temp.path().to_str().unwrap()], 
        &[])).run().unwrap();

    let app = App::new(args);


    // run
    app.run().unwrap();

    // test
    temp.child("test/src").assert(predicates::path::is_dir());   
    temp.child("test/whatever/this/is/nested/").assert(predicates::path::is_dir());
  
    temp.close().unwrap();
}
