use error::*;
use app;
use std::fmt;
use super::ActionTrait;


pub struct Check(Box<Fn(&app::Context) -> Result<()>>);
impl fmt::Debug for Check {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{runtime check}}")
    }
}
impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        fn run<'a>(&self, context: &'a app::Context) -> Result<()>{
            self.0(context)
        }

}
