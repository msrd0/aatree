#![cfg_attr(not(test), no_std)]
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(elided_lifetimes_in_paths, unreachable_pub)]

//! AA-Tree implementation in Rust.
//!
//! An AA-Tree is a self-balancing binary search tree based on a RedBlack-Tree
//! with a simplified self-balancing logic that should benefit performance.

extern crate alloc;

pub mod iter;
pub mod node;

mod map;
pub use map::*;

mod set;
pub use set::*;
