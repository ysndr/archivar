use std::path::PathBuf;
use crate::constants;
use super::Result;
use log::*;

pub fn is_valid_root(path: &PathBuf) -> Result<()> {
    debug!("Ensure {} path is valid root", path.display());
    if !path.join(constants::ARCHIVAR_FILE_NAME).exists() {
        bail!(
            "your selected path `{}` is not an archivar dir",
            path.display()
        );
    }
    Ok(())
}

pub fn is_no_archivar_root(dir: &PathBuf) -> Result<()> {
    let mut path: PathBuf = "/".into();
    debug!("Ensure {} is not already an archivar path", dir.display());
    for comp in dir.components() {
        path = path.join(comp);
        if path.join(constants::ARCHIVAR_FILE_NAME).exists() {
            bail!(
                "`{}` is subdir of an archivar path `{}`",
                dir.display(),
                path.display()
            );
        }
    }
    Ok(())
}

pub fn is_valid_project_path(dir: &PathBuf) -> Result<()> {
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

pub fn not_in_managed_subdir(dir: &PathBuf) -> Result<()> {
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

pub fn not_in_project(root: &PathBuf, project_path: &PathBuf) -> Result<()> {
    let mut path = root.to_owned();
    for comp in project_path.components() {
        path = path.join(comp);
        if path.join(constants::PROJECT_FILE_NAME).exists() {
            bail!("`{}` is subdir of existing project", project_path.display());
        }
    }
    Ok(())
}
