use aatree::AATreeSet;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{collections::BTreeSet, time::Duration};

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
		}
	};
	($group:literal = [$(($name:literal: $ty:ty, $amount:expr, $order:ident)),+]) => {
		$(benchmark!($ty, $amount, $order);)+
		paste::item! {
			fn [<bench_ $group:lower>](c: &mut Criterion) {
				let mut g = c.benchmark_group($group);
				g.sample_size(150).measurement_time(Duration::from_secs(20));
				$(g.bench_function(BenchmarkId::new(format!("{}_{}", $name, stringify!($order)), $amount), |b| b.iter([<$ty:lower _insert_ $amount _ $order>]));)+
				g.finish();
			}
		}
	};
}

benchmark!(
	"Insert" = [
		("AATree": AATreeSet, 10000, asc),
		("AATree": AATreeSet, 10000, desc),
		("AATree": AATreeSet, 100000, asc),
		("AATree": AATreeSet, 100000, desc),
		("BTree": BTreeSet, 10000, asc),
		("BTree": BTreeSet, 10000, desc),
		("BTree": BTreeSet, 100000, asc),
		("BTree": BTreeSet, 100000, desc)
	]
);

criterion_group!(benches, bench_insert);
criterion_main!(benches);
