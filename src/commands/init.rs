use crate::commands::{command::*, Context, Error};

use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug, PartialEq)]
pub struct Init {}
