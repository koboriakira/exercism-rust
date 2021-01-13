use criterion::{criterion_group, criterion_main, Criterion};
use sublist::{sublist, Comparison};

fn huge_sublist_not_in_huge_list() {
    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();

    assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("huge_sublist_not_in_huge_list", |b| {
        b.iter(|| huge_sublist_not_in_huge_list())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
