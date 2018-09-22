#![feature(box_syntax, box_patterns, extern_prelude)]
#[macro_use]
extern crate log;

#[cfg(test)] // <-- not needed in examples + integration tests
#[macro_use]
extern crate pretty_assertions;

extern crate shell;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[macro_use]
pub extern crate structopt;

#[macro_use]
extern crate error_chain;

extern crate assert_fs;
extern crate predicates;

extern crate chrono;
extern crate fern;

extern crate fs_extra;

mod action;
pub mod app;
mod args;
pub mod constants;
pub mod error;
pub mod logger;
