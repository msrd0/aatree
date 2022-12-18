//! This file defines the [`Entry`] type that is used by [`AATreeMap`](crate::AATreeMap).

use crate::iter::IterContent;
use core::{borrow::Borrow, cmp::Ordering};

/// An entry in an [`AATreeMap`](crate::AATreeMap). This type is used with iterators
/// returned by [`AATreeMap`](crate::AATreeMap).
// public but inaccessible
#[derive(Clone, Copy, Debug)]
pub struct KeyValue<K, V> {
	pub key: K,
	pub value: V
}

impl<K, V> KeyValue<K, V> {
	pub(super) fn as_tuple(&self) -> (&K, &V) {
		(&self.key, &self.value)
	}

	pub(super) fn into_tuple(self) -> (K, V) {
		(self.key, self.value)
	}
}

impl<K, V> IterContent<(K, V)> for KeyValue<K, V> {
	fn content(self) -> (K, V) {
		self.into_tuple()
	}
}

impl<'a, K, V> IterContent<(&'a K, &'a V)> for &'a KeyValue<K, V> {
	fn content(self) -> (&'a K, &'a V) {
		self.as_tuple()
	}
}

impl<K, V> Borrow<K> for KeyValue<K, V> {
	fn borrow(&self) -> &K {
		&self.key
	}
}

impl<K: PartialEq, V> PartialEq for KeyValue<K, V> {
	fn eq(&self, other: &Self) -> bool {
		self.key.eq(&other.key)
	}
}

impl<K: PartialEq, V> PartialEq<K> for KeyValue<K, V> {
	fn eq(&self, other: &K) -> bool {
		self.key.eq(other)
	}
}

impl<K: Eq, V> Eq for KeyValue<K, V> {}

impl<K: PartialOrd, V> PartialOrd for KeyValue<K, V> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.key.partial_cmp(&other.key)
	}
}

impl<K: PartialOrd, V> PartialOrd<K> for KeyValue<K, V> {
	fn partial_cmp(&self, other: &K) -> Option<Ordering> {
		self.key.partial_cmp(other)
	}
}

impl<K: Ord, V> Ord for KeyValue<K, V> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.key.cmp(&other.key)
	}
}
