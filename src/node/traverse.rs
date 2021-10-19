use super::{AANode, Node};

/// This type specifies the requested step for [`traverse`](AANode::traverse).
#[derive(Debug)]
pub enum TraverseStep<R> {
	Left,
	Right,
	Value(Option<R>)
}

#[cfg(feature = "log")]
fn error_recursive() {
	log::error!("Recursive traversal detected and prohibited");
}

#[cfg(not(feature = "log"))]
fn error_recursive() {}

impl<T> AANode<T> {
	/// Traverse the tree looking for a specific value. The `callback` is called as follows:
	///  1. While going down, with `(content, None)` as the input. The callback may return
	///     either `Left` or `Right` to continue the traversal, or `Value` to stop the
	///     traversal, for example because a value was found.
	///  2. While going back up, with `(content, Some(res))`, where `res` is the result from
	///     the fully traversed subgraph. The callback must produce a `Value` result, a
	///     traversal (returning `Left` or `Right`) is a logic error and will be ignored.
	pub fn traverse<'a, F, R>(&'a self, callback: F) -> Option<R>
	where
		F: Fn(&'a T, Option<TraverseStep<R>>) -> TraverseStep<R> + Copy
	{
		let res = self.traverse_impl(callback);
		match res {
			TraverseStep::Value(v) => v,
			_ => {
				error_recursive();
				None
			}
		}
	}

	/// Traverse the current node, calling the callback with `(content, None)` while going down and
	/// with `(content, Some(res))` when going up, where `res` is the result obtained from the lower
	/// subtree.
	pub(super) fn traverse_impl<'a, F, R>(&'a self, callback: F) -> TraverseStep<R>
	where
		F: Fn(&'a T, Option<TraverseStep<R>>) -> TraverseStep<R> + Copy
	{
		match self.as_ref() {
			None => TraverseStep::Value(None),
			Some(Node {
				content,
				left_child,
				right_child,
				..
			}) => {
				let step = callback(content, None);
				let res = match step {
					TraverseStep::Left => left_child.traverse_impl(callback),
					TraverseStep::Right => right_child.traverse_impl(callback),
					TraverseStep::Value(_) => return step
				};
				let res = callback(content, Some(res));
				if matches!(res, TraverseStep::Left | TraverseStep::Right) {
					error_recursive();
				}
				res
			}
		}
	}

	/// Traverse the tree, allowing for mutation of the nodes that are being traversed.
	///
	/// **It is a logic error to mutate the nodes in a way that changes their order with
	/// respect to the other nodes in the tree.**
	pub(crate) fn traverse_mut<'a, F, L, R>(&'a mut self, callback: F, leaf_callback: L) -> Option<R>
	where
		F: Fn(&'a mut T) -> TraverseStep<R> + Copy,
		L: Fn(&'a mut T) -> Option<R>
	{
		let res = self.traverse_mut_impl(callback, leaf_callback);
		match res {
			TraverseStep::Value(v) => v,
			_ => {
				error_recursive();
				None
			}
		}
	}

	fn traverse_mut_impl<'a, F, L, R>(&'a mut self, callback: F, leaf_callback: L) -> TraverseStep<R>
	where
		F: Fn(&'a mut T) -> TraverseStep<R> + Copy,
		L: Fn(&'a mut T) -> Option<R>
	{
		match self.as_mut() {
			None => TraverseStep::Value(None),
			Some(Node {
				content,
				left_child,
				right_child,
				..
			}) => {
				if left_child.is_nil() && right_child.is_nil() {
					TraverseStep::Value(leaf_callback(content))
				} else {
					let step = callback(content);
					match step {
						TraverseStep::Left => left_child.traverse_mut_impl(callback, leaf_callback),
						TraverseStep::Right => right_child.traverse_mut_impl(callback, leaf_callback),
						TraverseStep::Value(_) => step
					}
				}
			},
		}
	}
}
