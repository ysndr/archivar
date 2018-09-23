use libarchivar::app::Archivar as App;
use libarchivar::app::{Args, Command};

use libarchivar::logger;
use libarchivar::structopt::*;
use log::{debug, log};

pub fn cwd() -> ::std::path::PathBuf {
    ::std::env::current_dir().unwrap()
}
