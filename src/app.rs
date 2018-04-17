use clap::{App, AppSettings, Arg, ArgMatches, ClapResult, SubCommand};
use slog;
use sloggers::types::Severity;
use std::rc::Rc;

use action::Action;
use command::Command;
use error::*;
use logger::Logger;

#[derive(Debug)]
pub struct Config<'a> {
    log: &'a Logger,
}

#[derive(Debug)]
pub struct Archivar<'a> {
    config: Config<'a>,
    command: Option<Command>,
    actions: Option<Vec<Action>>,
}

impl<'args> Archivar<'args> {
    pub fn build_command(&mut self) -> Result<()> {
        match Command::from_matches(&self.matches, &self.logger) {
            Ok(command) => {
                self.command = command;
                debug!(self.logger, "`build_command` was ok");
                Ok(())
            }
            Err(e) => {
                debug!(self.logger, "`build_command` was not ok");
                Err(e)
            }
        }
    }
    pub fn build_actions(&mut self) -> Result<()> {
        match self.command.to_actions(&self.logger) {
            Ok(actions) => {
                self.actions = actions;
                debug!(self.logger, "`to_actions` was ok");
                Ok(())
            }
            Err(e) => {
                debug!(self.logger, "`to_actions` was not ok");
                Err(e)
            }
        }
    }

    fn handle_error(&self, e: Error) {}
}

pub fn parse_args<'a>() -> Result<(u64, ArgMatches<'a>)> {
    App::new("Archivar")
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
        .get_matches_safe()
        .and_then(|matches| Ok((matches.occurrences_of("VERBOSITY"), matches)))
        .or_else(|e| bail!(ErrorKind::Clap(e)))
}
