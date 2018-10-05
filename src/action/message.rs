use super::ActionTrait;
use crate::app;
use crate::error::*;

#[derive(Debug)]
pub enum Action {
    Info(String),
    Warn(String),
    Error(String),
}

impl ActionTrait for Action {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        match self {
            Action::Error(message) => context.shell().error(message),
            Action::Warn(message) => context.shell().warn(message),
            Action::Info(message) => context.shell().info(message),
        }
        .map_err(|e| e.into())
    }
}

impl From<Action> for super::Action {
    fn from(action: Action) -> super::Action {
        super::Action::Message(action)
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        use self::Action::*;
        match (self, other) {
            (Info(_), Info(_)) => true,
            (Warn(_), Warn(_)) => true,
            (Error(_), Error(_)) => true,
            _ => false,
        }
    }
}
