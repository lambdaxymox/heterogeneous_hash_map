use heterogeneous_hash_map::{
    HeterogeneousHashMap,
    Key,
};

use criterion::{
    Criterion,
    criterion_group,
};

use std::hash;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque::allocator_api::alloc;

use hashbrown::hash_map;

fn bench_hash_map_insert(c: &mut Criterion) {
    c.bench_function("hash_map_insert", |b| {
        b.iter(|| {
            let keys = (0..1000).map(Key::new);
            let values = 1..1001;
            let mut map = hash_map::HashMap::<Key<i32, i32>, i32>::new();
            for (key, value) in keys.zip(values) {
                map.insert(key, value);
            }

            map
        });
    });
}

fn bench_heterogeneous_hash_map_insert(c: &mut Criterion) {
    c.bench_function("heterogeneous_hash_map_insert", |b| {
        b.iter(|| {
            let keys = (0..1000).map(Key::new);
            let values = 1..1001;
            let mut het_map = HeterogeneousHashMap::new();
            het_map.insert_type::<i32>();
            for (key, value) in keys.zip(values) {
                het_map.insert(key, value);
            }

            het_map
        });
    });
}

criterion_group!(bench_insert, bench_heterogeneous_hash_map_insert, bench_hash_map_insert);
