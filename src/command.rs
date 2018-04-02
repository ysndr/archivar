use std::path::{PathBuf, Path};
use std::env;

use slog;
use clap::{Arg, App, SubCommand, ArgMatches};


#[derive(Debug)]
pub enum Command<'a> {
    // archivar init ..
    Init { path: PathBuf, with_git: bool },
    // archivar new path ..
    New {
        path: PathBuf,
        dir: PathBuf,

        template: Option<PathBuf>,
        // template_attr: String,
        template_args: Vec<&'a str>,

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

impl<'a> Default for Command<'a> {
    fn default() -> Self {
        Command::Empty
    }
}

impl<'a> Command<'a> {
    pub fn from_matches(matches: &'a ArgMatches, logger: &slog::Logger) -> Command<'a> {
        let command = match matches.subcommand() {
            ("init", Some(sub_m)) => Self::init(sub_m),
            ("new", Some(sub_m)) => Self::new(sub_m),
            ("archive", Some(sub_m)) => Self::archive(sub_m),
            ("unarchive", Some(sub_m)) => Self::unarchive(sub_m),
            _ => Command::Empty,
        };

        info!(logger, "command given: {:?}", command);
        command
    }

    fn init(matches: &ArgMatches) -> Command<'a> {
        let path = matches.value_of("PATH").map_or(
            env::current_dir().unwrap(),
            |p| {
                let pp = PathBuf::from(p);
                if pp.is_relative() {
                    env::current_dir().unwrap().join(pp)
                } else {
                    pp
                }
            },
        );
        let no_git = matches.is_present("GIT_DISABLED");

        Command::Init {
            path: path,
            with_git: !no_git,
        }
    }

    fn new(matches: &'a ArgMatches) -> Command<'a> {
        let root = matches.value_of("ARCHIVAR_ROOT").map_or(
            env::current_dir().unwrap(),
            PathBuf::from,
        );
        let path = PathBuf::from(matches.value_of("PATH").unwrap());
        let template = matches.value_of("TEMPLATE").map_or(None, |t| {
            Some(root.join(Path::new(t)))
        });
        let template_args = matches
            .values_of("TEMPLATE_ARGS")
            .unwrap_or_default()
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

    fn archive(matches: &ArgMatches) -> Command<'a> {
        let root = matches.value_of("ARCHIVAR_ROOT").map_or(
            env::current_dir().unwrap(),
            PathBuf::from,
        );
        let path = PathBuf::from(matches.value_of("PATH").unwrap());
        let no_commit = matches.is_present("NO_COMMIT");

        Command::Archive {
            dir: root,
            path: path,

            no_commit: no_commit,
        }
    }

    fn unarchive(matches: &ArgMatches) -> Command<'a> {
        let root = matches.value_of("ARCHIVAR_ROOT").map_or(
            env::current_dir().unwrap(),
            PathBuf::from,
        );
        let path = PathBuf::from(matches.value_of("PATH").unwrap());
        let no_commit = matches.is_present("NO_COMMIT");

        Command::Unarchive {
            dir: root,
            path: path,

            no_commit: no_commit,
        }
    }
}
