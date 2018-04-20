use action::{Action, Actionable};
use error::*;
use slog::Logger;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Template {
    actions: Vec<Action>,
    // TODO:
    // arguments: ???
}

impl Template {
    pub fn make(template_path: &Path, project_path: &Path, logger: &Logger) -> Result<Self> {
        Ok(Template {
            actions: Vec::new(),
        })
    }
}

impl Actionable for Template {
    fn commit(&self, logger: &Logger) -> io::Result<()> {
        Ok(())
    }
}

struct TemplateConfig<'a> {
    init: Option<Vec<String>>,
    paths: Option<Vec<&'a Path>>,
    include: Option<BTreeMap<&'a Path, Option<IncludeOptions<'a>>>>,
}

struct IncludeOptions<'a> {
    dest: Option<&'a Path>,
    extract: Option<bool>,
    gitignore: Option<bool>,
}
