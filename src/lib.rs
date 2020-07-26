//! AA-Tree implementation in Rust.
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(unreachable_pub)]

#[macro_use]
extern crate log;

pub mod iter;
pub mod node;

mod set;
pub use set::*;
