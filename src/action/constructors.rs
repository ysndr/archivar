use super::{check, template, Action, Message, OS};
use app;
use args::Command;
use error::*;
use predicates::prelude::*;

use constants;
use std::path::PathBuf;

pub fn make_init(_command: &Command) -> Action {
    debug!("make init actions");

    let mut actions = vec![];

    actions.push(
        check::Check::new(box |context| {
            not_in_managed_subdir(&context.path)?;
            is_no_archivar_root(&context.path)?;
            Ok(())
        }).into(),
    );

    actions.push(
        OS::Touch {
            path: constants::ARCHIVAR_FILE_NAME.into(),
            mkparents: true,
        }.into(),
    );
    actions.push(
        OS::Mkdir {
            path: constants::ARCHIVE_FOLDER_NAME.into(),
        }.into(),
    );

    Action::Group(actions)
}

pub fn make_archive(dir: &PathBuf) -> Action {
    debug!("make archive actions for `{}`", dir.display());

    let mut actions: Vec<Action> = vec![];
    let dir_copy = dir.clone();
    actions.push(
        check::Check::new(box move |context| {
            let project_path = context.path.join(dir_copy.clone());
            let archive_path = context
                .path
                .join(constants::ARCHIVE_FOLDER_NAME)
                .join(dir_copy.clone());

            debug!(
                "Check project_path ({}) and archive path ({})",
                project_path.display(),
                archive_path.display()
            );

            is_valid_root(&context.path)?;
            is_valid_project_path(&project_path)?;

            if archive_path.exists() {
                bail!("path `{}` already exists in archive", dir_copy.display());
            }

            Ok(())
        }).into(),
    );

    actions.push(
        OS::Move {
            from: dir.clone(),
            to: PathBuf::from(constants::ARCHIVE_FOLDER_NAME).join(&dir),
        }.into(),
    );

    Action::Group(actions)
}

pub fn make_unarchive(dir: &PathBuf) -> Action {
    debug!("make unarchive actions for `{}`", dir.display());

    let mut actions: Vec<Action> = vec![];
    let dir_copy = dir.clone();
    actions.push(
        check::Check::new(box move |context| {
            let project_path = context.path.join(dir_copy.clone());
            let archive_path = context
                .path
                .join(constants::ARCHIVE_FOLDER_NAME)
                .join(dir_copy.clone());

            is_valid_root(&context.path)?;
            is_valid_project_path(&archive_path)?;

            if project_path.exists() {
                bail!("path `{}` already exists in workspace", dir_copy.display());
            }

            Ok(())
        }).into(),
    );

    actions.push(
        OS::Move {
            from: PathBuf::from(constants::ARCHIVE_FOLDER_NAME).join(&dir),
            to: dir.clone(),
        }.into(),
    );

    Action::Group(actions)
}

pub fn make_new(dest: &PathBuf, template: &Option<PathBuf>) -> Action {
    let mut actions = vec![];

    let moved_dest = dest.clone();
    actions.push(
        check::Check::new(box move |context| {
            is_valid_root(&context.path)?;
            if context.path.join(&moved_dest).exists() {
                bail!(
                    "folder name for project {} already exists",
                    moved_dest.display()
                );
            }
            if context
                .path
                .join(constants::ARCHIVE_FOLDER_NAME)
                .join(&moved_dest)
                .exists()
            {
                bail!(
                    "archived folder for project {} exists",
                    moved_dest.display()
                );
            }
            not_in_project(&context.path, &moved_dest)?;
            Ok(())
        }).into(),
    );

    actions.push(
        OS::Touch {
            path: dest.join(constants::PROJECT_FILE_NAME),
            mkparents: true,
        }.into(),
    );
    actions.push(match template {
        None => Message::Info("no template given - skipping template generation".to_owned()).into(),
        Some(template_path) => template::Template::make(template_path, dest).into(), 
    });

    Action::Group(actions)
}

fn is_valid_root(path: &PathBuf) -> Result<()> {
     debug!("Ensure {} path is valid root", path.display());
    if !path.join(constants::ARCHIVAR_FILE_NAME).exists() {
        bail!(
            "your selected path `{}` is not an archivar dir",
            path.display()
        );
    }
    Ok(())
}

fn is_no_archivar_root(dir: &PathBuf) -> Result<()> {
    let mut path: PathBuf = "/".into();
    debug!("Ensure {} is not already an archivar path", dir.display());
    for comp in dir.components() {
        path = path.join(comp);
        if path.join(constants::ARCHIVE_FOLDER_NAME).exists() {
            bail!(
                "`{}` is subdir of an archivar path `{}`",
                dir.display(),
                path.display()
            );
        }
    }
    Ok(())
}

fn is_valid_project_path(dir: &PathBuf) -> Result<()> {
    let project_file_path = dir.join(constants::ARCHIVAR_FILE_NAME);
    debug!("Ensure {} path is valid", dir.display());
    if dir.starts_with(constants::TEMPLATE_NAMESPACE) {
        bail!(
            "its not allows to manage projects inside templates namespace (`{}`)",
            constants::TEMPLATE_NAMESPACE
        );
    }

    if !dir.exists() || project_file_path.exists() {
        bail!("no project file at `{}`", project_file_path.display());
    }

    Ok(())
}

fn not_in_managed_subdir(dir: &PathBuf) -> Result<()> {
    let mut path: PathBuf = "/".into();
    debug!("Ensure {} is not already managed", dir.display());
    for comp in dir.components() {
        path = path.join(comp);
        if path.join(constants::PROJECT_FILE_NAME).exists() {
            bail!(
                "`{}` is subdir of manged workspace `{}`",
                dir.display(),
                path.display()
            );
        }
    }
    Ok(())
}

fn not_in_project(root: &PathBuf, project_path: &PathBuf) -> Result<()> {
    let mut path = root.to_owned();
    for comp in project_path.components() {
        path = path.join(comp);
        if path.join(constants::PROJECT_FILE_NAME).exists() {
            bail!("`{}` is subdir of existing project", project_path.display());
        }
    }
    Ok(())
}
