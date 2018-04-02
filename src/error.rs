use std;
use std::path::PathBuf;
use std::fmt;


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
    CommandUnknown(String),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoSuchFileOrDirectory(cmd, path) => {
                write!(f, "[{}] NoSuchFileOrDirectory '{}'", cmd, path.display())
            }
            _ => write!(f, "hello"),

        }
    }
}

// type aliea for result
pub type Result<T> = std::result::Result<T, Error>;
