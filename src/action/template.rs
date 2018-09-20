use super::{Action, ActionTrait, Check, Fail};
use constants::{GITKEEP_FILE_NAME, TEMPLATE_FILE_NAME};
use error::*;
use serde_yaml;
use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::{fs, io};

use log;

use app;

use super::Message;
use super::OS;

#[derive(Debug, PartialEq)]
pub struct Template {
    action: Box<Action>,
}

impl From<Template> for super::Action {
    fn from(action: Template) -> super::Action {
        super::Action::Template(action)
    }
}

impl From<Result<Template>> for super::Action {
    fn from(result: Result<Template>) -> super::Action {
        match result {
            Err(e) => Fail::new(e.to_string()).into(),
            Ok(t) => t.into(),
        }
    }
}

impl Template {
    pub fn make(template_path: &Path, project_path: &Path) -> Result<Self> {
        // debug!(logger, "making Template actions"; "template" => %template_path.display(), "project" => %project_path.display());

        let (template_file, template_dir) = canonicalize_template_path(template_path);

        let config = TemplateConfig::from_file(&template_file)?;

        let mut actions = Vec::new();

        let mut init_command_actions =
            make_init_command_actions(config.init.as_ref().unwrap_or(&vec![]), project_path);

        let mut mkpath_actions =
            make_mkpath_actions(config.paths.as_ref().unwrap_or(&vec![]), project_path);

        let mut include_actions = make_include_actions(
            config.include.as_ref().unwrap_or(&BTreeMap::new()),
            &template_dir,
            project_path,
        );

        actions.append(&mut init_command_actions);
        actions.append(&mut mkpath_actions);
        actions.append(&mut include_actions);

        // debug!(logger, "read config from file";
        //        "file" => %template_path.display(),
        //        "config" => format!("{:#?}", config));

        Ok(Template {
            action: box Action::Group(actions),
        })
    }
}

fn make_init_command_actions(init_lines: &Vec<String>, cwd: &Path) -> Vec<Action> {

    error!("{}", cwd.display());

    let mut actions: Vec<Action> = vec![];

    for action_str in init_lines.iter() {
        actions.push(Action::OS(OS::Shell {
            command: action_str.to_string(),
            cwd: Some(cwd.to_path_buf()),
        }));
    }
    actions
}

fn make_mkpath_actions(paths: &Vec<PathBuf>, cwd: &Path) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];

    for path in paths.iter().filter(|p| p.is_relative()) {
        let mut path = cwd.join(path);
        // path.push(GITKEEP_FILE_NAME);
        actions.push(Action::OS(OS::Mkdir { path }));
    }

    actions
}

fn make_include_actions(
    includes: &BTreeMap<PathBuf, Option<IncludeOptions>>,
    template_dir: &PathBuf,
    project_dir: &Path,
) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];

    for (path, options) in includes {
        // determine source file/folder
        let from = if path.is_relative() {
            template_dir.join(path)
        } else {
            path.to_owned()
        };

        let to = options.clone().and_then(|o| {
            o.dest.map(|dest| {
                let mut o_dir = project_dir.join(dest.to_owned());
                
                if dest.to_str().unwrap().ends_with('/') {
                    o_dir = o_dir.join(from.file_name().unwrap());
                }
                o_dir
            })
        }).unwrap_or_else(|| project_dir.join(Path::new(from.file_name().unwrap())));
        

        actions.push(Action::OS(OS::Copy { from, to }));
    }

    actions
}

impl ActionTrait for Template {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        context.shell().info("Building by template...")?;
        self.action.run(context)?;
        context.shell().info("Done!")?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateConfig {
    init: Option<Vec<String>>,
    paths: Option<Vec<PathBuf>>,
    include: Option<BTreeMap<PathBuf, Option<IncludeOptions>>>,
}

impl TemplateConfig {
    pub fn from_file(template_file: &PathBuf) -> Result<Self> {
        debug!("create config from template: {:?}", template_file);
        let file = fs::File::open(template_file)?;
        let buf_reader = io::BufReader::new(file);
        let template: TemplateConfig = serde_yaml::from_reader(buf_reader)?;
        Ok(template)
    }
}


pub fn canonicalize_template_path(template_path: &Path) -> (PathBuf, PathBuf) {
    if template_path.is_file() {
            (template_path.to_owned(), template_path.parent().unwrap().to_owned())
    } else {
            let file = template_path.join(TEMPLATE_FILE_NAME);
            (file, template_path.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct IncludeOptions {
    dest: Option<PathBuf>,
    extract: Option<bool>,
    gitignore: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_from_file() {
        let config = TemplateConfig::from_file(&PathBuf::from("test/.template.yaml")).unwrap();

        assert_eq!(config.include.unwrap().len(), 6);
        assert_eq!(config.init.unwrap().len(), 2);
        assert_eq!(config.paths.unwrap().len(), 3);
    }

    #[test]
    fn creates_actions() {
        let now = ::std::time::SystemTime::now();
        let mut temp_dir = ::std::env::temp_dir();
        temp_dir.push("archivar-test");

        let template = Template::make(&Path::new("test"), &temp_dir);

        println!("{:#?}", template);

        let _ = ::std::fs::remove_dir_all(&temp_dir);
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
    }

}
