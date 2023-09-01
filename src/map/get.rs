//! This method defines several access methods for [`AATreeMap`].

use super::{AATreeMap, Entry, KeyValue, OccupiedEntry, VacantEntry};
use crate::node::{TraverseIface as _, TraverseStep};
use core::{borrow::Borrow, cmp::Ordering, fmt::Debug};

impl<K, V> AATreeMap<K, V> {
	fn kv<Q>(&self, key: &Q) -> Option<&KeyValue<K, V>>
	where
		K: Ord + Borrow<Q>,
		Q: Ord + ?Sized
	{
		self.root.traverse(
			|content| match key.cmp(content.key.borrow()) {
				Ordering::Equal => TraverseStep::Value(Some(content)),
				Ordering::Less => TraverseStep::Left,
				Ordering::Greater => TraverseStep::Right
			},
			|_, sub| sub
		)
	}

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
		self.kv(key).map(|kv| &kv.value)
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
		self.kv(key).map(|kv| kv.as_tuple())
	}

	fn kv_mut<Q>(&mut self, key: &Q) -> Option<&mut KeyValue<K, V>>
	where
		K: Ord + Borrow<Q>,
		Q: Ord + ?Sized
	{
		let mut traverse = self.root.traverse_mut()?;
		loop {
			let cmp: Ordering = traverse.peek().key.borrow().cmp(key);
			match cmp {
				Ordering::Greater => traverse = traverse.turn_left().ok()?,
				Ordering::Less => traverse = traverse.turn_right().ok()?,
				Ordering::Equal => return Some(traverse.into_content())
			}
		}
	}

	/// Gets the given key's corresponding entry, allowing for in-place manipulation of
	/// the entry as well as inserting an entry with that key if it does not exist yet.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::{AATreeMap, map::Entry};
	/// let mut map = AATreeMap::new();
	/// let entry = map.entry(1);
	/// assert!(matches!(entry, Entry::Vacant(_)));
	/// entry.or_insert('a');
	/// assert_eq!(map.get(&1), Some(&'a'));
	///
	/// let entry = map.entry(1);
	/// assert!(matches!(entry, Entry::Occupied(_)));
	/// entry.and_modify(|value| *value = 'b');
	/// assert_eq!(map.get(&1), Some(&'b'));
	/// ```
	#[allow(unsafe_code)]
	pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
	where
		K: Ord
	{
		// This is a known limitation of the borrow checker in Rust:
		// https://blog.rust-lang.org/2022/08/05/nll-by-default.html#looking-forward-what-can-we-expect-for-the-borrow-checker-of-the-future
		match unsafe { &mut *(self as *mut Self) }.kv_mut(&key) {
			Some(kv) => Entry::Occupied(OccupiedEntry { entry: kv }),
			None => Entry::Vacant(VacantEntry { key, map: self })
		}
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
	pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
	where
		K: Ord + Borrow<Q>,
		Q: Ord + ?Sized
	{
		self.kv_mut(key).map(|kv| &mut kv.value)
	}

	/// Gets the first entry (that is, with the smallest key) in the map, allowing for
	/// in-place manipulation of the entry.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::{AATreeMap, map::Entry};
	/// let mut map = AATreeMap::new();
	/// let entry = map.first_entry();
	/// assert!(entry.is_none());
	///
	/// map.insert(1, 'a');
	/// map.insert(3, 'c');
	/// println!("{map:?}");
	///
	/// let Some(mut entry) = map.first_entry() else {
	/// 	unreachable!()
	/// };
	/// *entry.get_mut() = 'b';
	/// assert_eq!(map.get(&1), Some(&'b'));
	/// assert_eq!(map.get(&3), Some(&'c'));
	/// ```
	pub fn first_entry(&mut self) -> Option<OccupiedEntry<'_, K, V>>
	where
		K: Ord + Debug,
		V: Debug
	{
		let mut traverse = self.root.traverse_mut()?;
		while traverse.has_left_child() {
			traverse = traverse.turn_left().unwrap();
		}
		Some(OccupiedEntry {
			entry: traverse.into_content()
		})
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
		self.root.remove_successor().map(KeyValue::into_tuple)
	}

	/// Gets the last entry (that is, with the largest key) in the map, allowing for
	/// in-place manipulation of the entry.
	///
	/// # Example
	///
	/// ```rust
	/// # use aatree::{AATreeMap, map::Entry};
	/// let mut map = AATreeMap::new();
	/// let entry = map.last_entry();
	/// assert!(entry.is_none());
	///
	/// map.insert(1, 'a');
	/// map.insert(3, 'c');
	///
	/// let Some(mut entry) = map.last_entry() else {
	/// 	unreachable!()
	/// };
	/// *entry.get_mut() = 'b';
	/// assert_eq!(map.get(&1), Some(&'a'));
	/// assert_eq!(map.get(&3), Some(&'b'));
	/// ```
	pub fn last_entry(&mut self) -> Option<OccupiedEntry<'_, K, V>>
	where
		K: Ord
	{
		let mut traverse = self.root.traverse_mut()?;
		while traverse.has_right_child() {
			traverse = traverse.turn_right().unwrap();
		}
		Some(OccupiedEntry {
			entry: traverse.into_content()
		})
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
		self.root.remove_predecessor().map(KeyValue::into_tuple)
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
	/// let (_, value) = map.first_key_value_mut_at_or_after(&15).unwrap();
	/// assert_eq!(*value, "c");
	/// *value = "d";
	/// assert_eq!(map.first_key_value_at_or_after(&15), Some((&20, &"d")));
	/// ```
	pub fn first_key_value_mut_at_or_after<Q>(&mut self, k: &Q) -> Option<(&K, &mut V)>
	where
		K: Borrow<Q> + Ord,
		Q: Ord + ?Sized
	{
		let mut traverse = self.root.traverse_mut()?;
		loop {
			match traverse.peek().key.borrow().cmp(k) {
				Ordering::Greater
					if traverse
						.peek_left_child()
						.map(|left| left.key.borrow() >= k)
						.unwrap_or(false) =>
				{
					traverse = traverse.turn_left().unwrap()
				},

				Ordering::Less => traverse = traverse.turn_right().ok()?,

				_ => return Some(traverse.into_content().as_tuple_mut())
			}
		}
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
	/// let (_, value) = map.last_key_value_mut_at_or_before(&25).unwrap();
	/// assert_eq!(*value, "c");
	/// *value = "d";
	/// assert_eq!(map.last_key_value_at_or_before(&25), Some((&20, &"d")));
	/// ```
	pub fn last_key_value_mut_at_or_before<Q>(&mut self, k: &Q) -> Option<(&K, &mut V)>
	where
		K: Borrow<Q> + Ord + Debug,
		V: Debug,
		Q: Ord + ?Sized
	{
		let mut traverse = self.root.traverse_mut()?;
		loop {
			match traverse.peek().key.borrow().cmp(k) {
				Ordering::Greater => traverse = traverse.turn_left().ok()?,

				Ordering::Less
					if traverse
						.peek_right_child()
						.map(|right| right.key.borrow() <= k)
						.unwrap_or(false) =>
				{
					traverse = traverse.turn_right().unwrap()
				},

				_ => return Some(traverse.into_content().as_tuple_mut())
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::AATreeMap;

	#[test]
	fn test_first_key_value() {
		let mut map = AATreeMap::new();
		map.insert(10, "a");
		map.insert(20, "b");
		map.insert(30, "c");
		map.insert(40, "d");
		map.insert(50, "e");
		map.insert(60, "f");
		map.insert(70, "g");

		// The tree now looks like this:
		//       40
		//     /    \
		//   20      60
		//   /\      /\
		// 10  30  50  70

		// To return the correct value for 15, we need to go left once but not twice
		let (key, value) = map.first_key_value_at_or_after(&15).unwrap();
		assert_eq!(*key, 20);
		assert_eq!(*value, "b");
		let (key, value) = map.first_key_value_mut_at_or_after(&15).unwrap();
		assert_eq!(*key, 20);
		assert_eq!(*value, "b");
	}

	#[test]
	fn test_last_key_value() {
		let mut map = AATreeMap::new();
		map.insert(10, "a");
		map.insert(20, "b");
		map.insert(30, "c");
		map.insert(40, "d");

		// The tree now looks like this:
		//    20 - 30
		//   /       \
		// 10         40

		// To return the correct value for 35, we need to go right once but not twice
		let (key, value) = map.last_key_value_at_or_before(&35).unwrap();
		assert_eq!(*key, 30);
		assert_eq!(*value, "c");
		let (key, value) = map.last_key_value_mut_at_or_before(&35).unwrap();
		assert_eq!(*key, 30);
		assert_eq!(*value, "c");
	}
}
