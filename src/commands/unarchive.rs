use crate::commands::{command, Context, Error};

use std::path::PathBuf;
use structopt::StructOpt;

/// `Unarchive` Command
#[derive(StructOpt, Debug, Clone, PartialEq)]
pub struct Command {
    #[structopt(parse(from_os_str), help = "Target project path")]
    dir: PathBuf,
}

impl command::State<Context> for Command {
    type State = ();
    fn state(&self, context: &Context) -> Self::State {
        ()
    }
}

impl command::Check<Context, ()> for Command {
    type Error = Error;
    fn check(&self, context: &Context, state: &()) -> Result<(), Self::Error> {
        unimplemented!();
    }
}

impl command::Execute<Context, ()> for Command {
    type Error = Error;
    fn execute(&self, context: &Context, state: &()) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

impl command::Command<Context, (), Error> for Command {}
