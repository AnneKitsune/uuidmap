use criterion::{black_box, criterion_group, criterion_main, Criterion};
use arrayhashmap::Table;

fn add_benchmark(c: &mut Criterion) {
    let mut table: Table<i32> = Table::default();
    c.bench_function("add", |b| b.iter(|| table.add(black_box(42))));
}

fn add_with_key_benchmark(c: &mut Criterion) {
    let mut table: Table<i32> = Table::default();
    c.bench_function("add_with_key", |b| b.iter(|| table.add_with_key(black_box(123), black_box(42))));
}

fn remove_benchmark(c: &mut Criterion) {
    let mut table: Table<i32> = Table::default();
    let key = table.add(42);
    c.bench_function("remove", |b| b.iter(|| table.remove(black_box(key))));
}

fn get_benchmark(c: &mut Criterion) {
    let mut table: Table<i32> = Table::default();
    let key = table.add(42);
    c.bench_function("get", |b| b.iter(|| table.get(black_box(key))));
}

fn count_benchmark(c: &mut Criterion) {
    let mut table: Table<i32> = Table::default();
    c.bench_function("count", |b| b.iter(|| table.count()));
}

fn values_benchmark(c: &mut Criterion) {
    let mut table: Table<i32> = Table::default();
    table.add(42);
    table.add(24);
    c.bench_function("values", |b| b.iter(|| table.values().collect::<Vec<_>>()));
}

criterion_group!(
    benches,
    add_benchmark,
    add_with_key_benchmark,
    remove_benchmark,
    get_benchmark,
    count_benchmark,
    values_benchmark
);

criterion_main!(benches);
