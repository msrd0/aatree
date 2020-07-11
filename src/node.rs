use super::tree::{TraverseStep, TreeType};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum AANode<T: TreeType> {
	Nil,
	Node {
		level: u8,
		content: T,
		left_child: Box<AANode<T>>,
		right_child: Box<AANode<T>>
	}
}

impl<T: TreeType> Default for AANode<T> {
	fn default() -> Self {
		Self::Nil
	}
}

impl<T: TreeType> AANode<T> {
	pub(super) fn new(content: T) -> Self {
		Self::Node {
			level: 1,
			content,
			left_child: Box::new(Self::Nil),
			right_child: Box::new(Self::Nil)
		}
	}

	pub(super) fn level(&self) -> u8 {
		match self {
			Self::Nil => 0,
			Self::Node { level, .. } => *level
		}
	}

	/// Update the level of this node. **Panic** if the node is `Nil`.
	fn set_level(&mut self, level: u8) {
		match self {
			Self::Nil => panic!("Cannot change level of Nil"),
			Self::Node { level: l, .. } => std::mem::replace(l, level)
		};
	}

	/// ```none
	///   L <--- S           S ---> T
	///  / \      \     =>  /      / \
	/// A   B      R       A      B   R
	/// ```
	fn skew(mut self) -> Self {
		match &mut self {
			Self::Nil => self,
			Self::Node {
				level, left_child: l, ..
			} => {
				// if level = l.level, remove the B node from L
				let b_node = match l.as_mut() {
					Self::Node {
						level: l_level,
						right_child: b,
						..
					} if level == l_level => std::mem::replace(b.as_mut(), Self::Nil),
					_ => return self
				};

				// add the B node as our left child, removing L
				let mut l_node = std::mem::replace(l.as_mut(), b_node);

				// add our node T as the right child of L
				match &mut l_node {
					Self::Nil => unreachable!(),
					Self::Node { right_child: t, .. } => {
						std::mem::replace(t.as_mut(), self);
					}
				};

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
		match &mut self {
			Self::Nil => self,
			Self::Node {
				level, right_child: r, ..
			} => {
				// remove the B node if R and X are not Nil
				let b_node = match r.as_mut() {
					Self::Node {
						left_child: b,
						right_child: x,
						..
					} if &x.level() == level => std::mem::replace(b.as_mut(), Self::Nil),
					_ => return self
				};

				// attach the B node to our node, removing R
				let mut r_node = std::mem::replace(r.as_mut(), b_node);

				// attach our node to R and increment its level
				match &mut r_node {
					Self::Nil => unreachable!(),
					Self::Node {
						level: r_level,
						left_child: t,
						..
					} => {
						std::mem::replace(r_level, *r_level + 1);
						std::mem::replace(t.as_mut(), self);
					}
				}

				// R is our new node
				r_node
			}
		}
	}

	/// Traverse the current node, calling the callback with `(content, None)` while going down and
	/// with `(content, Some(res))` when going up, where `res` is the result obtained from the lower
	/// subtree.
	pub(super) fn traverse<'a, F, R>(&'a self, callback: F) -> TraverseStep<R>
	where
		F: Fn(&'a T, Option<TraverseStep<R>>) -> TraverseStep<R> + Copy
	{
		match self {
			Self::Nil => TraverseStep::Value(None),
			Self::Node {
				content,
				left_child,
				right_child,
				..
			} => {
				let step = callback(content, None);
				let res = match step {
					TraverseStep::Left => left_child.traverse(callback),
					TraverseStep::Right => right_child.traverse(callback),
					TraverseStep::Value(_) => return step
				};
				let res = callback(content, Some(res));
				if matches!(res, TraverseStep::Left | TraverseStep::Right) {
					error!("Recursive traversal detected and prohibited");
				}
				res
			}
		}
	}

	/// Insert a new node with `content` into the tree. If a node with this value already exist,
	/// nothing will be inserted. Returns a pair of the new node and a boolean whether a new node
	/// was inserted.
	pub(super) fn insert(&mut self, content: T) -> bool {
		let inserted = self.bst_insert(content);
		if inserted {
			let mut node = std::mem::replace(self, Self::Nil);
			node = node.skew().split();
			std::mem::replace(self, node);
		}
		inserted
	}

	/// Simple unbalanced BST insert.
	fn bst_insert(&mut self, new: T) -> bool {
		match self {
			Self::Nil => {
				std::mem::replace(self, Self::new(new));
				true
			},
			Self::Node { content, left_child, .. } if &new < content => left_child.insert(new),
			Self::Node {
				content, right_child, ..
			} if &new > content => right_child.insert(new),
			_ => false
		}
	}

	/// Removes a node with the specified content from this tree and returns its content.
	pub(super) fn remove(self, to_remove: &T) -> (Self, Option<T>) {
		let (node, removed) = match self {
			Self::Nil => (self, None),
			Self::Node {
				level,
				content,
				left_child,
				right_child
			} if &content == to_remove => {
				// for leaves, return the right child
				if level == 1 {
					(*right_child, Some(content))
				}
				// if we don't have a left child, use the successor
				else if left_child.level() == 0 {
					let (right, successor) = right_child.remove_successor();
					(
						Self::Node {
							level,
							content: successor.unwrap(),
							left_child,
							right_child: Box::new(right)
						},
						Some(content)
					)
				}
				// else we can use the predecessor
				else {
					let (left, predecessor) = left_child.remove_predecessor();
					(
						Self::Node {
							level,
							content: predecessor.unwrap(),
							left_child: Box::new(left),
							right_child
						},
						Some(content)
					)
				}
			},
			Self::Node {
				level,
				content,
				left_child,
				right_child
			} if &content > to_remove => {
				let (left, value) = left_child.remove(to_remove);
				(
					Self::Node {
						level,
						content,
						left_child: Box::new(left),
						right_child
					},
					value
				)
			},
			Self::Node {
				level,
				content,
				left_child,
				right_child
			} => {
				let (right, value) = right_child.remove(to_remove);
				(
					Self::Node {
						level,
						content,
						left_child,
						right_child: Box::new(right)
					},
					value
				)
			}
		};
		(node.remove_cleanup(), removed)
	}

	/// Removes the successor (smallest node) of the parent of this node and return it.
	fn remove_successor(self) -> (Self, Option<T>) {
		let (node, successor) = match self {
			Self::Nil => (self, None),
			Self::Node {
				level,
				content,
				left_child,
				right_child
			} => {
				let (left, successor) = left_child.remove_successor();
				if let Some(successor) = successor {
					(
						Self::Node {
							level,
							content,
							left_child: Box::new(left),
							right_child
						},
						Some(successor)
					)
				} else {
					(*right_child, Some(content))
				}
			}
		};
		(node.remove_cleanup(), successor)
	}

	/// Removes the predecessor (largest node) of the parent of this node and return it.
	fn remove_predecessor(self) -> (Self, Option<T>) {
		let (node, predecessor) = match self {
			Self::Nil => (self, None),
			Self::Node {
				level,
				content,
				left_child,
				right_child
			} => {
				let (right, predecessor) = right_child.remove_predecessor();
				if let Some(predecessor) = predecessor {
					(
						Self::Node {
							level,
							content,
							left_child,
							right_child: Box::new(right)
						},
						Some(predecessor)
					)
				} else {
					(*left_child, Some(content))
				}
			}
		};
		(node.remove_cleanup(), predecessor)
	}

	/// Run fixes necessary after removing/replacing `self` or one of the child nodes to retain
	/// the AA tree properties.
	fn remove_cleanup(self) -> Self {
		match self {
			Self::Nil => self,
			Self::Node {
				mut level,
				content,
				left_child,
				mut right_child
			} => {
				// decrease the level if necessary
				let expected = left_child.level().min(right_child.level()) + 1;
				if expected < level {
					level = expected;
					if expected < right_child.level() {
						right_child.set_level(expected);
					}
				}
				let mut node = Self::Node {
					level,
					content,
					left_child,
					right_child
				};

				// rebalance the tree by applying 3x skew and 2x split
				node = node.skew();
				match &mut node {
					Self::Nil => unreachable!(),
					Self::Node { right_child, .. } => {
						let mut right = std::mem::replace(right_child.as_mut(), Self::Nil);
						right = right.skew();
						if let Self::Node { right_child, .. } = &mut right {
							let mut right_grandchild = std::mem::replace(right_child.as_mut(), Self::Nil);
							right_grandchild = right_grandchild.skew();
							std::mem::replace(right_child.as_mut(), right_grandchild);
						}
						std::mem::replace(right_child.as_mut(), right);
					}
				};
				node = node.split();
				match &mut node {
					Self::Nil => unreachable!(),
					Self::Node { right_child, .. } => {
						let mut right = std::mem::replace(right_child.as_mut(), Self::Nil);
						right = right.split();
						std::mem::replace(right_child.as_mut(), right);
					}
				};
				node
			}
		}
	}
}

#[cfg(all(test, not(feature = "benchmark")))]
mod test {
	use super::*;

	macro_rules! tree {
		() => {
			AANode::Nil
		};
		(Nil) => {
			AANode::Nil
		};
		($content:expr) => {
			AANode::new($content)
		};
		($content:expr => [$level:expr, $left:tt, $right:tt]) => {
			{
				let _left = tree!(@internal $left);
				let _right = tree!(@internal $right);
				AANode::Node {
					level: $level,
					content: $content,
					left_child: Box::new(_left),
					right_child: Box::new(_right)
				}
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
		let root = tree!('B' => [1, Nil, 'C']);
		println!("Input:  `{:?}`", root);
		let (tree, removed) = root.remove(&'B');
		let expected = tree!('C');
		assert_eq!(removed, Some('B'));
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_remove_predecessor() {
		let root = tree!('B' => [2, 'A', 'C']);
		println!("Input:  `{:?}`", root);
		let (tree, removed) = root.remove(&'B');
		let expected = tree!('A' => [1, Nil, 'C']);
		assert_eq!(removed, Some('B'));
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_remove_complex() {
		// example taken from https://web.eecs.umich.edu/~sugih/courses/eecs281/f11/lectures/12-AAtrees+Treaps.pdf
		let root =
			tree!(30 => [3, (15 => [2, 5, 20]), (70 => [3, (50 => [2, 35, (60 => [2, 55, 65])]), (85 => [2, 80, 90])])]);
		println!("Input:  `{:?}`", root);
		let (tree, removed) = root.remove(&5);
		let expected =
			tree!(50 => [3, (30 => [2, (15 => [1, Nil, 20]), 35]), (70 => [3, (60 => [2, 55, 65]), (85 => [2, 80, 90])])]);
		assert_eq!(removed, Some(5));
		assert_eq!(tree, expected);
	}
}
