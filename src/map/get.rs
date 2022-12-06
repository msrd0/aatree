//! This method defines several access methods for [`AATreeMap`].

use super::{entry::Entry, AATreeMap};
use crate::node::TraverseStep;
use core::{borrow::Borrow, cmp::Ordering};

impl<K, V> AATreeMap<K, V> {
	/// Returns a reference to the value corresponding to the key.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.get(&1), Some(&"a"));
	/// assert_eq!(map.get(&2), None);
	/// ```
	pub fn get<Q>(&self, key: &Q) -> Option<&V>
	where
		K: Ord + Borrow<Q>,
		Q: Ord + ?Sized
	{
		self.root.traverse(
			|content| match key.cmp(content.key.borrow()) {
				Ordering::Equal => TraverseStep::Value(Some(&content.value)),
				Ordering::Less => TraverseStep::Left,
				Ordering::Greater => TraverseStep::Right
			},
			|_, sub| sub
		)
	}

	/// Returns a reference to the key and value corresponding to the key.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
	/// assert_eq!(map.get_key_value(&2), None);
	/// ```
	pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
	where
		K: Ord + Borrow<Q>,
		Q: Ord + ?Sized
	{
		self.root.traverse(
			|content| match key.cmp(content.key.borrow()) {
				Ordering::Equal => TraverseStep::Value(Some(content.as_tuple())),
				Ordering::Less => TraverseStep::Left,
				Ordering::Greater => TraverseStep::Right
			},
			|_, sub| sub
		)
	}

	/// Returns a mutable reference to the value corresponding to the key.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.get(&1), Some(&"a"));
	/// *map.get_mut(&1).unwrap() = "b";
	/// assert_eq!(map.get(&1), Some(&"b"));
	/// ```
	pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
	where
		K: Ord + Borrow<Q>,
		Q: Ord + ?Sized
	{
		self.root.traverse_mut(
			|content, _| match content.key.borrow().cmp(k) {
				Ordering::Greater => TraverseStep::Left,
				Ordering::Less => TraverseStep::Right,
				Ordering::Equal => TraverseStep::Value(Some(&mut content.value))
			},
			|content| {
				if content.key.borrow() == k {
					Some(&mut content.value)
				} else {
					None
				}
			}
		)
	}

	/// Returns a reference to the first entry (that is, with the smallest key) in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.first_key_value(), None);
	/// map.insert(3, "a");
	/// map.insert(1, "b");
	/// map.insert(2, "c");
	/// assert_eq!(map.first_key_value(), Some((&1, &"b")));
	/// ```
	pub fn first_key_value(&self) -> Option<(&K, &V)>
	where
		K: Ord
	{
		self.root.traverse(
			|_| TraverseStep::Left,
			|content, sub| sub.or_else(|| Some(content.as_tuple()))
		)
	}

	/// Returns and removes the first entry (that is, with the smallest key) in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.pop_first(), None);
	/// map.insert(3, "a");
	/// map.insert(1, "b");
	/// map.insert(2, "c");
	/// assert_eq!(map.pop_first(), Some((1, "b")));
	/// assert_eq!(map.pop_first(), Some((2, "c")));
	/// assert_eq!(map.pop_first(), Some((3, "a")));
	/// assert_eq!(map.pop_first(), None);
	/// ```
	pub fn pop_first(&mut self) -> Option<(K, V)>
	where
		K: Ord
	{
		self.root.remove_successor().map(Entry::into_tuple)
	}

	/// Returns a reference to the last entry (that is, with the largest key) in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.last_key_value(), None);
	/// map.insert(1, "a");
	/// map.insert(3, "b");
	/// map.insert(2, "c");
	/// assert_eq!(map.last_key_value(), Some((&3, &"b")));
	/// ```
	pub fn last_key_value(&self) -> Option<(&K, &V)>
	where
		K: Ord
	{
		self.root.traverse(
			|_| TraverseStep::Right,
			|content, sub| sub.or_else(|| Some(content.as_tuple()))
		)
	}

	/// Returns and removes the last entry (that is, with the largest key) in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.pop_last(), None);
	/// map.insert(1, "a");
	/// map.insert(3, "b");
	/// map.insert(2, "c");
	/// assert_eq!(map.pop_last(), Some((3, "b")));
	/// assert_eq!(map.pop_last(), Some((2, "c")));
	/// assert_eq!(map.pop_last(), Some((1, "a")));
	/// assert_eq!(map.pop_last(), None);
	/// ```
	pub fn pop_last(&mut self) -> Option<(K, V)>
	where
		K: Ord
	{
		self.root.remove_predecessor().map(Entry::into_tuple)
	}

