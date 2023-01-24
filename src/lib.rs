#![cfg_attr(not(any(doc, test)), no_std)]
#![cfg_attr(doc, deny(rustdoc::broken_intra_doc_links))]
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(elided_lifetimes_in_paths, unreachable_pub, unsafe_code)]
// clippy doesn't like our code style
#![cfg_attr(feature = "cargo-clippy", allow(clippy::tabs_in_doc_comments))]

//! AA-Tree implementation in Rust.
//!
//! An AA-Tree is a self-balancing binary search tree based on a Red-Black Tree with a
//! simplified self-balancing logic that should benefit performance. For further details
//! on the data structure, see <https://en.wikipedia.org/wiki/AA_tree>.
//!
//! # Use cases
//!
//! The standard library comes with a good set of data structures. See
//! [`std::collections`] for a list of data structures and their (dis)advantages,
//! however, not all advertised functionality of [`BTreeMap`] is actually implemented (at
//! the time of writing). On average, this AA Tree implementation is about half as fast
//! as the standard libraries `BTree` data structures.
//!
//! ## When to use `AATree` over `std::collections::BTree`
//!
//! - Your application doesn't benefit from CPU caching but does from simpler
//!   implementations
//! - You want to find the largest or smallest key that is smaller or larger than
//!   something
//! - You need a BST that you can freely search with your own routine
//!
//! ## When to use `AATree` over `std::vec::Vec`
//!
//! - You need a sorted data structure
//! - You need to efficiently check whether a key is contained or not
//! - You need a BST that you can freely search with your own routine
//!
//! # Compatibility
//!
//! This crate tries to achieve API-level compatibility with the [`BTreeMap`] and
//! [`BTreeSet`] types from the standard library. While all functions that exist with the
//! same name for `AATree` and `BTree` are compatible, currently the `AATree` types do
//! not implement all functions that exist for the `BTree` types. Vice versa, this crate
//! implements some functions that are not available on the `BTree` types.
#![cfg_attr(feature = "document-features", doc = concat!(
	"\n\n# Features\n",
	document_features::document_features!()
))]
//!
//! # Versioning
//!
//! As all rust crates, this crate will follow semantic versioning guidelines. However,
//! increasing the MSRV (minimum supported rust version) is not considered a breaking
//! change.
//!
//!  [`BTreeMap`]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html
//!  [`BTreeSet`]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html
//!  [`std::collections`]: https://doc.rust-lang.org/stable/std/collections/index.html

extern crate alloc;

pub mod iter;
pub mod map;
pub mod node;
#[cfg(feature = "openapi")]
mod openapi;
#[cfg(feature = "serde")]
mod serde;
pub mod set;

pub use map::AATreeMap;
pub use set::AATreeSet;
