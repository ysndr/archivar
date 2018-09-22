use shell::{Shell, Verbosity};
use std::cell::{RefCell, RefMut};
use std::env;
use std::path::PathBuf;

pub use args::{Args, Command};
use action::*;
use error::*;


#[derive(Debug)]
pub struct Context {
    pub cwd: PathBuf,
    pub path: PathBuf,
    pub shell: RefCell<Shell>,
}

impl Context {
    pub fn shell(&self) -> RefMut<Shell> {
        self.shell.borrow_mut()
    }
}

#[derive(Debug)]
pub struct Archivar {
    command: Command,
    pub context: Context,
}

impl Archivar {
    pub fn new(command: Command, context: Context) -> Self {
        Archivar {
            command,
            context,
        }
    }

    pub fn setup_context(args: &Args) -> Context {
        let mut shell = Shell::new();
        shell.set_verbosity(match args.verbosity {
            0 => Verbosity::Normal,
            1 => Verbosity::Normal,
            _ => Verbosity::Verbose,
        });

        let cwd = env::current_dir().expect("couldn't get the current directory of the process");

        Context {
            cwd,
            path: args.path.clone(),
            shell: RefCell::new(shell),
        }
    }

    pub fn shell(&self) -> RefMut<Shell> {
        self.context.shell()
    }

    pub fn run(&self) -> Result<()> {
        debug!("start");
        let action: Action = self.command.clone().into();
        action.run(&self.context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn args_to_app() {
       
        let sub = Command::Init;

        let args = Args {
            verbosity: 0,
            git_disabled: false,
            path: PathBuf::from("."),
            sub: sub.clone(),
        };



        let app = Archivar::new(sub, Archivar::setup_context(&args));
        
        assert_eq!(app.command, Command::Init);
        assert_eq!(app.context.path, PathBuf::from("."))
    }

}
