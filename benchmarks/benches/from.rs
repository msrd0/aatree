use aatree::AATreeSet;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{collections::BTreeSet, time::Duration};

macro_rules! benchmark {
	($ty:ty, $amount:expr, from) => {
		paste::item! {
			fn [<$ty:lower _from_ $amount>]() -> $ty<u64> {
				let vec: Vec<_> = (0..$amount).collect();
				$ty::from(vec)
			}
		}
	};
	($ty:ty, $amount:expr, collect) => {
		paste::item! {
			fn [<$ty:lower _collect_ $amount>]() -> $ty<u64> {
				let vec: Vec<_> = (0..$amount).collect();
				vec.into_iter().collect()
			}
		}
	};
	($group:literal = [$(($name:literal: $ty:ty, $amount:expr, $fn:ident)),+]) => {
		$(benchmark!($ty, $amount, $fn);)+
		paste::item! {
			fn [<bench_ $group:lower>](c: &mut Criterion) {
				let mut g = c.benchmark_group($group);
				g.sample_size(150).measurement_time(Duration::from_secs(20));
				$(g.bench_function(
					BenchmarkId::new(concat!($name, "_", stringify!($fn)), $amount),
					|b| b.iter([<$ty:lower _ $fn _ $amount>])
				);)+
				g.finish();
			}
		}
	};
}

benchmark!(
	"From" = [
		("AATree": AATreeSet, 10000, from),
		("AATree": AATreeSet, 10000, collect),
		("AATree": AATreeSet, 100000, from),
		("AATree": AATreeSet, 100000, collect),
		("BTree": BTreeSet, 10000, collect),
		("BTree": BTreeSet, 100000, collect)
	]
);

criterion_group!(benches, bench_from);
criterion_main!(benches);
