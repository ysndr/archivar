use app;
use args::Command;
use constants::*;
use error::*;
use predicates::prelude::*;
use std::path::PathBuf;

use constants;

use assert_fs::prelude::*;
// use template::Template;

mod check;
mod constructors;
mod message;
mod os;
mod template;

use self::check::Check;
use self::check::Fail;
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
    Check(Check),
    Fail(Fail),
    Noop,
}

impl ActionTrait for Action {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        Ok(())
    }
}

impl<'a> From<&'a Command> for Action {
    fn from(command: &Command) -> Action {
        command.to_owned().into()
    }
}

// TODO: why is `impl <T: AsRef<Command> From<T>` not working
impl From<Command> for Action {
    fn from(command: Command) -> Action {
        match command {
            Command::Init => constructors::make_init(&command),
            Command::Archive { dir } => constructors::make_archive(&dir),
            Command::Unarchive { dir } => constructors::make_unarchive(&dir),
            Command::New { dest, template } => constructors::make_new(&dest, &template),

            _ => Action::Noop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_set_from_init_command() {
        let path: PathBuf = constants::ARCHIVAR_FILE_NAME.into();
        let mkparents = true;

        let command = Command::Init;
        let expected = Action::Group(vec![
            check::Check::new(box |_| Ok(())).into(),
            OS::Touch { path, mkparents }.into(),
        ]);

        assert_eq!(expected, Action::from(&command));
    }

    #[test]
    fn action_set_from_archive_command() {
        let path: PathBuf = constants::ARCHIVAR_FILE_NAME.into();
        let example_project: PathBuf = "examples/project".into();
        let archive_path = PathBuf::from(constants::ARCHIVE_FOLDER_NAME).join(&example_project);

        let command = Command::Archive {
            dir: example_project.clone(),
        };
        let expected = Action::Group(vec![
            check::Check::new(box |_| Ok(())).into(),
            OS::Move {
                from: example_project,
                to: archive_path,
            }.into(),
        ]);
        assert_eq!(expected, Action::from(&command));
    }

    #[test]
    fn action_set_from_unarchive_command() {
        let path: PathBuf = constants::ARCHIVAR_FILE_NAME.into();
        let example_project: PathBuf = "examples/project".into();
        let archive_path = PathBuf::from(constants::ARCHIVE_FOLDER_NAME).join(&example_project);

        let command = Command::Unarchive {
            dir: example_project.clone(),
        };
        let expected = Action::Group(vec![
            check::Check::new(box |_| Ok(())).into(),
            OS::Move {
                from: archive_path,
                to: example_project,
            }.into(),
        ]);
        assert_eq!(expected, Action::from(&command));
    }

    #[test]
    fn action_set_from_new_command() {
        let example_project: PathBuf = "examples/project".into();

        let command = Command::New {
            dest: example_project.clone(),
            template: None,
        };
        let expected = Action::Group(vec![
            check::Check::new(box |_| Ok(())).into(),
            OS::Touch {
                path: example_project.join(constants::PROJECT_FILE_NAME),
                mkparents: true,
            }.into(),
            Message::Info("".to_owned()).into(),
        ]);
        assert_eq!(expected, Action::from(&command));
    }

}
