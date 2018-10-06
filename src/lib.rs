#![feature(box_syntax, box_patterns)]
use log::*;
use structopt;

#[cfg(test)] // <-- not needed in examples + integration tests
extern crate pretty_assertions;

#[macro_use]
extern crate error_chain;

// mod action;
pub mod app;
//mod args;
mod commands;
pub mod constants;
pub mod error;
pub mod logger;

pub use self::commands::command;
