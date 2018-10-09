use crate::action::OS;
use crate::commands::{command, utils, Context, Error, Result};
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
    fn check(&self, context: &Context, state: &()) -> Result<()> {
        utils::is_no_archivar_root(&context.path)?;
        Ok(())
    }
}

impl command::Execute<Context, ()> for Command {
    type Error = Error;
    fn execute(&self, context: &Context, _state: &()) -> Result<()> {
        OS::new(context.path.clone(), &mut context.shell())
            .touch(constants::ARCHIVAR_FILE_NAME.into())?;
            Ok(())
    }
}

impl command::Command<Context, (), Error> for Command {}

#[cfg(test)]
mod tests {
    use super::{constants, Command, Context};
    use assert_fs::prelude::*;
    use crate::commands::command::*;
    use predicates::prelude::*;
    use std::path::PathBuf;

    fn context() -> Context {
        Context::default()
    }

    #[test]
    fn state() {
        let command = Command {};

        assert_eq!((), command.state(&context()));
    }

    #[test]
    fn check_ok_if_empty() {
        let command = Command {};

        let temp = assert_fs::TempDir::new().unwrap();
        let context = Context {
            path: temp.into_path(),
            ..context()
        };
        assert!(command.check(&context, &()).is_ok());
    }

    #[test]
    fn check_ok_if_not_exists() {
        let command = Command {};

        let temp = assert_fs::TempDir::new().unwrap();

        let path: PathBuf = temp.path().join("inner");
        let context = Context {
            path: path.clone(),
            ..context()
        };

        assert!(command.check(&context, &()).is_ok());
    }

    #[test]
    fn check_err_if_archivar_exits() {
        let command = Command {};

        let temp = assert_fs::TempDir::new().unwrap();
        let child = temp.child(constants::ARCHIVAR_FILE_NAME).touch();
        let context = Context {
            path: temp.into_path(),
            ..context()
        };

        assert!(command.check(&context, &()).is_err());
    }

    #[test]
    fn check_err_if_inside_archivar() {
        let command = Command {};

        let temp = assert_fs::TempDir::new().unwrap();
        let _ = temp.child(constants::ARCHIVAR_FILE_NAME).touch();
        let inside = temp.child("somewhere/inside");

        let context = Context {
            path: inside.path().into(),
            ..context()
        };
        let result = command.check(&context, &());

        assert!(result.is_err());
    }

    #[test]
    fn execute() {
        let command = Command {};

        let temp = assert_fs::TempDir::new().unwrap();
        let context = Context {
            path: temp.path().into(),
            ..context()
        };

        assert!(command.execute(&context, &()).is_ok());
        assert!(temp.child(constants::ARCHIVAR_FILE_NAME).path().exists());
    }

}