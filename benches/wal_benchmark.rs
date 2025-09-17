use SDB::lsm;
use criterion::{criterion_group, criterion_main, Criterion};

fn wal_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("wals");
    group.bench_function("wal_replay", |b| {
        b.iter(|| lsm::WAL::replay("./data/wal-000000.log"));
    });

    group.bench_function("wal_replay_two", |b| {
        b.iter(|| lsm::WAL::replay("./data"));
    });

    group.finish();
}

criterion_group!(benches, wal_benchmark);
criterion_main!(benches);