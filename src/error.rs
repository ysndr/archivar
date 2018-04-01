use std;
use std::path::PathBuf;


#[derive(Debug)]
pub enum Error {
    NoSuchFileOrDirectory(String, PathBuf),
    FileExists(String, PathBuf),
    DirectoryExists(String, PathBuf),
    DirectoryNotEmpty(String, PathBuf),
    PathNoDirectory(String, PathBuf),
    PathNotRelative(String, PathBuf),
    NoProjectFound(String, PathBuf),
    NoArchivarFound(String, PathBuf),
    ProjectExists(String, PathBuf, bool),
    ArchiveReferenced(String, PathBuf),
}

// type aliea for result
pub type Result<T> = std::result::Result<T, Error>;
