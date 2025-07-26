use heterogeneous_hash_map::{HomogeneousHashMap, Key};

use criterion::{
    Criterion,
    criterion_group,
};

use std::hash;
use std::vec::Vec;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque::allocator_api::alloc;

use hashbrown::hash_map;

fn bench_hash_map_remove(c: &mut Criterion) {
    let keys = (0..1000).map(Key::new);
    let values = 1..1001;
    let map = HomogeneousHashMap::<i32, i32>::from_iter(keys.zip(values));
    let keys: Vec<Key<i32, i32>> = map.keys().cloned().collect();

    c.bench_function("hash_map_remove", |b| {
        b.iter_batched(
            || map.clone(),
            |mut map| {
                for key in keys.iter() {
                    let _ = core::hint::black_box(map.remove(key));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_homogeneous_hash_map_remove(c: &mut Criterion) {
    let keys = (0..1000).map(Key::new);
    let values = 1..1001;

    c.bench_function("homogeneous_hash_map_remove", |b| {
        b.iter_batched(
            || HomogeneousHashMap::<i32, i32>::from_iter(keys.clone().zip(values.clone())),
            |mut map| {
                let keys: Vec<Key<i32, i32>> = map.keys().cloned().collect();
                for key in keys.iter() {
                    let _ = core::hint::black_box(map.remove(key));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(
    bench_remove,
    bench_homogeneous_hash_map_remove,
    bench_hash_map_remove
);
