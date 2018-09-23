extern crate assert_fs;
extern crate libarchivar;
extern crate log;
extern crate predicates;
extern crate pretty_assertions;
extern crate shell;

use assert_fs::assert::IntoPathPredicate;
use assert_fs::TempDir;
use libarchivar::app::Archivar as App;
use libarchivar::app::Context;
use libarchivar::app::{Args, Command};
use libarchivar::constants::{ARCHIVAR_FILE_NAME, ARCHIVE_FOLDER_NAME, PROJECT_FILE_NAME};
use std::cell::{RefCell, RefMut};
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use assert_fs::prelude::*;

mod utils;

use libarchivar::logger;

use utils::cwd;

fn setup(
    template: PathBuf,
) -> (
    assert_fs::TempDir,
    libarchivar::app::Archivar,
    libarchivar::app::Archivar,
) {
    logger::setup_logger(logger::level_from_verbosity(3)).unwrap_or(());
    // setuo
    let temp = assert_fs::TempDir::new().unwrap();

    temp.copy_from(cwd().join("tests/setups/example"), &["*"])
        .expect("Could not create test project structure");

    temp.child("test/").assert(predicates::path::is_dir());

    let archive = Command::Archive { dir: "test".into() };
    let unarchive = Command::Unarchive { dir: "test".into() };

    let app_archive = App::new(archive, get_context(&temp));
    let app_unarchive = App::new(unarchive, get_context(&temp));

    (temp, app_archive, app_unarchive)
}

fn get_context(temp: &TempDir) -> Context {
    Context {
        cwd: cwd(),
        path: temp.path().to_owned(),
        shell: RefCell::new(shell::Shell::default()),
    }
}

#[test]
fn test_template_archive_unarchive() {
    let (temp, archive, unarchive) = setup("tests/setups/templates/includes.yaml".into());

    // run
    archive.run().expect("Archive failed");

    // test
    temp.child(ARCHIVE_FOLDER_NAME)
        .assert(predicates::path::is_dir());
    temp.child(ARCHIVE_FOLDER_NAME.to_owned() + "/test")
        .assert(predicates::path::is_dir());
    temp.child("test").assert(predicates::path::missing());

    let a_file = temp.child(ARCHIVE_FOLDER_NAME.to_owned() + "/test/a.txt");
    a_file.assert(predicates::path::is_file());
    a_file.assert("A\n");

    // run unarchive
    unarchive.run().expect("Unarchive failed");
    temp.child("test").assert(predicates::path::is_dir());

    let a_file = temp.child("test/a.txt");
    a_file.assert(predicates::path::is_file());
    a_file.assert("A\n".into_path());

    //cleanup
    temp.close().unwrap();
}
