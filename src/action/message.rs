use super::ActionTrait;
use error::*;
use app;


#[derive(Debug, PartialEq)]
pub enum Action {
    Info(String),
    Warn(String),
    Error(String),
}

impl ActionTrait for Action {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        Ok(())
    }
}