	pub fn pop_largest(&mut self) -> Option<(K, V)>
	where
		K: Ord
	{
		self.pop_last()
	}

	/// Returns a reference to the first entry with a key greater than or equal to `k` in
	/// the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.first_key_value_at_or_after(&15), None);
	/// map.insert(10, "a");
	/// map.insert(30, "b");
	/// map.insert(20, "c");
	/// assert_eq!(map.first_key_value_at_or_after(&15), Some((&20, &"c")));
	/// ```
	pub fn first_key_value_at_or_after<Q>(&self, k: &Q) -> Option<(&K, &V)>
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.traverse(
			|content| match content.key.borrow().cmp(k) {
				Ordering::Greater => TraverseStep::Left,
				Ordering::Less => TraverseStep::Right,
				Ordering::Equal => TraverseStep::Value(Some(content.as_tuple()))
			},
			|content, sub| match sub {
				None if content.key.borrow() > k => Some(content.as_tuple()),
				sub => sub
			}
		)
	}

	/// Returns a mutable reference to the first entry with a key greater than or equal
	/// to `k` in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.first_key_value_mut_at_or_after(&15), None);
	/// map.insert(10, "a");
	/// map.insert(30, "b");
	/// map.insert(20, "c");
	/// let value: &mut &str = map.first_key_value_mut_at_or_after(&15).unwrap().1;
	/// assert_eq!(*value, "c");
	/// *value = "d";
	/// assert_eq!(map.first_key_value_at_or_after(&15), Some((&20, &"d")));
	/// ```
	pub fn first_key_value_mut_at_or_after<Q>(&mut self, k: &Q) -> Option<(&K, &mut V)>
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.traverse_mut(
			|content, ctx| match content.key.borrow().cmp(k) {
				Ordering::Less => TraverseStep::Right,
				Ordering::Greater if ctx.has_left_child() => TraverseStep::Left,
				_ => TraverseStep::Value(Some((&content.key, &mut content.value)))
			},
			|content| {
				if content.key.borrow() > k {
					Some((&content.key, &mut content.value))
				} else {
					None
				}
			}
		)
	}

	/// Returns a reference to the last entry with a key smaller than or equal to `k` in
	/// the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.last_key_value_at_or_before(&25), None);
	/// map.insert(10, "a");
	/// map.insert(30, "b");
	/// map.insert(20, "c");
	/// assert_eq!(map.last_key_value_at_or_before(&25), Some((&20, &"c")));
	/// ```
	pub fn last_key_value_at_or_before<Q>(&self, k: &Q) -> Option<(&K, &V)>
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.traverse(
			|content| match content.key.borrow().cmp(k) {
				Ordering::Greater => TraverseStep::Left,
				Ordering::Less => TraverseStep::Right,
				Ordering::Equal => TraverseStep::Value(Some(content.as_tuple()))
			},
			|content, sub| match sub {
				None if content.key.borrow() < k => Some(content.as_tuple()),
				sub => sub
			}
		)
	}

	/// Returns a mutable reference to the last entry with a key smaller than or equal to
	/// `k` in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.last_key_value_mut_at_or_before(&25), None);
	/// map.insert(10, "a");
	/// map.insert(30, "b");
	/// map.insert(20, "c");
	/// let value: &mut &str = map.last_key_value_mut_at_or_before(&25).unwrap().1;
	/// assert_eq!(*value, "c");
	/// *value = "d";
	/// assert_eq!(map.last_key_value_at_or_before(&25), Some((&20, &"d")));
	/// ```
	pub fn last_key_value_mut_at_or_before<Q>(&mut self, k: &Q) -> Option<(&K, &mut V)>
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.traverse_mut(
			|content, ctx| match content.key.borrow().cmp(k) {
				Ordering::Greater => TraverseStep::Left,
				Ordering::Less if ctx.has_right_child() => TraverseStep::Right,
				_ => TraverseStep::Value(Some((&content.key, &mut content.value)))
			},
			|content| {
				if content.key.borrow() < k {
					Some((&content.key, &mut content.value))
				} else {
					None
				}
			}
		)
	}
}
