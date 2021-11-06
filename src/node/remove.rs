use super::{AANode, Node};
use core::{borrow::Borrow, cmp::Ordering, mem};

impl<T> AANode<T> {
	/// Remove a value from this tree. If the value was found, it will be returned.
	pub fn remove<Q, R>(&mut self, value: &Q) -> Option<T>
	where
		T: Borrow<R> + Ord,
		R: Borrow<Q> + ?Sized,
		Q: Ord + ?Sized
	{
		let (equal, mut removed) = match self.as_mut() {
			None => return None,
			Some(Node {
				content,
				left_child,
				right_child,
				..
			}) => match T::borrow(content).borrow().cmp(value) {
				Ordering::Equal => (true, None),
				Ordering::Greater => (false, left_child.remove(value)),
				Ordering::Less => (false, right_child.remove(value))
			}
		};

		if equal {
			// if we have a left child, use the predecessor
			if let Some(left_child) = self.left_child_mut() {
				let pred = left_child.remove_predecessor();
				removed = Some(mem::replace(self.content_mut().unwrap(), pred.unwrap()));
			}
			// if we have a right child but no left child, use the successor
			else if let Some(right_child) = self.right_child_mut() {
				let suc = right_child.remove_successor();
				removed = Some(mem::replace(self.content_mut().unwrap(), suc.unwrap()));
			}
			// else we have a leaf, so just delete it
			else {
				removed = Some(self.take().unbox().unwrap().content);
			}
		}

		if removed.is_some() {
			self.remove_cleanup();
		}
		removed
	}

	/// Remove the successor (smallest node) of the parent of this node and return its content.
	pub(crate) fn remove_successor(&mut self) -> Option<T> {
		let suc = if let Some(left_child) = self.left_child_mut() {
			left_child.remove_successor()
		} else {
			match self.take().unbox() {
				None => None,
				Some(Node {
					right_child, content, ..
				}) => {
					*self = right_child;
					Some(content)
				}
			}
		};

		if suc.is_some() {
			self.remove_cleanup();
		}
		suc
	}

	/// Remove the predecessor (largest node) of the parent of this node and return its content.
	pub(crate) fn remove_predecessor(&mut self) -> Option<T> {
		let pred = if let Some(right_child) = self.right_child_mut() {
			right_child.remove_predecessor()
		} else {
			match self.take().unbox() {
				None => None,
				Some(Node { left_child, content, .. }) => {
					*self = left_child;
					Some(content)
				}
			}
		};

		if pred.is_some() {
			self.remove_cleanup();
		}
		pred
	}

	/// Run fixes necessary after removing/replacing `self` or one of the child nodes to retain
	/// the AA tree properties.
	fn remove_cleanup(&mut self) {
		if let Some(Node {
			level,
			left_child,
			right_child,
			..
		}) = self.as_mut()
		{
			// decrease the level if necessary
			let expected = left_child.level().min(right_child.level()) + 1;
			if expected < *level {
				*level = expected;
				if expected < right_child.level() {
					right_child.set_level(expected);
				}
			}

			// rebalance the tree by applying 3x skew and 2x split
			let mut node = self.take();
			node = node.skew();
			if let Some(right_child) = node.right_child_mut() {
				let mut right_child = right_child.take().skew();
				if let Some(right_grandchild) = right_child.right_child_mut() {
					let right_grandchild = right_grandchild.take().skew();
					right_child.set_right_child(right_grandchild);
				}
				node.set_right_child(right_child);
			}
			node = node.split();
			if let Some(right_child) = node.right_child_mut() {
				let right_child = right_child.take().split();
				node.set_right_child(right_child);
			}
			*self = node;
		}
	}
}
