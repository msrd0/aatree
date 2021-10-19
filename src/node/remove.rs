use super::{AANode, Node};
use core::borrow::Borrow;

impl<T: Eq + Ord> AANode<T> {
	/// Remove a value from this tree. If the value was found, it will be returned.
	pub fn remove<Q, R>(&mut self, value: &Q) -> Option<T>
	where
		T: Borrow<R>,
		R: Borrow<Q> + ?Sized,
		Q: Ord + ?Sized
	{
		let root = self.take();
		let (root, removed) = root.remove_impl(value);
		*self = root;
		removed
	}

	/// Removes a node with the specified content from this tree and returns its content.
	pub(super) fn remove_impl<Q, R>(self, to_remove: &Q) -> (Self, Option<T>)
	where
		T: Borrow<R>,
		R: Borrow<Q> + ?Sized,
		Q: Ord + ?Sized
	{
		let (node, removed) = match self.unbox() {
			None => (Self::new(), None),
			Some(Node {
				level,
				content,
				left_child,
				right_child
			}) if content.borrow().borrow() == to_remove => (Self::remove_node(level, left_child, right_child), Some(content)),
			Some(Node {
				level,
				content,
				left_child,
				right_child
			}) if content.borrow().borrow() > to_remove => {
				let (left, value) = left_child.remove_impl(to_remove);
				(
					Node {
						level,
						content,
						left_child: left,
						right_child
					}
					.into(),
					value
				)
			},
			Some(Node {
				level,
				content,
				left_child,
				right_child
			}) => {
				let (right, value) = right_child.remove_impl(to_remove);
				(
					Node {
						level,
						content,
						left_child,
						right_child: right
					}
					.into(),
					value
				)
			}
		};
		(node.remove_cleanup(), removed)
	}

	/// Removes this (first) node, and returns its value if it wasn't `Nil`.
	pub fn remove_self(&mut self) -> Option<T> {
		let root = self.take();
		let (root, removed) = root.remove_self_impl();
		*self = root;
		removed
	}

	fn remove_self_impl(self) -> (Self, Option<T>) {
		match self.unbox() {
			None => (Self::new(), None),
			Some(Node {
				level,
				content,
				left_child,
				right_child
			}) => (Self::remove_node(level, left_child, right_child), Some(content))
		}
	}

	fn remove_node(level: u8, left_child: Self, right_child: Self) -> Self {
		// for leaves, return the right child
		if level == 1 {
			right_child
		}
		// if we don't have a left child, use the successor
		else if left_child.level() == 0 {
			let (right, successor) = right_child.remove_successor();
			Node {
				level,
				content: successor.unwrap(),
				left_child,
				right_child: right
			}
			.into()
		}
		// else we can use the predecessor
		else {
			let (left, predecessor) = left_child.remove_predecessor();
			Node {
				level,
				content: predecessor.unwrap(),
				left_child: left,
				right_child
			}
			.into()
		}
	}

	/// Removes the successor (smallest node) of the parent of this node and return it.
	fn remove_successor(self) -> (Self, Option<T>) {
		let (node, successor) = match self.unbox() {
			None => (Self::new(), None),
			Some(Node {
				level,
				content,
				left_child,
				right_child
			}) => {
				let (left, successor) = left_child.remove_successor();
				if let Some(successor) = successor {
					(
						Node {
							level,
							content,
							left_child: left,
							right_child
						}
						.into(),
						Some(successor)
					)
				} else {
					(right_child, Some(content))
				}
			}
		};
		(node.remove_cleanup(), successor)
	}

	/// Removes the predecessor (largest node) of the parent of this node and return it.
	fn remove_predecessor(self) -> (Self, Option<T>) {
		let (node, predecessor) = match self.unbox() {
			None => (Self::new(), None),
			Some(Node {
				level,
				content,
				left_child,
				right_child
			}) => {
				let (right, predecessor) = right_child.remove_predecessor();
				if let Some(predecessor) = predecessor {
					(
						Node {
							level,
							content,
							left_child,
							right_child: right
						}
						.into(),
						Some(predecessor)
					)
				} else {
					(left_child, Some(content))
				}
			}
		};
		(node.remove_cleanup(), predecessor)
	}

	/// Run fixes necessary after removing/replacing `self` or one of the child nodes to retain
	/// the AA tree properties.
	fn remove_cleanup(self) -> Self {
		match self.unbox() {
			None => Self::new(),
			Some(Node {
				mut level,
				content,
				left_child,
				mut right_child
			}) => {
				// decrease the level if necessary
				let expected = left_child.level().min(right_child.level()) + 1;
				if expected < level {
					level = expected;
					if expected < right_child.level() {
						right_child.set_level(expected);
					}
				}
				let mut node: AANode<_> = Node {
					level,
					content,
					left_child,
					right_child
				}
				.into();

				// rebalance the tree by applying 3x skew and 2x split
				node = node.skew();
				match node.as_mut() {
					None => unreachable!(),
					Some(node) => {
						let mut right = node.right_child.take();
						right = right.skew();
						if let Some(right) = right.as_mut() {
							let mut right_grandchild = right.right_child.take();
							right_grandchild = right_grandchild.skew();
							right.right_child = right_grandchild;
						}
						node.right_child = right;
					}
				};
				node = node.split();
				match node.as_mut() {
					None => unreachable!(),
					Some(node) => {
						let mut right = node.right_child.take();
						right = right.split();
						node.right_child = right;
					}
				};
				node
			}
		}
	}
}
