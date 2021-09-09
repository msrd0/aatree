use crate::{
	iter::{AAIntoIter, AAIter},
	node::{AANode, TraverseStep}
};
use core::{
	borrow::Borrow,
	fmt::{self, Debug},
	iter::FromIterator
};

/// A set based on an AA-Tree.
///
/// See [`AATreeMap`]'s documentation for a detailed discussion of this collection's performance benefits and drawbacks.
///
/// It is a logic error for an item to be modified in such a way that the item's ordering relative to any
/// other item, as determined by the `Ord` trait, changes while it is in the set. This is normally only possible
/// through `Cell`, `RefCell`, global state, I/O, or unsafe code.
///
/// # Example
///
/// This example is adopted from [`BTreeSet`]'s documentation:
///
/// ```rust
/// use aatree::AATreeSet;
///
/// let mut books = AATreeSet::new();
///
/// // Add some books.
/// books.insert("A Dance With Dragons");
/// books.insert("To Kill a Mockingbird");
/// books.insert("The Odyssey");
/// books.insert("The Great Gatsby");
///
/// // Check for a specific one
/// if !books.contains("The Winds of Winter") {
/// 	println!("We have {} books, but The Winds of Winter ain't one.", books.len());
/// }
/// # else { assert!(false) }
///
/// // Remove a book.
/// books.remove("The Odyssey");
///
/// // Iterate over everything.
/// for book in &books {
/// 	println!("{}", book);
/// }
/// # assert_eq!(books.into_iter().collect::<Vec<_>>(), vec!["A Dance With Dragons", "The Great Gatsby", "To Kill a Mockingbird"]);
/// ```
///
///  [`AATreeMap`]: crate::AATreeMap
///  [`BTreeSet`]: std::collections::BTreeSet
#[derive(Clone)]
pub struct AATreeSet<T> {
	root: AANode<T>,
	len: usize
}

impl<T> Default for AATreeSet<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: Debug> Debug for AATreeSet<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("[")?;
		for (i, v) in self.iter().enumerate() {
			if i > 0 {
				f.write_str(", ")?;
			}
			v.fmt(f)?;
		}
		f.write_str("]")
	}
}

impl<T> AATreeSet<T> {
	/// Construct a new, empty AA-Tree based set.
	pub const fn new() -> Self {
		Self {
			root: AANode::new(),
			len: 0
		}
	}

	/// Returns the number of elements in the set.
	pub fn len(&self) -> usize {
		self.len
	}

	/// Returns `true` if the set contains no elements.
	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	/// Creates an iterator over this set that visits the values in ascending order.
	pub fn iter<'a>(&'a self) -> AAIter<'a, T> {
		self.into_iter()
	}
}

impl<T: Ord> AATreeSet<T> {
	/// Adds a value to the set.
	///
	/// If the set did already contain this value, the entry is not updated, and
	/// `false` is returned.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeSet;
	/// let mut set = AATreeSet::new();
	/// set.insert(42);
	/// set.insert(42);
	/// assert_eq!(set.len(), 1);
	/// ```
	pub fn insert(&mut self, value: T) -> bool {
		let inserted = self.root.insert(value);
		if inserted {
			self.len += 1;
		}
		inserted
	}

	/// Returns the smallest element of the set.
	///
	/// # Example
	/// ```rust
	/// # use aatree::AATreeSet;
	/// let mut set = AATreeSet::new();
	/// assert!(set.smallest().is_none());
	/// set.insert(42);
	/// set.insert(44);
	/// set.insert(40);
	/// assert_eq!(set.smallest(), Some(&40));
	/// ```
	pub fn smallest(&self) -> Option<&T> {
		self.root.traverse(|content, sub| match sub {
			Some(TraverseStep::Value(None)) => TraverseStep::Value(Some(content)),
			Some(sub) => sub,
			None => TraverseStep::Left
		})
	}

