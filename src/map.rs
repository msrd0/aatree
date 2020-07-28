use crate::{
	iter::{AAIntoIter, AAIter},
	node::{AANode, TraverseStep}
};
use core::iter::FromIterator;

#[derive(Clone, Copy, Debug)]
pub struct AATreeMapEntry<K, V> {
	key: K,
	value: V
}

type Entry<K, V> = AATreeMapEntry<K, V>;

impl<K: PartialEq, V> PartialEq for Entry<K, V> {
	fn eq(&self, other: &Self) -> bool {
		self.key.eq(&other.key)
	}
}

impl<K: PartialEq, V> PartialEq<K> for Entry<K, V> {
	fn eq(&self, other: &K) -> bool {
		self.key.eq(other)
	}
}

impl<K: Eq, V> Eq for Entry<K, V> {}

impl<K: PartialOrd, V> PartialOrd for Entry<K, V> {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		self.key.partial_cmp(&other.key)
	}
}

impl<K: PartialOrd, V> PartialOrd<K> for Entry<K, V> {
	fn partial_cmp(&self, other: &K) -> Option<core::cmp::Ordering> {
		self.key.partial_cmp(&other)
	}
}

impl<K: Ord, V> Ord for Entry<K, V> {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		self.key.cmp(&other.key)
	}
}

#[derive(Clone, Debug)]
pub struct AATreeMap<K, V> {
	root: AANode<Entry<K, V>>,
	len: usize
}

impl<K, V> Default for AATreeMap<K, V> {
	fn default() -> Self {
		Self::new()
	}
}

impl<K, V> AATreeMap<K, V> {
	/// Construct a new, empty AA-Tree based map.
	pub fn new() -> Self {
		Self {
			root: AANode::new(),
			len: 0
		}
	}

	/// Returns the number of elements in the map.
	pub fn len(&self) -> usize {
		self.len
	}

	/// Returns `true` if the map contains no elements.
	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	/// Creates an iterator over this map that visits all entries with the keys in ascending order.
	pub fn iter<'a>(&'a self) -> AAIter<'a, Entry<K, V>> {
		self.into_iter()
	}
}

impl<K: Ord, V> AATreeMap<K, V> {
	pub fn insert(&mut self, key: K, value: V) -> bool {
		let inserted = self.root.insert(Entry { key, value });
		if inserted {
			self.len += 1;
		}
		inserted
	}

	// TODO duplicated from set
	pub fn smallest(&self) -> Option<&Entry<K, V>> {
		self.root.traverse(|content, sub| match sub {
			Some(TraverseStep::Value(None)) => TraverseStep::Value(Some(content)),
			Some(sub) => sub,
			None => TraverseStep::Left
		})
	}

	// TODO duplicated from set
	pub fn largest(&self) -> Option<&Entry<K, V>> {
		self.root.traverse(|content, sub| match sub {
			Some(TraverseStep::Value(None)) => TraverseStep::Value(Some(content)),
			Some(sub) => sub,
			None => TraverseStep::Right
		})
	}

	// TODO duplicated from set
	pub fn contains_key(&self, k: &K) {
		self.root
			.traverse(|content, sub| match sub {
				Some(sub) => sub,
				None => {
					if content == k {
						TraverseStep::Value(Some(()))
					} else if content < k {
						TraverseStep::Right
					} else {
						TraverseStep::Left
					}
				},
			})
			.is_some()
	}

	// TODO duplicated from set
	pub fn smallest_geq_than(&self, k: &K) -> Option<&Entry<K, V>> {
		self.root.traverse(|content, sub| match sub {
			Some(TraverseStep::Value(None)) if content > k => TraverseStep::Value(Some(content)),
			Some(sub) => sub,
			None => {
				if content < k {
					TraverseStep::Right
				} else if content > k {
					TraverseStep::Left
				} else {
					TraverseStep::Value(Some(content))
				}
			},
		})
	}

	// TODO duplicated from set
	pub fn largest_leq_than(&self, k: &K) -> Option<&Entry<K, V>> {
		self.root.traverse(|content, sub| match sub {
			Some(TraverseStep::Value(None)) if content < k => TraverseStep::Value(Some(content)),
			Some(sub) => sub,
			None => {
				if content > k {
					TraverseStep::Left
				} else if content < k {
					TraverseStep::Right
				} else {
					TraverseStep::Value(Some(content))
				}
			},
		})
	}
}

impl<K: Ord, V> FromIterator<Entry<K, V>> for AATreeMap<K, V> {
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = Entry<K, V>>
	{
		let mut map = Self::new();
		for value in iter {
			map.insert(value.key, value.value);
		}
		map
	}
}

impl<K: Ord, V> FromIterator<(K, V)> for AATreeMap<K, V> {
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = (K, V)>
	{
		let mut map = Self::new();
		for value in iter {
			map.insert(value.0, value.1);
		}
		map
	}
}

impl<K, V> IntoIterator for AATreeMap<K, V> {
	type Item = Entry<K, V>;
	type IntoIter = AAIntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		AAIntoIter::new(self.root, self.len)
	}
}

impl<'a, K, V> IntoIterator for &'a AATreeMap<K, V> {
	type Item = &'a Entry<K, V>;
	type IntoIter = AAIter<'a, Entry<K, V>>;

	fn into_iter(self) -> Self::IntoIter {
		AAIter::new(&self.root, self.len)
	}
}
