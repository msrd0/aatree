use aatree::AATreeSet;
use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::BTreeSet;

#[derive(Default)]
struct Vec<T>(std::vec::Vec<T>);
impl<T> Vec<T> {
	fn insert(&mut self, elem: T) {
		self.0.push(elem);
	}
}

macro_rules! benchmark {
	($ty:ty, $amount:expr, asc) => {
		benchmark!(@internal, $ty, $amount, 0..$amount, asc);
	};
	($ty:ty, $amount:expr, desc) => {
		benchmark!(@internal, $ty, $amount, (0..$amount).rev(), desc);
	};
	(@internal, $ty:ty, $amount:expr, $iter:expr, $order:ident) => {
		paste::item! {
			fn [<$ty:lower _insert_ $amount _ $order>]() -> $ty<u64> {
				let mut container = $ty::default();
				for i in $iter {
					container.insert(i);
				}
				container
			}
			fn [<bench_ $ty:lower _insert_ $amount _ $order>](c: &mut Criterion) {
				c.bench_function(stringify!([<$ty:lower _insert_ $amount _ $order>]), |b| b.iter([<$ty:lower _insert_ $amount _ $order>]));
			}
		}
	};
}

benchmark!(AATreeSet, 1000, asc);
benchmark!(AATreeSet, 1000, desc);
benchmark!(BTreeSet, 1000, asc);
benchmark!(BTreeSet, 1000, desc);
benchmark!(Vec, 1000, asc);

benchmark!(AATreeSet, 100000, asc);
benchmark!(AATreeSet, 100000, desc);
benchmark!(BTreeSet, 100000, asc);
benchmark!(BTreeSet, 100000, desc);
benchmark!(Vec, 100000, asc);

fn criterion_benchmark(c: &mut Criterion) {
	bench_aatreeset_insert_1000_asc(c);
	bench_aatreeset_insert_1000_desc(c);
	bench_btreeset_insert_1000_asc(c);
	bench_btreeset_insert_1000_desc(c);
	bench_vec_insert_1000_asc(c);

	bench_aatreeset_insert_100000_asc(c);
	bench_aatreeset_insert_100000_desc(c);
	bench_btreeset_insert_100000_asc(c);
	bench_btreeset_insert_100000_desc(c);
	bench_vec_insert_100000_asc(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
