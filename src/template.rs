use action::{Action, ActionSet, Actionable};
use constants::{GITKEEP_FILE_NAME, TEMPLATE_FILE_NAME};
use error::*;
use serde_yaml;
use slog::Logger;
use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug)]
pub struct Template {
    actions: Vec<Box<Actionable>>,
    // TODO:
    // arguments: ???
}

impl Template {
    pub fn make(template_path: &Path, project_path: &Path, logger: &Logger) -> Result<Self> {
        debug!(logger, "making Template actions"; "template" => %template_path.display(), "project" => %project_path.display());

        let template_file = template_path.join(TEMPLATE_FILE_NAME);

        let config = TemplateConfig::from_file(&template_file)?;

        let mut actions = Vec::new();

        let mut init_command_actions =
            make_init_command_actions(config.init.as_ref().unwrap_or(&vec![]), project_path);

        let mut mkpath_actions =
            make_mkpath_actions(config.paths.as_ref().unwrap_or(&vec![]), project_path);

        let mut include_actions = make_include_actions(
            config.include.as_ref().unwrap_or(&BTreeMap::new()),
            template_path,
            project_path,
        );

        actions.append(&mut init_command_actions);
        actions.append(&mut mkpath_actions);
        actions.append(&mut include_actions);

        debug!(logger, "read config from file";
               "file" => %template_path.display(),
               "config" => format!("{:#?}", config));

        Ok(Template { actions })
    }
}

impl<'a> ActionSet<'a> for Template {
    fn get_actionables(&'a self) -> Box<Iterator<Item = &Box<Actionable>> + 'a> {
        Box::new(self.actions.iter())
    }
}

fn make_init_command_actions(init_lines: &Vec<String>, cwd: &Path) -> Vec<Box<Actionable>> {
    let mut actions: Vec<Box<Actionable>> = vec![];

    for action_str in init_lines.iter() {
        actions.push(Box::new(Action::Shell(
            action_str.to_string(),
            cwd.to_path_buf(),
        )));
    }
    actions
}

fn make_mkpath_actions(paths: &Vec<PathBuf>, cwd: &Path) -> Vec<Box<Actionable>> {
    let mut actions: Vec<Box<Actionable>> = vec![];

    for path in paths.iter().filter(|p| p.is_relative()) {
        let mut path = cwd.join(path);
        // path.push(GITKEEP_FILE_NAME);
        actions.push(Box::new(Action::Mkdir { path }));
    }

    actions
}

fn make_include_actions(
    includes: &BTreeMap<PathBuf, Option<IncludeOptions>>,
    template_dir: &Path,
    cwd: &Path,
) -> Vec<Box<Actionable>> {
    let mut actions: Vec<Box<Actionable>> = vec![];

    for (path, options) in includes {
        // determine source file/folder
        let from = if path.is_relative() {
            template_dir.join(path)
        } else {
            path.to_owned()
        };

        let to = match options {
            Some(o) if o.dest.is_some() => cwd.join(o.dest.to_owned().unwrap()),
            _ => cwd.join(Path::new(path.file_name().unwrap())),
        };

        actions.push(Box::new(Action::Copy { from, to }));
    }

    actions
}

impl Actionable for Template {
    fn commit(&self, logger: &Logger) -> Result<()> {
        debug!(logger, "commiting actions"; "n" => self.actions.len());

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateConfig {
    init: Option<Vec<String>>,
    paths: Option<Vec<PathBuf>>,
    include: Option<BTreeMap<PathBuf, Option<IncludeOptions>>>,
}

impl TemplateConfig {
    pub fn from_file(template_path: &Path) -> Result<Self> {
        let file = fs::File::open(template_path)?;
        let buf_reader = io::BufReader::new(file);
        let template: TemplateConfig = serde_yaml::from_reader(buf_reader)?;
        Ok(template)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct IncludeOptions {
    dest: Option<PathBuf>,
    extract: Option<bool>,
    gitignore: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn reader_test() {
    //     use logger;
    //     let template = Template::make(
    //         &Path::new("./example"),
    //         &Path::new("example"),
    //         &logger::Logger::new(2),
    //     );
    //
    //     println!("template: {:#?}", template);
    // }
    //
    // #[test]
    // fn exec_test() {
    //     use std::process::Command;
    //     let shell = "bash";
    //     let cmd = "echo hello && echo world";
    //
    //     println!(
    //         "{:?}",
    //         Command::new(shell)
    //             .arg("-c")
    //             .arg("--")
    //             .arg(cmd)
    //             .output()
    //             .unwrap()
    //     );
    // }

    #[test]
    fn read_from_file() {
        let config = TemplateConfig::from_file(&Path::new("test/.template.yaml")).unwrap();

        assert_eq!(config.include.unwrap().len(), 6);
        assert_eq!(config.init.unwrap().len(), 2);
        assert_eq!(config.paths.unwrap().len(), 3);
    }

    #[test]
    fn creates_actions() {
        use logger;

        let now = ::std::time::SystemTime::now();
        let mut temp_dir = ::std::env::temp_dir();
        temp_dir.push("archivar-test");

        let template = Template::make(&Path::new("test"), &temp_dir, &logger::Logger::new(2));

        println!("{:#?}", template);

        ::std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn makes_paths() {
        let paths = vec![
            PathBuf::from("src/"),
            PathBuf::from("nested/deeply/wow/so/deep"),
        ];

        let cwd = &Path::new("/tmp/ARCHIVAR");
        let actions = make_mkpath_actions(&paths, cwd);

        assert_eq!(actions.len(), 2);
        assert_eq!(
            actions.iter().fold(0, |sum, action| sum + match action {
                Action::Mkdir { path: _ } => 1,
                _ => 0,
            }),
            2
        )
    }

}
