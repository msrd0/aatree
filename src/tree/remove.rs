use super::AANode;

impl<T: Eq + Ord> AANode<T> {
	/// Remove a value from this tree. If the value was found, it will be returned.
	pub fn remove(&mut self, value: &T) -> Option<T> {
		let root = std::mem::replace(self, Self::Nil);
		let (root, removed) = root.remove_impl(value);
		*self = root;
		removed
	}

	/// Removes a node with the specified content from this tree and returns its content.
	pub(super) fn remove_impl(self, to_remove: &T) -> (Self, Option<T>) {
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
				let (left, value) = left_child.remove_impl(to_remove);
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
				let (right, value) = right_child.remove_impl(to_remove);
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
							**right_child = right_grandchild;
						}
						**right_child = right;
					}
				};
				node = node.split();
				match &mut node {
					Self::Nil => unreachable!(),
					Self::Node { right_child, .. } => {
						let mut right = std::mem::replace(right_child.as_mut(), Self::Nil);
						right = right.split();
						**right_child = right;
					}
				};
				node
			}
		}
	}
}
