use super::{check, Action, OS};
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
            Ok(())
        }).into(),
    );

    actions.push(
        OS::Touch {
            path: constants::ARCHIVAR_FILE_NAME.into(),
            mkparents: true,
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

            is_valid_root(&context)?;
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

            is_valid_root(&context)?;
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

fn make_new(dest: &PathBuf, template: &PathBuf) -> Action {

    Action::Noop
}




fn is_valid_root(context: &app::Context) -> Result<()> {
    if !context.path.join(constants::ARCHIVAR_FILE_NAME).exists() {
        bail!(
            "your selected path `{}` is not an archivar dir",
            context.path.display()
        );
    }

    Ok(())
}

fn is_valid_project_path(dir: &PathBuf) -> Result<()> {
    let project_file_path = dir.join(constants::ARCHIVAR_FILE_NAME);

    if dir.starts_with(constants::TEMPLATE_NAMESPACE) {
        bail!("its not allows to manage projects inside templates namespace (`{}`)", constants::TEMPLATE_NAMESPACE);
    }

    if !dir.exists() || project_file_path.exists() {
        bail!("no project file at `{}`", project_file_path.display());
    }

    Ok(())
}

fn not_in_managed_subdir(dir: &PathBuf) -> Result<()> {
    let mut path: PathBuf = "/".into();
    for comp in dir.components() {
        path = path.join(comp);
        if path.join(constants::PROJECT_FILE_NAME).exists() {
            bail!("`{}` is subdir of manged workspace `{}`", dir.display(), path.display());
        }
    }
    Ok(())

}
