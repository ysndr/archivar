
use std::path::PathBuf;


#[derive(Debug, PartialEq)]
pub enum Action {
    Mkdir { path: PathBuf },
    Touch { path: PathBuf, mkparents: bool },
    Move { from: PathBuf, to: PathBuf },
    Copy { from: PathBuf, to: PathBuf },
    Chmod { target: PathBuf, mode: u32 },

    Shell(String, PathBuf),
}

impl From<Action> for super::Action {
    fn from(action: Action) -> super::Action {
        super::Action::OS(action)
    }
}
