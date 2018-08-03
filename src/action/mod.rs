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
    Check(check::Check),
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
            Command::Init => Self::make_init(&command),
            Command::Archive { dir } => Self::make_archive(&dir),
            _ => Action::Noop,
        }
    }
}

impl Action {
    fn make_init(_command: &Command) -> Action {
        debug!("make init actions");

        let mut actions = vec![];

        actions.push(
            check::Check::new(box |context| {
                if !predicate::path::missing()
                    .eval(&context.path.join(constants::ARCHIVAR_FILE_NAME))
                {
                    bail!("There is an achivar dir in here already");
                }
                Ok(())
            }).into(),
        );

        actions.push(
            OS::Touch {
                path: constants::ARCHIVAR_FILE_NAME.into(),
                mkparents: true,
            }.into(),
        );

        Action::Group(actions)
    }

    fn make_archive(dir: &PathBuf) -> Action {
        debug!("make archive actions for `{}`", dir.display());

        let mut actions: Vec<Action> = vec![];
        let dir_copy = dir.clone();
        actions.push(
            check::Check::new(box move |context| {
                if !context.path.join(constants::ARCHIVAR_FILE_NAME).exists() {
                    bail!(
                        "your selected path `{}` is not an archivar dir",
                        context.path.display()
                    );
                }

                let project_path = context.path.join(dir_copy.clone());
                let project_file_path = context
                    .path
                    .join(dir_copy.clone())
                    .join(constants::ARCHIVAR_FILE_NAME);

                let archive_path = context
                    .path
                    .join(constants::ARCHIVE_FOLDER_NAME)
                    .join(dir_copy.clone());

                if !project_path.exists() || project_file_path.exists() {
                    bail!("no project at `{}`", project_path.display());
                }

                if archive_path.exists() {
                    bail!(
                        "path `{}` already exists in archive",
                        project_path.display()
                    );
                }

                Ok(())
            }).into(),
        );

        actions.push(
            OS::Move {
                from: dir.clone(),
                to: PathBuf::from(constants::ARCHIVE_FOLDER_NAME).join(&dir),
            }.into(),
        );

        Action::Group(actions)
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

}
