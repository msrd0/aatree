use super::AANode;

/// This type specifies the requested step for [`traverse`](AANode::traverse).
#[derive(Debug)]
pub enum TraverseStep<R> {
	Left,
	Right,
	Value(Option<R>)
}

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
				error!("Recursive traversal detected and prohibited");
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
					TraverseStep::Left => left_child.traverse_impl(callback),
					TraverseStep::Right => right_child.traverse_impl(callback),
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

	/// Traverse the tree, allowing for mutation of the nodes that are being traversed.
	///
	/// **It is a logic error to mutate the nodes in a way that changes their order with
	/// respect to the other nodes in the tree.**
	pub(crate) fn traverse_mut<'a, F, R>(&'a mut self, callback: F) -> Option<R>
	where
		F: Fn(&'a mut T) -> TraverseStep<R> + Copy
	{
		let res = self.traverse_mut_impl(callback);
		match res {
			TraverseStep::Value(v) => v,
			_ => {
				error!("Recursive traversal detected and prohibited");
				None
			}
		}
	}

	fn traverse_mut_impl<'a, F, R>(&'a mut self, callback: F) -> TraverseStep<R>
	where
		F: Fn(&'a mut T) -> TraverseStep<R> + Copy
	{
		match self {
			Self::Nil => TraverseStep::Value(None),
			Self::Node {
				content,
				left_child,
				right_child,
				..
			} => {
				let step = callback(content);
				match step {
					TraverseStep::Left => left_child.traverse_mut_impl(callback),
					TraverseStep::Right => right_child.traverse_mut_impl(callback),
					TraverseStep::Value(_) => step
				}
			}
		}
	}
}
