use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, PartialEq)]
pub struct Unarchive {
    #[structopt(parse(from_os_str), help = "Target project path")]
    dir: PathBuf,
}
