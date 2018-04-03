use std::path::{PathBuf, Path};
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use slog;

use command::Command;
use constants::*;
use error::*;


#[derive(Debug)]
pub enum Action {
    Mkdir { path: PathBuf },
    Touch { path: PathBuf, mkparents: bool },
    Move { from: PathBuf, to: PathBuf },
    Copy { from: PathBuf, to: PathBuf },
    Chmod { target: PathBuf, mode: u32 },
    Message(String),
    Git(GitAction),
    Noop,
}

pub trait Actionable {
    fn commit(&self, logger: &slog::Logger) -> io::Result<()>;
}


impl Actionable for Action {
    fn commit(&self, logger: &slog::Logger) -> io::Result<()> {
        match self {
            Action::Mkdir { path } => {
                info!(logger, "mkdir {}", path.display());
                fs::create_dir_all(&path)
            }
            Action::Touch { path, mkparents } => {
                info!(logger, "touch file {}", path.display());
                if *mkparents {
                    debug!(logger, "making parant paths");
                    fs::create_dir_all(&path.parent().unwrap())
                } else {
                    Ok(())
                }.and_then(|_| {
                    debug!(logger, "writing file to fs");
                    fs::OpenOptions::new().create(true).write(true).open(&path)
                })
                    .and(Ok(()))
            }
            Action::Move { from, to } => {
                info!(logger, "move '{}' -> '{}'", from.display(), to.display());
                fs::rename(&from, &to)
            }
            Action::Copy { from, to } => {
                info!(logger, "copy '{}' -> '{}'", from.display(), to.display());
                fs::copy(&from, &to).and(Ok(()))
            }
            Action::Chmod { target, mode } => {
                info!(
                    logger,
                    "set permission of '{}' to `{:x}`",
                    target.display(),
                    mode
                );
                target
                    .metadata()
                    .and_then(|meta| Ok(meta.permissions()))
                    .and_then(|mut perms| Ok(perms.set_mode(*mode)))
            }
            Action::Message(msg) => {
                info!(logger, "{}", msg);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}





impl Actionable for Vec<Action> {
    fn commit(&self, logger: &slog::Logger) -> io::Result<()> {
        for action in self.iter() {}
        Ok(())

    }
}


#[derive(Debug)]
struct GitAction;


impl Command {
    pub fn to_actions(&self, logger: &slog::Logger) -> Result<Vec<Action>> {
        let mut actions = Vec::new();
        match self {
            Command::Init { path, with_git } => {
                if path.exists() && !path.is_dir() {
                    return Err(Error::PathNoDirectory("init".to_string(), path.to_owned()));
                }
                if path.exists() && !path.read_dir().unwrap().count() > 0 {
                    return Err(Error::DirectoryNotEmpty(
                        "init".to_string(),
                        path.to_owned(),
                    ));
                } else {
                    let mut archivar_file_path = path.to_owned();
                    archivar_file_path.push(ARCHIVAR_FILE_NAME);
                    actions.push(Action::Touch {
                        path: archivar_file_path,
                        mkparents: true,
                    })
                }
                if *with_git {
                    actions.push(Action::Noop);
                }

                actions.push(Action::Message("done!".to_string()));

                Ok(actions)
            }
            Command::New {
                path,
                dir,
                template,
                template_args,
                no_commit,
            } => {
                if !path.is_relative() {
                    return Err(Error::PathNotRelative("new".to_string(), path.to_owned()));
                }

                if !dir.join(ARCHIVAR_FILE_NAME).exists() {
                    return Err(Error::NoArchivarFound("new".to_string(), dir.to_owned()));
                }

                let abs_path = dir.join(path);

                // Question: add to archivar afterwards
                // if abs_path.exists() {
                //     return Err(Error::DirectoryExists(
                //         "new".to_string(),
                //         abs_path.to_owned(),
                //     ));
                // }

                let mut parents = vec![abs_path.parent().unwrap()];
                while parents.last().unwrap() != dir {
                    let last = *parents.last().unwrap();
                    debug!(
                        logger,
                        "adding parent '{}'",
                        last.parent().unwrap().display()
                    );
                    parents.push(last.parent().unwrap());
                }

                for parent in parents.iter() {
                    if parent.join(Path::new(PROJECT_FILE_NAME)).exists() {
                        return Err(Error::ProjectExists(
                            "new".to_string(),
                            path.to_owned(),
                            false,
                        ));
                    }
                }

                if path.starts_with("archive") {
                    return Err(Error::ArchiveReferenced(
                        "new".to_string(),
                        abs_path.to_owned(),
                    ));
                }

                let archived_abs_path = dir.join("archive").join(path);

                if archived_abs_path.exists() {
                    return Err(Error::ProjectExists(
                        "new".to_string(),
                        abs_path.to_owned(),
                        true,
                    ));
                }

                // TODO: implement templating

                // TODO: implement git
                actions.push(Action::Touch {
                    path: abs_path.to_owned(),
                    mkparents: true,
                });

                Ok(actions)
            }
            Command::Archive {
                dir,
                path,
                no_commit,
            } => {
                if !path.is_relative() {
                    return Err(Error::PathNotRelative(
                        "archive".to_string(),
                        path.to_owned(),
                    ));
                }

                if !dir.join(ARCHIVAR_FILE_NAME).exists() {
                    return Err(Error::NoArchivarFound("new".to_string(), dir.to_owned()));
                }

                let abs_path = dir.join(path);

                if !abs_path.exists() {
                    return Err(Error::NoSuchFileOrDirectory(
                        "archive".to_string(),
                        abs_path.to_owned(),
                    ));
                }

                let project_file_path = abs_path.join(PROJECT_FILE_NAME);
                if !project_file_path.exists() {
                    return Err(Error::NoProjectFound(
                        "archive".to_string(),
                        abs_path.to_owned(),
                    ));
                }

                if path.starts_with("archive") {
                    return Err(Error::ArchiveReferenced(
                        "new".to_string(),
                        abs_path.to_owned(),
                    ));
                }

                let archived_abs_path = dir.join("archive").join(path);

                actions.push(Action::Move {
                    from: abs_path,
                    to: archived_abs_path,
                });

                Ok(actions)
            }

            Command::Archive {
                dir,
                path,
                no_commit,
            } => {
                if !path.is_relative() {
                    return Err(Error::PathNotRelative(
                        "unarchive".to_string(),
                        path.to_owned(),
                    ));
                }

                if !dir.join(ARCHIVAR_FILE_NAME).exists() {
                    return Err(Error::NoArchivarFound("new".to_string(), dir.to_owned()));
                }



                let abs_path;
                let archived_abs_path;

                if path.starts_with("archive") {
                    abs_path = dir.join(path.strip_prefix("archive").unwrap());
                    archived_abs_path = dir.join(path);
                } else {
                    abs_path = dir.join(path);
                    archived_abs_path = dir.join("archive").join(path);
                }


                if !archived_abs_path.exists() {
                    return Err(Error::NoSuchFileOrDirectory(
                        "unarchive".to_string(),
                        abs_path.to_owned(),
                    ));
                }

                let project_file_path = archived_abs_path.join(PROJECT_FILE_NAME);
                if !project_file_path.exists() {
                    return Err(Error::NoProjectFound(
                        "unarchive".to_string(),
                        archived_abs_path.to_owned(),
                    ));
                }


                actions.push(Action::Move {
                    from: archived_abs_path,
                    to: abs_path,
                });

                Ok(actions)
            }

            _ => Ok(Vec::new()),
        }
    }
}
