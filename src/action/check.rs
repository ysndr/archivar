use super::ActionTrait;
use crate::app;
use crate::error::*;
use std::fmt;

pub struct Check(Box<dyn Fn(&app::Context) -> Result<()>>);
impl fmt::Debug for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{runtime check}}")
    }
}
impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{runtime check}}")
    }
}
/// Always return true for we can not check functions for equality
impl PartialEq for Check {
    fn eq(&self, _other: &Check) -> bool {
        true
    }
}

impl From<Check> for super::Action {
    fn from(action: Check) -> super::Action {
        super::Action::Check(action)
    }
}

impl ActionTrait for Check {
    fn run<'a>(&self, context: &'a app::Context) -> Result<()> {
        debug!("Performing runtime check");
        self.0(context)
    }
}

impl Check {
    pub fn new(fun: Box<dyn Fn(&app::Context) -> Result<()>>) -> Check {
        Check(fun)
    }
}

impl From<Box<dyn Fn(&app::Context) -> Result<()>>> for super::Action {
    fn from(fun: Box<dyn Fn(&app::Context) -> Result<()>>) -> super::Action {
        Check::new(fun).into()
    }
}

#[derive(Debug)]
pub struct Fail(String);

impl PartialEq for Fail {
    fn eq(&self, _other: &Fail) -> bool {
        true
    }
}

impl ActionTrait for Fail {
    fn run<'a>(&self, _context: &'a app::Context) -> Result<()> {
        Err(self.0.clone().into())
    }
}

impl From<Fail> for super::Action {
    fn from(action: Fail) -> super::Action {
        super::Action::Fail(action)
    }
}

impl Fail {
    pub fn new(msg: String) -> Fail {
        Fail(msg)
    }
}
