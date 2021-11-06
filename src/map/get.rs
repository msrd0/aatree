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
		self.root.traverse(|content, sub| match sub {
			Some(sub) => sub,
			None => match key.cmp(content.key.borrow()) {
				Ordering::Equal => TraverseStep::Value(Some(&content.value)),
				Ordering::Less => TraverseStep::Left,
				Ordering::Greater => TraverseStep::Right
			}
		})
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
		self.root.traverse(|content, sub| match sub {
			Some(sub) => sub,
			None => match key.cmp(content.key.borrow()) {
				Ordering::Equal => TraverseStep::Value(Some(content.as_tuple())),
				Ordering::Less => TraverseStep::Left,
				Ordering::Greater => TraverseStep::Right
			}
		})
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

	/// Returns a reference to the entry with the smallest key in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.smallest(), None);
	/// map.insert(3, "a");
	/// map.insert(1, "b");
	/// map.insert(2, "c");
	/// assert_eq!(map.smallest(), Some((&1, &"b")));
	/// ```
	pub fn smallest(&self) -> Option<(&K, &V)>
	where
		K: Ord
	{
		self.root.traverse(|content, sub| match sub {
			Some(TraverseStep::Value(None)) => TraverseStep::Value(Some(content.as_tuple())),
			Some(sub) => sub,
			None => TraverseStep::Left
		})
	}

	/// Returns and removes the entry with the smallest key in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.pop_smallest(), None);
	/// map.insert(3, "a");
	/// map.insert(1, "b");
	/// map.insert(2, "c");
	/// assert_eq!(map.pop_smallest(), Some((1, "b")));
	/// assert_eq!(map.pop_smallest(), Some((2, "c")));
	/// assert_eq!(map.pop_smallest(), Some((3, "a")));
	/// assert_eq!(map.pop_smallest(), None);
	/// ```
	pub fn pop_smallest(&mut self) -> Option<(K, V)>
	where
		K: Clone + Ord
	{
		self.root.remove_successor().map(Entry::into_tuple)
	}

	/// Returns a reference to the entry with the largest key in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.largest(), None);
	/// map.insert(1, "a");
	/// map.insert(3, "b");
	/// map.insert(2, "c");
	/// assert_eq!(map.largest(), Some((&3, &"b")));
	/// ```
	pub fn largest(&self) -> Option<(&K, &V)>
	where
		K: Ord
	{
		self.root.traverse(|content, sub| match sub {
			Some(TraverseStep::Value(None)) => TraverseStep::Value(Some(content.as_tuple())),
			Some(sub) => sub,
			None => TraverseStep::Right
		})
	}

	/// Returns and removes the entry with the largest key in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.pop_largest(), None);
	/// map.insert(1, "a");
	/// map.insert(3, "b");
	/// map.insert(2, "c");
	/// assert_eq!(map.pop_largest(), Some((3, "b")));
	/// assert_eq!(map.pop_largest(), Some((2, "c")));
	/// assert_eq!(map.pop_largest(), Some((1, "a")));
	/// assert_eq!(map.pop_largest(), None);
	/// ```
	pub fn pop_largest(&mut self) -> Option<(K, V)>
	where
		K: Clone + Ord
	{
		self.root.remove_predecessor().map(Entry::into_tuple)
	}

	/// Returns a reference to the entry with the smallest key greater than or equal to `k` in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.smallest_geq_than(&15), None);
	/// map.insert(10, "a");
	/// map.insert(30, "b");
	/// map.insert(20, "c");
	/// assert_eq!(map.smallest_geq_than(&15), Some((&20, &"c")));
	/// ```
	pub fn smallest_geq_than<Q>(&self, k: &Q) -> Option<(&K, &V)>
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.traverse(|content, sub| {
			let key = content.key.borrow();
			match sub {
				Some(TraverseStep::Value(None)) if key > k => TraverseStep::Value(Some(content.as_tuple())),
				Some(sub) => sub,
				None => match key.cmp(k) {
					Ordering::Greater => TraverseStep::Left,
					Ordering::Less => TraverseStep::Right,
					Ordering::Equal => TraverseStep::Value(Some(content.as_tuple()))
				}
			}
		})
	}

	/// Returns a mutable reference to the entry with the smallest key greater than or equal to `k` in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.smallest_geq_than_mut(&15), None);
	/// map.insert(10, "a");
	/// map.insert(30, "b");
	/// map.insert(20, "c");
	/// let value: &mut &str = map.smallest_geq_than_mut(&15).unwrap().1;
	/// assert_eq!(*value, "c");
	/// *value = "d";
	/// assert_eq!(map.smallest_geq_than(&15), Some((&20, &"d")));
	/// ```
	pub fn smallest_geq_than_mut<Q>(&mut self, k: &Q) -> Option<(&K, &mut V)>
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

	/// Returns a reference to the entry with the largest key smaller than or equal to `k` in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.largest_leq_than(&25), None);
	/// map.insert(10, "a");
	/// map.insert(30, "b");
	/// map.insert(20, "c");
	/// assert_eq!(map.largest_leq_than(&25), Some((&20, &"c")));
	/// ```
	pub fn largest_leq_than<Q>(&self, k: &Q) -> Option<(&K, &V)>
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.traverse(|content, sub| {
			let key = content.key.borrow();
			match sub {
				Some(TraverseStep::Value(None)) if key < k => TraverseStep::Value(Some(content.as_tuple())),
				Some(sub) => sub,
				None => match key.cmp(k) {
					Ordering::Greater => TraverseStep::Left,
					Ordering::Less => TraverseStep::Right,
					Ordering::Equal => TraverseStep::Value(Some(content.as_tuple()))
				}
			}
		})
	}

	/// Returns a mutable reference to the entry with the largest key smaller than or equal to `k` in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.largest_leq_than_mut(&25), None);
	/// map.insert(10, "a");
	/// map.insert(30, "b");
	/// map.insert(20, "c");
	/// let value: &mut &str = map.largest_leq_than_mut(&25).unwrap().1;
	/// assert_eq!(*value, "c");
	/// *value = "d";
	/// assert_eq!(map.largest_leq_than(&25), Some((&20, &"d")));
	/// ```
	pub fn largest_leq_than_mut<Q>(&mut self, k: &Q) -> Option<(&K, &mut V)>
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
