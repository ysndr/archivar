use app;
use args::Command;
use constants::*;
use error::*;
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

// impl Action for ActionSet {
//     fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
//         for action in &self.actions {
//             action.run(context)?;
//         }
//         Ok(())
//     }

//     fn kind<'a>() -> &'a str {
//         "actionset"
//     }
// }

impl From<Command> for Action {
    fn from(command: Command) -> Action {
        let actions: Vec<Action> = match command {
            Command::Init => vec![
                Action::Noop,
                Action::Message(Message::Info("hello".to_owned())),
            ],
            _ => vec![],
        };

        Action::Group(actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn action_set_from_init_command() {
        let command = Command::Init;
    }
}
