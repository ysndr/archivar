#![feature(box_syntax, box_patterns, extern_prelude)]
#[macro_use]
extern crate log;

#[cfg(test)] // <-- not needed in examples + integration tests
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate serde_derive;

#[macro_use]
pub extern crate structopt;

#[macro_use]
extern crate error_chain;

mod action;
pub mod app;
mod args;
pub mod constants;
pub mod error;
pub mod logger;
