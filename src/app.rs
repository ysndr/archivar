// use action::Actionable;
use shell::{ColorChoice, Shell, Verbosity};
use std::cell::{RefCell, RefMut};
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

use error::*;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "init")]
    Init,
    // archivar new path ..
    #[structopt(name = "new")]
    New {
        #[structopt(parse(from_os_str), help = "destination path")]
        dest: PathBuf,

        #[structopt(parse(from_os_str), help = "template path")]
        template: Option<PathBuf>,
    },

    #[structopt(name = "archive")]
    Archive {
        #[structopt(parse(from_os_str), help = "target path")]
        dir: PathBuf,
    },

    #[structopt(name = "unarchive")]
    Unarchive {
        #[structopt(parse(from_os_str), help = "target path")]
        dir: PathBuf,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "archivar", about = "the trachkeeper of your stuff")]
pub struct Opt {
    #[structopt(
        short = "v",
        long = "verbosity",
        parse(from_occurrences),
        help = "switch on verbosity"
    )]
    verbosity: usize,

    #[structopt(help = "disable git integration", long = "no-git")]
    git_disabled: bool,

    #[structopt(
        short = "p",
        long = "path",
        default_value = ".",
        parse(from_os_str)
    )]
    path: PathBuf,

    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    sub: Command,
}

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
    pub fn new() -> Self {
        let opt = Opt::from_args();

        let mut shell = Shell::new();
        shell.set_verbosity(match opt.verbosity {
            1 => Verbosity::Normal,
            _ => Verbosity::Verbose,
        });

        let cwd = env::current_dir().expect("couldn't get the current directory of the process");

        let context = Context {
            cwd,
            path: opt.path,
            shell: RefCell::new(shell),
        };

        Archivar {
            command: opt.sub,
            context,
        }
    }

    pub fn shell(&self) -> RefMut<Shell> {
        self.context.shell()
    }
}
