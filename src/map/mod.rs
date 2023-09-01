use crate::{
	iter::{AAIntoIter, AAIter},
	node::AANode
};
use alloc::vec::Vec;
use core::{
	borrow::Borrow,
	cmp::Ordering,
	fmt::{self, Debug},
	iter::FromIterator,
	mem
};

mod entry;
mod get;
mod kv;

pub use entry::{Entry, OccupiedEntry, VacantEntry};
use kv::KeyValue;

#[derive(Clone)]
pub struct AATreeMap<K, V> {
	root: AANode<KeyValue<K, V>>,
	len: usize
}

impl<K, V> Default for AATreeMap<K, V> {
	fn default() -> Self {
		Self::new()
	}
}

impl<K: Debug, V: Debug> Debug for AATreeMap<K, V> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("{")?;
		for (i, (k, v)) in self.iter().enumerate() {
			if i > 0 {
				f.write_str(", ")?;
			}
			k.fmt(f)?;
			f.write_str(": ")?;
			v.fmt(f)?;
		}
		f.write_str("}")
	}
}

impl<K: PartialEq, V: PartialEq> PartialEq for AATreeMap<K, V> {
	fn eq(&self, other: &Self) -> bool {
		self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a == b)
	}
}

impl<K: Eq, V: Eq> Eq for AATreeMap<K, V> {}

impl<K: PartialOrd, V: PartialOrd> PartialOrd for AATreeMap<K, V> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.iter().partial_cmp(other.iter())
	}
}

impl<K: Ord, V: Ord> Ord for AATreeMap<K, V> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.iter().cmp(other.iter())
	}
}

impl<K, V> AATreeMap<K, V> {
	/// Construct a new, empty AA-Tree based map.
	///
	/// # Example
	///
	/// ```rust
	/// # type AATreeMap = aatree::AATreeMap<i64, ()>;
	/// let map = AATreeMap::new();
	/// assert!(map.is_empty());
	/// ```
	pub const fn new() -> Self {
		Self {
			root: AANode::new(),
			len: 0
		}
	}

	/// Returns the number of elements in the map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert_eq!(map.len(), 0);
	/// map.insert(1, "a");
	/// assert_eq!(map.len(), 1);
	/// ```
	pub fn len(&self) -> usize {
		self.len
	}

	/// Returns `true` if the map contains no elements.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert!(map.is_empty());
	/// map.insert(1, "a");
	/// assert!(!map.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	/// Clears the map, removing all elements.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// map.insert(1, "a");
	/// map.clear();
	/// assert!(map.is_empty());
	/// ```
	pub fn clear(&mut self) {
		self.root = AANode::new();
		self.len = 0;
	}

