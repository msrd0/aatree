//! This file defines the [`Entry`] type that is used by [`AATreeMap`](crate::AATreeMap).

use core::borrow::Borrow;

/// An entry in an [`AATreeMap`](crate::AATreeMap). This type is used with iterators returned
/// by [`AATreeMap`](crate::AATreeMap).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive] // prevent initialization
pub struct Entry<K, V> {
	pub key: K,
	pub value: V
}

impl<K, V> Entry<K, V> {
	pub(super) fn as_tuple(&self) -> (&K, &V) {
		(&self.key, &self.value)
	}

	pub(super) fn into_tuple(self) -> (K, V) {
		(self.key, self.value)
	}
}

impl<K, V> Borrow<K> for Entry<K, V> {
	fn borrow(&self) -> &K {
		&self.key
	}
}

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
		self.key.partial_cmp(other)
	}
}

impl<K: Ord, V> Ord for Entry<K, V> {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		self.key.cmp(&other.key)
	}
}
