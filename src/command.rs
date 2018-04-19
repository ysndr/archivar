use std::env;
use std::path::{Path, PathBuf};

use clap::{App, Arg, ArgMatches, SubCommand};
use slog;

use error::*;

#[derive(Debug)]
pub enum Command {
    // archivar init ..
    Init {
        path: PathBuf,
        with_git: bool,
    },
    // archivar new path ..
    New {
        path: PathBuf,
        dir: PathBuf,

        template: Option<PathBuf>,
        // template_attr: String,
        template_args: Vec<String>, // TODO: revisit

        no_commit: bool,
    },
    Archive {
        path: PathBuf,
        dir: PathBuf,
        no_commit: bool,
    },
    Unarchive {
        path: PathBuf,
        dir: PathBuf,
        no_commit: bool,
    },
    Empty,
}

impl Default for Command {
    fn default() -> Self {
        Command::Empty
    }
}

impl Command {
    pub fn from_matches(matches: &ArgMatches, logger: &slog::Logger) -> Result<Command> {
        let command = match matches.subcommand() {
            ("init", Some(sub_m)) => Ok(Self::init(sub_m)),
            ("new", Some(sub_m)) => Ok(Self::new(sub_m)),
            ("archive", Some(sub_m)) => Ok(Self::archive(sub_m)),
            ("unarchive", Some(sub_m)) => Ok(Self::unarchive(sub_m)),
            (command, _) => Err(ErrorKind::CommandUnknown(command.to_owned()).into()),
        };

        info!(logger, "command given: {:?}", command);
        command
    }

    fn init(matches: &ArgMatches) -> Command {
        let path = matches
            .value_of("PATH")
            .map_or(env::current_dir().unwrap(), |p| {
                let pp = PathBuf::from(p);
                if pp.is_relative() {
                    env::current_dir().unwrap().join(pp)
                } else {
                    pp
                }
            });
        let no_git = matches.is_present("GIT_DISABLED");

        Command::Init {
            path: path,
            with_git: !no_git,
        }
    }

    fn new(matches: &ArgMatches) -> Command {
        let root = matches
            .value_of("ARCHIVAR_ROOT")
            .map_or(env::current_dir().unwrap(), PathBuf::from);
        let path = PathBuf::from(matches.value_of("PATH").unwrap());
        let template = matches
            .value_of("TEMPLATE")
            .map_or(None, |t| Some(root.join(Path::new(t))));
        let template_args: Vec<String> = matches
            .values_of("TEMPLATE_ARGS")
            .unwrap_or_default()
            .map(|_str| String::from(_str))
            .collect();
        let no_commit = matches.is_present("NO_COMMIT");

        Command::New {
            dir: root,
            path: path,

            template: template,
            template_args: template_args,
            no_commit: no_commit,
        }
    }

    fn archive(matches: &ArgMatches) -> Command {
        let root = matches
            .value_of("ARCHIVAR_ROOT")
            .map_or(env::current_dir().unwrap(), PathBuf::from);
        let path = PathBuf::from(matches.value_of("PATH").unwrap());
        let no_commit = matches.is_present("NO_COMMIT");

        Command::Archive {
            dir: root,
            path: path,

            no_commit: no_commit,
        }
    }

    fn unarchive(matches: &ArgMatches) -> Command {
        let root = matches
            .value_of("ARCHIVAR_ROOT")
            .map_or(env::current_dir().unwrap(), PathBuf::from);
        let path = PathBuf::from(matches.value_of("PATH").unwrap());
        let no_commit = matches.is_present("NO_COMMIT");

        Command::Unarchive {
            dir: root,
            path: path,

            no_commit: no_commit,
        }
    }
}
