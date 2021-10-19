//! Iterator implementations for [`AATreeSet`](crate::AATreeSet) and [`AATreeMap`](crate::AATreeMap).

use super::node::{AANode, Node};
use alloc::vec::Vec;
use core::iter::FusedIterator;

/// The iterator produces from an reference of an AATree-based data structure when turned into an iterator.
#[derive(Debug)]
pub struct AAIter<'a, T> {
	stack: Vec<(bool, &'a AANode<T>)>,
	len: usize
}

impl<'a, T> AAIter<'a, T> {
	pub(super) fn new(root: &'a AANode<T>, len: usize) -> Self {
		let mut stack = Vec::with_capacity(root.level() as usize * 2 + 1);
		stack.push((false, root));
		Self { stack, len }
	}
}

impl<'a, T> Iterator for AAIter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<&'a T> {
		loop {
			let (visited_left, last) = self.stack.pop()?;
			if let Some(Node { left_child, .. }) = last.as_ref() {
				self.stack.push((true, last));
				if !visited_left && !left_child.is_nil() {
					self.stack.push((false, &left_child));
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
				Some(content)
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

impl<T> ExactSizeIterator for AAIter<'_, T> {}

impl<T> FusedIterator for AAIter<'_, T> {}

/// The iterator produces from an AATree-based data structure when turned into an iterator.
#[derive(Debug)]
pub struct AAIntoIter<T> {
	stack: Vec<AANode<T>>,
	len: usize
}

impl<T> AAIntoIter<T> {
	pub(super) fn new(root: AANode<T>, len: usize) -> Self {
		let mut stack = Vec::with_capacity(root.level() as usize * 2 + 1);
		stack.push(root);
		Self { stack, len }
	}
}

impl<T> Iterator for AAIntoIter<T> {
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
				Some(content)
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

impl<T> ExactSizeIterator for AAIntoIter<T> {}

impl<T> FusedIterator for AAIntoIter<T> {}
