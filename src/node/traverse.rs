use super::{AANode, Node};

/// This type specifies the requested step for [`traverse`](AANode::traverse).
#[derive(Debug)]
pub enum TraverseStep<R> {
	Left,
	Right,
	Value(Option<R>)
}

#[derive(Debug)]
pub(crate) struct TraverseMutContext {
	pub(crate) has_left_child: bool,
	pub(crate) has_right_child: bool
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

	/// Traverse the tree, allowing for mutation of the nodes that are being traversed.
	///
	/// **It is a logic error to mutate the nodes in a way that changes their order with
	/// respect to the other nodes in the tree.**
	pub(crate) fn traverse_mut<'a, F, L, R>(
		&'a mut self,
		callback: F,
		leaf_callback: L
	) -> Option<R>
	where
		F: Fn(&'a mut T, TraverseMutContext) -> TraverseStep<R> + Copy,
		L: Fn(&'a mut T) -> Option<R>
	{
		self.as_mut().and_then(
			|Node {
			     content,
			     left_child,
			     right_child,
			     ..
			 }| {
				match (left_child.is_nil(), right_child.is_nil()) {
					(true, true) => leaf_callback(content),
					(left_is_nil, right_is_nil) => {
						let ctx = TraverseMutContext {
							has_left_child: !left_is_nil,
							has_right_child: !right_is_nil
						};
						let child = match callback(content, ctx) {
							TraverseStep::Left => left_child,
							TraverseStep::Right => right_child,
							TraverseStep::Value(val) => return val
						};
						child.traverse_mut(callback, leaf_callback)
					}
				}
			}
		)
	}
}
