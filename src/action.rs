use slog;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use app;
use command::Command;
use constants::*;
use error::*;
use template::Template;

#[derive(Debug)]
pub enum Action {
    Mkdir { path: PathBuf },
    Touch { path: PathBuf, mkparents: bool },
    Move { from: PathBuf, to: PathBuf },
    Copy { from: PathBuf, to: PathBuf },
    Chmod { target: PathBuf, mode: u32 },
    Message(String),
    Git(GitAction),
    Shell(String, PathBuf),
    Noop,
}

pub trait Actionable: Debug {
    fn commit(&self, logger: &slog::Logger) -> Result<()>;
}

impl Actionable for Action {
    fn commit(&self, logger: &slog::Logger) -> Result<()> {
        match self {
            Action::Mkdir { path } => {
                info!(logger, "mkdir {}", path.display());
                fs::create_dir_all(&path)?;
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
                })?;
            }
            Action::Move { from, to } => {
                info!(logger, "move '{}' -> '{}'", from.display(), to.display());
                fs::rename(&from, &to)?;
            }
            Action::Copy { from, to } => {
                info!(logger, "copy '{}' -> '{}'", from.display(), to.display());
                fs::copy(&from, &to)?;
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
                    .and_then(|mut perms| Ok(perms.set_mode(*mode)))?
            }
            Action::Message(msg) => {
                info!(logger, "{}", msg);
            }
            Action::Shell(command, cwd) => {
                // info!(logger, "execute.."; "command" => ?command);

                debug!(
                    logger,
                    "executing command {} from {}",
                    command,
                    cwd.display()
                )
            }
            _ => {}
        }
        Ok(())
        // if we got here nothing failed
    }
}

impl Actionable for Vec<Action> {
    fn commit(&self, logger: &slog::Logger) -> Result<()> {
        for action in self.iter() {}
        Ok(())
    }
}

#[derive(Debug)]
struct GitAction;

impl Command {
    pub fn to_actions(&self, config: &app::Config) -> Result<Vec<Box<Actionable>>> {
        let logger = config.log;
        let shell = config.shell;

        let mut actions: Vec<Box<Actionable>> = Vec::new();
        match self {
            Command::Init { path, with_git } => {
                debug!(logger, "creating actions for command::init");
                if path.exists() && !path.is_dir() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "not a directory".to_owned(),
                    ).into());
                }
                if path.exists() && !path.read_dir().unwrap().count() > 0 {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "not empty".to_owned(),
                    ).into());
                } else {
                    let mut archivar_file_path = path.to_owned();
                    archivar_file_path.push(ARCHIVAR_FILE_NAME);
                    actions.push(Box::new(Action::Touch {
                        path: archivar_file_path,
                        mkparents: true,
                    }))
                }
                if *with_git {
                    actions.push(Box::new(Action::Noop));
                }

                actions.push(Box::new(Action::Message("done!".to_string())));

                Ok(actions)
            }

            Command::New {
                path,
                dir,
                template,
                template_args,
                no_commit,
            } => {
                debug!(logger, "creating actions for command::new");
                let dir = dir.canonicalize()?;

                if !path.is_relative() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "expected relative path".to_owned(),
                    ).into());
                }

                if !dir.join(ARCHIVAR_FILE_NAME).exists() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "ARCHIVAR_ROOT (dir)".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "failed to find Archivar managed directory".to_owned(),
                    ).into());
                }

                let abs_path = dir.join(path);
                debug!(logger, "abspath"; "abspath" => %abs_path.display());

                // Question: add to archivar afterwards
                // if abs_path.exists() {
                //     return Err(Error::DirectoryExists(
                //         "new".to_string(),
                //         abs_path.to_owned(),
                //     ).into());
                // }

                let mut parents = vec![abs_path.parent().unwrap()];
                while *parents.last().unwrap() != dir {
                    let last = *parents.last().unwrap();
                    trace!(
                        logger,
                        "adding parent dir to search for project"; "directory" =>  %last.parent().unwrap().display()
                    );
                    parents.push(last.parent().unwrap());
                }

                for parent in parents.iter() {
                    if parent.join(Path::new(PROJECT_FILE_NAME)).exists() {
                        return Err(ErrorKind::InvalidCommandArgs(
                            "path".to_owned(),
                            path.to_str().unwrap().to_owned(),
                            "is inside an existing project".to_owned(),
                        ).into());
                    }
                }

                if path.starts_with("archive") {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "is archived".to_owned(),
                    ).into());
                }

                let archived_abs_path = dir.join("archive").join(path);

                if archived_abs_path.exists() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "already exists in archived (use `archivar unarchive <path>`)".to_owned(),
                    ).into());
                }

                // TODO: implement git
                actions.push(Box::new(Action::Touch {
                    path: abs_path.to_owned(),
                    mkparents: true,
                }));

                // TODO: implement templating
                if let Some(template_path) = template {
                    let template_actions = Template::make(&template_path, &abs_path, logger)?;
                    actions.push(Box::new(template_actions));
                }

                Ok(actions)
            }

            Command::Archive {
                dir,
                path,
                no_commit,
            } => {
                debug!(logger, "creating actions for command::archive");

                let dir = dir.canonicalize()?;

                if !path.is_relative() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "is not relative".to_owned(),
                    ).into());
                }

                if !dir.join(ARCHIVAR_FILE_NAME).exists() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "ARCHIVAR_ROOT (dir)".to_owned(),
                        dir.to_str().unwrap().to_owned(),
                        "no Archivar found".to_owned(),
                    ).into());
                }

                let abs_path = dir.join(path);

                if !abs_path.exists() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "<ARCHIVAR_ROOT>/<path>".to_owned(),
                        abs_path.to_str().unwrap().to_owned(),
                        "No such directory".to_owned(),
                    ).into());
                }

                let project_file_path = abs_path.join(PROJECT_FILE_NAME);
                if !project_file_path.exists() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "could not find project here".to_owned(),
                    ).into());
                }

                if path.starts_with("archive") {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "points into archive".to_owned(),
                    ).into());
                }

                let archived_abs_path = dir.join("archive").join(path);

                actions.push(Box::new(Action::Move {
                    from: abs_path,
                    to: archived_abs_path,
                }));

                Ok(actions)
            }

            Command::Unarchive {
                dir,
                path,
                no_commit,
            } => {
                let dir = dir.canonicalize()?;

                debug!(logger, "creating actions for command::unarchive");
                if !path.is_relative() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "is not relative".to_owned(),
                    ).into());
                }

                if !dir.join(ARCHIVAR_FILE_NAME).exists() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "ARCHIVAR_ROOT (dir)".to_owned(),
                        dir.to_str().unwrap().to_owned(),
                        "no Archivar found".to_owned(),
                    ).into());
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
                    return Err(ErrorKind::InvalidCommandArgs(
                        "<ARCHIVAR_ROOT>/<path>".to_owned(),
                        abs_path.to_str().unwrap().to_owned(),
                        "No such directory".to_owned(),
                    ).into());
                }

                let project_file_path = archived_abs_path.join(PROJECT_FILE_NAME);
                if !project_file_path.exists() {
                    return Err(ErrorKind::InvalidCommandArgs(
                        "path".to_owned(),
                        path.to_str().unwrap().to_owned(),
                        "could not find project here".to_owned(),
                    ).into());
                }

                actions.push(Box::new(Action::Move {
                    from: archived_abs_path,
                    to: abs_path,
                }));

                Ok(actions)
            }

            _ => Ok(Vec::new()),
        }
    }
}
