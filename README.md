# AA Tree

[![Build Status](https://github.com/msrd0/aatree/actions/workflows/rust.yml/badge.svg)](https://github.com/msrd0/aatree/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/aatree.svg)](https://crates.io/crates/aatree)
[![docs.rs](https://docs.rs/aatree/badge.svg)](https://docs.rs/aatree)
[![Documentation](https://img.shields.io/badge/docs-main-blue.svg)](https://msrd0.github.io/aatree/doc/aatree/index.html)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0.html)
[![MSRV](https://img.shields.io/badge/rustc-1.50+-orange.svg)](https://blog.rust-lang.org/2021/02/11/Rust-1.50.0.html)
[![Chat](https://img.shields.io/badge/chat-on%20matrix-brightgreen?logoWidth=16&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAAAIGNIUk0AAHomAACAhAAA+gAAAIDoAAB1MAAA6mAAADqYAAAXcJy6UTwAAAAEZ0FNQQAAsY8L/GEFAAAAJXRFWHRkYXRlOm1vZGlmeQAyMDIwLTExLTA5VDE2OjQxOjMxKzAwOjAwWDZBzgAAAAd0SU1FB+QLChI0GgCGX9IAAACfUExURQ29iw29iw29iw29iw29iw29iw29iw29i/j9/PT8+vP8+fL7+fH7+Ov69ur59d3279z17s7y6M7y57Lq2rHq2qzo16Dl0ZfjzZbizYbexXvbwHrbv2vXuGnWt2PUtFrSsFLQrEvOqUbNp0bNpkPMpTbInzDGnC7GmyTDlha/jw++jA+9jA69iw29iwy9iwu9igu8igq8igm8iQi8iQe7iMqi3Q4AAAAIdFJOUwhsbd7g8f3+M9WqDQAAAAFiS0dENd622WsAAADFSURBVHheJc9HgoJQEEDBVhQDihEUA2Ekw4Pmw/3PNgvqBiWyWG1gGE0Hm9VCZLkG+PuklQLrpVgAjb/d3zIFLLFB2zKJA/daKNgCWjxOx3CKDs8WEDS7ON4r19o7lw1IV93dqDY95u0m/m8QkzrBpECfh/HuO8r43cVh3gM6BU5qZPj5ifs29KaO3HvVCTTl2as1f3nOJVMEaJ+HaAqPp0ehIDZocXWDOClbBVssQLPbfus3ANac0yr9/Jhzc78z4zD3/wGAlyDwGzAPLAAAAABJRU5ErkJggg==)](https://matrix.to/#/#aatree:msrd0.de)

This crate implements an AA Tree in Rust. An AA Tree is basically a Red-Black Tree that disallows right children to be red. This results for less complexity in the implementation. For further details on the data structure, see https://en.wikipedia.org/wiki/AA_tree for more information on the data structure.

The standard library comes with a good set of data structures. See [`std::collections`] for a list of data structures and their (dis)advantages, however, not all advertised functionality of `BTreeMap` is actually implemented (at the time of writing). On average, this AA Tree implementation is about half as fast as the standard libraries `BTree` data structures.

## When to use `AATree` over `std::collections::BTree`

- Your application doesn't benefit from CPU caching but does from simpler implementations
- You want to find the largest or smallest key that is smaller or larger than something
- You need a BST that you can freely search with your own routine

## When to use `AATree` over `std::vec::Vec`

- You need a sorted data structure
- You need to efficiently check whether a key is contained or not
- You need a BST that you can freely search with your own routine

## Runtime Comparison

<div>
	<img alt="Insert Operation Comparison" src="benchmarks/target/criterion/Insert/report/lines.svg" width="32%" />
	<img alt="Contains Operation Comparison" src="benchmarks/target/criterion/Contains/report/lines.svg" width="32%" />
	<img alt="Remove Operation Comparison" src="benchmarks/target/criterion/Remove/report/lines.svg" width="32%" />
</div>

## Versioning

As all rust crates, this crate will follow semantic versioning guidelines. However, increasing the MSRV (minimum supported rust version) is not considered a breaking change.

## License

```
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

 [`std::collections`]: https://doc.rust-lang.org/stable/std/collections/index.html
