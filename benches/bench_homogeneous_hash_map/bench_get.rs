use heterogeneous_hash_map::{HomogeneousHashMap, Key};

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

fn bench_hash_map_get(c: &mut Criterion) {
    let keys = (0..1000).map(Key::new);
    let values = 1..1001;
    let map = hash_map::HashMap::<Key<i32, i32>, i32, hash::RandomState>::from_iter(keys.zip(values));

    c.bench_function("hash_map_get", |b| {
        b.iter(|| {
            for key in map.keys() {
                let _ = core::hint::black_box(map.get(key));
            }
        });
    });
}

fn bench_homogeneous_hash_map_get(c: &mut Criterion) {
    let keys = (0..1000).map(Key::new);
    let values = 1..1001;
    let map = HomogeneousHashMap::<i32, i32, hash::RandomState>::from_iter(keys.zip(values));

    c.bench_function("homogeneous_hash_map_get", |b| {
        b.iter(|| {
            for key in map.keys() {
                let _ = core::hint::black_box(map.get(key));
            }
        });
    });
}

criterion_group!(bench_get, bench_homogeneous_hash_map_get, bench_hash_map_get);
