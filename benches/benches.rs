use uuidmap::Table;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn add_benchmark(c: &mut Criterion) {
    let mut table: Table<i32> = Table::default();
    c.bench_function("add", |b| b.iter(|| table.add(black_box(42))));
}

fn add_with_key_benchmark(c: &mut Criterion) {
    let mut table: Table<i32> = Table::default();
    c.bench_function("add_with_key", |b| {
        b.iter(|| table.add_with_key(black_box(123), black_box(42)))
    });
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

fn values_benchmark(c: &mut Criterion) {
    let mut table: Table<i32> = Table::default();
    table.add(42);
    table.add(24);
    c.bench_function("values", |b| b.iter(|| table.values().collect::<Vec<_>>()));
}

struct A(pub f32);
struct B(pub f32, pub u128);
fn join_benchmark(c: &mut Criterion) {
    let mut a_table: Table<A> = Table::default();
    let mut b_table: Table<B> = Table::default();
    (0..10000).for_each(|i| {
        let a_key = a_table.add(A(1.0));
        if i % 2 == 0 {
            b_table.add(B(1.0, a_key));
        }
    });
    c.bench_function("join", |b| {
        b.iter(|| {
            b_table.values().for_each(|b_val| {
                // unsafe: the key in B pointing to A must exist and be valid. saves 26%
                unsafe {
                    a_table.get_mut(b_val.1).unwrap_unchecked().0 += b_val.0;
                }
            });
        })
    });
}

struct Entity(pub u128, pub Option<u128>);
struct C(pub f32);
struct D(pub f32);
fn ecs_like_benchmark(c: &mut Criterion) {
    let mut entity_table: Table<Entity> = Table::default();
    let mut c_table: Table<C> = Table::default();
    let mut d_table: Table<D> = Table::default();
    (0..10000).for_each(|i| {
        let c_key = c_table.add(C(1.0));
        let d_key = if i % 2 == 0 {
            Some(d_table.add(D(1.0)))
        } else {
            None
        };
        entity_table.add(Entity(c_key, d_key));
    });
    c.bench_function("ecs_like", |b| {
        b.iter(|| {
            entity_table.values().for_each(|entity| {
                if let Some(d_key) = entity.1 {
                    // unsafe: the key in Entity pointing to C and D must exist and be valid. saves 33%.
                    unsafe {
                        c_table.get_mut(entity.0).unwrap_unchecked().0 +=
                            d_table.get(d_key).unwrap_unchecked().0;
                    }
                }
            });
        })
    });
}

criterion_group!(
    benches,
    add_benchmark,
    add_with_key_benchmark,
    remove_benchmark,
    get_benchmark,
    values_benchmark,
    join_benchmark,
    ecs_like_benchmark,
);

criterion_main!(benches);
