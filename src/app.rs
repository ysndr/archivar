use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use slog;
use sloggers::types::Severity;
use std::cmp::min;
use std::rc::Rc;

use action::Action;
use command::Command;
use error::*;
use logger::Logger;

#[derive(Debug)]
pub struct Config<'a> {
    log: &'a Logger,
    matches: ArgMatches<'a>,
}

impl<'a> Config<'a> {
    pub fn new(log: &'a Logger, matches: ArgMatches<'a>) -> Self {
        Self { log, matches }
    }
}

#[derive(Debug)]
pub struct Archivar<'a> {
    config: &'a Config<'a>,
    command: Option<Command>,
    actions: Option<Vec<Action>>,
}

impl<'a> Archivar<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            command: None,
            actions: None,
        }
    }
    pub fn build_command(&mut self) -> Result<()> {
        let result = Command::from_matches(&self.config.matches, self.config.log);
        match result {
            Ok(command) => {
                self.command = Some(command);
                debug!(self.config.log, "`build_command` was ok");
                Ok(())
            }
            Err(e) => bail!(e),
        }
    }

    pub fn build_actions(&mut self) -> Result<()> {
        let result = match &self.command {
            Some(command) => command.to_actions(self.config.log),
            None => bail!("no command yet"),
        };

        match result {
            Ok(actions) => {
                self.actions = Some(actions);
                debug!(self.config.log, "`build_actions` was ok");
                Ok(())
            }
            Err(e) => bail!(e),
        }
    }
    fn handle_error(&self, e: Error) {}
}

pub fn parse_args<'a>() -> (u64, ArgMatches<'a>) {
    let matches = App::new("Archivar")
        .version(crate_version!())
        .author("Yannik Sander <me@ysndr.de>")
        .about("Tool to archive projects")
        .arg(
            Arg::with_name("VERBOSITY")
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
                .arg(
                    Arg::with_name("TEMPLATE_ARGS")
                        .required(false)
                        .multiple(true),
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
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::StrictUtf8)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    (matches.occurrences_of("VERBOSITY"), matches)
}
