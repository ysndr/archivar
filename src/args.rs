use std::path::PathBuf;

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
pub struct Args {
    #[structopt(
        short = "v",
        long = "verbosity",
        parse(from_occurrences),
        help = "switch on verbosity"
    )]
    pub verbosity: usize,

    #[structopt(help = "disable git integration", long = "no-git")]
    pub git_disabled: bool,

    #[structopt(
        short = "p",
        long = "path",
        default_value = ".",
        parse(from_os_str)
    )]
    pub path: PathBuf,

    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub sub: Command,
}
