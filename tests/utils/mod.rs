use libarchivar::app::{Args, Command};
use libarchivar::app::Archivar as App;

use log::{debug, log};
use libarchivar::logger;
use libarchivar::structopt::*;


pub fn cwd() -> ::std::path::PathBuf {
    ::std::env::current_dir().unwrap()
}
