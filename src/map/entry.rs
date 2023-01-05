use super::{AATreeMap, KeyValue};
use core::{
	fmt::{self, Debug, Formatter},
	mem
};

pub enum Entry<'a, K, V> {
	Vacant(VacantEntry<'a, K, V>),
	Occupied(OccupiedEntry<'a, K, V>)
}

impl<K: Debug, V: Debug> Debug for Entry<'_, K, V> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Vacant(v) => f.debug_tuple("Entry").field(v).finish(),
			Self::Occupied(o) => f.debug_tuple("Entry").field(o).finish()
		}
	}
}

impl<'a, K, V> Entry<'a, K, V> {
	pub fn key(&self) -> &K {
		match self {
			Self::Vacant(entry) => entry.key(),
			Self::Occupied(entry) => entry.key()
		}
	}

	pub fn and_modify<F>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut V)
	{
		if let Self::Occupied(entry) = &mut self {
			f(entry.get_mut());
		}
		self
	}

	pub fn or_insert(self, default: V) -> &'a mut V
	where
		K: Ord + Clone
	{
		// TODO remove K: Clone once this doesn't require it anymore
		self.or_insert_with(|| default)
	}

	pub fn or_insert_with<F>(self, default: F) -> &'a mut V
	where
		F: FnOnce() -> V,
		K: Ord + Clone
	{
		// TODO remove K: Clone once this doesn't require it anymore
		self.or_insert_with_key(|_| default())
	}

	pub fn or_insert_with_key<F>(self, default: F) -> &'a mut V
	where
		F: FnOnce(&K) -> V,
		K: Ord + Clone
	{
		match self {
			Self::Occupied(entry) => entry.into_mut(),
			// TODO remove K: Clone once this doesn't require it anymore
			Self::Vacant(entry) => {
				let value = default(entry.key());
				entry.insert(value)
			}
		}
	}

	pub fn or_default(self) -> &'a mut V
	where
		K: Ord + Clone,
		V: Default
	{
		// TODO remove K: Clone once this doesn't require it anymore
		self.or_insert_with(V::default)
	}
}

pub struct OccupiedEntry<'a, K, V> {
	pub(crate) entry: &'a mut KeyValue<K, V>
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
	pub fn key(&self) -> &K {
		&self.entry.key
	}

	pub fn get(&self) -> &V {
		&self.entry.value
	}

	pub fn get_mut(&mut self) -> &mut V {
		&mut self.entry.value
	}

	pub fn into_mut(self) -> &'a mut V {
		&mut self.entry.value
	}

	pub fn insert(&mut self, value: V) -> V {
		mem::replace(&mut self.entry.value, value)
	}
}

impl<K: Debug, V: Debug> Debug for OccupiedEntry<'_, K, V> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("OccupiedEntry")
			.field("key", self.key())
			.field("value", self.get())
			.finish()
	}
}

pub struct VacantEntry<'a, K, V> {
	pub(crate) key: K,
	pub(crate) map: &'a mut AATreeMap<K, V>
}

impl<'a, K, V> VacantEntry<'a, K, V> {
	pub fn key(&self) -> &K {
		&self.key
	}

	pub fn into_key(self) -> K {
		self.key
	}

	pub fn insert(self, value: V) -> &'a mut V
	where
		K: Ord + Clone
	{
		// TODO properly return a reference and stop cloning the key
		self.map.insert(self.key.clone(), value);
		self.map.get_mut(&self.key).unwrap()
	}
}

impl<K: Debug, V: Debug> Debug for VacantEntry<'_, K, V> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("VacantEntry")
			.field("key", self.key())
			.finish()
	}
}
