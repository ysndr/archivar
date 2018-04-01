#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

extern crate clap;

use slog::Drain;
use std::path::{PathBuf, Path};
use std::env;

const ARCHIVAR_FILE_NAME: &'static str = ".archivar";
const TEMPLATE_FILE_NAME: &'static str = "template.yaml";



// use slog::DrainExt;

use clap::{Arg, App, SubCommand, ArgMatches};

fn main() {
    println!("Hello, world!");

    let matches = match_args();
    let logger = create_logger(&matches).unwrap();
    let command = Command::from_matches(&matches, &logger);

    info!(&logger, "info");
    debug!(&logger, "debug");

}

fn match_args() -> ArgMatches<'static> {
    let matches = App::new("Archivar")
        .version("0.1.0")
        .author("Yannik Sander <me@ysndr.de>")
        .about("Tool to archive projects")
        .arg(
            Arg::with_name("VERBOSITY")
                .required(false)
                .takes_value(false)
                .short("v")
                .multiple(true),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("command to execute")
                .arg(
                    Arg::with_name("PATH")
                        .required(false)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("GIT_DISABLED")
                        .help("disable git")
                        .long("no-git")
                        .required(false)
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("create new project")
                .arg(
                    Arg::with_name("PATH")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("ARCHIVAR_ROOT")
                        .help("override root dir")
                        .short("d")
                        .long("dir")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("TEMPLATE")
                        .help("template to use")
                        .short("t")
                        .long("template")
                        .required(false)
                        .takes_value(true),
                )
                .arg(Arg::with_name("TEMPLATE_ARGS").required(false).multiple(
                    true,
                ))
                .arg(
                    Arg::with_name("NO_COMMIT")
                        .help("inhibit git commit")
                        .long("no-commit")
                        .required(false)
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("archive")
                .about("archive project")
                .arg(
                    Arg::with_name("PATH")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("ARCHIVAR_ROOT")
                        .help("override root dir")
                        .short("d")
                        .long("dir")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("NO_COMMIT")
                        .help("inhibit git commit")
                        .long("no-commit")
                        .required(false)
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("unarchive")
                .about("unarchive project")
                .arg(
                    Arg::with_name("PATH")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("ARCHIVAR_ROOT")
                        .help("override root dir")
                        .short("d")
                        .long("dir")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("NO_COMMIT")
                        .help("inhibit git commit")
                        .long("no-commit")
                        .required(false)
                        .takes_value(false),
                ),
        )
        .get_matches();
    matches
}

fn create_logger(matches: &ArgMatches) -> Option<slog::Logger> {
    println!("{}", matches.occurrences_of("VERBOSITY"));

    let min_log_level = match matches.occurrences_of("VERBOSITY") {
        0 => slog::Level::Warning,
        1 => slog::Level::Info,
        2 => slog::Level::Debug,
        3 | _ => slog::Level::Trace,
    };

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog::LevelFilter::new(drain, min_log_level).ignore_res();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    info!(&logger, "logger created");

    Some(logger)
}





#[derive(Debug)]
enum Command<'a> {
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

impl<'a> Command<'a> {
    fn from_matches(matches: &'a ArgMatches, logger: &slog::Logger) -> Command<'a> {
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
    // }

    fn to_actions(&self, logger: &slog::Logger) -> Result<Vec<Action>> {
        let mut actions = Vec::new();
        match self {
            Command::Init { path, with_git } => {
                if !path.is_dir() {
                    return Err(Error::NoSuchFileOrDirectory(
                        "init".to_string(),
                        path.to_owned(),
                    ));
                }
                if !path.exists() {
                    actions.push(Action::Create {
                        path: path.to_owned(),
                        iotype: IOType::Directory,
                    })
                } else if !path.read_dir().unwrap().count() > 0 {
                    return Err(Error::DirectoryNotEmpty(
                        "init".to_string(),
                        path.to_owned(),
                    ));
                } else {
                    let mut archivar_file_path = path.to_owned();
                    archivar_file_path.push(ARCHIVAR_FILE_NAME);
                    actions.push(Action::Create {
                        path: archivar_file_path,
                        iotype: IOType::File,
                    })
                }
                if *with_git {
                    actions.push(Action::Noop);
                }

                actions.push(Action::Message("done!".to_string()));

                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        }

    }
}

#[derive(Debug)]
enum IOType {
    File,
    Directory,
}

#[derive(Debug)]
enum Action {
    Create { path: PathBuf, iotype: IOType },
    Move { from: PathBuf, to: PathBuf },
    Copy { from: PathBuf, to: PathBuf },
    Message(String),
    Git(GitAction),
    Noop,
}


impl Action {
    fn commit(&self) {
        match self {
            Action::Create { path, iotype } => {}
            Action::Move { from, to } => {}
            Action::Copy { from, to } => {}
            Action::Git(git_action) => {}
            Action::Message(String) => {}
            Noop => {}
        }
    }
}

#[derive(Debug)]
struct GitAction;



enum Error {
    NoSuchFileOrDirectory(String, PathBuf),
    FileExists(String, PathBuf),
    DirectoryExists(String, PathBuf),
    DirectoryNotEmpty(String, PathBuf),
}

// type aliea for result
type Result<T> = std::result::Result<T, Error>;
