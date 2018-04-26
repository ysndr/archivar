use action::{Action, Actionable};
use constants::TEMPLATE_FILE_NAME;
use error::*;
use serde_yaml;
use slog::Logger;
use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug)]
pub struct Template {
    actions: Vec<Action>,
    // TODO:
    // arguments: ???
}

impl Template {
    pub fn make(template_path: &Path, project_path: &Path, logger: &Logger) -> Result<Self> {
        debug!(logger, "making Template actions"; "template" => %template_path.display(), "project" => %project_path.display());

        let template_path = template_path.join(TEMPLATE_FILE_NAME);

        let config = TemplateConfig::from_file(&template_path)?;

        debug!(logger, "read config from file";
               "file" => %template_path.display(),
               "config" => ?config);

        Ok(Template {
            actions: Vec::new(),
        })
    }
}

#[test]
fn reader_test() {
    use logger;
    let template = Template::make(
        &Path::new("./example"),
        &Path::new("example"),
        &logger::Logger::new(2),
    );

    println!("template: {:#?}", template);
}

#[test]
fn exec_test() {
    use std::process::Command;
    let shell = "bash";
    let cmd = "echo hello && echo world";

    println!(
        "{:?}",
        Command::new(shell)
            .arg("-c")
            .arg("--")
            .arg(cmd)
            .output()
            .unwrap()
    );
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
