use std::rc::Rc;
use clap::{Arg, App, SubCommand, ArgMatches};
use slog;

use logger::Logger;
use command::Command;
use action::Action;


#[derive(Debug, Default)]
pub struct Archivar<'a> {
    logger: Rc<Logger>,
    matches: ArgMatches<'a>,
    command: Command<'a>,
    actions: Vec<Action>,
}


impl<'a> Archivar<'a> {
    pub fn match_args(&mut self) {
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
        self.matches = matches;
    }
    pub fn configure_logger(&mut self) {
        let min_log_level = match self.matches.occurrences_of("VERBOSITY") {
            0 => slog::Level::Warning,
            1 => slog::Level::Info,
            2 => slog::Level::Debug,
            3 | _ => slog::Level::Trace,
        };

        self.logger = Rc::new(Logger::new(min_log_level));
    }
}
