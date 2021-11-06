#![allow(missing_debug_implementations)]

//! Iterator implementations for [`AATreeSet`](crate::AATreeSet) and [`AATreeMap`](crate::AATreeMap).

use super::node::{AANode, Node};
use alloc::vec::Vec;
use core::{iter::FusedIterator, marker::PhantomData};

/// This trait allows iterators to return elements other than that stored inside the tree. Useful
/// for returning key-value-pairs from `AATreeMap`.
pub trait IterContent<T> {
	fn content(self) -> T;
}

impl<T> IterContent<T> for T {
	fn content(self) -> Self {
		self
	}
}

/// The iterator produces from an reference of an AATree-based data structure when turned into an iterator.
pub struct AAIter<'a, C, T> {
	stack: Vec<(bool, &'a AANode<C>)>,
	len: usize,
	_ty: PhantomData<T>
}

impl<'a, C, T> AAIter<'a, C, T> {
	pub(super) fn new(root: &'a AANode<C>, len: usize) -> Self {
		let mut stack = Vec::with_capacity(root.level() as usize * 2 + 1);
		stack.push((false, root));
		Self {
			stack,
			len,
			_ty: PhantomData::default()
		}
	}
}

impl<'a, C, T> Iterator for AAIter<'a, C, T>
where
	&'a C: IterContent<T>
{
	type Item = T;

	fn next(&mut self) -> Option<T> {
		loop {
			let (visited_left, last) = self.stack.pop()?;
			if let Some(Node { left_child, .. }) = last.as_ref() {
				self.stack.push((true, last));
				if !visited_left && !left_child.is_nil() {
					self.stack.push((false, left_child));
				} else {
					break;
				}
			}
		}

		let (_, last) = self.stack.pop()?;
		match last.as_ref() {
			None => unreachable!(),
			Some(Node {
				content, right_child, ..
			}) => {
				if !right_child.is_nil() {
					self.stack.push((false, right_child));
				}
				self.len -= 1;
				Some(content.content())
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

impl<'a, C, T> ExactSizeIterator for AAIter<'a, C, T> where &'a C: IterContent<T> {}

impl<'a, C, T> FusedIterator for AAIter<'a, C, T> where &'a C: IterContent<T> {}

/// The iterator produces from an AATree-based data structure when turned into an iterator.
pub struct AAIntoIter<C, T> {
	stack: Vec<AANode<C>>,
	len: usize,
	_ty: PhantomData<T>
}

impl<C, T> AAIntoIter<C, T> {
	pub(super) fn new(root: AANode<C>, len: usize) -> Self {
		let mut stack = Vec::with_capacity(root.level() as usize * 2 + 1);
		stack.push(root);
		Self {
			stack,
			len,
			_ty: PhantomData::default()
		}
	}
}

impl<C, T> Iterator for AAIntoIter<C, T>
where
	C: IterContent<T>
{
	type Item = T;

	fn next(&mut self) -> Option<T> {
		loop {
			let last = self.stack.pop()?;
			if let Some(Node {
				level,
				content,
				left_child,
				right_child
			}) = last.unbox()
			{
				self.stack.push(
					Node {
						level,
						content,
						left_child: AANode::new(),
						right_child
					}
					.into()
				);
				if !left_child.is_nil() {
					self.stack.push(left_child);
				} else {
					break;
				}
			}
		}

		let last = self.stack.pop()?;
		match last.unbox() {
			None => unreachable!(),
			Some(Node {
				content, right_child, ..
			}) => {
				if !right_child.is_nil() {
					self.stack.push(right_child);
				}
				self.len -= 1;
				Some(content.content())
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

impl<C, T> ExactSizeIterator for AAIntoIter<C, T> where C: IterContent<T> {}

impl<C, T> FusedIterator for AAIntoIter<C, T> where C: IterContent<T> {}
