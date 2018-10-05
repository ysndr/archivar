use super::app;
use crate::args::Command;
use crate::error::*;


// use template::Template;

mod check;
mod constructors;
mod message;
mod os;
pub mod template;

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
    #[cfg(test)]
    Wildcard(Wildcard),
}

impl ActionTrait for Action {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        match self {
            Action::OS(action) => action.run(context),
            Action::Message(action) => action.run(context),
            Action::Template(action) => action.run(context),
            Action::Check(action) => action.run(context),
            Action::Fail(action) => action.run(context),
            Action::Noop => Ok(()),
            #[cfg(test)]
            Action::Wildcard(action) => action.run(context),
            Action::Group(list) => {
                let elems = list.len();

                context
                    .shell()
                    .info(format!("Running {} actions...", elems))?;
                debug!("Group: {:?}", list);

                for (cur, action) in list.iter().enumerate() {
                    context
                        .shell()
                        .info(format!("Running action {} of {}", cur + 1, elems))?;
                    action.run(context)?;
                }
                context.shell().info("Done!")?;

                Ok(())
            }
        }
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
        }
    }
}

#[derive(Debug)]
#[cfg(test)]
pub struct Wildcard;
#[cfg(test)]
impl ActionTrait for Wildcard {
    fn run<'a>(&self, _context: &'a app::Context) -> Result<()> {
        Ok(())
    }
}
#[cfg(test)]
impl<T: ActionTrait> PartialEq<T> for Wildcard {
    fn eq(&self, _: &T) -> bool {
        true
    }
}
#[cfg(test)]
impl From<Wildcard> for Action {
    fn from(_: Wildcard) -> Action {
        Action::Wildcard(Wildcard)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use assert_fs::prelude::*;
    use super::*;
    use crate::constants::{ARCHIVAR_FILE_NAME, ARCHIVE_FOLDER_NAME, PROJECT_FILE_NAME};
    use crate::logger;

    #[test]
    fn action_set_from_init_command() {
        let archivar_file: PathBuf = ARCHIVAR_FILE_NAME.into();
        let archive_path: PathBuf = ARCHIVE_FOLDER_NAME.into();
        let mkparents = true;

        let command = Command::Init;
        let expected = Action::Group(vec![
            check::Check::new(box |_| Ok(())).into(),
            OS::Touch {
                path: archivar_file.clone(),
                mkparents,
            }.into(),
            OS::Mkdir {
                path: archive_path.clone(),
            }.into(),
        ]);

        assert_eq!(expected, Action::from(&command));
    }

    #[test]
    fn action_set_from_archive_command() {
        let _path: PathBuf = ARCHIVAR_FILE_NAME.into();
        let example_project: PathBuf = "examples/project".into();
        let archive_path = PathBuf::from(ARCHIVE_FOLDER_NAME).join(&example_project);

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
        let _path: PathBuf = ARCHIVAR_FILE_NAME.into();
        let example_project: PathBuf = "examples/project".into();
        let archive_path = PathBuf::from(ARCHIVE_FOLDER_NAME).join(&example_project);

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
        let example_project: PathBuf = "examples/archivar/project".into();

        let command = Command::New {
            dest: example_project.clone(),
            template: None,
        };

        let expected = Action::Group(vec![
            check::Check::new(box |_| Ok(())).into(),
            OS::Touch {
                path: example_project.join(PROJECT_FILE_NAME),
                mkparents: true,
            }.into(),
            Message::Info("".to_owned()).into(),
        ]);
        assert_eq!(expected, Action::from(&command));
    }

    #[test]
    fn action_set_from_new_command_with_template() {
        logger::setup_logger(log::LevelFilter::Trace).unwrap();

        let temp = assert_fs::TempDir::new().unwrap();
        temp.copy_from("example", &["*"]).unwrap();

        let example_project: PathBuf = temp.path().join("archivar/project");
        let template_file: PathBuf = temp.path().to_owned();

        let result_A = Action::from(Command::New {
            dest: example_project.clone(),
            template: Option::Some(template_file.clone()),
        });

        debug!("result: {:?}", result_A);

        temp.close().unwrap();
    }

}
