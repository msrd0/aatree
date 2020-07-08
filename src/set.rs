use crate::tree::AATree;

/// A set based on an AA-Tree. An AA-Tree is a self-balancing binary search tree based on a RedBlack-Tree
/// with a simplified self-balancing logic that should benefit performance. See [`AATreeMap`]'s documentation
/// for a detailed discussion of this collection's performance benefits and drawbacks.
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
/// //if !books.contains("The Winds of Winter") {
/// //	println!("We have {} books, but The Winds of Winter ain't one.", books.len());
/// 	//}
/// // else { assert!(false) }
///
/// // Remove a book.
/// //books.remove("The Odyssey");
///
/// // Iterate over everything.
/// //for book in &books {
/// //	println!("{}", book);
/// 	//}
/// ```
///
///  [`AATreeMap`]: struct.AATreeMap.html
///  [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
#[derive(Clone, Debug)]
pub struct AATreeSet<T> {
	tree: AATree<T>,
	len: usize
}

impl<T> AATreeSet<T> {
	/// Construct a new, empty AA-Tree based set.
	pub fn new() -> Self {
		Self {
			tree: AATree::new(),
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
}

impl<T> Default for AATreeSet<T> {
	fn default() -> Self {
		Self::new()
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
	/// use aatree::AATreeSet;
	///
	/// let mut set = AATreeSet::new();
	/// set.insert(42);
	/// set.insert(42);
	/// assert_eq!(set.len(), 1);
	/// ```
	pub fn insert(&mut self, value: T) -> bool {
		let inserted = self.tree.insert(value);
		if inserted {
			self.len += 1;
		}
		inserted
	}
}
