#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![no_std]
extern crate std;

mod bench_homogeneous_hash_map;
mod bench_heterogeneous_hash_map;

use criterion::criterion_main;

criterion_main!(
    bench_homogeneous_hash_map::bench_get,
    bench_homogeneous_hash_map::bench_get_key_value,
    bench_homogeneous_hash_map::bench_insert,
    bench_homogeneous_hash_map::bench_remove,
    bench_heterogeneous_hash_map::bench_get,
    bench_heterogeneous_hash_map::bench_get_key_value,
    bench_heterogeneous_hash_map::bench_insert,
    bench_heterogeneous_hash_map::bench_remove,
);
