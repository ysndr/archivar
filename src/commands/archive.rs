use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Clone, Debug, PartialEq)]
pub struct Archive {
        #[structopt(parse(from_os_str), help = "Target project path")]
        dir: PathBuf,
    }
