use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, PartialEq)]
pub struct New {
    #[structopt(parse(from_os_str), help = "Destination path",)]
    dest: PathBuf,

    #[structopt(parse(from_os_str), help = "Template path")]
    template: Option<PathBuf>,
}
