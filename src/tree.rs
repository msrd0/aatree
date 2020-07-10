use super::node::AANode;

/// An AA Tree. This basically stores the root `AANode<T>` but hides the fact that the root node
/// needs to be replaced in several operations. Therefore, this type, unlike `AANode`, is safe
/// to be used outside of this crate, but most use cases should prefer [`AATreeSet`] or [`AATreeMap`].
///
///  [`AATreeMap`]: ../struct.AATreeMap.html
///  [`AATreeSet`]: ../struct.AATreeSet.html
#[derive(Clone, Debug)]
pub struct AATree<T> {
	pub(super) root: AANode<T>
}

impl<T> AATree<T> {
	pub fn new() -> Self {
		Self { root: AANode::Nil }
	}
}

impl<T> Default for AATree<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: Ord> AATree<T> {
	/// Add a value to this tree. If the value already exists in the tree, nothing
	/// is inserted and `false` is returned.
	pub fn insert(&mut self, value: T) -> bool {
		let root = std::mem::replace(&mut self.root, AANode::Nil);
		let (root, inserted) = root.insert(value);
		self.root = root;
		inserted
	}
}

impl<T: Ord + PartialEq> AATree<T> {
	/// Remove a value from this tree. If the value was found, it will be returned.
	pub fn remove(&mut self, value: &T) -> Option<T> {
		let root = std::mem::replace(&mut self.root, AANode::Nil);
		let (root, removed) = root.remove(value);
		self.root = root;
		removed
	}
}

#[derive(Debug)]
pub enum TraverseStep<R> {
	Left,
	Right,
	Value(Option<R>)
}

impl<T> AATree<T> {
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
		let res = self.root.traverse(callback);
		match res {
			TraverseStep::Value(v) => v,
			_ => {
				error!("Recursive traversal attempt detected and prohibited");
				None
			}
		}
	}
}
