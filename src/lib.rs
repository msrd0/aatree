#![cfg_attr(not(any(doc, test)), no_std)]
#![cfg_attr(doc, deny(rustdoc::broken_intra_doc_links))]
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(elided_lifetimes_in_paths, unreachable_pub)]
#![forbid(unsafe_code)]
// clippy doesn't like our code style
#![cfg_attr(feature = "cargo-clippy", allow(clippy::tabs_in_doc_comments))]

//! AA-Tree implementation in Rust.
//!
//! An AA-Tree is a self-balancing binary search tree based on a RedBlack-Tree
//! with a simplified self-balancing logic that should benefit performance.

extern crate alloc;

pub mod iter;
pub mod node;

mod map;
pub use map::{entry::Entry, AATreeMap};

mod set;
pub use set::*;
