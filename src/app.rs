// use action::Actionable;
use shell::{Shell, Verbosity};
use std::cell::{RefCell, RefMut};
use std::env;
use std::path::PathBuf;

use args::{Args,Command};
use error::*;


#[derive(Debug)]
pub struct Context {
    cwd: PathBuf,
    path: PathBuf,
    shell: RefCell<Shell>,
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
    pub fn new(args: Args) -> Self {
        
        let mut shell = Shell::new();
        shell.set_verbosity(match args.verbosity {
            1 => Verbosity::Normal,
            _ => Verbosity::Verbose,
        });

        let cwd = env::current_dir().expect("couldn't get the current directory of the process");

        let context = Context {
            cwd,
            path: args.path,
            shell: RefCell::new(shell),
        };

        Archivar {
            command: args.sub,
            context,
        }
    }

    pub fn shell(&self) -> RefMut<Shell> {
        self.context.shell()
    }
}
