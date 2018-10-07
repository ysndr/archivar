use super::app;
use crate::error::*;

use log::*;
// use template::Template;

//mod check;
//mod constructors;
//mod message;
pub mod os;
//pub mod template;

pub trait ActionTrait{
    fn run(&self, context: &app::Context) -> Result<()>;
}
