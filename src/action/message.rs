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
        match self {
            Action::Error(message) =>  context.shell().error(message),
            Action::Warn(message) =>  context.shell().warn(message),
            Action::Info(message) => context.shell().info(message),
        }.map_err(|e| e.into())
    }
}

impl From<Action> for super::Action {
    fn from(action: Action) -> super::Action {
        super::Action::Message(action)
    }
}
