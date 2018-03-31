#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate clap;

use std::path::{PathBuf,Path};
use std::env;

use slog::DrainExt;

use clap::{Arg, App, SubCommand, ArgMatches};


fn main() {
    println!("Hello, world!");




    let url = matches.value_of("URL").unwrap();
    println!("{}", url);

}

fn match_args() -> ArgMatches {
    let matches = App::new("Archivar")
        .version("0.1.0")
        .author("Yannik Sander <me@ysndr.de>")
        .about("Tool to archive projects")
        .arg(Argwith_name("VERBOCITY")
            .required(false)
            .short("v")
            .multiple(true))
        .subcommand(SubCommand::with_name("init")
            .about("command to execute")
            .arg(Arg::with_name("PATH")
                 .required(false)
                 .takes_value(true)
                 .index(1))
            .arg(Arg::with_name("GIT_DISABLED")
                .help("disable git")
                .long("no-git")
                .required(false)
                .takes_value(false)))
        .subcommand(SubCommand::with_name("new")
            .about("create new project")
            .arg(Arg::with_name("PATH")
                 .required(false)
                 .takes_value(true)
                 .index(1))
            .arg(Arg::with_name("ARCHIVAR_ROOT")
                .help("override root dir")
                .short("d")
                .long("dir")
                .required(false)
                .takes_value(true))
            .arg(Arg::with_name("TEMPLATE")
                .help("template to use")
                .short("t")
                .long("template")
                .required(false)
                .takes_value(true))
            .arg(Arg::with_name("TEMPLATE_ARGS")
                .required(false)
                .multiple(true))
            .arg(Arg::with_name("NO_COMMIT")
                 .help("inhibit git commit")
                 .long("no-commit")
                 .required(false)
                 .takes_value(false)))
        .get_matches();
}

fn log_level(matches: &ArgMatches) -> slog:: {
    let min_log_level = match matches.occurrences_of("verbose") {
        0 => slog::Level::Info,
        1 => slog::Level::Debug,
        2 | _ => slog::Level::Trace,
    };
    min_
}


fn matches_to_command(matches: &ArgMatches) -> Command {
    match matches.subcommand() {
        ("init", Some(sub_m)) =>  { Command::Init::test() },
        _                     =>  { Command::Empty }
    }
}


#[derive(Debug)]
enum Command {
    // archivar init ..
    Init {
        path: PathBuf,
        with_git: bool,
    },
    // archivar new path ..
    New {
        path: PathBuf,
        dir: PathBuf,

        template: PathBuf,
        template_attr: String,
        template_args: Vec<String>,

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
    Empty
}



impl From<&ArgMatches> for Command::Init {
    fn from(matches: &ArgMatches) -> bool {
        let path = matches.value_of("PATH")
            .map_or(env::current_dir(), PathBuf::from);

        let no_git = matches.value_of("GIT_DISABLED").is_some();

        Command::Init { path: path, with_git: !no_git }
    }
}
