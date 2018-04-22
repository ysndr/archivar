use action::{Action, Actionable};
use constants::TEMPLATE_FILE_NAME;
use error::*;
use serde_yaml;
use slog::Logger;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Template {
    actions: Vec<Action>,
    // TODO:
    // arguments: ???
}

impl Template {
    pub fn make(template_path: &Path, project_path: &Path, logger: &Logger) -> Result<Self> {
        debug!(logger, "making Template actions"; "template" => %template_path.display(), "project" => %project_path.display());

        let template_file = template_path.join(TEMPLATE_FILE_NAME);

        // TODO: read + parse template file
        let config = TemplateConfig {
            init: None,
            paths: None,
            include: None,
        };

        Ok(Template {
            actions: Vec::new(),
        })
    }
}

#[test]
fn reader_test() {
    let path = PathBuf::from("./example/.template.yaml");
    let br = ::std::io::BufReader::new(::std::fs::File::open(path).unwrap());
    let template: TemplateConfig = serde_yaml::from_reader(br).unwrap();
    println!("template: {:#?}", template);
}

impl Actionable for Template {
    fn commit(&self, logger: &Logger) -> io::Result<()> {
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

#[derive(Debug, Serialize, Deserialize)]
struct IncludeOptions {
    dest: Option<PathBuf>,
    extract: Option<bool>,
    gitignore: Option<bool>,
}
