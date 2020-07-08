//! AA-Tree implementation in Rust.
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(unreachable_pub)]

pub mod iter;
pub mod tree;

mod set;
pub use set::*;
