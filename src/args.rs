use std::path::PathBuf;

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub enum Command {
    #[structopt(name = "init", about = "Initializes archivar")]
    Init,
    // archivar new path ..
    #[structopt(name = "new", about = "Creates new project")]
    New {
        #[structopt(parse(from_os_str), help = "Destination path",)]
        dest: PathBuf,

        #[structopt(parse(from_os_str), help = "Template path")]
        template: Option<PathBuf>,
    },

    #[structopt(name = "archive", about = "Archives project")]
    Archive {
        #[structopt(parse(from_os_str), help = "Target project path")]
        dir: PathBuf,
    },

    #[structopt(name = "unarchive", about = "Restores project from archive")]
    Unarchive {
        #[structopt(parse(from_os_str), help = "Target project path")]
        dir: PathBuf,
    },
}

#[derive(StructOpt, Debug, Clone, PartialEq)]
#[structopt(
    raw(setting = "structopt::clap::AppSettings::ColoredHelp"),
    name = "archivar",
    about = "the trackkeeper of your stuff"
)]
pub struct Args {
    #[structopt(
        short = "v",
        long = "verbosity",
        parse(from_occurrences),
        help = "Switches on verbosity (increase verbosity by applying multiple times)",
    )]
    pub verbosity: usize,

    #[structopt(help = "Disables git integration", long = "no-git")]
    pub git_disabled: bool,

    #[structopt(
        short = "p",
        long = "path",
        help = "The basedir of the archive",
        default_value = ".",
        parse(from_os_str)
    )]
    pub path: PathBuf,

    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub sub: Command,
}
