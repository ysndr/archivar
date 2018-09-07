use libarchivar::app::Args;
use libarchivar::app::Archivar as App;

use log::{debug, log};
use libarchivar::logger;
use libarchivar::structopt::*;


pub fn make_args(subcommand: &str, arg_vec: &[&str], sub_arg_vec: &[&str] ) -> Args {
    let clap = Args::clap();

    let args = [&["archivar"], arg_vec, &["-vvv"], &[subcommand], sub_arg_vec].concat();

    debug!("args: {:?}", args);

    let matches = clap.get_matches_from(args);
    Args::from_clap(&matches)
}
