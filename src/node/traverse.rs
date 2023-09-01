use super::{AANode, Node};
use core::fmt::{self, Debug, Display, Formatter};

pub(crate) struct TraverseError(&'static str);

impl Display for TraverseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Attempt to turn {} but there is no such child", self.0)
	}
}

impl Debug for TraverseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Display::fmt(self, f)
	}
}

/// Implementation detail of the common traverse implementation.
mod private {
	use super::AANode;

	pub trait TraverseIfaceCommon<T> {
		fn node(&self) -> &AANode<T>;
	}
}
use private::TraverseIfaceCommon;

/// Traverse Interface.
pub trait TraverseIface<T>: TraverseIfaceCommon<T> {
	/// Return if this node is a leaf (i.e. it is not nil and has no children).
	fn is_leaf(&self) -> bool {
		self.node().is_leaf()
	}

	/// Peek the content of this node (unless it is nil).
	fn peek(&self) -> &T {
		&self
			.node()
			.as_ref()
			.expect("This node should not be nil")
			.content
	}

	/// Return if this node has a left child.
	fn has_left_child(&self) -> bool {
		self.node()
			.as_ref()
			.map(|node| !node.left_child.is_nil())
			.unwrap_or(false)
	}

	/// Peek the content of the left child.
	fn peek_left_child(&self) -> Option<&T> {
		self.node()
			.as_ref()
			.and_then(|node| node.left_child.as_ref().map(|left| &left.content))
	}

	/// Return if this node has a right child.
	fn has_right_child(&self) -> bool {
		self.node()
			.as_ref()
			.map(|node| !node.right_child.is_nil())
			.unwrap_or(false)
	}

	/// Peek the content of the left child.
	fn peek_right_child(&self) -> Option<&T> {
		self.node()
			.as_ref()
			.and_then(|node| node.right_child.as_ref().map(|left| &left.content))
	}
}

impl<T, I: TraverseIfaceCommon<T>> TraverseIface<T> for I {}

/// This type specifies the requested step for [`traverse`](AANode::traverse).
#[derive(Debug)]
pub enum TraverseStep<R> {
	Left,
	Right,
	Value(Option<R>)
}

impl<T> AANode<T> {
	/// Traverse the tree looking for a specific value.
	///
	/// `down_callback` is called for each node on the way down the tree. It is passed the
	/// value contained in the current node and may return either `Left` or `Right` to
	/// continue the traversal in that direction, or `Value` to stop the traversal, for
	/// example because a value was found.
	///
	/// `up_callback` is called while going back up with the content of each node and the
	/// result of traversing so far (i.e., `None` for the first call when the search hit a
	/// leaf, or the return value of the last callback execution otherwise).
	pub fn traverse<'a, F, G, R>(&'a self, down_callback: F, up_callback: G) -> Option<R>
	where
		F: Fn(&'a T) -> TraverseStep<R> + Copy,
		G: Fn(&'a T, Option<R>) -> Option<R> + Copy
	{
		self.as_ref().and_then(
			|Node {
			     content,
			     left_child,
			     right_child,
			     ..
			 }| {
				let child = match down_callback(content) {
					TraverseStep::Left => left_child,
					TraverseStep::Right => right_child,
					TraverseStep::Value(v) => return v
				};
				up_callback(content, child.traverse(down_callback, up_callback))
			}
		)
	}
}

pub(crate) struct TraverseMut<'a, T> {
	node: &'a mut AANode<T>
}

impl<T> TraverseIfaceCommon<T> for TraverseMut<'_, T> {
	fn node(&self) -> &AANode<T> {
		self.node
	}
}

#[allow(dead_code)] // I might need that in the future
impl<'a, T> TraverseMut<'a, T> {
	/// Return the content of this node (unless it is nil).
	pub(crate) fn into_content(self) -> &'a mut T {
		&mut self
			.node
			.as_mut()
			.expect("This node should not be nil")
			.content
	}

	/// Continue traversing the tree with the left child of the current node.
	pub(crate) fn turn_left(self) -> Result<Self, TraverseError> {
		Ok(Self {
			node: self
				.node
				.as_mut()
				.and_then(|node| {
					(!node.left_child.is_nil()).then(|| &mut node.left_child)
				})
				.ok_or(TraverseError("left"))?
		})
	}

	/// Continue traversing the tree with the left child of the current node.
	pub(crate) fn turn_right(self) -> Result<Self, TraverseError> {
		Ok(Self {
			node: self
				.node
				.as_mut()
				.and_then(|node| {
					(!node.right_child.is_nil()).then(|| &mut node.right_child)
				})
				.ok_or(TraverseError("right"))?
		})
	}
}

impl<T> AANode<T> {
	/// Traverse the tree, allowing for mutation of the nodes that are being traversed.
	///
	/// **It is a logic error to mutate the nodes in a way that changes their order with
	/// respect to the other nodes in the tree.**
	pub(crate) fn traverse_mut(&mut self) -> Option<TraverseMut<'_, T>> {
		(!self.is_nil()).then(|| TraverseMut { node: self })
	}
}
