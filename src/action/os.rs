use crate::error::*;
use fs_extra::dir;
use log::*;
use shell;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// OS Specific Actions
///
/// Struct holds root path and shell for simplified internal use
pub struct Action<'a> {
    root: PathBuf,
    shell: &'a mut shell::Shell,
}
pub type Continue<'a> = Result<&'a Action<'a>>;

impl<'a> Action<'a> {
    pub fn new(root: PathBuf, shell: &'a mut shell::Shell) -> Action {
        Action { root, shell }
    }

    pub fn mkdir(&self, path: PathBuf) -> Continue {
        info!("mkdir ({})", path.display());
        fs::create_dir_all(self.root.join(path))?;
        Ok(self)
    }
    pub fn touch(&self, path: PathBuf) -> Continue {
        info!("touch ({})", path.display());
        fs::create_dir_all(self.root.join(&path).parent().unwrap())?;
        fs::File::create(self.root.join(&path))?;
        Ok(self)
    }
    pub fn mv(&self, from: PathBuf, to: PathBuf) -> Continue {
        info!("move ({} -> {})", from.display(), to.display());
        let from = self.root.join(from);
        let to = self.root.join(to);
        let options = dir::CopyOptions {
            copy_inside: true,
            ..dir::CopyOptions::new()
        };

        fs::create_dir_all(to.parent().unwrap())?;
        if from.is_dir() {
            dir::move_dir(from, to, &options)?;
        } else {
            fs::rename(from, to)?;
        }
        Ok(self)
    }
    pub fn cp(&self, from: PathBuf, to: PathBuf) -> Continue {
        info!("copy ({} -> {})", from.display(), to.display());
        let from = self.root.join(from);
        let to = self.root.join(to);
        let options = dir::CopyOptions {
            copy_inside: true,
            ..dir::CopyOptions::new()
        };

        fs::create_dir_all(self.root.join(&to).parent().unwrap())?;
        if from.is_dir() {
            dir::copy(from, to, &options)?;
        } else {
            fs::copy(from, self.root.join(to))?;
        }
        Ok(self)
    }
    pub fn include(&self, from: PathBuf, to: PathBuf) -> Continue {
        info!("include file ({} -> {})", from.display(), to.display());
        let to = self.root.join(to);
        let options = dir::CopyOptions {
            copy_inside: true,
            ..dir::CopyOptions::new()
        };

        fs::create_dir_all(self.root.join(&to).parent().unwrap())?;
        if from.is_dir() {
            dir::copy(from, to, &options)?;
        } else {
            fs::copy(from, self.root.join(to))?;
        }
        Ok(self)
    }
    pub fn shell(&self, command: String, cwd: Option<PathBuf>) -> Continue {
        let mut base = Command::new("sh");

        let process = base.arg("-c").arg(command);

        let process = if let Some(cwd) = cwd {
            let cwd = self.root.join(cwd);
            self.shell
                .info(format!("setting shell cwd to {}", cwd.display()));
            process.current_dir(cwd)
        } else {
            process
        };

        self.shell.info(format!("command: `{}`", command)).unwrap();

        let status = process.status()?;

        self.shell
            .info(match status.code() {
                Some(code) => format!("command exited with status code: {}", code),
                None => "command terminated by signal".into(),
            })
            .unwrap();
        Ok(self)
    }
}
