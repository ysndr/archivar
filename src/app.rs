use log::*;
use shell::{Shell, Verbosity};
use std::cell::{RefCell, RefMut};
use std::env;
use std::path::PathBuf;

use crate::error::*;

#[derive(Debug)]
pub struct Context {
    pub path: PathBuf,         // project root path
    pub shell: RefCell<Shell>, // a global handle to a configured Shell instance
}
impl Default for Context {
    fn default() -> Context {
        Context {
            path: std::path::Path::new(".").canonicalize().unwrap(),
            shell: RefCell::default(),
        }
    }
}

impl Context {
    pub fn shell(&self) -> RefMut<'_, Shell> {
        self.shell.borrow_mut()
    }
}

// #[derive(Debug)]
// pub struct Archivar {
//     command: Command,
//     pub context: Context,
// }

// impl Archivar {
//     pub fn new(command: Command, context: Context) -> Self {
//         Archivar { command, context }
//     }

//     pub fn setup_context(args: &Args) -> Context {
//         let mut shell = Shell::new();
//         shell.set_verbosity(match args.verbosity {
//             0 => Verbosity::Normal,
//             1 => Verbosity::Normal,
//             _ => Verbosity::Verbose,
//         });

//         let cwd = env::current_dir().expect("couldn't get the current directory of the process");

//         Context {
//             path: args.path.clone(),
//             shell: RefCell::new(shell),
//         }
//     }

//     pub fn shell(&self) -> RefMut<'_, Shell> {
//         self.context.shell()
//     }

//     pub fn run(&self) -> Result<()> {
//         debug!("start");
//         let action: Action = self.command.clone().into();
//         action.run(&self.context)
//     }
// }
