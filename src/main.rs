fn main() {
    println!("Hello, world!");
    
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
}
