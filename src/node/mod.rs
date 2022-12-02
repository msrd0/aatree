//! Low-level implementation of an AA tree. You shouldn't have to use this directly; instead, use
//! the implementations in [`AATreeSet`](crate::AATreeSet) and [`AATreeMap`](crate::AATreeMap).

use alloc::boxed::Box;
use core::mem;

mod bulk;
mod insert;
mod remove;
#[cfg(test)]
mod tests;
mod traverse;

pub use traverse::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AANode<T>(Option<Box<Node<T>>>);

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Node<T> {
	pub(super) level: u8,
	pub(super) content: T,
	pub(super) left_child: AANode<T>,
	pub(super) right_child: AANode<T>
}

impl<T> From<Node<T>> for AANode<T> {
	fn from(node: Node<T>) -> Self {
		Self(Some(Box::new(node)))
	}
}

impl<T> Default for AANode<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> From<T> for AANode<T> {
	fn from(content: T) -> Self {
		Node {
			level: 1,
			content,
			left_child: Self(None),
			right_child: Self(None)
		}
		.into()
	}
}

impl<T> AANode<T> {
	pub(super) fn unbox(self) -> Option<Node<T>> {
		self.0.map(|this| *this)
	}

	pub(super) fn as_ref(&self) -> Option<&Node<T>> {
		self.0.as_ref().map(Box::as_ref)
	}

	fn as_mut(&mut self) -> Option<&mut Node<T>> {
		self.0.as_mut().map(Box::as_mut)
	}

	fn take(&mut self) -> Self {
		Self(self.0.take())
	}

	/// Create a new `Nil` node.
	pub const fn new() -> Self {
		Self(None)
	}

	/// Return true if this node is `Nil`.
	pub const fn is_nil(&self) -> bool {
		self.0.is_none()
	}

	/// Return true if this node is a leaf.
	pub fn is_leaf(&self) -> bool {
		match self.as_ref() {
			None => false,
			Some(Node {
				left_child,
				right_child,
				..
			}) => left_child.is_nil() && right_child.is_nil()
		}
	}

	/// Return true if this node has a left child.
	pub fn has_left_child(&self) -> bool {
		match self.as_ref() {
			None => false,
			Some(Node { left_child, .. }) => !left_child.is_nil()
		}
	}

	/// Return true if this node has a right child.
	pub fn has_right_child(&self) -> bool {
		match self.as_ref() {
			None => false,
			Some(Node { right_child, .. }) => !right_child.is_nil()
		}
	}

	fn left_child_mut(&mut self) -> Option<&mut Self> {
		self.as_mut().and_then(|Node { left_child, .. }| {
			(!left_child.is_nil()).then(|| left_child)
		})
	}

	fn right_child_mut(&mut self) -> Option<&mut Self> {
		self.as_mut().and_then(|Node { right_child, .. }| {
			(!right_child.is_nil()).then(|| right_child)
		})
	}

	fn set_right_child(&mut self, child: Self) {
		match self.as_mut() {
			None => panic!("I don't have a right child"),
			Some(Node { right_child, .. }) => {
				*right_child = child;
			}
		}
	}

	pub(super) fn level(&self) -> u8 {
		match self.as_ref() {
			None => 0,
			Some(Node { level, .. }) => *level
		}
	}

	fn content_mut(&mut self) -> Option<&mut T> {
		self.as_mut().map(|Node { content, .. }| content)
	}

	/// Update the level of this node. **Panic** if the node is [`Nil`](Self::Nil).
	fn set_level(&mut self, level: u8) {
		match self.as_mut() {
			None => panic!("Cannot change level of Nil"),
			Some(Node { level: l, .. }) => mem::replace(l, level)
		};
	}

	/// ```none
	///   L <--- S           S ---> T
	///  / \      \     =>  /      / \
	/// A   B      R       A      B   R
	/// ```
	fn skew(mut self) -> Self {
		match self.as_mut() {
			None => self,
			Some(Node {
				level,
				left_child: l,
				..
			}) => {
				// if level = l.level, remove the B node from L
				let b_node = match l.as_mut() {
					Some(Node {
						level: l_level,
						right_child: b,
						..
					}) if level == l_level => b.take(),
					_ => return self
				};

				// add the B node as our left child, removing L
				let mut l_node = mem::replace(l, b_node);

				// add our node T as the right child of L
				l_node
					.as_mut()
					.unwrap_or_else(|| unreachable!())
					.right_child = self;

				// L is our new node
				l_node
			}
		}
	}

	/// ```none
	///   S --> R --> X          R
	///  /     /          =>    / \
	/// A     B                T   X
	///                       / \
	///                      A   B
	/// ```
	fn split(mut self) -> Self {
		match self.as_mut() {
			None => self,
			Some(Node {
				level,
				right_child: r,
				..
			}) => {
				// remove the B node if R and X are not Nil
				let b_node = match r.as_mut() {
					Some(Node {
						left_child: b,
						right_child: x,
						..
					}) if &x.level() == level => b.take(),
					_ => return self
				};

				// attach the B node to our node, removing R
				let mut r_node = mem::replace(r, b_node);

				// attach our node to R and increment its level
				let r_node_mut = r_node.as_mut().unwrap_or_else(|| unreachable!());
				r_node_mut.level += 1;
				r_node_mut.left_child = self;

				// R is our new node
				r_node
			}
		}
	}
}
