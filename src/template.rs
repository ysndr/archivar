use action::{Action, Actionable};
use error::*;
use slog::Logger;
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
