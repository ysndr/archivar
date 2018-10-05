use crate::commands::{command, Context, Error};

use structopt::StructOpt;

/// `Init` Command
#[derive(StructOpt, Clone, Debug, PartialEq)]
pub struct Command {}

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