	/// Returns the largest element of the set.
	///
	/// # Example
	/// ```rust
	/// # use aatree::AATreeSet;
	/// let mut set = AATreeSet::new();
	/// assert!(set.largest().is_none());
	/// set.insert(42);
	/// set.insert(44);
	/// set.insert(40);
	/// assert_eq!(set.largest(), Some(&44));
	/// ```
	pub fn largest(&self) -> Option<&T> {
		self.root.traverse(|content, sub| match sub {
			Some(TraverseStep::Value(None)) => TraverseStep::Value(Some(content)),
			Some(sub) => sub,
			None => TraverseStep::Right
		})
	}

	/// Returns `true` if the set contains an element with the given value.
	///
	/// # Example
	/// ```rust
	/// # use aatree::AATreeSet;
	/// let mut set = AATreeSet::new();
	/// set.insert(43);
	/// assert_eq!(set.contains(&42), false);
	/// set.insert(42);
	/// assert_eq!(set.contains(&42), true);
	/// ```
	pub fn contains<Q>(&self, x: &Q) -> bool
	where
		T: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root
			.traverse(|content, sub| match sub {
				Some(sub) => sub,
				None => {
					let content = content.borrow();
					if content == x {
						TraverseStep::Value(Some(()))
					} else if content < x {
						TraverseStep::Right
					} else {
						TraverseStep::Left
					}
				}
			})
			.is_some()
	}

	/// Returns the smallest element of the set that is greater or equal to `x`.
	///
	/// # Example
	/// ```rust
	/// # use aatree::AATreeSet;
	/// let mut set = AATreeSet::new();
	/// assert!(set.smallest_geq_than(&41).is_none());
	/// set.insert(42);
	/// set.insert(44);
	/// set.insert(40);
	/// assert_eq!(set.smallest_geq_than(&41), Some(&42));
	/// ```
	pub fn smallest_geq_than<Q>(&self, x: &Q) -> Option<&T>
	where
		T: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.traverse(|content, sub| {
			let c = content.borrow();
			match sub {
				Some(TraverseStep::Value(None)) if c > x => TraverseStep::Value(Some(content)),
				Some(sub) => sub,
				None => {
					if c < x {
						TraverseStep::Right
					} else if c > x {
						TraverseStep::Left
					} else {
						TraverseStep::Value(Some(content))
					}
				},
			}
		})
	}

	/// Returns the largest element of the set that is smaller or equal to `x`.
	///
	/// # Example
	/// ```rust
	/// # use aatree::AATreeSet;
	/// let mut set = AATreeSet::new();
	/// assert!(set.largest_leq_than(&43).is_none());
	/// set.insert(42);
	/// set.insert(44);
	/// set.insert(40);
	/// assert_eq!(set.largest_leq_than(&43), Some(&42));
	/// ```
	pub fn largest_leq_than<Q>(&self, x: &Q) -> Option<&T>
	where
		T: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.traverse(|content, sub| {
			let c = content.borrow();
			match sub {
				Some(TraverseStep::Value(None)) if c < x => TraverseStep::Value(Some(content)),
				Some(sub) => sub,
				None => {
					if c > x {
						TraverseStep::Left
					} else if c < x {
						TraverseStep::Right
					} else {
						TraverseStep::Value(Some(content))
					}
				},
			}
		})
	}

	/// Removes a value from the set, and returns `true` if it was removed.
	pub fn remove<Q>(&mut self, x: &Q) -> bool
	where
		T: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.remove(x).is_some()
	}

	/// Removes a value from the set, and returns the value that was removed.
	pub fn take<Q>(&mut self, x: &Q) -> Option<T>
	where
		T: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.remove(x)
	}
}

impl<T: Ord> FromIterator<T> for AATreeSet<T> {
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = T>
	{
		let mut set = Self::new();
		for value in iter {
			set.insert(value);
		}
		set
	}
}

impl<T> IntoIterator for AATreeSet<T> {
	type Item = T;
	type IntoIter = AAIntoIter<T>;

	fn into_iter(self) -> AAIntoIter<T> {
		AAIntoIter::new(self.root, self.len)
	}
}

impl<'a, T> IntoIterator for &'a AATreeSet<T> {
	type Item = &'a T;
	type IntoIter = AAIter<'a, T>;

	fn into_iter(self) -> AAIter<'a, T> {
		AAIter::new(&self.root, self.len)
	}
}
