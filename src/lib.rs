#![crate_name = "cabocha"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

mod chunk;
pub mod consts;
pub mod parser;
mod sys;
mod token;
mod tree;
mod utils;
