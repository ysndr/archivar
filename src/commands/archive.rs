use crate::commands::{command::*, Context, Error};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug, PartialEq)]
pub struct Archive {
    #[structopt(parse(from_os_str), help = "Target project path")]
    dir: PathBuf,
}
