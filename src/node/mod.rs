//! Low-level implementation of an AA tree. You shouldn't have to use this directly; instead, use
//! the implementations in [`AATreeSet`](crate::AATreeSet) and [`AATreeMap`](crate::AATreeMap).

use alloc::boxed::Box;
use core::mem;

mod insert;
pub use insert::*;

mod remove;
pub use remove::*;

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

impl<T> Into<AANode<T>> for Node<T> {
	fn into(self) -> AANode<T> {
		AANode(Some(Box::new(self)))
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

	pub const fn new() -> Self {
		Self(None)
	}

	pub const fn is_nil(&self) -> bool {
		self.0.is_none()
	}

	pub fn is_leaf(&self) -> bool {
		match self.as_ref() {
			None => false,
			Some(Node {
				left_child, right_child, ..
			}) => left_child.is_nil() && right_child.is_nil()
		}
	}

	pub fn has_left_child(&self) -> bool {
		match self.as_ref() {
			None => false,
			Some(Node { left_child, .. }) => !left_child.is_nil()
		}
	}

	pub fn has_right_child(&self) -> bool {
		match self.as_ref() {
			None => false,
			Some(Node { right_child, .. }) => !right_child.is_nil()
		}
	}

	pub fn left_child_mut(&mut self) -> Option<&mut Self> {
		self.as_mut()
			.and_then(|Node { left_child, .. }| (!left_child.is_nil()).then(|| left_child))
	}

	pub fn right_child_mut(&mut self) -> Option<&mut Self> {
		self.as_mut()
			.and_then(|Node { right_child, .. }| (!right_child.is_nil()).then(|| right_child))
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
				level, left_child: l, ..
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
				l_node.as_mut().unwrap_or_else(|| unreachable!()).right_child = self;

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
				level, right_child: r, ..
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

#[cfg(all(test, not(feature = "benchmark")))]
mod test {
	use super::*;

	macro_rules! tree {
		() => {
			AANode::new()
		};
		(Nil) => {
			AANode::new()
		};
		($content:expr) => {
			AANode::from($content)
		};
		($content:expr => [$level:expr, $left:tt, $right:tt]) => {
			{
				let _left = tree!(@internal $left);
				let _right = tree!(@internal $right);
				AANode(Some(Box::new(Node {
					level: $level,
					content: $content,
					left_child: _left,
					right_child: _right
				})))
			}
		};
		(@internal ($content:expr => [$level:expr, $left:tt, $right:tt])) => {
			tree!($content => [$level, $left, $right])
		};
		(@internal $inner:tt) => {
			tree!($inner)
		};
	}

	// ### TEST SKEW ###

	#[test]
	fn test_skew_nil() {
		let root: AANode<char> = tree!();
		println!("Input: {:?}", root);
		let skewed = root.skew();
		let expected = tree!();
		assert_eq!(skewed, expected);
	}

	#[test]
	fn test_skew_leaf() {
		let root = tree!('T');
		println!("Input: {:?}", root);
		let skewed = root.skew();
		let expected = tree!('T');
		assert_eq!(skewed, expected);
	}

	#[test]
	fn test_skew_simple() {
		let root = tree!('T' => [2, ('L' => [2, Nil, Nil]), 'R']);
		println!("Input: {:?}", root);
		let skewed = root.skew();
		let expected = tree!('L' => [2, Nil, ('T' => [2, Nil, 'R'])]);
		assert_eq!(skewed, expected);
	}

	#[test]
	fn test_skew_full() {
		let root = tree!('T' => [2, ('L' => [2, 'A', 'B']), 'R']);
		println!("Input: {:?}", root);
		let skewed = root.skew();
		let expected = tree!('L' => [2, 'A', ('T' => [2, 'B', 'R'])]);
		assert_eq!(skewed, expected);
	}

	// ### TEST SPLIT ###

	#[test]
	fn test_split_nil() {
		let root: AANode<char> = tree!();
		println!("Input: {:?}", root);
		let splitted = root.split();
		let expected = tree!();
		assert_eq!(splitted, expected);
	}

	#[test]
	fn test_split_leaf() {
		let root = tree!('T');
		println!("Input: {:?}", root);
		let splitted = root.split();
		let expected = tree!('T');
		assert_eq!(splitted, expected);
	}

	#[test]
	fn test_split_good_tree() {
		let root = tree!('T' => [2, 'A', ('R' => [2, 'B', 'X'])]);
		println!("Input: {:?}", root);
		let splitted = root.split();
		let expected = tree!('T' => [2, 'A', ('R' => [2, 'B', 'X'])]);
		assert_eq!(splitted, expected);
	}

	#[test]
	fn test_split_bad_tree() {
		let root = tree!('T' => [2, 'A', ('R' => [2, 'B', ('X' => [2, 'Y', 'Z'])])]);
		println!("Input: {:?}", root);
		let splitted = root.split();
		let expected = tree!('R' => [3, ('T' => [2, 'A', 'B']), ('X' => [2, 'Y', 'Z'])]);
		assert_eq!(splitted, expected);
	}

	// ### TEST INSERT ###

	#[test]
	fn test_insert_greater() {
		let mut root = tree!();
		for content in ['A', 'B', 'C', 'D', 'E', 'F', 'G'].iter() {
			assert!(root.insert(*content));
		}
		let expected = tree!('D' => [3, ('B' => [2, 'A', 'C']), ('F' => [2, 'E', 'G'])]);
		assert_eq!(root, expected);
	}

	#[test]
	fn test_insert_smaller() {
		let mut root = tree!();
		for content in ['Z', 'Y', 'X', 'W', 'V'].iter() {
			assert!(root.insert(*content));
		}
		let expected = tree!('W' => [2, 'V', ('Y' => [2, 'X', 'Z'])]);
		assert_eq!(root, expected);
	}

	#[test]
	fn test_insert_multiple() {
		let mut root = tree!();
		for content in ['A', 'A'].iter() {
			root.insert(*content);
		}
		let expected = tree!('A');
		assert_eq!(root, expected);
	}

	// ### TEST REMOVE ###

	#[test]
	fn test_remove_successor() {
		let mut root = tree!('B' => [1, Nil, 'C']);
		println!("Input:  `{:?}`", root);
		let removed = root.remove(&'B');
		let expected = tree!('C');
		assert_eq!(removed, Some('B'));
		assert_eq!(root, expected);
	}

	#[test]
	fn test_remove_predecessor() {
		let mut root = tree!('B' => [2, 'A', 'C']);
		println!("Input:  `{:?}`", root);
		let removed = root.remove(&'B');
		let expected = tree!('A' => [1, Nil, 'C']);
		assert_eq!(removed, Some('B'));
		assert_eq!(root, expected);
	}

	#[test]
	fn test_remove_complex() {
		// example taken from https://web.eecs.umich.edu/~sugih/courses/eecs281/f11/lectures/12-AAtrees+Treaps.pdf
		let mut root =
			tree!(30 => [3, (15 => [2, 5, 20]), (70 => [3, (50 => [2, 35, (60 => [2, 55, 65])]), (85 => [2, 80, 90])])]);
		println!("Input:  `{:?}`", root);
		let removed = root.remove(&5);
		let expected =
			tree!(50 => [3, (30 => [2, (15 => [1, Nil, 20]), 35]), (70 => [3, (60 => [2, 55, 65]), (85 => [2, 80, 90])])]);
		assert_eq!(removed, Some(5));
		assert_eq!(root, expected);
	}
}
