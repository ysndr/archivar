use crate::commands::{command, Context, Error};
use crate::constants;

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


#[cfg(test)]
mod tests {
    use super::{Command, Context};
    use crate::commands::command::*;
    use assert_fs;



    fn context()  -> Context { Context::default() }
    

    #[test]
    fn state() {
        let command = Command {};

        assert_eq!((), command.state(&context()));
    }

    #[test]
    fn check() {
        let command = Command {};
        
        let temp = assert_fs::TempDir::new().unwrap();
        let context = Context { path: temp.path().into(), ..context() };

        assert!(command.check(&context, &()).is_ok());
    }
}
