use crate::{AATreeMap, AATreeSet};
use core::{
	fmt::{self, Formatter},
	marker::PhantomData
};
use serde::{
	de::{self, Deserialize, Deserializer},
	ser::{Serialize, Serializer}
};

// ### Deserialize AATreeSet

struct AATreeSetVisitor<T>(PhantomData<T>);

impl<'de, T> de::Visitor<'de> for AATreeSetVisitor<T>
where
	T: Deserialize<'de> + Ord
{
	type Value = AATreeSet<T>;

	fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("a set")
	}

	fn visit_seq<A>(self, mut acc: A) -> Result<Self::Value, A::Error>
	where
		A: de::SeqAccess<'de>
	{
		let mut set = AATreeSet::new();
		while let Some(next) = acc.next_element()? {
			set.insert(next);
		}
		Ok(set)
	}
}

impl<'de, T> Deserialize<'de> for AATreeSet<T>
where
	T: Deserialize<'de> + Ord
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		deserializer.deserialize_seq(AATreeSetVisitor(PhantomData))
	}
}

// ### Deserialize AATreeMap

struct AATreeMapVisitor<K, V>(PhantomData<(K, V)>);

impl<'de, K, V> de::Visitor<'de> for AATreeMapVisitor<K, V>
where
	K: Deserialize<'de> + Ord,
	V: Deserialize<'de>
{
	type Value = AATreeMap<K, V>;

	fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("a map")
	}

	fn visit_map<A>(self, mut acc: A) -> Result<Self::Value, A::Error>
	where
		A: de::MapAccess<'de>
	{
		let mut map = AATreeMap::new();
		while let Some((key, value)) = acc.next_entry()? {
			map.insert(key, value);
		}
		Ok(map)
	}
}

impl<'de, K, V> Deserialize<'de> for AATreeMap<K, V>
where
	K: Deserialize<'de> + Ord,
	V: Deserialize<'de>
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		deserializer.deserialize_map(AATreeMapVisitor(PhantomData))
	}
}

// ### Serialize AATreeSet

impl<T> Serialize for AATreeSet<T>
where
	T: Serialize
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		serializer.collect_seq(self)
	}
}

// ### Serialize AATreeMap

impl<K, V> Serialize for AATreeMap<K, V>
where
	K: Serialize,
	V: Serialize
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		serializer.collect_map(self)
	}
}

#[cfg(test)]
mod tests {
	use crate::{AATreeMap, AATreeSet};
	use core::fmt::Debug;
	use serde::{de::DeserializeOwned, Serialize};

	#[track_caller]
	fn test<T>(value: &T, json: &str)
	where
		T: DeserializeOwned + Serialize + Debug + PartialEq
	{
		assert_eq!(serde_json::to_string(value).unwrap(), json);
		assert_eq!(&serde_json::from_str::<T>(json).unwrap(), value);
	}

	#[test]
	fn test_set() {
		let mut set = AATreeSet::new();
		test(&set, "[]");

		set.insert(5);
		test(&set, "[5]");

		set.insert(7);
		test(&set, "[5,7]");

		set.insert(6);
		test(&set, "[5,6,7]");
	}

	#[test]
	fn test_map() {
		let mut map = AATreeMap::new();
		test(&map, "{}");

		map.insert(5, 50);
		test(&map, r#"{"5":50}"#);

		map.insert(7, 70);
		test(&map, r#"{"5":50,"7":70}"#);

		map.insert(6, 60);
		test(&map, r#"{"5":50,"6":60,"7":70}"#);
	}
}
