use heterogeneous_hash_map::{
    HomogeneousHashMap,
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

fn bench_homogeneous_hash_map_insert(c: &mut Criterion) {
    c.bench_function("homogeneous_hash_map_insert", |b| {
        b.iter(|| {
            let keys = (0..1000).map(Key::new);
            let values = 1..1001;
            let mut map = HomogeneousHashMap::<i32, i32>::new();
            for (key, value) in keys.zip(values) {
                map.insert(key, value);
            }

            map
        });
    });
}

criterion_group!(bench_insert, bench_homogeneous_hash_map_insert, bench_hash_map_insert);
