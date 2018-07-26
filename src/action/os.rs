
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
