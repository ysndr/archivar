use app;
use args::Command;
use constants::*;
use error::*;
use std::borrow::Borrow;

use assert_fs::prelude::*;
// use template::Template;

mod message;
mod os;
mod template;

use self::message::Action as Message;
use self::os::Action as OS;

pub trait ActionTrait {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()>;
}

#[derive(Debug, PartialEq)]
pub enum Action {
    OS(OS),
    // Git(GitAction),
    Message(Message),
    Group(Vec<Action>),
    Template(template::Template),
    Noop,
}


impl ActionTrait for Action {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        Ok(())
    }
}



// TODO: why is `impl <T: AsRef<Command> From<T>` not working
impl From<Command> for Action {
    fn from(command: Command) -> Action {
        let actions: Vec<Action> = match command {
            Command::Init => vec![Action::Noop, Message::Info("hello".to_owned()).into()],
            _ => vec![],
        };

        Action::Group(actions)
    }
}
impl<'a> From<&'a Command> for Action {
    fn from(command: &Command) -> Action {
        command.to_owned().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_set_from_init_command() {
        let command = Command::Init;
        let expected = Action::Group(vec![
            Action::Noop,
            Message::Info("hello".to_owned()).into()]);

        assert_eq!(expected, Action::from(&command));
        assert_eq!(expected, command.into());
    }
}
