use aatree::AATreeSet;
use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::BTreeSet;

macro_rules! benchmark {
	($ty:ty, $amount:expr, hit) => {
		benchmark!(@internal, $ty, $amount, (0..$amount).map(|x| x*2), (0..$amount).map(|x| x*2), hit);
	};
	($ty:ty, $amount:expr, miss) => {
		benchmark!(@internal, $ty, $amount, (0..$amount).map(|x| x*2), (0..$amount).map(|x| x*2+1), miss);
	};
	(@internal, $ty:ty, $amount:expr, $iter_fill:expr, $iter_test:expr, $success:ident) => {
		paste::item! {
			fn [<$ty:lower _contains_ $amount _ $success>](container: &$ty<u64>, test: &[u64]) {
				for i in test {
					criterion::black_box(container.contains(i));
				}
			}
			fn [<bench_ $ty:lower _contains_ $amount _ $success>](c: &mut Criterion) {
				let container: $ty<u64> = $iter_fill.collect();
				let test: Vec<u64> = $iter_test.collect();
				c.bench_function(stringify!([<$ty:lower _contains_ $amount _ $success>]), |b| b.iter(|| [<$ty:lower _contains_ $amount _ $success>](&container, &test)));
			}
		}
	};
}

benchmark!(AATreeSet, 1000, hit);
benchmark!(AATreeSet, 1000, miss);
benchmark!(BTreeSet, 1000, hit);
benchmark!(BTreeSet, 1000, miss);
benchmark!(Vec, 1000, hit);
benchmark!(Vec, 1000, miss);

benchmark!(AATreeSet, 100000, hit);
benchmark!(AATreeSet, 100000, miss);
benchmark!(BTreeSet, 100000, hit);
benchmark!(BTreeSet, 100000, miss);

fn criterion_benchmark(c: &mut Criterion) {
	bench_aatreeset_contains_1000_hit(c);
	bench_aatreeset_contains_1000_miss(c);
	bench_btreeset_contains_1000_hit(c);
	bench_btreeset_contains_1000_miss(c);
	bench_vec_contains_1000_hit(c);
	bench_vec_contains_1000_miss(c);

	bench_aatreeset_contains_100000_hit(c);
	bench_aatreeset_contains_100000_miss(c);
	bench_btreeset_contains_100000_hit(c);
	bench_btreeset_contains_100000_miss(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
