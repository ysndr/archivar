use log;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use app;
use args::Command;
use constants::*;
use error::*;
// use template::Template;

pub trait Action: Debug {
    fn run<'a>(&self,context: &'a app::Context) -> Result<()>;
}

#[derive(Debug, PartialEq)]
pub enum FileAction {
    Mkdir { path: PathBuf },
    Touch { path: PathBuf, mkparents: bool },
    Move { from: PathBuf, to: PathBuf },
    Copy { from: PathBuf, to: PathBuf },
    Chmod { target: PathBuf, mode: u32 },
    Message(String),
    // Git(GitAction),
    Shell(String, PathBuf),
    Noop,
}

impl Action for FileAction {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        Ok(())
    }
}


#[derive(Debug, Default)]
pub struct ActionSet {
    actions: Vec<Box<dyn Action>>,    
}

impl Action for ActionSet {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        for action in &self.actions {
            action.run(context)?;
        }
        Ok(())
    }
}
