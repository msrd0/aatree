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

/// A struct to guide traversel with immutable access.
#[allow(missing_debug_implementations)]
pub struct Traverse<'a, T> {
	node: &'a AANode<T>
}

impl<T> TraverseIfaceCommon<T> for Traverse<'_, T> {
	fn node(&self) -> &AANode<T> {
		self.node
	}
}

impl<'a, T> Traverse<'a, T> {
	/// Return the content of this node (unless it is nil).
	pub fn into_content(self) -> &'a T {
		&self
			.node
			.as_ref()
			.expect("This node should not be nil")
			.content
	}

	/// Continue traversing the tree with the left child of the current node.
	pub fn turn_left(self) -> Result<Self, TraverseError> {
		Ok(Self {
			node: self
				.node
				.as_ref()
				.and_then(|node| (!node.left_child.is_nil()).then(|| &node.left_child))
				.ok_or(TraverseError("left"))?
		})
	}

	/// Continue traversing the tree with the left child of the current node.
	pub fn turn_right(self) -> Result<Self, TraverseError> {
		Ok(Self {
			node: self
				.node
				.as_ref()
				.and_then(|node| (!node.right_child.is_nil()).then(|| &node.right_child))
				.ok_or(TraverseError("right"))?
		})
	}
}

/// A struct to guide traversal with mutable access.
pub(crate) struct TraverseMut<'a, T> {
	node: &'a mut AANode<T>
}

impl<T> TraverseIfaceCommon<T> for TraverseMut<'_, T> {
	fn node(&self) -> &AANode<T> {
		self.node
	}
}

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
	/// Traverse the tree. You can call `turn_left` and `turn_right` to guide the
	/// traversal into the direction you like.
	pub(crate) fn traverse(&self) -> Option<Traverse<'_, T>> {
		(!self.is_nil()).then(|| Traverse { node: self })
	}

	/// Traverse the tree, allowing for mutation of the nodes that are being traversed.
	///
	/// **It is a logic error to mutate the nodes in a way that changes their order with
	/// respect to the other nodes in the tree.**
	pub(crate) fn traverse_mut(&mut self) -> Option<TraverseMut<'_, T>> {
		(!self.is_nil()).then(|| TraverseMut { node: self })
	}
}
