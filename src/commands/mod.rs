pub mod archive;
pub mod command;
pub mod init;
pub mod new;
pub mod unarchive;

mod utils;

pub type Context = crate::app::Context;
pub type Error = crate::error::Error;
pub type Result<T> = crate::error::Result<T>;

#[test]
fn test() {
    assert!(false);
}
