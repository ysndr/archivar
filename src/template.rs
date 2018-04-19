use action::Actionable;

pub struct Template {
    template_path: Path,
    project_path: Path,
    // TODO:
    // arguments: ???
}

impl Actionable for Template {
    fn commit(&self, logger: &slog::Logger) -> io::Result<()> {
        Ok(())
    }
}