	/// Creates an iterator over this map that visits all entries with the keys in ascending order.
	pub fn iter(&self) -> AAIter<'_, KeyValue<K, V>, (&K, &V)> {
		self.into_iter()
	}

	/// Creates an iterator visiting all the keys, in sorted order.
	pub fn keys(&self) -> impl Iterator<Item = &K> {
		// TODO is there a better way to implement this?
		self.iter().map(|(k, _)| k)
	}

	/// Creates an iterator visiting all the values, in sorted order.
	pub fn values(&self) -> impl Iterator<Item = &V> {
		// TODO is there a better way to implement this?
		self.iter().map(|(_, v)| v)
	}

	/// Creates a consuming iterator visiting all the keys, in sorted order. The map
	/// cannot be used after calling this.
	pub fn into_keys(self) -> impl Iterator<Item = K> {
		// TODO is there a better way to implement this?
		self.into_iter().map(|(k, _)| k)
	}

	/// Creates a consuming iterator visiting all the values, in order by key. The map
	/// cannot be used after calling this.
	pub fn into_values(self) -> impl Iterator<Item = V> {
		// TODO is there a better way to implement this?
		self.into_iter().map(|(_, v)| v)
	}

	/// Insert a new element into the map, or overwrite an existing element
	/// with the same key. If a value was overwritten, the old value will be
	/// returned.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.get(&1), Some(&"a"));
	/// map.insert(1, "b");
	/// assert_eq!(map.get(&1), Some(&"b"));
	/// ```
	pub fn insert(&mut self, key: K, value: V) -> Option<V>
	where
		K: Ord
	{
		let inserted = self.root.insert_or_replace(KeyValue { key, value });
		match inserted {
			None => {
				self.len += 1;
				None
			},
			Some(entry) => Some(entry.value)
		}
	}

	/// Moves all elements from `other` into `self`, leaving `other` empty.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::BTreeMap;
	///
	/// let mut a = BTreeMap::new();
	/// a.insert(1, "a");
	/// a.insert(2, "b");
	/// a.insert(3, "c");
	///
	/// let mut b = BTreeMap::new();
	/// b.insert(3, "d");
	/// b.insert(4, "e");
	/// b.insert(5, "f");
	///
	/// a.append(&mut b);
	///
	/// assert_eq!(a.len(), 5);
	/// assert_eq!(b.len(), 0);
	///
	/// assert_eq!(a[&1], "a");
	/// assert_eq!(a[&2], "b");
	/// assert_eq!(a[&3], "d");
	/// assert_eq!(a[&4], "e");
	/// assert_eq!(a[&5], "f");
	/// ```
	pub fn append(&mut self, other: &mut Self)
	where
		K: Ord
	{
		self.extend(mem::take(other));
	}

	/// Check if a key is contained within this map.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// assert!(!map.contains_key(&1));
	/// map.insert(1, "a");
	/// assert!(map.contains_key(&1));
	/// ```
	pub fn contains_key<Q>(&self, k: &Q) -> bool
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.kv(k).is_some()
	}

	/// Remove a key from the map if it exists, and return the value that was previously stored
	/// in the map for that key.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// map.insert(1, "a");
	/// map.insert(2, "b");
	/// assert_eq!(map.get(&1), Some(&"a"));
	/// let value = map.remove(&1);
	/// assert_eq!(value, Some("a"));
	/// assert_eq!(map.get(&1), None);
	/// ```
	pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.remove::<Q, K>(k).map(|entry| entry.value)
	}

	/// Remove a key from the map if it exists, and return the key and the value that was
	/// previously stored in the map for that key.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::AATreeMap;
	/// let mut map = AATreeMap::new();
	/// map.insert(1, "a");
	/// map.insert(2, "b");
	/// assert_eq!(map.get(&1), Some(&"a"));
	/// let value = map.remove(&1);
	/// assert_eq!(value, Some("a"));
	/// assert_eq!(map.get(&1), None);
	/// ```
	pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		self.root.remove::<Q, K>(k).map(KeyValue::into_tuple)
	}
}

impl<K: Ord, V> FromIterator<(K, V)> for AATreeMap<K, V> {
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = (K, V)>
	{
		let mut map = Self::new();
		for (key, value) in iter {
			map.insert(key, value);
		}
		map
	}
}

impl<K: Ord, V, const N: usize> From<[(K, V); N]> for AATreeMap<K, V> {
	fn from(array: [(K, V); N]) -> Self {
		array.into_iter().collect()
	}
}

impl<K: Ord, V> From<Vec<(K, V)>> for AATreeMap<K, V> {
	fn from(vec: Vec<(K, V)>) -> Self {
		vec.into_iter().collect()
	}
}

impl<K: Ord, V> Extend<(K, V)> for AATreeMap<K, V> {
	fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
		for (key, value) in iter {
			self.insert(key, value);
		}
	}
}

impl<'a, K: Ord + Copy + 'a, V: Ord + Copy + 'a> Extend<(&'a K, &'a V)>
	for AATreeMap<K, V>
{
	fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
		self.extend(iter.into_iter().map(|(k, v)| (*k, *v)))
	}
}

impl<K, V> IntoIterator for AATreeMap<K, V> {
	type Item = (K, V);
	type IntoIter = AAIntoIter<KeyValue<K, V>, (K, V)>;

	fn into_iter(self) -> Self::IntoIter {
		AAIntoIter::new(self.root, self.len)
	}
}

impl<'a, K, V> IntoIterator for &'a AATreeMap<K, V> {
	type Item = (&'a K, &'a V);
	type IntoIter = AAIter<'a, KeyValue<K, V>, (&'a K, &'a V)>;

	fn into_iter(self) -> Self::IntoIter {
		AAIter::new(&self.root, self.len)
	}
}
