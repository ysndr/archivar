use action::ActionTrait;
use app;
use error::*;
use fs_extra::dir;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// OS Specific Actions
///
/// Basically, the enum variants hold information about target paths used to
/// perform filesystem related tasks in the ActionTrait implementation
#[derive(Debug, PartialEq)]
pub enum Action {
    Mkdir {
        path: PathBuf,
    },

    Touch {
        path: PathBuf,
        mkparents: bool,
    },

    Move {
        from: PathBuf,
        to: PathBuf,
    },

    Copy {
        from: PathBuf,
        to: PathBuf,
    },

    Chmod {
        target: PathBuf,
        ro: bool,
    },

    Shell {
        command: String,
        cwd: Option<PathBuf>,
    },
}

impl From<Action> for super::Action {
    fn from(action: Action) -> super::Action {
        super::Action::OS(action)
    }
}

/// ActionTraitt implementation
///
/// Mostly proxy the `std` provided methods.
/// For shell commands we reach out for a bash shell
/// to run the given command sring in
impl ActionTrait for Action {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        use action::os::Action::*;

        let root = &context.path;
        let mut shell = context.shell();

        match self {
            // create folders
            Mkdir { path } => {
                info!("mkdir ({})", path.display());
                fs::create_dir_all(root.join(path))?;
            }

            // create files
            Touch { path, mkparents } => {
                info!("touch ({})", path.display());
                if *mkparents {
                    fs::create_dir_all(root.join(path).parent().unwrap())?;
                }
                fs::File::create(root.join(path))?;
            }

            // move files/folders
            Move { from, to } => {
                info!("move ({} -> {})", from.display(), to.display());
                let from = root.join(from);
                let to = root.join(to);

                fs::create_dir_all(to.parent().unwrap())?;
                if from.is_dir() {
                    dir::move_dir(
                        from,
                        to,
                        &dir::CopyOptions {
                            copy_inside: true,
                            ..dir::CopyOptions::new()
                        },
                    )?;
                } else {
                    fs::rename(from, to)?;
                }
            }

            // copy files/folders
            Copy { from, to } => {
                info!("copy ({} -> {})", from.display(), to.display());
                fs::create_dir_all(root.join(to).parent().unwrap())?;
                if from.is_dir() {
                    dir::copy(
                        from,
                        root.join(to),
                        &dir::CopyOptions {
                            copy_inside: true,
                            ..dir::CopyOptions::new()
                        },
                    )?;
                } else {
                    fs::copy(from, root.join(to))?;
                }
            }

            // chmod files/folders
            Chmod { target, ro } => {
                info!(
                    "chmod ({} := {})",
                    target.display(),
                    if *ro { "ro" } else { "rw" }
                );

                let mut perms = fs::metadata(root.join(target))?.permissions();
                perms.set_readonly(*ro);
                fs::set_permissions(root.join(target), perms)?;
            }

            // run shell commands
            Shell { command, cwd } => {
                let mut base = Command::new("sh");

                let process = base.arg("-c").arg(command);

                let process = if let Some(cwd) = cwd {
                    let cwd = root.join(cwd);
                    error!("{}", cwd.display());
                    process.current_dir(cwd)
                } else {
                    process
                };

                shell.info(format!("command: `{}`", command)).unwrap();

                let status = process.status()?;

                shell
                    .info(match status.code() {
                        Some(code) => format!("command exited with status code: {}", code),
                        None => "command terminated by signal".to_string(),
                    }).unwrap();
            }
        };

        Ok(())
    }
}
