use crate::commands::{command, Context, Error};
use std::path::PathBuf;
use structopt::StructOpt;

/// `New` Command
#[derive(StructOpt, Debug, Clone, PartialEq)]
pub struct Command {
    #[structopt(parse(from_os_str), help = "Destination path")]
    dest: PathBuf,

    #[structopt(parse(from_os_str), help = "Template path")]
    template: Option<PathBuf>,
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
